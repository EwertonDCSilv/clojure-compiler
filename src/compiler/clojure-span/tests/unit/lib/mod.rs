//! Unit tests for lib.rs.

use super::*;

#[test]
fn line_col_basic() {
    let mut sm = SourceMap::new();
    let id = sm.add("t.clj", "abc\ndef\n(x)");
    assert_eq!(sm.line_col(id, 0), LineCol { line: 1, col: 1 });
    assert_eq!(sm.line_col(id, 2), LineCol { line: 1, col: 3 });
    assert_eq!(sm.line_col(id, 4), LineCol { line: 2, col: 1 });
    // '(' in "(x)" is at the start of the third line.
    assert_eq!(sm.line_col(id, 8), LineCol { line: 3, col: 1 });
    assert_eq!(sm.line_text(id, 9), "(x)");
}

#[test]
fn span_union() {
    let a = Span::new(0, 2, 5);
    let b = Span::new(0, 7, 9);
    assert_eq!(a.to(b), Span::new(0, 2, 9));
}

#[test]
fn snippet_extracts() {
    let mut sm = SourceMap::new();
    let id = sm.add("t", "hello world");
    assert_eq!(sm.snippet(Span::new(id, 6, 11)), "world");
}

#[test]
fn span_and_spanned_helpers_preserve_location() {
    let point = Span::point(7, 12);
    assert!(point.is_empty());
    assert_eq!(point.len(), 0);
    assert_eq!(format!("{point:?}"), "7:12..12");

    let value = Spanned::new(21, Span::new(7, 3, 5));
    let mapped = value.map(|n| n * 2);
    assert_eq!(mapped.node, 42);
    assert_eq!(mapped.span, Span::new(7, 3, 5));
    assert_eq!(*mapped.as_ref().node, 42);
    assert_eq!(format!("{mapped:?}"), "42");
}

#[test]
fn source_map_exposes_name_text_and_unicode_columns() {
    let mut sm = SourceMap::new();
    let id = sm.add("unicode.clj", "áβ\nfim");
    assert_eq!(sm.name(id), "unicode.clj");
    assert_eq!(sm.text(id), "áβ\nfim");
    assert_eq!(sm.line_col(id, 2), LineCol { line: 1, col: 2 });
    assert_eq!(sm.line_col(id, 5), LineCol { line: 2, col: 1 });
    assert_eq!(sm.line_text(id, 0), "áβ");
    assert_eq!(sm.line_text(id, 8), "fim");
}

#[test]
fn source_map_handles_exact_line_boundaries_and_eof() {
    let mut sm = SourceMap::new();
    let id = sm.add("lines.clj", "a\nb\n");
    assert_eq!(sm.line_col(id, 2), LineCol { line: 2, col: 1 });
    assert_eq!(sm.line_col(id, 4), LineCol { line: 3, col: 1 });
    assert_eq!(sm.line_text(id, 4), "");
}
