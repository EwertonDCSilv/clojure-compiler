//! Unit tests for lib.rs.

use super::*;

#[test]
fn render_has_location_and_cursor() {
    let mut sm = SourceMap::new();
    let id = sm.add("t.clj", "(foo\n  bar)");
    let d = Diagnostic::error("E0001", "símbolo não resolvido: bar")
        .with_span(Span::new(id, 7, 10))
        .with_help("adicione um require ou defina `bar`");
    let s = d.render(&sm);
    assert!(s.contains("t.clj:2:3"), "{s}");
    assert!(s.contains("bar"), "{s}");
    assert!(s.contains('^'), "{s}");
    assert!(s.contains("help:"), "{s}");
}

#[test]
fn warning_with_notes_is_not_an_error() {
    let sm = SourceMap::new();
    let warning = Diagnostic::warning("W0001", "forma obsoleta")
        .with_note("prefira a forma nova")
        .with_help("consulte a especificação");
    let mut diagnostics = Diagnostics::new();
    assert!(diagnostics.is_empty());
    diagnostics.push(warning);

    assert!(!diagnostics.is_empty());
    assert!(!diagnostics.has_errors());
    let rendered = diagnostics.render(&sm);
    assert!(rendered.starts_with("warning[W0001]"));
    assert!(rendered.contains("note: prefira"));
    assert!(rendered.contains("help: consulte"));
}

#[test]
fn diagnostics_from_error_and_multiple_rendering() {
    let sm = SourceMap::new();
    let mut diagnostics = Diagnostics::from(Diagnostic::error("E1000", "primeiro erro"));
    diagnostics.push(Diagnostic::warning("W1000", "aviso"));

    assert!(diagnostics.has_errors());
    assert_eq!(diagnostics.items.len(), 2);
    let rendered = diagnostics.render(&sm);
    assert!(rendered.contains("error[E1000]"));
    assert!(rendered.contains("\n\nwarning[W1000]"));
}

#[test]
fn zero_width_span_still_renders_one_cursor() {
    let mut sm = SourceMap::new();
    let id = sm.add("eof.clj", "x");
    let rendered = Diagnostic::error("E0002", "fim inesperado")
        .with_span(Span::point(id, 1))
        .render(&sm);
    assert!(rendered.contains("eof.clj:1:2"));
    assert!(rendered.ends_with('^'));
}
