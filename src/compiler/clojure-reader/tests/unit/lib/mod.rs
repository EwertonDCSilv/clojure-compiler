//! Unit tests for lib.rs.

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
    assert_eq!(read1("2.5").node, Form::Float(2.5));
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

#[test]
fn shebang_commas_unicode_and_nested_discard_are_trivia_safe() {
    assert_eq!(dump("#!/usr/bin/env clojure\n[1, 2 #_#_3 4 5]"), "[1 2 5]");
    assert_eq!(read1(r#""olá \u03b2""#).node, Form::Str("olá β".into()));
}

#[test]
fn named_and_unicode_characters() {
    assert_eq!(read1(r"\tab").node, Form::Char('\t'));
    assert_eq!(read1(r"\space").node, Form::Char(' '));
    assert_eq!(read1(r"\return").node, Form::Char('\r'));
    assert_eq!(read1(r"\backspace").node, Form::Char('\u{8}'));
    assert_eq!(read1(r"\formfeed").node, Form::Char('\u{c}'));
    assert_eq!(read1(r"\u03b2").node, Form::Char('β'));
}

#[test]
fn structural_errors_have_stable_codes() {
    let cases = [
        (")", "E0003"),
        ("[1 2", "E0004"),
        ("[1}", "E0005"),
        ("{:a 1 :b}", "E0006"),
        ("\"open", "E0007"),
        (r#""\u12x4""#, "E0008"),
        (r#""\q""#, "E0009"),
        ("~@x", "E0011"),
        (r#"#"x""#, "E0012"),
        ("#?x", "E0013"),
        ("#z", "E0014"),
        ("\\", "E0015"),
        (r"\unknown", "E0016"),
        ("::auto", "E0017"),
        (":", "E0018"),
    ];
    for (source, code) in cases {
        assert_eq!(
            read_all(0, source).unwrap_err().items[0].code,
            code,
            "{source:?}"
        );
    }
}

#[test]
fn anonymous_function_tracks_positional_and_rest_arguments() {
    assert_eq!(
        dump("#(list % %3 %& {:value %})"),
        "(fn* [%1 %2 %3 & %&] (list %1 %3 %& {:value %1}))"
    );
}

#[test]
fn qualified_keywords_and_numeric_forms() {
    assert_eq!(
        read1(":algo/name").node,
        Form::Keyword(Name::qualified("algo", "name"))
    );
    assert_eq!(read1("+12").node, Form::Int(12));
    assert_eq!(read1("-2.5e2").node, Form::Float(-250.0));
    assert_eq!(read1("/").node, Form::sym("/"));
    assert_eq!(read_all(0, "12oops").unwrap_err().items[0].code, "E0022");
}
