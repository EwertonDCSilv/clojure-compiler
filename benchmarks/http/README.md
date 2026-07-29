# HTTP benchmark — Native vs Clojure/JVM Pedestal

[Benchmarks](../README.md) · [ADR-0013](../../specs/adr/0013-compiled-clojure-pedestal-native-connector.md)

ADR-0013 Gate 6. This suite compares the compiled `cljn.pedestal.*` connector
against the pinned upstream Pedestal http-kit connector on a single minimal HTTP
route. It is **versioned separately** from the language conformance suite and the
Cracking/Cormen/Exercism performance catalog (ADR-0013 §12) and contains no
runtime or code-generation code, so it cannot affect those results.

> Engineering measurement, not a universal performance promise. Compare results
> only within the same environment, revision, toolchain, and scale.

## What it measures

Both servers expose exactly one route, `GET /greet → 200 "Hello, world!\n"`, and
nothing else:

- **native** — [`native/greet_server.clj`](native/greet_server.clj), the compiled
  connector over the loopback HTTP provider (`cljn.pedestal.service`).
- **jvm** — [`jvm/src/greet_server.clj`](jvm/src/greet_server.clj), the pinned
  Pedestal http-kit connector. Pedestal's default interceptor stack is omitted so
  the server exposes the same P1 capability level as the native connector (route
  match, handler, response serialization); the comparison isolates dispatch and
  serialization rather than optional security middleware.

One identical [load client](client/load_client.clj) drives both servers, so the
measurement — not the tool — is what differs. It runs a closed-loop,
single-connection sequential load: a fresh TCP socket per request with
`Connection: close`, reading the whole response to EOF. Every response is checked
(`HTTP/1.1 200 OK` status and exact body), and the body SHA-256 is recorded so the
run **proves both servers serve byte-identical content before comparing timing**.

## Reproduce

The benchmark needs a JVM, the Clojure CLI, and network access to Clojars and
Maven Central to resolve the pinned Pedestal dependencies. Like the manual JVM
conformance oracle, it therefore runs **on demand and not in CI**.

```bash
benchmarks/http/run.sh --reps 5 --requests 20000 --warmup 2000
```

Options: `--reps N` (repetitions per server), `--requests N` (measured requests
per repetition), `--warmup N` (unmeasured priming requests), `--out DIR`. The
native server is always launched under an address-space `ulimit` and a hard
`timeout` so a runaway process cannot exhaust memory.

The runner writes [`results/http-benchmark.json`](results/http-benchmark.json)
(full per-repetition data and environment metadata) and
[`results/summary.md`](results/summary.md) (a median table). It exits non-zero if
the two servers ever disagree on a response.

### Pinned versions

| Component | Version |
| --- | --- |
| Pedestal | `io.pedestal/pedestal.http-kit` 0.8.2-beta-10 (Clojars) |
| http-kit | 2.8.1 |
| Clojure | 1.12.5 |
| SLF4J | slf4j-simple 2.0.17 |

These match the upstream Level E fixture `e.pedestal.hello_world_api`.

## Latest result

See [`results/summary.md`](results/summary.md). The committed run records the
environment (CPU, cores, JDK, revision) it was measured on; rerun locally before
drawing conclusions, because absolute numbers depend on the host.
