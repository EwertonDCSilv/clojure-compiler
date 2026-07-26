//! `Form`: a representação de dados produzida pelo reader (o "código como dado").
//!
//! Bootstrap: nomes de símbolos/keywords são `String` (interning é otimização futura,
//! ver specs/ARCHITECTURE.md — não começar pela otimização). Todo nó carrega `Span`
//! via [`clojure_span::Spanned`].

use clojure_span::Spanned;
use std::fmt;

pub type SForm = Spanned<Form>;

/// Nome qualificado opcionalmente por namespace: `ns/name` ou apenas `name`.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Name {
    pub ns: Option<String>,
    pub name: String,
}

impl Name {
    pub fn simple(name: impl Into<String>) -> Self {
        Name { ns: None, name: name.into() }
    }

    pub fn qualified(ns: impl Into<String>, name: impl Into<String>) -> Self {
        Name { ns: Some(ns.into()), name: name.into() }
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.ns {
            Some(ns) => write!(f, "{}/{}", ns, self.name),
            None => write!(f, "{}", self.name),
        }
    }
}

impl fmt::Debug for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

/// Uma forma lida. `List` em posição de operador vira chamada/forma especial no
/// analisador; aqui é apenas estrutura.
#[derive(Clone, PartialEq)]
pub enum Form {
    Nil,
    Bool(bool),
    Int(i64),
    Float(f64),
    Char(char),
    Str(String),
    Symbol(Name),
    Keyword(Name),
    List(Vec<SForm>),
    Vector(Vec<SForm>),
    /// Pares chave/valor preservando a ordem de leitura.
    Map(Vec<(SForm, SForm)>),
    Set(Vec<SForm>),
    /// Metadata de leitura (`^meta form`): a forma com sua metadata anexada.
    Meta { meta: Box<SForm>, form: Box<SForm> },
}

impl Form {
    pub fn sym(name: &str) -> Form {
        Form::Symbol(Name::simple(name))
    }

    pub fn kw(name: &str) -> Form {
        Form::Keyword(Name::simple(name))
    }

    /// Nome da variante, para mensagens de erro.
    pub fn kind(&self) -> &'static str {
        match self {
            Form::Nil => "nil",
            Form::Bool(_) => "boolean",
            Form::Int(_) => "integer",
            Form::Float(_) => "float",
            Form::Char(_) => "char",
            Form::Str(_) => "string",
            Form::Symbol(_) => "symbol",
            Form::Keyword(_) => "keyword",
            Form::List(_) => "list",
            Form::Vector(_) => "vector",
            Form::Map(_) => "map",
            Form::Set(_) => "set",
            Form::Meta { .. } => "meta",
        }
    }

    /// Se for `Meta`, devolve a forma subjacente; senão, ela mesma.
    pub fn strip_meta(&self) -> &Form {
        match self {
            Form::Meta { form, .. } => form.node.strip_meta(),
            other => other,
        }
    }
}

/// Impressão estilo `pr-str` (determinística), usada em dumps e golden tests.
impl fmt::Display for Form {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Form::Nil => write!(f, "nil"),
            Form::Bool(b) => write!(f, "{b}"),
            Form::Int(n) => write!(f, "{n}"),
            Form::Float(x) => {
                // Sempre com marca de float para não confundir com inteiro.
                if x.fract() == 0.0 && x.is_finite() {
                    write!(f, "{x:.1}")
                } else {
                    write!(f, "{x}")
                }
            }
            Form::Char(c) => write!(f, "\\{}", char_name(*c)),
            Form::Str(s) => write!(f, "\"{}\"", escape_str(s)),
            Form::Symbol(n) => write!(f, "{n}"),
            Form::Keyword(n) => write!(f, ":{n}"),
            Form::List(items) => write_seq(f, "(", items, ")"),
            Form::Vector(items) => write_seq(f, "[", items, "]"),
            Form::Set(items) => write_seq(f, "#{", items, "}"),
            Form::Map(pairs) => {
                write!(f, "{{")?;
                for (i, (k, v)) in pairs.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{} {}", k.node, v.node)?;
                }
                write!(f, "}}")
            }
            Form::Meta { meta, form } => write!(f, "^{} {}", meta.node, form.node),
        }
    }
}

impl fmt::Debug for Form {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

fn write_seq(f: &mut fmt::Formatter<'_>, open: &str, items: &[SForm], close: &str) -> fmt::Result {
    write!(f, "{open}")?;
    for (i, it) in items.iter().enumerate() {
        if i > 0 {
            write!(f, " ")?;
        }
        write!(f, "{}", it.node)?;
    }
    write!(f, "{close}")
}

fn char_name(c: char) -> String {
    match c {
        '\n' => "newline".to_string(),
        '\t' => "tab".to_string(),
        ' ' => "space".to_string(),
        '\r' => "return".to_string(),
        '\u{0}' => "backspace".to_string(),
        c => c.to_string(),
    }
}

fn escape_str(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\t' => out.push_str("\\t"),
            '\r' => out.push_str("\\r"),
            c => out.push(c),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_roundtrip_ish() {
        assert_eq!(Form::Int(42).to_string(), "42");
        assert_eq!(Form::Float(1.0).to_string(), "1.0");
        assert_eq!(Form::sym("foo").to_string(), "foo");
        assert_eq!(Form::kw("bar").to_string(), ":bar");
        assert_eq!(Form::Symbol(Name::qualified("a", "b")).to_string(), "a/b");
        assert_eq!(Form::Str("hi\n".into()).to_string(), "\"hi\\n\"");
        assert_eq!(Form::Char('\n').to_string(), "\\newline");
    }
}
