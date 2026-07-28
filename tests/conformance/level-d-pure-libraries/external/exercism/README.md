# Exercism Clojure conformance corpus

These 13 fixtures preserve the official concept-exercise exemplars from
[`exercism/clojure`](https://github.com/exercism/clojure) at commit
[`4a4c4fd0eb5a232ad1e5f2b81c751bfbfcbd0190`](https://github.com/exercism/clojure/tree/4a4c4fd0eb5a232ad1e5f2b81c751bfbfcbd0190).
The original code is MIT-licensed by Exercism and its contributors; the local license
copy is [`benchmarks/exercism/LICENSE.exercism`](../../../../../benchmarks/exercism/LICENSE.exercism).

Each `input.clj` contains:

1. source attribution;
2. the upstream exemplar without semantic changes;
3. a small conformance driver after `BEGIN LOCAL CONFORMANCE DRIVER`.

This corpus tests language and standard-library compatibility. It does not measure
performance. The only concept exercise currently present in the benchmark suite is
`annalyns-infiltration`, because it executes successfully on both Native and
Clojure/JVM and therefore supports a meaningful runtime comparison.

Current state: 2 active cases and 11 expected failures. An `xfail` that begins to pass
breaks the conformance gate until it is reviewed and promoted to `active`.
