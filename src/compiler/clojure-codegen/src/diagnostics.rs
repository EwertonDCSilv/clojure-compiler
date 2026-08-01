//! Single-diagnostic construction helpers for backend errors, all reported
//! under the stable `E0120` code shared by every codegen-stage failure.

use clojure_diagnostics::{Diagnostic, Diagnostics};

pub(crate) fn single(msg: impl Into<String>) -> Diagnostics {
    Diagnostic::error("E0120", msg).into()
}

pub(crate) fn single_d(msg: impl Into<String>) -> Diagnostic {
    Diagnostic::error("E0120", msg)
}
