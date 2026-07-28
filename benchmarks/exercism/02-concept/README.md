# Chapter 02 — Exercism concept benchmarks

This benchmark chapter contains only concept exercises that execute successfully on
both Native and Clojure/JVM. That is a stricter condition than being present in the
language conformance catalog.

Current case:

| Case | Performance pressure |
| --- | --- |
| `01-annalyns-infiltration.clj` | boolean branches, truthiness and short calls |

The other 12 official concept exemplars are language/stdlib compatibility targets, not
performance cases. They live in the executable
[Exercism conformance corpus](../../../tests/conformance/level-d-pure-libraries/external/exercism/README.md)
as `xfail` cases. When one is promoted to `active` and produces the same observable
result as Clojure/JVM, it may receive a separate performance workload here if it adds a
useful metric.
