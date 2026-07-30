//! Unit tests for lib.rs.

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
            "nil", "boolean", "integer", "float", "char", "string", "symbol", "keyword", "list",
            "vector", "map", "set", "meta"
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
