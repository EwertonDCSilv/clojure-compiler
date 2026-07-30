//! Source locations shared by the compiler frontend.
//!
//! [`Span`] and [`Spanned`] preserve half-open UTF-8 byte ranges as source text
//! moves from the reader into syntax and diagnostics. [`SourceMap`] owns the
//! corresponding source buffers and converts byte offsets to one-based
//! character columns for human-readable diagnostics. This crate has no
//! dependency on Clojure syntax and is the lowest layer of the frontend.

use std::fmt;

/// Numeric handle for a source registered in a [`SourceMap`].
///
/// Handles are local to the map that allocated them and are assigned in
/// insertion order.
pub type SourceId = u32;

/// Half-open UTF-8 byte range `[start, end)` within one source.
///
/// Offsets are bytes, not Unicode scalar values or display columns. Consumers
/// that slice source text must keep them on UTF-8 boundaries.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Span {
    /// Source containing this range.
    pub source: SourceId,
    /// Inclusive start byte offset.
    pub start: u32,
    /// Exclusive end byte offset.
    pub end: u32,
}

impl Span {
    /// Creates a source range.
    ///
    /// # Panics
    ///
    /// Debug builds panic when `start > end`.
    ///
    /// # Examples
    ///
    /// ```
    /// use clojure_span::Span;
    ///
    /// let span = Span::new(0, 4, 7);
    /// assert_eq!(span.len(), 3);
    /// ```
    pub fn new(source: SourceId, start: u32, end: u32) -> Self {
        debug_assert!(start <= end);
        Span { source, start, end }
    }

    /// Creates an empty range at `at`.
    ///
    /// Point spans represent synthetic nodes and end-of-file diagnostics.
    pub fn point(source: SourceId, at: u32) -> Self {
        Span {
            source,
            start: at,
            end: at,
        }
    }

    /// Returns the smallest range covering `self` and `other`.
    ///
    /// INVARIANT: callers provide spans from the same source. The method does
    /// not check source IDs; mixing them produces a meaningless range.
    pub fn to(self, other: Span) -> Span {
        Span {
            source: self.source,
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }

    /// Returns the range length in bytes.
    pub fn len(self) -> u32 {
        self.end - self.start
    }

    /// Returns whether the range contains no bytes.
    pub fn is_empty(self) -> bool {
        self.start == self.end
    }
}

impl fmt::Debug for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}..{}", self.source, self.start, self.end)
    }
}

/// Value annotated with the source range that produced it.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Spanned<T> {
    /// Parsed or analyzed value.
    pub node: T,
    /// Source range associated with `node`.
    pub span: Span,
}

impl<T> Spanned<T> {
    /// Associates `node` with `span`.
    pub fn new(node: T, span: Span) -> Self {
        Spanned { node, span }
    }

    /// Transforms the node while preserving its source range.
    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> Spanned<U> {
        Spanned {
            node: f(self.node),
            span: self.span,
        }
    }

    /// Borrows the node while copying its source range.
    pub fn as_ref(&self) -> Spanned<&T> {
        Spanned {
            node: &self.node,
            span: self.span,
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for Spanned<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Keep syntax dumps readable; callers can inspect `.span` explicitly.
        fmt::Debug::fmt(&self.node, f)
    }
}

/// One-based line and character-column location.
///
/// Columns count Unicode scalar values from the start of the line, rather than
/// UTF-8 bytes or terminal display cells.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LineCol {
    /// One-based line number.
    pub line: u32,
    /// One-based Unicode scalar-value column.
    pub col: u32,
}

/// Registered source buffer and its precomputed line index.
struct Source {
    name: String,
    text: String,
    /// Byte offset of every line start; the first entry is always zero.
    line_starts: Vec<u32>,
}

/// Builds a byte-offset index in one linear pass over the source.
fn compute_line_starts(text: &str) -> Vec<u32> {
    let mut starts = vec![0u32];
    for (i, b) in text.bytes().enumerate() {
        if b == b'\n' {
            starts.push((i + 1) as u32);
        }
    }
    starts
}

