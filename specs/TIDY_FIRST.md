# Tidy First for AI-Assisted Changes

## Purpose

Tidy First separates small structural improvements from behavior changes. A tidy makes
the next change easier to understand or safer to implement without changing observable
compiler, runtime, CLI, fixture, or documentation behavior.

This policy applies to AI agents and human contributors. It supplements, rather than
replaces, the feature workflow in [`AGENTS.md`](../AGENTS.md) and the
Red–Green–Refactor cycle in [`TDD_WORKFLOW.md`](TDD_WORKFLOW.md).

## Classify the change before editing

Classify every proposed edit as one of:

- **Tidy:** changes structure while preserving behavior.
- **Behavior:** changes an observable contract or fixes a defect.
- **Mixed:** contains both and must be split before implementation.

Examples of suitable tidies include a narrowly scoped rename, extraction of a helper,
removal of proven duplication, movement of code without semantic edits, or a local
representation cleanup already protected by tests.

The following are not tidies:

- changing diagnostics, CLI output, public APIs, ABI, GC behavior, or language
  semantics;
- changing dependencies, fixture status, snapshots, checksums, benchmark inputs, or
  expected benchmark results;
- fixing a bug discovered during cleanup;
- broad formatting, speculative abstraction, or repository-wide renaming unrelated to
  the next delivery.

When classification is uncertain, treat the change as behavior-changing and follow
TDD.

## Decide whether to tidy first

Create a preceding tidy only when it directly reduces the risk or review cost of the
next registered task. State the concrete obstruction it removes. Do not create a tidy
merely because nearby code could look cleaner.

A tidy must be:

- small enough to review independently;
- limited to one structural objective;
- reversible without changing the following task's contract;
- useful even if the following behavior change is abandoned.

If the tidy is larger than the intended behavior change, requires a new abstraction
with no current consumer, or expands beyond the owning subsystem, stop and keep the
existing structure.

## Branch, commit, and pull-request boundaries

Use an existing issue or a dedicated task issue whose acceptance criteria explicitly
describe the structural outcome. For a tracked tidy issue, branch from an up-to-date
`master` as:

```text
tidy/<issue-number>-<semantic-description>
```

Use lowercase kebab case. Keep the tidy in its own pull request targeting `master`.
Do not place the subsequent behavior change in the tidy branch or PR.

The pull-request title follows the repository convention:

```text
#<issue-number>: tidy <structural outcome>
```

The body must explain:

- which future or parent task the tidy prepares, when applicable;
- why the current structure obstructs that task;
- why observable behavior is unchanged;
- which baseline and post-change commands were run.

Do not add labels unless the user explicitly requests them. Merge the tidy before
creating the behavior branch from the updated `master`; do not silently stack a feature
on an unmerged tidy PR.

## Execution workflow

1. Record the clean baseline and identify the smallest tests that characterize the
   affected structure or behavior.
2. Run those tests before editing.
3. Make only the structural change.
4. Run the same focused tests and inspect the diff for accidental behavior changes.
5. Update required technical documentation and `CHANGELOG.md` `Unreleased` without
   claiming new capability.
6. Run `make pre-commit`; run `make pre-push` before handoff or push.
7. Open the tidy PR and stop. Start the behavior task only after the tidy is merged and
   `master` is refreshed.

Documentation-only changes use `make docs-check` as the closest structural gate.
Runtime-adjacent tidies still require sanitizer, compatibility, or benchmark gates when
the touched boundary normally requires them.

## Stop conditions

Stop the tidy and open or update a behavior issue when:

- a test output, snapshot, checksum, diagnostic, or public contract must change;
- the cleanup exposes a defect that needs a regression test;
- the diff needs compatibility exceptions or benchmark justification;
- the tidy can no longer be explained as one behavior-preserving objective.

Do not hide the behavior change in the tidy, weaken its tests, or rename it as
refactoring to bypass the feature guard or TDD workflow.

## Review checklist

- The PR contains one structural objective and no behavior delivery.
- Baseline and post-change evidence use the same commands.
- Public behavior, diagnostics, ABI, fixtures, checksums, and benchmark expectations
  are unchanged.
- The changelog describes maintenance, not new capability.
- The subsequent behavior change is absent from the branch.
