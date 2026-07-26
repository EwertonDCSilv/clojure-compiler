//! Reader de `clojure-native`: texto-fonte → `Vec<SForm>` com spans.
//!
//! Escopo (specs/LANGUAGE_SCOPE.md — Reader, coluna MVP): números `i64`/`f64`,
//! strings, chars, símbolos, keywords, listas/vetores/mapas/sets, metadata `^`,
//! `quote '`, `deref @`, `var #'`, discard `#_`, anon-fn `#(...)`.
//! Fora do MVP inicial (erro diagnóstico claro): syntax-quote `` ` ``, unquote,
//! reader conditionals `#?`, regex `#"..."`, ratios, bignum/bigdec, `::kw`.

use clojure_diagnostics::{Diagnostic, Diagnostics};
use clojure_span::{Span, SourceId, Spanned};
use clojure_syntax::{Form, Name, SForm};

/// Lê todas as forms de nível superior de uma fonte.
pub fn read_all(src: SourceId, text: &str) -> Result<Vec<SForm>, Diagnostics> {
    let mut r = Reader::new(src, text);
    let mut forms = Vec::new();
    let mut diags = Diagnostics::new();
    loop {
        if let Err(d) = r.skip_trivia() {
            diags.push(d);
            break;
        }
        if r.at_eof() {
            break;
        }
        match r.read_form() {
            Ok(f) => forms.push(f),
            Err(d) => {
                diags.push(d);
                break; // recuperação simples: para no primeiro erro estrutural
            }
        }
    }
    if diags.is_empty() {
        Ok(forms)
    } else {
        Err(diags)
    }
}

struct Reader<'a> {
    src: SourceId,
    /// (offset de byte, char) para cada caractere; permite spans exatos em UTF-8.
    chars: Vec<(u32, char)>,
    /// Comprimento em bytes da fonte (offset do EOF).
    len: u32,
    pos: usize,
    #[allow(dead_code)]
    text: &'a str,
}

impl<'a> Reader<'a> {
    fn new(src: SourceId, text: &'a str) -> Self {
        let chars: Vec<(u32, char)> = text.char_indices().map(|(i, c)| (i as u32, c)).collect();
        Reader { src, chars, len: text.len() as u32, pos: 0, text }
    }

