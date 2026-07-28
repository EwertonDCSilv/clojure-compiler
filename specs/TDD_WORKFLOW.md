# Test-Driven Compiler Evolution

## Scope

This specification governs changes to compiler behavior across the Rust frontend,
bootstrap interpreter, analyzer, Cranelift backend, native C runtime, compiled Clojure
core, CLI, and conformance infrastructure.

The default development loop is Red–Green–Refactor. Tests are executable contracts:
they establish the intended language behavior before implementation and remain as
regression protection afterward.

## Red: specify the missing behavior

Start every behavior change with the smallest test at the layer responsible for the
contract:

- a bug fix begins with a regression test that reproduces the bug;
- a parser or analyzer change begins with a focused unit test that checks the returned
  form, AST, span, or diagnostic;
- a lowering change begins with a positive case, a similar negative case that must not
  transform, and an observable semantic assertion;
- a runtime change begins in the C harness when it concerns tags, collections, ABI,
  rooting, ownership, errors, or GC;
- a user-visible language feature includes a CLI end-to-end or conformance case;
- a compiled-core change includes its function-level contract and the applicable
  conformance fixture.

Run the focused test before editing production behavior. Confirm that it fails because
the requested capability is absent or incorrect, not because the harness, fixture,
environment, or assertion is broken. Preserve the failing output as development
evidence in the handoff or commit description when it helps reviewers.

Tests must assert meaningful results. A bare `is_err()` is insufficient when the error
category, code, span, exit status, or stable message fragment is part of the contract.
Optimization tests require both recognition and non-recognition cases.

## Green: implement the smallest coherent change

Implement only the production behavior needed to satisfy the new contract. Keep the
change at the owning layer and avoid speculative APIs, unrelated cleanup, architecture
changes, or broad fixture regeneration.

Run the focused test until it passes, then run neighboring tests for the crate or
subsystem. A green focused test is not sufficient when the change crosses a boundary:
Rust/C ABI, GC, filesystem, process, reader/analyzer, analyzer/codegen, and compiled
core/runtime boundaries require their integration layer.

## Refactor: improve while preserving the contract

After Green:

- remove duplication and improve names or structure without widening scope;
- update comments and docstrings according to
  [`DOCUMENTATION_STYLE.md`](DOCUMENTATION_STYLE.md);
- record non-obvious invariants with the required contract markers;
- rerun focused and neighboring tests after each meaningful refactor;
- keep public APIs private unless architecture, rather than test convenience, requires
  public visibility.

Refactoring must not weaken the original assertion or replace a structural assertion
with a broad snapshot. Snapshots may supplement explicit invariant checks.

## Conformance and fixtures

Conformance status is an executable claim:

- `active` means the behavior is supported and blocking;
- `xfail` means the declared gap must still fail for the recorded reason;
- `pending` is inventory and is not evidence of implementation.

Add or update the fixture during the integration step. Do not regenerate expectations,
bless snapshots, or edit checksums until the focused implementation is green and the
new output has been reviewed. Update only the checksums owned by the intentional
fixture change.

An unexpected pass is not ignored. Determine whether it is stable implemented
behavior; if so, promote the fixture to `active`, update its reason/tracking metadata,
and keep zero new active failures. If it is accidental or incomplete, add a focused
test that exposes the missing contract.

External sources and byte-exact fixtures are immutable inputs unless the task
explicitly updates their pinned version or expected bytes.

## Determinism and test placement

Follow [`TESTING_STRATEGY.md`](TESTING_STRATEGY.md) and
[ADR-0011](adr/0011-rust-crate-unit-testing-strategy.md):

- private pure logic stays in module unit tests;
- cross-crate contracts use crate integration tests;
- compiled programs and process behavior use CLI/codegen end-to-end tests;
- public examples use short doctests;
- runtime internals and public ABI use the dedicated C harnesses;
- language compatibility uses self-contained conformance fixtures.

Tests do not depend on network access, a JVM, execution order, shared current working
directories, or unbounded sleeps. Control environment, clocks, random seeds,
filesystem state, and process timeouts. Restore global state even on failure.

## Exceptions to failing-first

Do not manufacture a failing test for:

- documentation-only changes;
- lint, hook, or CI configuration with no production behavior change;
- behavior-preserving refactors already protected by characterization tests;
- test-only additions that document behavior already implemented.

For these changes, identify the closest executable or structural baseline, run it
before and after, and add a characterization test first if the protected behavior is
not observable. Explain the exception in the handoff.

Prototype or investigative code may precede a test only when it is not merged as
production behavior. Convert the result into a failing contract before integrating it.

## Validation ladder

Use the narrowest command during Red and Green, then widen validation:

```bash
cargo test -p <crate>
make test-runtime
make test-runtime-sanitize
make compatibility
make docs-check
make pre-commit
make pre-push
```

Select commands according to the affected subsystem; do not replace focused tests with
the full suite. Before handoff, report which Red test failed initially, which focused
test became green, and which broader gates actually ran.

## Review and commit boundaries

A reviewable behavior change contains:

1. the regression or feature contract;
2. the minimal implementation;
3. required documentation and invariant updates;
4. intentional integration/conformance fixture changes;
5. evidence from the relevant gates.

Keep unrelated generated reports, benchmark output, external sources, and local
working-tree edits out of the change. Documentation required by a changed contract
belongs with that change; unrelated documentation cleanup remains a separate commit.
