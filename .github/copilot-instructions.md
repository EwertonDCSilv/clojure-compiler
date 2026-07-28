# Repository instructions

`AGENTS.md` is the repository-wide authority for AI-assisted changes. Read and follow
it before modifying code, tests, fixtures, comments, or documentation.

The mandatory policies are:

- `specs/DOCUMENTATION_STYLE.md`: technical English, current behavior only, complete
  Rust/C/Clojure contracts, and explicit `INVARIANT:`, `SAFETY:`, `ABI:`, and `GC:`
  markers;
- `specs/TDD_WORKFLOW.md`: failing test first, minimal implementation, refactor while
  green, and broader integration/conformance validation;
- `CONTRIBUTING.md`: repository hooks, language linters, and required local gates.

Do not weaken tests or change snapshots, fixture statuses, or checksums to hide a
regression. Run the narrowest test during development, `make pre-commit` before commit,
and `make pre-push` before handoff.