    fn at_eof(&self) -> bool {
        self.pos >= self.chars.len()
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).map(|&(_, c)| c)
    }

    fn peek2(&self) -> Option<char> {
        self.chars.get(self.pos + 1).map(|&(_, c)| c)
    }

    /// Offset de byte da posição atual (ou EOF).
    fn offset(&self) -> u32 {
        self.chars.get(self.pos).map(|&(o, _)| o).unwrap_or(self.len)
    }

    fn bump(&mut self) -> Option<char> {
        let c = self.peek();
        if c.is_some() {
            self.pos += 1;
        }
        c
    }

    fn span_from(&self, start: u32) -> Span {
        Span::new(self.src, start, self.offset())
    }

    fn err(&self, code: &'static str, msg: impl Into<String>, span: Span) -> Diagnostic {
        Diagnostic::error(code, msg).with_span(span)
    }

    /// Pula whitespace, vírgulas, comentários `;`/shebang `#!` e formas descartadas `#_`.
    fn skip_trivia(&mut self) -> Result<(), Diagnostic> {
        loop {
            match self.peek() {
                Some(c) if c.is_whitespace() || c == ',' => {
                    self.bump();
                }
                Some(';') => {
                    while let Some(c) = self.peek() {
                        self.bump();
                        if c == '\n' {
                            break;
                        }
                    }
                }
                Some('#') if self.peek2() == Some('!') => {
                    // shebang: trata a linha como comentário
                    while let Some(c) = self.peek() {
                        self.bump();
                        if c == '\n' {
                            break;
                        }
                    }
                }
                Some('#') if self.peek2() == Some('_') => {
                    // discard: consome `#_` e a próxima form (que é ignorada)
                    self.bump();
                    self.bump();
                    self.read_form()?;
                }
                _ => return Ok(()),
            }
        }
    }

    fn read_form(&mut self) -> Result<SForm, Diagnostic> {
        self.skip_trivia()?;
        let start = self.offset();
        let c = match self.peek() {
            Some(c) => c,
            None => return Err(self.err("E0002", "fim de arquivo inesperado", self.span_from(start))),
        };

        match c {
            '(' => self.read_coll(start, '(', ')').map(|(items, span)| Spanned::new(Form::List(items), span)),
            '[' => self.read_coll(start, '[', ']').map(|(items, span)| Spanned::new(Form::Vector(items), span)),
            '{' => self.read_map(start),
            ')' | ']' | '}' => {
                self.bump();
                Err(self.err("E0003", format!("delimitador inesperado `{c}`"), self.span_from(start)))
            }
            '"' => self.read_string(start),
            '\\' => self.read_char(start),
            ':' => self.read_keyword(start),
            '\'' => self.read_wrapper(start, "quote"),
            '@' => self.read_wrapper(start, "deref"),
            '^' => self.read_meta(start),
            '`' => {
                self.bump();
                Err(self
                    .err("E0010", "syntax-quote (`) ainda não é suportado", self.span_from(start))
                    .with_help("fora do escopo do MVP inicial; ver specs/LANGUAGE_SCOPE.md"))
            }
            '~' => {
                self.bump();
                if self.peek() == Some('@') {
                    self.bump();
                }
                Err(self
                    .err("E0011", "unquote (~/~@) fora de syntax-quote", self.span_from(start))
                    .with_help("só é válido dentro de `; não suportado no MVP inicial"))
            }
            '#' => self.read_dispatch(start),
            _ => self.read_atom(start),
        }
    }

    /// `'form` → `(quote form)`, `@form` → `(deref form)`.
    fn read_wrapper(&mut self, start: u32, op: &str) -> Result<SForm, Diagnostic> {
        self.bump(); // consome ' ou @
        let inner = self.read_form()?;
        let span = self.span_from(start);
        let op_form = Spanned::new(Form::sym(op), Span::point(self.src, start));
        Ok(Spanned::new(Form::List(vec![op_form, inner]), span))
    }

    fn read_meta(&mut self, start: u32) -> Result<SForm, Diagnostic> {
        self.bump(); // ^
        let meta = self.read_form()?;
        let form = self.read_form()?;
        let span = self.span_from(start);
        Ok(Spanned::new(
            Form::Meta { meta: Box::new(meta), form: Box::new(form) },
            span,
        ))
    }

    fn read_dispatch(&mut self, start: u32) -> Result<SForm, Diagnostic> {
        self.bump(); // consome '#'
        match self.peek() {
            Some('{') => {
                let (items, span) = self.read_coll(start, '{', '}')?;
                Ok(Spanned::new(Form::Set(items), span))
            }
            Some('\'') => {
                // #'x → (var x)
                self.bump();
                let inner = self.read_form()?;
                let span = self.span_from(start);
                let op = Spanned::new(Form::sym("var"), Span::point(self.src, start));
                Ok(Spanned::new(Form::List(vec![op, inner]), span))
            }
            Some('(') => self.read_anon_fn(start),
            Some('"') => Err(self
                .err("E0012", "literal de regex (#\"...\") não é suportado", self.span_from(start))
                .with_help("use a feature `regex` quando disponível; fora do MVP inicial")),
            Some('?') => Err(self
                .err("E0013", "reader conditional (#?) não é suportado no MVP inicial", self.span_from(start))),
            other => {
                let msg = match other {
                    Some(c) => format!("macro de leitura desconhecida `#{c}`"),
                    None => "`#` seguido de fim de arquivo".to_string(),
                };
                Err(self.err("E0014", msg, self.span_from(start)))
            }
        }
    }

    /// `#(...)` → `(fn* [%1 %2 ... %&?] (...))`, com `%` canonizado para `%1`.
    fn read_anon_fn(&mut self, start: u32) -> Result<SForm, Diagnostic> {
        let (body_items, span) = self.read_coll(start, '(', ')')?;
        let body = Spanned::new(Form::List(body_items), span);

        // Descobre a maior aridade posicional e se usa rest (`%&`).
        let mut max_pos = 0u32;
        let mut has_rest = false;
        collect_pct(&body, &mut max_pos, &mut has_rest);

        // Reescreve `%` → `%1` no corpo.
        let body = rewrite_pct(body);

        let mut params: Vec<SForm> = Vec::new();
        for i in 1..=max_pos {
            params.push(Spanned::new(Form::sym(&format!("%{i}")), Span::point(self.src, start)));
        }
        if has_rest {
            params.push(Spanned::new(Form::sym("&"), Span::point(self.src, start)));
            params.push(Spanned::new(Form::sym("%&"), Span::point(self.src, start)));
        }
        let params = Spanned::new(Form::Vector(params), Span::point(self.src, start));
        let fnsym = Spanned::new(Form::sym("fn*"), Span::point(self.src, start));
        let full = self.span_from(start);
        Ok(Spanned::new(Form::List(vec![fnsym, params, body]), full))
    }

    fn read_coll(
        &mut self,
        start: u32,
        open: char,
        close: char,
    ) -> Result<(Vec<SForm>, Span), Diagnostic> {
        debug_assert_eq!(self.peek(), Some(open));
        self.bump(); // consome delimitador de abertura
        let mut items = Vec::new();
        loop {
            self.skip_trivia()?;
            match self.peek() {
                None => {
                    return Err(self
                        .err("E0004", format!("`{open}` sem `{close}` correspondente"), self.span_from(start))
                        .with_help(format!("adicione `{close}`")));
                }
                Some(c) if c == close => {
                    self.bump();
                    return Ok((items, self.span_from(start)));
                }
                Some(c) if c == ')' || c == ']' || c == '}' => {
                    self.bump();
                    return Err(self.err(
                        "E0005",
                        format!("delimitador `{c}` não corresponde a `{open}`"),
                        self.span_from(start),
                    ));
                }
                _ => items.push(self.read_form()?),
            }
        }
    }

    fn read_map(&mut self, start: u32) -> Result<SForm, Diagnostic> {
        let (items, span) = self.read_coll(start, '{', '}')?;
        if items.len() % 2 != 0 {
            return Err(self
                .err("E0006", "mapa com número ímpar de formas", span)
                .with_help("mapas exigem pares chave/valor"));
        }
        let pairs = items
            .chunks_exact(2)
            .map(|c| (c[0].clone(), c[1].clone()))
            .collect();
        Ok(Spanned::new(Form::Map(pairs), span))
    }

    fn read_string(&mut self, start: u32) -> Result<SForm, Diagnostic> {
        self.bump(); // "
        let mut s = String::new();
        loop {
            match self.bump() {
                None => {
                    return Err(self.err("E0007", "string sem aspas de fechamento", self.span_from(start)))
                }
                Some('"') => break,
                Some('\\') => {
                    let esc_start = self.offset();
                    match self.bump() {
                        Some('n') => s.push('\n'),
                        Some('t') => s.push('\t'),
                        Some('r') => s.push('\r'),
                        Some('\\') => s.push('\\'),
                        Some('"') => s.push('"'),
                        Some('0') => s.push('\0'),
                        Some('u') => {
                            let mut code = 0u32;
                            for _ in 0..4 {
                                match self.peek().and_then(|c| c.to_digit(16)) {
                                    Some(d) => {
                                        code = code * 16 + d;
                                        self.bump();
                                    }
                                    None => {
                                        return Err(self.err(
                                            "E0008",
                                            "escape unicode inválido (esperado \\uXXXX)",
                                            self.span_from(esc_start),
                                        ))
                                    }
                                }
                            }
                            match char::from_u32(code) {
                                Some(c) => s.push(c),
                                None => {
                                    return Err(self.err(
                                        "E0008",
                                        format!("code point unicode inválido: {code:#x}"),
                                        self.span_from(esc_start),
                                    ))
                                }
                            }
                        }
                        Some(other) => {
                            return Err(self.err(
                                "E0009",
                                format!("sequência de escape desconhecida `\\{other}`"),
                                self.span_from(esc_start),
                            ))
                        }
                        None => {
                            return Err(self.err("E0007", "string sem aspas de fechamento", self.span_from(start)))
                        }
                    }
                }
                Some(c) => s.push(c),
            }
        }
        Ok(Spanned::new(Form::Str(s), self.span_from(start)))
    }

    fn read_char(&mut self, start: u32) -> Result<SForm, Diagnostic> {
        self.bump(); // '\'
        // Primeiro caractere após a barra é obrigatório.
        let first = match self.bump() {
            Some(c) => c,
            None => return Err(self.err("E0015", "`\\` sem caractere", self.span_from(start))),
        };
        // Se for início de palavra, pode ser um char nomeado (newline, uXXXX, ...).
        if first.is_alphabetic() {
            let mut word = String::new();
            word.push(first);
            while let Some(c) = self.peek() {
                if c.is_alphanumeric() {
                    word.push(c);
                    self.bump();
                } else {
                    break;
                }
            }
            if word.chars().count() == 1 {
                return Ok(Spanned::new(Form::Char(first), self.span_from(start)));
            }
            let ch = match word.as_str() {
                "newline" => '\n',
                "tab" => '\t',
                "space" => ' ',
                "return" => '\r',
                "backspace" => '\u{8}',
                "formfeed" => '\u{c}',
                w if w.starts_with('u') && w.len() == 5 => {
                    match u32::from_str_radix(&w[1..], 16).ok().and_then(char::from_u32) {
                        Some(c) => c,
                        None => {
                            return Err(self.err(
                                "E0016",
                                format!("char unicode inválido `\\{w}`"),
                                self.span_from(start),
                            ))
                        }
                    }
                }
                w => {
                    return Err(self.err(
                        "E0016",
                        format!("nome de caractere desconhecido `\\{w}`"),
                        self.span_from(start),
                    ))
                }
            };
            Ok(Spanned::new(Form::Char(ch), self.span_from(start)))
        } else {
            Ok(Spanned::new(Form::Char(first), self.span_from(start)))
        }
    }

    fn read_keyword(&mut self, start: u32) -> Result<SForm, Diagnostic> {
        self.bump(); // ':'
        if self.peek() == Some(':') {
            self.bump();
            return Err(self
                .err("E0017", "keyword auto-resolvida (::) não suportada no MVP inicial", self.span_from(start))
                .with_help("use uma keyword qualificada explícita `:ns/nome`"));
        }
        let tok = self.read_token_str();
        if tok.is_empty() {
            return Err(self.err("E0018", "keyword vazia", self.span_from(start)));
        }
        let name = parse_name(&tok);
        Ok(Spanned::new(Form::Keyword(name), self.span_from(start)))
    }

    /// Lê um átomo: número, `nil`/`true`/`false`, ou símbolo.
    fn read_atom(&mut self, start: u32) -> Result<SForm, Diagnostic> {
        let tok = self.read_token_str();
        let span = self.span_from(start);
        if tok.is_empty() {
            let c = self.peek().unwrap_or('?');
            self.bump();
            return Err(self.err("E0019", format!("caractere inesperado `{c}`"), span));
        }
        let form = self.classify_token(&tok, span)?;
        Ok(Spanned::new(form, span))
    }

    fn classify_token(&self, tok: &str, span: Span) -> Result<Form, Diagnostic> {
        match tok {
            "nil" => return Ok(Form::Nil),
            "true" => return Ok(Form::Bool(true)),
            "false" => return Ok(Form::Bool(false)),
            "/" => return Ok(Form::Symbol(Name::simple("/"))),
            _ => {}
        }

        let bytes = tok.as_bytes();
        let first = bytes[0];
        let numeric_start = first.is_ascii_digit()
            || ((first == b'-' || first == b'+') && bytes.get(1).is_some_and(|b| b.is_ascii_digit()));

        if numeric_start {
            return self.parse_number(tok, span);
        }
        Ok(Form::Symbol(parse_name(tok)))
    }

    fn parse_number(&self, tok: &str, span: Span) -> Result<Form, Diagnostic> {
        if tok.contains('/') {
            return Err(self
                .err("E0020", format!("ratios não são suportados no MVP: `{tok}`"), span)
                .with_help("ver specs/LANGUAGE_SCOPE.md — Ratio é [FUTURO]"));
        }
        if tok.ends_with('N') || tok.ends_with('M') {
            return Err(self
                .err("E0021", format!("BigInt/BigDecimal não suportados no MVP: `{tok}`"), span)
                .with_help("remova o sufixo N/M; ver specs/LANGUAGE_SCOPE.md"));
        }
        if let Ok(n) = tok.parse::<i64>() {
            return Ok(Form::Int(n));
        }
        if let Ok(x) = tok.parse::<f64>() {
            return Ok(Form::Float(x));
        }
        Err(self.err("E0022", format!("número inválido: `{tok}`"), span))
    }

    fn read_token_str(&mut self) -> String {
        let mut s = String::new();
        while let Some(c) = self.peek() {
            if is_terminator(c) {
                break;
            }
            s.push(c);
            self.bump();
        }
        s
    }
}

