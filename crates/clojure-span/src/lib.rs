//! Posições de origem: `SourceId`, `Span`, `Spanned<T>` e um `SourceMap` que
//! resolve offsets de byte para linha/coluna. Base de diagnósticos e stack traces.
//!
//! Ver `specs/ARCHITECTURE.md` (crate `clojure-span`).

use std::fmt;

/// Identificador de uma fonte registrada no [`SourceMap`].
pub type SourceId = u32;

/// Intervalo de bytes `[start, end)` dentro de uma fonte.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Span {
    pub source: SourceId,
    pub start: u32,
    pub end: u32,
}

impl Span {
    pub fn new(source: SourceId, start: u32, end: u32) -> Self {
        debug_assert!(start <= end);
        Span { source, start, end }
    }

    /// Span vazio em uma posição (útil para nós sintéticos/erros de EOF).
    pub fn point(source: SourceId, at: u32) -> Self {
        Span {
            source,
            start: at,
            end: at,
        }
    }

    /// Une dois spans (assume mesma fonte).
    pub fn to(self, other: Span) -> Span {
        Span {
            source: self.source,
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }

    pub fn len(self) -> u32 {
        self.end - self.start
    }

    pub fn is_empty(self) -> bool {
        self.start == self.end
    }
}

impl fmt::Debug for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}..{}", self.source, self.start, self.end)
    }
}

/// Um valor anotado com seu span de origem.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Spanned<T> {
    pub node: T,
    pub span: Span,
}

impl<T> Spanned<T> {
    pub fn new(node: T, span: Span) -> Self {
        Spanned { node, span }
    }

    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> Spanned<U> {
        Spanned {
            node: f(self.node),
            span: self.span,
        }
    }

    pub fn as_ref(&self) -> Spanned<&T> {
        Spanned {
            node: &self.node,
            span: self.span,
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for Spanned<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Debug foca no nó; o span é ruído na maioria dos dumps.
        fmt::Debug::fmt(&self.node, f)
    }
}

/// Linha e coluna 1-indexadas (colunas em caracteres, não bytes).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LineCol {
    pub line: u32,
    pub col: u32,
}

/// Uma fonte registrada: nome (caminho) + conteúdo + índice de início de linhas.
struct Source {
    name: String,
    text: String,
    /// Offset de byte do início de cada linha (line_starts[0] == 0).
    line_starts: Vec<u32>,
}

fn compute_line_starts(text: &str) -> Vec<u32> {
    let mut starts = vec![0u32];
    for (i, b) in text.bytes().enumerate() {
        if b == b'\n' {
            starts.push((i + 1) as u32);
        }
    }
    starts
}

/// Registro de todas as fontes compiladas. Resolve spans para linha/coluna e
/// extrai o trecho de código de uma linha (para renderizar diagnósticos).
#[derive(Default)]
pub struct SourceMap {
    sources: Vec<Source>,
}

impl SourceMap {
    pub fn new() -> Self {
        SourceMap {
            sources: Vec::new(),
        }
    }

    /// Registra uma fonte e devolve seu id.
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

    pub fn name(&self, id: SourceId) -> &str {
        &self.sources[id as usize].name
    }

    pub fn text(&self, id: SourceId) -> &str {
        &self.sources[id as usize].text
    }

    /// Fatia de fonte coberta por um span.
    pub fn snippet(&self, span: Span) -> &str {
        let s = &self.sources[span.source as usize];
        &s.text[span.start as usize..span.end as usize]
    }

    /// Resolve um offset de byte para linha/coluna 1-indexadas.
    pub fn line_col(&self, source: SourceId, offset: u32) -> LineCol {
        let s = &self.sources[source as usize];
        // Maior line_start <= offset.
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

    /// Texto completo da linha (sem o `\n`) que contém `offset`.
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
        // '(' de "(x)" está no início da linha 3.
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