/// Owns compiler source buffers and resolves their byte locations.
///
/// Sources remain stable after insertion, so every [`SourceId`] and [`Span`]
/// stays valid for the lifetime of the map. Line lookup uses a binary search in
/// the precomputed line-start table.
#[derive(Default)]
pub struct SourceMap {
    sources: Vec<Source>,
}

impl SourceMap {
    /// Creates an empty source registry.
    pub fn new() -> Self {
        SourceMap {
            sources: Vec::new(),
        }
    }

    /// Registers source text and returns its stable identifier.
    ///
    /// # Panics
    ///
    /// Panics if more than `u32::MAX` sources are registered.
    ///
    /// # Examples
    ///
    /// ```
    /// use clojure_span::{SourceMap, Span};
    ///
    /// let mut sources = SourceMap::new();
    /// let id = sources.add("example.clj", "(println :ok)");
    /// assert_eq!(sources.snippet(Span::new(id, 1, 8)), "println");
    /// ```
    pub fn add(&mut self, name: impl Into<String>, text: impl Into<String>) -> SourceId {
        let text = text.into();
        let line_starts = compute_line_starts(&text);
        let id = self.sources.len() as SourceId;
        self.sources.push(Source {
            name: name.into(),
            text,
            line_starts,
        });
        id
    }

    /// Returns the registered display name.
    ///
    /// # Panics
    ///
    /// Panics if `id` does not belong to this map.
    pub fn name(&self, id: SourceId) -> &str {
        &self.sources[id as usize].name
    }

    /// Returns the complete registered UTF-8 source text.
    ///
    /// # Panics
    ///
    /// Panics if `id` does not belong to this map.
    pub fn text(&self, id: SourceId) -> &str {
        &self.sources[id as usize].text
    }

    /// Returns the source slice covered by `span`.
    ///
    /// # Panics
    ///
    /// Panics if the source ID is invalid, either offset is out of bounds, or an
    /// offset is not on a UTF-8 boundary.
    pub fn snippet(&self, span: Span) -> &str {
        let s = &self.sources[span.source as usize];
        &s.text[span.start as usize..span.end as usize]
    }

    /// Resolves a byte offset to a one-based line and character column.
    ///
    /// The lookup is `O(log L + C)`, where `L` is the number of lines and `C`
    /// is the number of Unicode scalar values from the line start to `offset`.
    ///
    /// # Panics
    ///
    /// Panics if the source ID is invalid, `offset` is outside the source, or it
    /// is not on a UTF-8 boundary.
    pub fn line_col(&self, source: SourceId, offset: u32) -> LineCol {
        let s = &self.sources[source as usize];
        // INVARIANT: line_starts is sorted and begins with zero, so the
        // insertion point is never zero for an in-range offset.
        let line_idx = match s.line_starts.binary_search(&offset) {
            Ok(i) => i,
            Err(i) => i - 1,
        };
        let line_start = s.line_starts[line_idx];
        let col = s.text[line_start as usize..offset as usize].chars().count() as u32 + 1;
        LineCol {
            line: line_idx as u32 + 1,
            col,
        }
    }

    /// Returns the complete line containing `offset`, without its newline.
    ///
    /// # Panics
    ///
    /// Panics if the source ID is invalid or `offset` is outside the source.
    pub fn line_text(&self, source: SourceId, offset: u32) -> &str {
        let s = &self.sources[source as usize];
        let line_idx = match s.line_starts.binary_search(&offset) {
            Ok(i) => i,
            Err(i) => i - 1,
        };
        let start = s.line_starts[line_idx] as usize;
        let end = s.text[start..]
            .find('\n')
            .map(|n| start + n)
            .unwrap_or(s.text.len());
        &s.text[start..end]
    }
}

#[cfg(test)]
mod tests {
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
}
