//! Structured compiler diagnostics and terminal-oriented rendering.
//!
//! Frontend and backend stages return stable diagnostic codes instead of
//! printing directly. A [`Diagnostic`] may carry a primary [`Span`], notes, and
//! help; [`Diagnostics`] groups multiple reports at a pipeline boundary.
//! Rendering uses a [`SourceMap`] to add source name, one-based location,
//! source line, and caret. User-facing text remains in Portuguese.

use clojure_span::{SourceMap, Span};
use std::fmt::Write as _;

/// Classification that determines the diagnostic label and failure semantics.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Severity {
    /// A condition that prevents the requested compilation stage from finishing.
    Error,
    /// A report that does not by itself make the pipeline fail.
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

/// One structured compiler report.
///
/// Diagnostic codes are stable machine-readable identifiers. Message, note, and
/// help text are intended for users and remain Portuguese.
#[derive(Clone, Debug)]
pub struct Diagnostic {
    /// Stable identifier such as `E0004`.
    pub code: &'static str,
    /// Whether the report is fatal to the current pipeline stage.
    pub severity: Severity,
    /// Primary user-facing explanation.
    pub message: String,
    /// Optional primary source range.
    pub span: Option<Span>,
    /// Additional contextual statements, in display order.
    pub notes: Vec<String>,
    /// Optional action the user can take.
    pub help: Option<String>,
}

impl Diagnostic {
    /// Creates an error without a source range.
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

    /// Creates a warning without a source range.
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

    /// Sets the primary source range.
    pub fn with_span(mut self, span: Span) -> Self {
        self.span = Some(span);
        self
    }

    /// Sets the corrective-help line.
    pub fn with_help(mut self, help: impl Into<String>) -> Self {
        self.help = Some(help.into());
        self
    }

    /// Appends one contextual note.
    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.notes.push(note.into());
        self
    }

    /// Renders this report against `sm`, including a source line and caret.
    ///
    /// A report without a span contains only its heading, notes, and help.
    ///
    /// # Panics
    ///
    /// Panics if the diagnostic span refers to an invalid source or byte offset
    /// in `sm`.
    ///
    /// # Examples
    ///
    /// ```
    /// use clojure_diagnostics::Diagnostic;
    /// use clojure_span::{SourceMap, Span};
    ///
    /// let mut sources = SourceMap::new();
    /// let id = sources.add("example.clj", "unknown");
    /// let text = Diagnostic::error("E1001", "símbolo não resolvido")
    ///     .with_span(Span::new(id, 0, 7))
    ///     .render(&sources);
    /// assert!(text.contains("example.clj:1:1"));
    /// ```
    pub fn render(&self, sm: &SourceMap) -> String {
        let mut out = String::new();
        let _ = write!(
            out,
            "{}[{}]: {}",
            self.severity.label(),
            self.code,
            self.message
        );

        if let Some(span) = self.span {
            let lc = sm.line_col(span.source, span.start);
            let name = sm.name(span.source);
            let _ = write!(out, "\n  --> {}:{}:{}", name, lc.line, lc.col);
            let line = sm.line_text(span.source, span.start);
            let gutter = format!("{} | ", lc.line);
            let _ = write!(out, "\n{}{}", gutter, line);

            // Align the cursor to the character column. Clamp the byte-sized
            // range to the line's character count to keep malformed ranges from
            // producing unbounded output.
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

/// Ordered collection returned when a pipeline stage emits reports.
///
/// A collection may contain warnings only; callers use [`Self::has_errors`] to
/// decide whether compilation can continue.
#[derive(Clone, Debug, Default)]
pub struct Diagnostics {
    /// Reports in emission order.
    pub items: Vec<Diagnostic>,
}

impl Diagnostics {
    /// Creates an empty collection.
    pub fn new() -> Self {
        Diagnostics { items: Vec::new() }
    }

    /// Appends a report.
    pub fn push(&mut self, d: Diagnostic) {
        self.items.push(d);
    }

    /// Returns whether the collection contains no reports.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Returns whether at least one report has [`Severity::Error`].
    pub fn has_errors(&self) -> bool {
        self.items.iter().any(|d| d.severity == Severity::Error)
    }

    /// Renders all reports separated by one blank line.
    ///
    /// # Panics
    ///
    /// Panics when a contained span is invalid for `sm`; see
    /// [`Diagnostic::render`].
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
#[path = "../tests/unit/lib/mod.rs"]
mod tests;
