//! Reads UTF-8 Clojure source into spanned structural forms.
//!
//! [`read_all`] tokenizes one source buffer and returns [`SForm`] values whose
//! [`Span`] offsets refer to that exact buffer. The reader recognizes numeric,
//! string, character, symbol, keyword, collection, metadata, quote, dereference,
//! var-quote, discard, and anonymous-function syntax. It performs no macro
//! expansion or name resolution.
//!
//! Unsupported reader features fail with stable diagnostics rather than being
//! accepted approximately. These currently include syntax quote, unquote,
//! reader conditionals, regex literals, ratios, arbitrary-precision numeric
//! literals, and auto-resolved keywords.

use clojure_diagnostics::{Diagnostic, Diagnostics};
use clojure_span::{SourceId, Span, Spanned};
use clojure_syntax::{Form, Name, SForm};

/// Reads every top-level form from `text`.
///
/// `src` is copied into every returned span and must identify the same text in
/// the caller's source map. Commas, whitespace, comments, shebang lines, and
/// discarded forms are treated as trivia.
///
/// # Errors
///
/// Returns the first lexical or structural error as a [`Diagnostics`]
/// collection. Successfully read forms before that error are not returned.
///
/// # Examples
///
/// ```
/// use clojure_reader::read_all;
/// use clojure_syntax::Form;
///
/// let forms = read_all(7, "(inc 1) :done").unwrap();
/// assert_eq!(forms.len(), 2);
/// assert!(matches!(forms[0].node, Form::List(_)));
/// assert_eq!(forms[0].span.source, 7);
/// ```
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
                break; // Recovery is intentionally limited to the first error.
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
    /// `(byte offset, scalar value)` pairs preserve exact UTF-8 spans.
    chars: Vec<(u32, char)>,
    /// Source length in bytes, also used as the EOF offset.
    len: u32,
    pos: usize,
    #[allow(dead_code)]
    text: &'a str,
}

