# Pedestal interceptor-chain differential (ADR-0013 Gate 3)

[ADR-0013](../../../specs/adr/0013-compiled-clojure-pedestal-native-connector.md)

This harness proves that the compiled `cljn.pedestal.chain` reproduces the
observable interceptor semantics of the pinned upstream Pedestal
(`io.pedestal.interceptor.chain`, 0.8.2-beta-10) — ADR-0013 acceptance #4:
*interceptor order, termination, unwind, and recovery match the pinned manual
oracle*.

## How it works

A fixed corpus of interceptor-chain scenarios is expressed twice, scenario for
scenario:

- [`native_chain.clj`](native_chain.clj) — compiled with the repo compiler over
  `cljn.pedestal.chain`.
- [`jvm/src/jvm_chain.clj`](jvm/src/jvm_chain.clj) — run on the JVM over Pedestal.

Each side prints one normalized `name value` line per scenario. The JVM side
installs the same termination rule the native connector applies (terminate as
soon as the context carries a `:response`, via `chain/terminate-when`), so the two
chains are compared on equal footing. [`run.sh`](run.sh) builds and runs both and
diffs the output; identical output passes, any divergence fails.

Scenarios cover enter-forward/leave-reverse order, single interceptors,
termination at the first, middle, and last positions, and error unwind with
recovery. `:error` handlers only recover and never inspect the thrown value:
ADR-0013 records that typed Pedestal/JVM exception identity is unavailable in P1,
so exception identity is deliberately outside the compared observable.

## Reproduce

Needs a JVM, the Clojure CLI, and network access to Clojars/Maven Central to
resolve the pinned Pedestal dependencies, so — like the JVM conformance oracle and
the HTTP benchmark — it runs on demand and not in CI.

```bash
tests/differential/pedestal/run.sh
```

The last matching run is recorded in [`results/observed.txt`](results/observed.txt)
and [`results/summary.md`](results/summary.md).
