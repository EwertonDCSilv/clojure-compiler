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

New features also require a pre-existing issue in the `clojure-compiler Roadmap`, a
`feature/<issue-number>-<semantic-description>` branch, and a pull request targeting
`master`, as defined by `AGENTS.md`. Run `make agent-feature-guard
ISSUE=<issue-number>` before implementation. Missing estimates, epics, roll-ups, and
issues above 8 story points must be refined into independently estimated tasks first.

Do not weaken tests or change snapshots, fixture statuses, or checksums to hide a
regression. Run the narrowest test during development, `make pre-commit` before commit,
and `make pre-push` before handoff.