fn is_terminator(c: char) -> bool {
    c.is_whitespace()
        || matches!(
            c,
            ',' | '(' | ')' | '[' | ']' | '{' | '}' | '"' | ';' | '@' | '^' | '`' | '\'' | '~' | '\\'
        )
}

/// Divide um token em `ns/name`. `/` só separa se não estiver na borda.
fn parse_name(tok: &str) -> Name {
    if let Some(idx) = tok.find('/') {
        if idx > 0 && idx < tok.len() - 1 {
            return Name::qualified(&tok[..idx], &tok[idx + 1..]);
        }
    }
    Name::simple(tok)
}

/// Coleta a maior posição `%N` e presença de `%&` no corpo de um `#(...)`.
fn collect_pct(f: &SForm, max_pos: &mut u32, has_rest: &mut bool) {
    match &f.node {
        Form::Symbol(n) if n.ns.is_none() => {
            let s = &n.name;
            if s == "%" {
                *max_pos = (*max_pos).max(1);
            } else if s == "%&" {
                *has_rest = true;
            } else if let Some(rest) = s.strip_prefix('%') {
                if let Ok(n) = rest.parse::<u32>() {
                    *max_pos = (*max_pos).max(n);
                }
            }
        }
        Form::List(v) | Form::Vector(v) | Form::Set(v) => {
            for it in v {
                collect_pct(it, max_pos, has_rest);
            }
        }
        Form::Map(pairs) => {
            for (k, val) in pairs {
                collect_pct(k, max_pos, has_rest);
                collect_pct(val, max_pos, has_rest);
            }
        }
        Form::Meta { meta, form } => {
            collect_pct(meta, max_pos, has_rest);
            collect_pct(form, max_pos, has_rest);
        }
        _ => {}
    }
}

