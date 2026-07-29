# ADR-0016 — Classify conformance cases for the Clojure/JVM oracle

- Status: **Accepted**
- Date: 2026-07-29
- Related:
  [ADR-0007](0007-native-io-and-runtime-reader.md),
  [ADR-0013](0013-compiled-clojure-pedestal-native-connector.md),
  [Compatibility spec](../COMPATIBILITY_SPEC.md),
  [Testing strategy](../TESTING_STRATEGY.md)

## Context

The conformance suite carries a manual Clojure/JVM oracle
(`make compatibility-oracle`, `clojure-conformance oracle --check`). For every
non-pending `Reader`/`BuildRun` case whose `oracle` field is not `not-applicable`,
the oracle runs the case `input.clj` on a pinned Clojure/JVM (1.12.5) through
`tests/conformance/oracle/runner.clj` and compares the result to the committed
expectation. It is the independent check that a committed native expectation is
faithful to real Clojure rather than to a native accident.

The oracle had never been executed offline, so the `oracle` field was assigned
without running it. Running it against the full matrix surfaces 44 cases where the
JVM diverges from the committed expectation. Inspection shows none is a native
code-generation bug; every one falls into a construct the native compiler
implements more leniently than, or in addition to, Clojure/JVM:

- native-only namespaces and primitives — `cljn.io/*` readers and writers,
  `count-if`, the native `List` type — that the JVM cannot resolve at all;
- native value semantics the JVM rejects — throwing and catching non-`Throwable`
  values, `derive` with unqualified keywords, duplicate set-literal keys, using a
  transient after `persistent!`; and
- deliberate representation differences the JVM evaluates but prints differently —
  the reader expansion of `@x`, set iteration order, `*out*` routing of
  `pr`/`prn`/`newline`, and native exception-value representation.

The harness already skips `oracle = not-applicable`, but these cases were left as
`oracle = equal` (or, in one case, a stale `expected-diff`), so the oracle reports
them as failures. The suite needs a written classification policy so the oracle is
executable and every divergence is intentional and reviewed.

## Decision drivers

- The JVM oracle must be runnable end-to-end and green, so a real regression stands
  out instead of drowning in known, intentional divergences.
- Classification must not weaken a native assertion: `status` and every committed
  `expected.*` output stay unchanged, so `make compatibility` still enforces the
  native behavior exactly as before.
- The `oracle` field must state a truth about the JVM, not a convenience.
- The rule must be mechanical and reproducible from the harness's own signal.

## Decision

### Oracle taxonomy

Every non-pending `Reader`/`BuildRun` case declares exactly one relationship to the
Clojure/JVM oracle:

- **`oracle = equal`** — the JVM evaluates the same `input.clj` and produces the
  committed expectation. This is the default and the only value the oracle
  actively verifies for equality.
- **`oracle = not-applicable`** — the JVM cannot evaluate the case to a comparable
  result. The input depends on a native-only namespace, primitive, or type, or on
  native value semantics that make the JVM abort (unresolved symbol, cast error,
  assertion, or timeout). The oracle skips these.
- **`oracle = expected-diff`** (with `class = expected-diff`) — the JVM evaluates
  the input cleanly but the project deliberately commits a different native result
  (representation, ordering, or `*out*` routing). The oracle records the difference
  and flags it only if the divergence disappears.

### Decision rule

The deciding question is what the JVM does with the case, which the oracle already
observes:

1. If the JVM **cannot produce a comparable result** — it fails to compile or run
   the input, or times out — the case is **`not-applicable`**. This covers every
   native-only capability and every native leniency the JVM rejects.
2. If the JVM **runs cleanly but the observable output differs by design**, the
   case is **`expected-diff`** and its `class` is `expected-diff`.
3. Otherwise the case is **`equal`** and the JVM must match the committed
   expectation.

A case whose committed `expected-diff` no longer diverges (the JVM matches) is
demoted to `equal`; it was never a real difference.

### Scope and non-goals

This ADR classifies existing behavior. It does not add native features, change any
`status`, or edit any `expected.*` output. Making a native-only capability portable
(for example, implementing `pr`/`prn`/`newline` output routing to match JVM
`*out*`) is separate feature work; if such a case is later made portable, it moves
back to `oracle = equal` with its own change. Deciding whether a native leniency
(throwing non-`Throwable` values, duplicate set keys) should instead become a
native error is a language-scope question tracked separately, not resolved here.

## Consequences

### Positive

- `make compatibility-oracle` runs green against Clojure/JVM 1.12.5, so a future
  real divergence is visible.
- Every native/JVM difference is now labeled and justified in its `case.toml`
  `reason`.
- No native assertion is weakened: `make compatibility` still enforces the same
  `status` and `expected.*` outputs.

### Costs

- The oracle remains a manual, on-demand check: it needs a JVM and the pinned
  Clojure classpath and does not run in CI.
- `expected-diff` cases must be revisited if native behavior is later aligned with
  the JVM.

## Acceptance

This decision is implemented when:

1. every non-pending case declares `equal`, `not-applicable`, or
   `expected-diff` per the rule above;
2. `clojure-conformance oracle --check` reports no failures and no unexpected
   passes against Clojure/JVM 1.12.5;
3. `make compatibility` remains green with unchanged `status` and `expected.*`
   outputs; and
4. each reclassified case records a `reason` naming the native-only capability or
   the deliberate difference.