impl<'a> Reader<'a> {
    fn new(src: SourceId, text: &'a str) -> Self {
        let chars: Vec<(u32, char)> = text.char_indices().map(|(i, c)| (i as u32, c)).collect();
        Reader {
            src,
            chars,
            len: text.len() as u32,
            pos: 0,
            text,
        }
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

    /// Returns the current byte offset, or the source length at EOF.
    fn offset(&self) -> u32 {
        self.chars
            .get(self.pos)
            .map(|&(o, _)| o)
            .unwrap_or(self.len)
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

    /// Skips whitespace, commas, comments, shebang lines, and `#_` forms.
    ///
    /// Discard is recursive because the ignored form is parsed with the normal
    /// reader, including nested discard macros.
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
                    // A shebang occupies the same trivia category as a comment.
                    while let Some(c) = self.peek() {
                        self.bump();
                        if c == '\n' {
                            break;
                        }
                    }
                }
                Some('#') if self.peek2() == Some('_') => {
                    // Parse the discarded form to preserve delimiter validation.
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
            None => {
                return Err(self.err("E0002", "fim de arquivo inesperado", self.span_from(start)))
            }
        };

        match c {
            '(' => self
                .read_coll(start, '(', ')')
                .map(|(items, span)| Spanned::new(Form::List(items), span)),
            '[' => self
                .read_coll(start, '[', ']')
                .map(|(items, span)| Spanned::new(Form::Vector(items), span)),
            '{' => self.read_map(start),
            ')' | ']' | '}' => {
                self.bump();
                Err(self.err(
                    "E0003",
                    format!("delimitador inesperado `{c}`"),
                    self.span_from(start),
                ))
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
                    .err(
                        "E0010",
                        "syntax-quote (`) ainda não é suportado",
                        self.span_from(start),
                    )
                    .with_help("fora do escopo do MVP inicial; ver specs/LANGUAGE_SCOPE.md"))
            }
            '~' => {
                self.bump();
                if self.peek() == Some('@') {
                    self.bump();
                }
                Err(self
                    .err(
                        "E0011",
                        "unquote (~/~@) fora de syntax-quote",
                        self.span_from(start),
                    )
                    .with_help("só é válido dentro de `; não suportado no MVP inicial"))
            }
            '#' => self.read_dispatch(start),
            _ => self.read_atom(start),
        }
    }

    /// Desugars `'form` to `(quote form)` and `@form` to `(deref form)`.
    fn read_wrapper(&mut self, start: u32, op: &str) -> Result<SForm, Diagnostic> {
        self.bump(); // Consume either quote or dereference.
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
            Form::Meta {
                meta: Box::new(meta),
                form: Box::new(form),
            },
            span,
        ))
    }

    fn read_dispatch(&mut self, start: u32) -> Result<SForm, Diagnostic> {
        self.bump(); // Consume '#'.
        match self.peek() {
            Some('{') => {
                let (items, span) = self.read_coll(start, '{', '}')?;
                Ok(Spanned::new(Form::Set(items), span))
            }
            Some('\'') => {
                // #'x desugars to (var x).
                self.bump();
                let inner = self.read_form()?;
                let span = self.span_from(start);
                let op = Spanned::new(Form::sym("var"), Span::point(self.src, start));
                Ok(Spanned::new(Form::List(vec![op, inner]), span))
            }
            Some('(') => self.read_anon_fn(start),
            Some('"') => Err(self
                .err(
                    "E0012",
                    "literal de regex (#\"...\") não é suportado",
                    self.span_from(start),
                )
                .with_help("use a feature `regex` quando disponível; fora do MVP inicial")),
            Some('?') => Err(self.err(
                "E0013",
                "reader conditional (#?) não é suportado no MVP inicial",
                self.span_from(start),
            )),
            other => {
                let msg = match other {
                    Some(c) => format!("macro de leitura desconhecida `#{c}`"),
                    None => "`#` seguido de fim de arquivo".to_string(),
                };
                Err(self.err("E0014", msg, self.span_from(start)))
            }
        }
    }

    /// Desugars `#(...)` to an `fn*` form with inferred `%N` parameters.
    ///
    /// The highest positional placeholder determines fixed arity; `%&` appends
    /// a rest binding. Bare `%` is canonicalized to `%1`.
    fn read_anon_fn(&mut self, start: u32) -> Result<SForm, Diagnostic> {
        let (body_items, span) = self.read_coll(start, '(', ')')?;
        let body = Spanned::new(Form::List(body_items), span);

        // Infer the highest positional placeholder and rest usage first.
        let mut max_pos = 0u32;
        let mut has_rest = false;
        collect_pct(&body, &mut max_pos, &mut has_rest);

        // Canonicalize `%` only after parameter inference.
        let body = rewrite_pct(body);

        let mut params: Vec<SForm> = Vec::new();
        for i in 1..=max_pos {
            params.push(Spanned::new(
                Form::sym(&format!("%{i}")),
                Span::point(self.src, start),
            ));
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
        self.bump(); // Consume the opening delimiter.
        let mut items = Vec::new();
        loop {
            self.skip_trivia()?;
            match self.peek() {
                None => {
                    return Err(self
                        .err(
                            "E0004",
                            format!("`{open}` sem `{close}` correspondente"),
                            self.span_from(start),
                        )
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
                    return Err(self.err(
                        "E0007",
                        "string sem aspas de fechamento",
                        self.span_from(start),
                    ))
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
                            return Err(self.err(
                                "E0007",
                                "string sem aspas de fechamento",
                                self.span_from(start),
                            ))
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
                     // A character after the slash is mandatory.
        let first = match self.bump() {
            Some(c) => c,
            None => return Err(self.err("E0015", "`\\` sem caractere", self.span_from(start))),
        };
        // Alphabetic input may start a named character such as newline or uXXXX.
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
                    match u32::from_str_radix(&w[1..], 16)
                        .ok()
                        .and_then(char::from_u32)
                    {
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
                .err(
                    "E0017",
                    "keyword auto-resolvida (::) não suportada no MVP inicial",
                    self.span_from(start),
                )
                .with_help("use uma keyword qualificada explícita `:ns/nome`"));
        }
        let tok = self.read_token_str();
        if tok.is_empty() {
            return Err(self.err("E0018", "keyword vazia", self.span_from(start)));
        }
        let name = parse_name(&tok);
        Ok(Spanned::new(Form::Keyword(name), self.span_from(start)))
    }

    /// Reads a numeric literal, scalar literal, or symbol.
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
            || ((first == b'-' || first == b'+')
                && bytes.get(1).is_some_and(|b| b.is_ascii_digit()));

        if numeric_start {
            return self.parse_number(tok, span);
        }
        Ok(Form::Symbol(parse_name(tok)))
    }

    fn parse_number(&self, tok: &str, span: Span) -> Result<Form, Diagnostic> {
        if tok.contains('/') {
            return Err(self
                .err(
                    "E0020",
                    format!("ratios não são suportados no MVP: `{tok}`"),
                    span,
                )
                .with_help("ver specs/LANGUAGE_SCOPE.md — Ratio é [FUTURO]"));
        }
        if tok.ends_with('N') || tok.ends_with('M') {
            return Err(self
                .err(
                    "E0021",
                    format!("BigInt/BigDecimal não suportados no MVP: `{tok}`"),
                    span,
                )
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
            ',' | '('
                | ')'
                | '['
                | ']'
                | '{'
                | '}'
                | '"'
                | ';'
                | '@'
                | '^'
                | '`'
                | '\''
                | '~'
                | '\\'
        )
}

/// Splits `namespace/name`; a slash at either edge remains part of the name.
fn parse_name(tok: &str) -> Name {
    if let Some(idx) = tok.find('/') {
        if idx > 0 && idx < tok.len() - 1 {
            return Name::qualified(&tok[..idx], &tok[idx + 1..]);
        }
    }
    Name::simple(tok)
}

/// Finds the highest `%N` and whether `%&` occurs in an anonymous function.
///
/// INVARIANT: metadata and every collection container are traversed, so
/// placeholder discovery and rewriting observe the same tree.
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

/// Recursively rewrites bare `%` to `%1`, preserving every source span.
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
#[path = "../tests/unit/lib/mod.rs"]
mod tests;
