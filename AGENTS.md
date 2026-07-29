# Repository Instructions

These instructions apply to every AI-assisted change in this repository. Treat the
linked specifications as requirements, not optional background reading.

## Required references

Before editing code, tests, fixtures, or technical documentation, read:

- `specs/DOCUMENTATION_STYLE.md` for documentation language, structure, contracts,
  and the `INVARIANT:`, `SAFETY:`, `ABI:`, and `GC:` markers;
- `specs/TDD_WORKFLOW.md` for the required Red–Green–Refactor development cycle;
- `specs/TESTING_STRATEGY.md` and the relevant subsystem spec or ADR;
- `CONTRIBUTING.md` for local hooks, linters, and validation commands.

If implementation and a document disagree, verify the executable behavior. Correct
stale documentation in the same change, but do not silently claim planned behavior as
implemented.

## Feature delivery workflow

This workflow is mandatory for every new feature implemented by an AI assistant,
regardless of the model, tool, or editor integration:

1. Before implementation starts, the feature must have an open issue in
   `EwertonDCSilv/clojure-compiler` and that issue must be an item in the public
   [`clojure-compiler Roadmap`](https://github.com/users/EwertonDCSilv/projects/2).
   If either record is missing, stop feature implementation and register it first.
2. Create the implementation branch from an up-to-date `master` using exactly
   `feature/<issue-number>-<semantic-description>`. Use a lowercase, hyphenated
   description of the behavior, for example `feature/8-pedestal-syntax-support`.
3. Keep the feature scope aligned with its issue. Update the issue and project item
   before materially widening the implementation.
4. Commit and push the implementation only on that feature branch. Never implement a
   new feature directly on `master`.
5. After the required gates pass, open a pull request from the feature branch to
   `master`. The pull request must reference the issue and preserve its project
   tracking.
6. Merge feature work into `master` only through that pull request. Do not bypass
   review history with a direct feature push to `master`.

Bug fixes, documentation, maintenance, release, and investigation branches may use
their established workflow. Work that adds user-visible or compiler/runtime capability
is a feature and must follow the workflow above.

## Documentation requirements

- Every change intended for commit must update the appropriate section under
  [`CHANGELOG.md`](CHANGELOG.md) `Unreleased`. Keep entries user- and
  contributor-relevant, group them as Added, Changed, Deprecated, Removed, Fixed,
  Security, or Performance, and move them to a dated version section when releasing.
  A change whose sole purpose is changelog or release bookkeeping is exempt from
  adding a second self-referential entry.
- Write technical documentation, docstrings, and source comments in concise English.
  Keep compiler diagnostics, CLI messages, and user-visible snapshots in Portuguese.
- Follow `specs/DOCUMENTATION_STYLE.md` whenever adding or changing a crate, module,
  public API, internal invariant, native ABI function, or compiled-core function.
- Document current behavior only. Mark future behavior as **Planned** and link its spec
  or ADR.
- Keep documentation synchronized with the contract changed by the code. Do not mix
  unrelated documentation cleanup into a functional change.
- Rust crates/modules require `//!`; public items, fields, and variants require `///`.
  Add `Errors`, `Panics`, `Safety`, and executable `Examples` only when applicable.
- C runtime fragments require subsystem headers and immediate contract comments for
  exported `cljn_*` functions. State allocation, rooting, ownership, failure behavior,
  ABI constraints, and meaningful complexity.
- Clojure `defn` forms in compiled core require docstrings covering semantics,
  eagerness/laziness, limits, and relevant JVM differences.
- Never edit generated `target/doc` output or reformat byte-exact fixtures and pinned
  external sources.

## Test-driven development

Behavior-changing work must follow `specs/TDD_WORKFLOW.md`:

1. **Red:** add the smallest test that expresses the missing behavior or reproduces
   the bug. Run it and confirm it fails for the intended reason.
2. **Green:** implement only enough production code to make that test pass.
3. **Refactor:** improve the code and documentation while keeping the focused and
   neighboring tests green.
4. **Integrate:** add or update the appropriate end-to-end or conformance contract,
   then run the broader gates required by the affected subsystem.

Do not weaken assertions, delete regression tests, bless snapshots, change fixture
checksums, or convert `active` cases to `xfail` merely to obtain a green run. An
unexpected conformance pass must be investigated and promoted when it represents
implemented behavior.

Documentation-only, test-infrastructure, and behavior-preserving refactors do not need
an artificial failing test. They do require the closest structural check,
characterization test, or lint gate before and after the change.

## Validation

- During development, run the narrowest relevant test (`cargo test -p <crate>` or the
  specific integration/conformance case).
- Before commit, run `make pre-commit`.
- Before handoff or push, run `make pre-push`.
- Run `make test-runtime-sanitize` for GC, allocation, pointer, exception, ABI, or I/O
  runtime changes.
- Run `make compatibility` for reader, analyzer, language semantics, compiled core, or
  fixture-status changes.
- Run `make docs-check` for documentation and public-contract changes.
- Never claim a gate passed unless it was executed. Report baseline failures and keep
  unrelated working-tree changes out of the patch.

## Code Review Rules

- Flag behavior changes that lack evidence of a failing-first regression or feature
  test, unless the TDD exception is explicitly justified.
- Flag documentation that violates `specs/DOCUMENTATION_STYLE.md`, describes planned
  behavior as implemented, or omits a changed public/ABI/GC contract.
- Flag a commit that changes the repository without updating the `Unreleased`
  changelog, except for changelog-only or release-bookkeeping commits.
- Flag weakened tests, unexplained snapshot/checksum changes, hidden conformance
  regressions, unrooted live `Value` objects, and Rust/C ABI mismatches.
