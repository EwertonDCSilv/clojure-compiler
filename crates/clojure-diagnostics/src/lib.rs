//! Diagnósticos estruturados e sua renderização estilo rustc.
//!
//! Regra do projeto (specs/COMPILER_PIPELINE.md §7): todo erro de usuário carrega
//! arquivo:linha:coluna, trecho do código e, quando possível, uma sugestão. `panic!`
//! nunca é usado como erro normal.

use clojure_span::{SourceMap, Span};
use std::fmt::Write as _;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
}

impl Severity {
    fn label(self) -> &'static str {
        match self {
            Severity::Error => "error",
            Severity::Warning => "warning",
        }
    }
}

/// Um diagnóstico: código estável, severidade, mensagem, span primário e notas/ajuda.
#[derive(Clone, Debug)]
pub struct Diagnostic {
    pub code: &'static str,
    pub severity: Severity,
    pub message: String,
    pub span: Option<Span>,
    pub notes: Vec<String>,
    pub help: Option<String>,
}

impl Diagnostic {
    pub fn error(code: &'static str, message: impl Into<String>) -> Self {
        Diagnostic {
            code,
            severity: Severity::Error,
            message: message.into(),
            span: None,
            notes: Vec::new(),
            help: None,
        }
    }

    pub fn warning(code: &'static str, message: impl Into<String>) -> Self {
        Diagnostic {
            code,
            severity: Severity::Warning,
            message: message.into(),
            span: None,
            notes: Vec::new(),
            help: None,
        }
    }

    pub fn with_span(mut self, span: Span) -> Self {
        self.span = Some(span);
        self
    }

    pub fn with_help(mut self, help: impl Into<String>) -> Self {
        self.help = Some(help.into());
        self
    }

    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.notes.push(note.into());
        self
    }

    /// Renderiza o diagnóstico contra um `SourceMap`, com trecho e cursor `^`.
    pub fn render(&self, sm: &SourceMap) -> String {
        let mut out = String::new();
        let _ = write!(out, "{}[{}]: {}", self.severity.label(), self.code, self.message);

        if let Some(span) = self.span {
            let lc = sm.line_col(span.source, span.start);
            let name = sm.name(span.source);
            let _ = write!(out, "\n  --> {}:{}:{}", name, lc.line, lc.col);
            let line = sm.line_text(span.source, span.start);
            let gutter = format!("{} | ", lc.line);
            let _ = write!(out, "\n{}{}", gutter, line);

            // Cursor: alinhado à coluna (em caracteres), com til do tamanho do span
            // limitado ao fim da linha.
            let pad = " ".repeat(gutter.len() + (lc.col as usize - 1));
            let width = span.len().max(1).min(line.chars().count() as u32) as usize;
            let _ = write!(out, "\n{}{}", pad, "^".repeat(width.max(1)));
        }

        for note in &self.notes {
            let _ = write!(out, "\n  note: {}", note);
        }
        if let Some(help) = &self.help {
            let _ = write!(out, "\n  help: {}", help);
        }
        out
    }
}

/// Erro que carrega um ou mais diagnósticos. Usado como `Err` no pipeline.
#[derive(Clone, Debug, Default)]
pub struct Diagnostics {
    pub items: Vec<Diagnostic>,
}

impl Diagnostics {
    pub fn new() -> Self {
        Diagnostics { items: Vec::new() }
    }

    pub fn push(&mut self, d: Diagnostic) {
        self.items.push(d);
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn has_errors(&self) -> bool {
        self.items.iter().any(|d| d.severity == Severity::Error)
    }

    pub fn render(&self, sm: &SourceMap) -> String {
        self.items
            .iter()
            .map(|d| d.render(sm))
            .collect::<Vec<_>>()
            .join("\n\n")
    }
}

impl From<Diagnostic> for Diagnostics {
    fn from(d: Diagnostic) -> Self {
        Diagnostics { items: vec![d] }
    }
}

#[cfg(test)]
mod tests {
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
}
