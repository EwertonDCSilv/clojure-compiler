//! Source-level Clojure forms produced by the reader.
//!
//! [`Form`] is deliberately structural: lists are not calls and symbols are not
//! resolved until `clojure-analyzer` consumes them. Every node is wrapped in
//! [`SForm`] so reader byte ranges survive macro expansion and diagnostics.
//! Symbols and keywords use owned strings in this bootstrap representation;
//! this is separate from the tagged native runtime ABI.

use clojure_span::Spanned;
use std::fmt;

/// A source-level form paired with its UTF-8 byte range.
pub type SForm = Spanned<Form>;

/// Symbol or keyword name, optionally qualified as `namespace/name`.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Name {
    /// Namespace before the slash, or `None` for an unqualified name.
    pub ns: Option<String>,
    /// Unqualified name component.
    pub name: String,
}

impl Name {
    /// Creates an unqualified name.
    ///
    /// # Examples
    ///
    /// ```
    /// use clojure_syntax::Name;
    ///
    /// assert_eq!(Name::simple("map").to_string(), "map");
    /// ```
    pub fn simple(name: impl Into<String>) -> Self {
        Name {
            ns: None,
            name: name.into(),
        }
    }

    /// Creates a namespace-qualified name.
    ///
    /// The constructor does not validate slash placement; the reader owns token
    /// syntax validation.
    pub fn qualified(ns: impl Into<String>, name: impl Into<String>) -> Self {
        Name {
            ns: Some(ns.into()),
            name: name.into(),
        }
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

/// Structural source form.
///
/// A [`Form::List`] is only data at this layer. Operator position, special forms,
/// lexical bindings, and calls are assigned meaning by `clojure-analyzer`.
#[derive(Clone, PartialEq)]
pub enum Form {
    /// The `nil` literal.
    Nil,
    /// A boolean literal.
    Bool(bool),
    /// A signed 64-bit integer literal.
    Int(i64),
    /// An IEEE-754 double-precision literal.
    Float(f64),
    /// A Unicode scalar-value character literal.
    Char(char),
    /// An owned UTF-8 string literal after escape decoding.
    Str(String),
    /// A symbol that may be namespace-qualified.
    Symbol(Name),
    /// A keyword that may be namespace-qualified.
    Keyword(Name),
    /// Parenthesized forms in source order.
    List(Vec<SForm>),
    /// Bracketed forms in source order.
    Vector(Vec<SForm>),
    /// Key/value pairs preserving reader order.
    Map(Vec<(SForm, SForm)>),
    /// Set elements preserving reader order at this stage.
    Set(Vec<SForm>),
    /// Reader metadata (`^meta form`) attached to another form.
    Meta {
        /// Metadata expression following `^`.
        meta: Box<SForm>,
        /// Form carrying the metadata.
        form: Box<SForm>,
    },
}

impl Form {
    /// Creates an unqualified symbol form.
    pub fn sym(name: &str) -> Form {
        Form::Symbol(Name::simple(name))
    }

    /// Creates an unqualified keyword form.
    pub fn kw(name: &str) -> Form {
        Form::Keyword(Name::simple(name))
    }

    /// Returns a stable human-readable category for diagnostics.
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

    /// Recursively removes outer metadata wrappers.
    ///
    /// Non-metadata forms are returned unchanged by reference.
    pub fn strip_meta(&self) -> &Form {
        match self {
            Form::Meta { form, .. } => form.node.strip_meta(),
            other => other,
        }
    }
}

/// Produces deterministic `pr-str`-like text for dumps and golden tests.
impl fmt::Display for Form {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Form::Nil => write!(f, "nil"),
            Form::Bool(b) => write!(f, "{b}"),
            Form::Int(n) => write!(f, "{n}"),
            Form::Float(x) => {
                // Preserve an explicit floating-point marker for integral
                // finite values so dumps do not change the form category.
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

    #[test]
    fn kind_names_cover_every_form_variant() {
        let span = clojure_span::Span::point(0, 0);
        let atom = Spanned::new(Form::Nil, span);
        let forms = vec![
            Form::Nil,
            Form::Bool(true),
            Form::Int(1),
            Form::Float(1.5),
            Form::Char('x'),
            Form::Str("x".into()),
            Form::sym("x"),
            Form::kw("x"),
            Form::List(vec![]),
            Form::Vector(vec![]),
            Form::Map(vec![]),
            Form::Set(vec![]),
            Form::Meta {
                meta: Box::new(atom.clone()),
                form: Box::new(atom),
            },
        ];
        let names: Vec<_> = forms.iter().map(Form::kind).collect();
        assert_eq!(
            names,
            [
                "nil", "boolean", "integer", "float", "char", "string", "symbol", "keyword",
                "list", "vector", "map", "set", "meta"
            ]
        );
    }

    #[test]
    fn strip_meta_descends_through_nested_metadata() {
        let span = clojure_span::Span::point(0, 0);
        let inner = Spanned::new(Form::Int(7), span);
        let once = Spanned::new(
            Form::Meta {
                meta: Box::new(Spanned::new(Form::kw("a"), span)),
                form: Box::new(inner),
            },
            span,
        );
        let twice = Form::Meta {
            meta: Box::new(Spanned::new(Form::kw("b"), span)),
            form: Box::new(once),
        };
        assert_eq!(twice.strip_meta(), &Form::Int(7));
    }

    #[test]
    fn displays_collections_escapes_and_named_chars() {
        let span = clojure_span::Span::point(0, 0);
        let sf = |node| Spanned::new(node, span);
        assert_eq!(
            Form::List(vec![sf(Form::Int(1)), sf(Form::Bool(false))]).to_string(),
            "(1 false)"
        );
        assert_eq!(
            Form::Map(vec![(sf(Form::kw("a")), sf(Form::Int(1)))]).to_string(),
            "{:a 1}"
        );
        assert_eq!(Form::Set(vec![sf(Form::Int(1))]).to_string(), "#{1}");
        assert_eq!(
            Form::Str("\"\t\\\r".into()).to_string(),
            "\"\\\"\\t\\\\\\r\""
        );
        assert_eq!(Form::Char('\t').to_string(), "\\tab");
        assert_eq!(Form::Char(' ').to_string(), "\\space");
        assert_eq!(Form::Char('\r').to_string(), "\\return");
        assert_eq!(Form::Char('\0').to_string(), "\\backspace");
    }
}
