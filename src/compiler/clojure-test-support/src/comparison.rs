//! Structural comparison of Clojure source forms, ignoring incidental formatting.

use clojure_span::SourceMap;
use clojure_syntax::Form;

/// Compares two sequences of Clojure forms after structural canonicalization.
///
/// Map entries and set elements are sorted during canonicalization, while
/// sequence order and metadata remain significant.
///
/// # Errors
///
/// Returns a rendered reader diagnostic if either input is not valid forms.
pub fn structurally_equal(expected: &str, actual: &str) -> Result<bool, String> {
    let expected = parse_forms("<expected>", expected)?;
    let actual = parse_forms("<actual>", actual)?;
    if expected.len() != actual.len() {
        return Ok(false);
    }
    Ok(expected
        .iter()
        .zip(actual.iter())
        .all(|(left, right)| canonical_form(&left.node) == canonical_form(&right.node)))
}

pub(crate) fn parse_forms(name: &str, text: &str) -> Result<Vec<clojure_syntax::SForm>, String> {
    let mut sources = SourceMap::new();
    let id = sources.add(name, text);
    clojure_reader::read_all(id, text).map_err(|diagnostics| {
        format!(
            "{name} is not valid EDN/forms:\n{}",
            diagnostics.render(&sources)
        )
    })
}

pub(crate) fn canonical_form(form: &Form) -> String {
    match form {
        Form::List(items) => canonical_sequence("list", items),
        Form::Vector(items) => canonical_sequence("vector", items),
        Form::Set(items) => {
            let mut values = items
                .iter()
                .map(|item| canonical_form(&item.node))
                .collect::<Vec<_>>();
            values.sort();
            format!("set[{}]", values.join("|"))
        }
        Form::Map(pairs) => {
            let mut values = pairs
                .iter()
                .map(|(key, value)| {
                    format!(
                        "{}=>{}",
                        canonical_form(&key.node),
                        canonical_form(&value.node)
                    )
                })
                .collect::<Vec<_>>();
            values.sort();
            format!("map[{}]", values.join("|"))
        }
        Form::Meta { meta, form } => format!(
            "meta[{}]{}",
            canonical_form(&meta.node),
            canonical_form(&form.node)
        ),
        other => format!("{}:{}", other.kind(), other),
    }
}

pub(crate) fn canonical_sequence(kind: &str, items: &[clojure_syntax::SForm]) -> String {
    format!(
        "{kind}[{}]",
        items
            .iter()
            .map(|item| canonical_form(&item.node))
            .collect::<Vec<_>>()
            .join("|")
    )
}
