# Contributing

The repository keeps fast checks close to each commit and reserves the complete test
suite for push and CI. Technical comments and documentation are written in English;
compiler diagnostics exposed to users remain in Portuguese.

## AI-assisted changes

Repository instructions for Codex, Cursor, GitHub Copilot, and Claude are rooted in
[`AGENTS.md`](AGENTS.md). Tool-specific files import or point to that authority instead
of maintaining divergent copies.

All behavior-changing work follows the Red–Green–Refactor cycle in
[`specs/TDD_WORKFLOW.md`](specs/TDD_WORKFLOW.md). Documentation follows
[`specs/DOCUMENTATION_STYLE.md`](specs/DOCUMENTATION_STYLE.md) and is updated with the
contract it describes.

Preparatory refactors follow [`specs/TIDY_FIRST.md`](specs/TIDY_FIRST.md). A tidy is a
small behavior-preserving structural change in its own
`tidy/<issue-number>-<semantic-description>` branch and pull request. Run the same
focused characterization or structural check before and after it. If an observable
contract, fixture, snapshot, checksum, diagnostic, ABI, GC behavior, or benchmark
expectation must change, it is not a tidy and must follow the behavior workflow.

Every commit also updates the relevant category under
[`CHANGELOG.md`](CHANGELOG.md) `Unreleased`. Release preparation moves those entries
into a dated version section without rewriting historical releases. Changelog-only and
release-bookkeeping commits do not need a self-referential entry. The staged
pre-commit hook enforces this policy.

## Feature issues, branches, and pull requests

New features must be registered before implementation:

1. create an issue in
   [`EwertonDCSilv/clojure-compiler`](https://github.com/EwertonDCSilv/clojure-compiler/issues);
2. add the issue to the public
   [`clojure-compiler Roadmap`](https://github.com/users/EwertonDCSilv/projects/2);
3. assign at most 8 `Story points` and run
   `make agent-feature-guard ISSUE=<issue-number>`;
4. branch from the current `master` as
   `feature/<issue-number>-<semantic-description>`;
5. keep commits on that branch and run the gates required by the affected subsystem;
6. open a pull request targeting `master` that references the issue; and
7. merge the feature only through the pull request.

Use lowercase kebab case for the semantic description, such as
`feature/8-pedestal-syntax-support`. Update the issue and project before expanding the
feature beyond its registered scope. Direct feature implementation or direct feature
pushes on `master` are not accepted.

An issue without an estimate, above 8 story points, or classified as an epic/roll-up
cannot enter implementation. Refine it first, preferably as independent sub-issues
with their own acceptance criteria and estimates of at most 8 points. The repository
hooks repeat this fail-closed check on feature branches. The check requires an
authenticated `gh` session with Projects v2 read access; a failed lookup blocks the
feature commit rather than silently bypassing the policy.

## Enable the repository hooks

Run this once in each checkout:

```bash
make hooks-install
```

The installer sets the local `core.hooksPath` to `.githooks`. It refuses to replace a
different hook directory unless `scripts/install-git-hooks.sh --force` is explicitly
requested. The hooks are ordinary Bash scripts and do not require the Python
`pre-commit` package.

The pre-commit hook selects checks from the staged paths:

- every maintained text file is checked for conflict markers, trailing whitespace,
  a final newline, and executable mode on shell scripts;
- Rust changes run `rustfmt --check` and Clippy with warnings denied;
- C changes compile every runtime entry point as C11 in syntax-only mode with strict
  GCC/Clang warnings denied;
- Clojure changes run `clj-kondo` with warnings denied;
- shell changes run `bash -n`.

Exact conformance inputs and outputs, binary fixtures, and pinned Exercism sources are
excluded from whitespace normalization because their bytes may be part of a test or
upstream contract. Merge-conflict markers are still rejected.

Language tools inspect the working tree. When a staged file also has unstaged changes,
the hook prints that file so the contributor can verify the tested content.

The pre-push hook runs `make quality` and the C runtime suite with AddressSanitizer and
UndefinedBehaviorSanitizer. CI repeats the authoritative gates.

## Run checks manually

```bash
# All pre-commit checks, independent of the staged paths
make pre-commit

# Complete pre-push gate
make pre-push

# Individual language linters
make lint-files
make lint-rust
make lint-c
make lint-clojure
```

`make lint-files` also runs the gradual Rust module-size gate. New files must satisfy
the ADR-0012 limit for their kind; files in `config/rust-file-size-baseline.json` are
grandfathered only at their recorded line count and cannot grow silently.
`make coverage` writes the machine-readable raw report and its crate/module summary to
`target/coverage/`. Its versioned baseline is
[`config/coverage-baseline.json`](config/coverage-baseline.json): lowering a recorded
metric fails the ratchet. New or changed executable Rust lines must reach 90% coverage;
non-executable lines are excluded from that calculation. Update a baseline only after
reviewing the report and documenting the intentional coverage change in the same pull
request.

`clj-kondo` is installed automatically with a pinned checksum on Linux x86_64. On
other platforms, set `CLJ_KONDO_BIN` or place it on `PATH`. Set `CC` to choose a
GCC- or Clang-compatible C compiler.

## Language expectations

### Rust

- Keep `cargo fmt` output unchanged and introduce no Clippy warnings.
- Document public APIs and non-obvious ownership, safety, ABI, or GC invariants.
- Avoid `unsafe`; when it is required, keep its scope narrow and state the safety
  argument next to the block.
- Add unit tests near pure logic and integration tests at crate or process boundaries.

### C runtime

- Preserve C11 compatibility and the public `cljn_*` ABI.
- Treat compiler warnings as defects. New suppressions require a narrow scope and a
  comment explaining the invariant.
- Root live `Value` objects across allocation and safepoints, and document ownership
  of buffers and resources.
- Run `make test-runtime-sanitize` for changes involving allocation, pointers, I/O,
  exceptions, or GC.

### Clojure

- Introduce no `clj-kondo` warnings in maintained source.
- Keep public and bootstrap functions documented with docstrings.
- Preserve external fixtures verbatim and do not lint deliberately malformed
  conformance inputs as ordinary source.
- Add or promote a conformance case when implemented behavior becomes supported.