/// Reescreve `%` → `%1` recursivamente.
fn rewrite_pct(f: SForm) -> SForm {
    let span = f.span;
    let node = match f.node {
        Form::Symbol(n) if n.ns.is_none() && n.name == "%" => Form::sym("%1"),
        Form::List(v) => Form::List(v.into_iter().map(rewrite_pct).collect()),
        Form::Vector(v) => Form::Vector(v.into_iter().map(rewrite_pct).collect()),
        Form::Set(v) => Form::Set(v.into_iter().map(rewrite_pct).collect()),
        Form::Map(pairs) => Form::Map(
            pairs
                .into_iter()
                .map(|(k, val)| (rewrite_pct(k), rewrite_pct(val)))
                .collect(),
        ),
        Form::Meta { meta, form } => Form::Meta {
            meta: Box::new(rewrite_pct(*meta)),
            form: Box::new(rewrite_pct(*form)),
        },
        other => other,
    };
    Spanned::new(node, span)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read1(text: &str) -> SForm {
        let forms = read_all(0, text).expect("deve ler");
        assert_eq!(forms.len(), 1, "esperava 1 form em {text:?}");
        forms.into_iter().next().unwrap()
    }

    fn dump(text: &str) -> String {
        read_all(0, text)
            .expect("deve ler")
            .iter()
            .map(|f| f.node.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    }

    #[test]
    fn atoms() {
        assert_eq!(read1("42").node, Form::Int(42));
        assert_eq!(read1("-7").node, Form::Int(-7));
        assert_eq!(read1("3.14").node, Form::Float(3.14));
        assert_eq!(read1("nil").node, Form::Nil);
        assert_eq!(read1("true").node, Form::Bool(true));
        assert_eq!(read1("false").node, Form::Bool(false));
        assert_eq!(read1("foo").node, Form::sym("foo"));
        assert_eq!(read1(":kw").node, Form::kw("kw"));
        assert_eq!(read1("a/b").node, Form::Symbol(Name::qualified("a", "b")));
    }

    #[test]
    fn strings_and_chars() {
        assert_eq!(read1(r#""hi\n""#).node, Form::Str("hi\n".into()));
        assert_eq!(read1(r#""A""#).node, Form::Str("A".into()));
        assert_eq!(read1(r"\newline").node, Form::Char('\n'));
        assert_eq!(read1(r"\a").node, Form::Char('a'));
        assert_eq!(read1(r"\A").node, Form::Char('A'));
    }

    #[test]
    fn collections() {
        assert_eq!(dump("(1 2 3)"), "(1 2 3)");
        assert_eq!(dump("[1 2 3]"), "[1 2 3]");
        assert_eq!(dump("#{1 2}"), "#{1 2}");
        assert_eq!(dump("{:a 1 :b 2}"), "{:a 1, :b 2}");
        assert_eq!(dump("(a [b {:c 1}])"), "(a [b {:c 1}])");
    }

    #[test]
    fn reader_macros_desugar() {
        assert_eq!(dump("'x"), "(quote x)");
        assert_eq!(dump("@x"), "(deref x)");
        assert_eq!(dump("#'x"), "(var x)");
        assert_eq!(dump("#(+ % 1)"), "(fn* [%1] (+ %1 1))");
        assert_eq!(dump("#(+ %1 %2)"), "(fn* [%1 %2] (+ %1 %2))");
    }

    #[test]
    fn discard_and_comments() {
        assert_eq!(dump("1 #_2 3"), "1\n3");
        assert_eq!(dump("; comment\n42"), "42");
        assert_eq!(dump("[1 #_2 3]"), "[1 3]");
    }

    #[test]
    fn metadata() {
        assert_eq!(dump("^:private x"), "^:private x");
        assert_eq!(dump("^{:a 1} x"), "^{:a 1} x");
    }

    #[test]
    fn errors_have_spans() {
        let e = read_all(0, "(1 2").unwrap_err();
        assert!(e.has_errors());
        assert_eq!(e.items[0].code, "E0004");
        assert!(e.items[0].span.is_some());
    }

    #[test]
    fn unsupported_are_diagnosed() {
        assert_eq!(read_all(0, "1/2").unwrap_err().items[0].code, "E0020");
        assert_eq!(read_all(0, "42N").unwrap_err().items[0].code, "E0021");
        assert_eq!(read_all(0, "`x").unwrap_err().items[0].code, "E0010");
    }
}
