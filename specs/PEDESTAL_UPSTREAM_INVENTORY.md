# Upstream Pedestal inventory (ADR-0013 Gate 7)

This document pins the upstream Pedestal source snapshot the native connector is
measured against, inventories its namespaces, records the first compilation
blocker for each P2 candidate, and states whether compiling upstream Pedestal (P2)
is useful before proposing direct protocol compatibility (P3).

It is an analysis record, not an implementation contract. The P1 connector
([`PEDESTAL_NATIVE_CONNECTOR_SPEC.md`](PEDESTAL_NATIVE_CONNECTOR_SPEC.md)) and
[ADR-0013](adr/0013-compiled-clojure-pedestal-native-connector.md) remain the
authority on what the compiler ships.

## Pinned snapshot

| Component | Coordinate | Version | Repository |
| --- | --- | --- | --- |
| Pedestal http-kit connector | `io.pedestal/pedestal.http-kit` | 0.8.2-beta-10 | Clojars |
| Pedestal service | `io.pedestal/pedestal.service` | 0.8.2-beta-10 | Clojars |
| Pedestal route | `io.pedestal/pedestal.route` | 0.8.2-beta-10 | Clojars |
| Pedestal interceptor | `io.pedestal/pedestal.interceptor` | 0.8.2-beta-10 | Clojars |
| http-kit | `http-kit/http-kit` | 2.8.1 | Clojars |
| Clojure | `org.clojure/clojure` | 1.12.5 | Maven Central |

This is the same beta the upstream Level E fixture `e.pedestal.hello_world_api`
targets, the HTTP benchmark ([`benchmarks/http/`](../benchmarks/http/README.md))
runs, and the interceptor-chain differential
([`tests/differential/pedestal/`](../tests/differential/pedestal/README.md))
compares against.

## Namespace inventory

Enumerated from the pinned jars. The P1 connector reimplements the *behavior* of
the starred rows in the compiled subset under `cljn.*`; it does not compile these
files.

| Artifact | Namespaces |
| --- | --- |
| pedestal.interceptor | `io.pedestal.interceptor` ★, `io.pedestal.interceptor.chain` ★, `.chain.debug`, `.impl`, `.specs` |
| pedestal.route | `io.pedestal.http.route` ★, `.definition`, `.definition.table`, `.definition.terse`, `.definition.verbose`, `.internal`, `.linear-search`, `.map-tree`, `.path`, `.prefix-tree`, `.sawtooth`, `.sawtooth.impl`, `.specs`, `.types` |
| pedestal.service | `io.pedestal.connector` ★, `.connector.dev`, `.connector.specs`, `.connector.test`, `io.pedestal.http.body-params`, `.content-negotiation`, `.cors`, `.csrf`, `.params`, `.response`, `.ring-middlewares`, `.secure-headers`, `.sse`, `.tracing`, `io.pedestal.json*`, `io.pedestal.service.data`, `.impl`, `.interceptors`, `.protocols`, `.resources*`, `.websocket` |
| pedestal.http-kit | `io.pedestal.http.http-kit` ★, `.impl`, `.response`, `.specs` |
| pedestal.common | `io.pedestal.environment`, `io.pedestal.internal` |
| pedestal.log | `io.pedestal.log` |
| pedestal.telemetry | `io.pedestal.metrics*`, `io.pedestal.tracing*`, `io.pedestal.telemetry.*` |

## First compilation blocker per P2 candidate

Each blocker is a construct outside the P1 compiled subset (ADR-0013 §9). "First"
means the earliest one encountered; a namespace usually has several.

| Namespace | First blocker | Also requires |
| --- | --- | --- |
| `io.pedestal.interceptor.chain` | `clojure.core.async` (`go`/channel-driven async execution) | Java interop (`java.util.concurrent.atomic.AtomicLong`), stateful terminators |
| `io.pedestal.http.http-kit` | Java interop (`import org.httpkit.server.AsyncChannel`) | `clojure.core.async`, `atom`, the http-kit Java/Clojure server |
| `io.pedestal.connector` | user `defmacro` (`with-routes`) | `io.pedestal.service.protocols`, ring middlewares, cors/secure-headers graph |
| `io.pedestal.http.route` | protocol/record route expansion and multiple matcher backends | terse/table/verbose definition macros, spec |
| `io.pedestal.service.interceptors` | ring-middleware Java interop | content negotiation, protocols |

The recurring blockers are the constructs ADR-0013 §9 explicitly keeps out of P1:
`clojure.core.async`, Java interop and Java exception classes, user-defined macros
and syntax-quote, protocols/records used pervasively, and mutable `atom`/`ref`
state. `http-kit` itself is a Java-backed server and cannot be compiled without a
general Java-interop and native-socket-server story well beyond the P1 loopback
provider.

## P2 usefulness decision

**P2 (compiling selected upstream namespaces) is not useful yet.** Every P1
candidate namespace is blocked by capabilities the compiler does not have, and the
smallest, most self-contained one (`io.pedestal.interceptor.chain`) already needs
`core.async` and Java interop. Compiling it would deliver nothing the native
`cljn.pedestal.chain` does not already provide — and the differential harness
already proves the two agree on the observable order/termination/unwind/recovery
contract.

The prerequisites that would make P2 worth revisiting, in dependency order:

1. General Java interop (`import`, member access, `new`) and Java exception
   identity — required by http-kit, ring middlewares, and telemetry.
2. `clojure.core.async` or an equivalent asynchronous execution model — required by
   the upstream chain and connector.
3. User-defined macros and syntax-quote — required by `with-routes` and the route
   definition namespaces.
4. Pervasive protocol/record dispatch and Maven dependency resolution.

Until at least (1)–(3) exist, the project stays on P1: reimplement the observable
Pedestal semantics in the compiled subset and hold the line with the conformance
fixture, the HTTP benchmark, and the interceptor-chain differential. **No P3
protocol-compatibility work is proposed.**
