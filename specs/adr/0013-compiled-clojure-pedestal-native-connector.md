# ADR-0013 — Build the Pedestal-compatible native connector in compiled Clojure

- Status: **proposed**
- Date: 2026-07-28
- Baseline: `b3be68d`
- Related:
  [ADR-0002](0002-memory-management.md),
  [ADR-0003](0003-value-representation.md),
  [ADR-0005](0005-bootstrap-strategy.md),
  [ADR-0007](0007-native-io-and-runtime-reader.md),
  [ADR-0011](0011-rust-crate-unit-testing-strategy.md),
  [ADR-0012](0012-rust-crate-modularization.md),
  [native I/O spec](../IO_SPEC.md), and
  [Pedestal connector spec](../PEDESTAL_NATIVE_CONNECTOR_SPEC.md)

## Context

The Pedestal connector specification defines a Linux x86_64, synchronous HTTP/1.1
vertical slice whose application-facing behavior is expressed through Clojure maps,
functions, interceptors, and routes. It intentionally excludes Servlet, Jetty,
HTTP-Kit, Java interop, asynchronous interceptors, TLS, HTTP/2, streaming, and
concurrent execution.

The current compiler already provides several prerequisites:

- maps, vectors, sets, strings, keywords, records, protocols, closures, higher-order
  calls, and explicit `throw`/`try`/`catch`/`finally`;
- a uniform native function ABI, `(self, argc, argv) -> Value`;
- boxed `Float`, immutable `Bytes`, dynamic bindings for compiler-known Vars, file
  readers/writers, and blocking I/O primitives;
- a precise, non-moving, single-threaded GC with explicit shadow-stack roots;
- a C runtime split into ordered subsystem fragments and tested through unit, ABI, error,
  GC-stress, ASan, and UBSan harnesses; and
- a Level E Pedestal project fixture that records the desired HTTP response.

Those capabilities are not sufficient to build the connector. The remaining gaps cross
three boundaries:

1. **Compiler and packaging:** there is no static multi-file namespace graph, local
   source-path resolution, cross-namespace alias resolution, or initialized top-level
   data `def` with stable global roots.
2. **Language and standard library:** there are no compiled Clojure modules for the
   connector, interceptor chain, HTTP contracts, or router. General typed exceptions
   and `ex-data` are also incomplete.
3. **Native runtime:** there is no socket/server handle, signal-aware service loop,
   strict HTTP parser, response serializer, or resource-leak accounting for network
   resources.

The existing Level E fixture is not the first native delivery gate. It imports
`io.pedestal.connector` and `io.pedestal.http.http-kit` from Maven and therefore
represents direct upstream compatibility, or P3 in the connector specification. A
connector implemented under `cljn.pedestal.*` is P0/P1 and needs a separate fixture.
Promoting the upstream fixture for a P1 implementation would make a false compatibility
claim.

This ADR resolves the open architectural choices in the connector specification. It
does not mark any connector API as implemented.

## Decision drivers

- Keep connector semantics, interceptor execution, routing, and application composition
  in compiled Clojure.
- Keep sockets, file descriptors, partial reads/writes, and unsafe parsing behind the
  existing C ABI.
- Avoid a general FFI or a second Rust runtime boundary solely to deliver HTTP.
- Preserve a deterministic single-threaded GC and explicit root ownership.
- Produce a useful P1 connector before attempting to compile Pedestal upstream.
- Reject ambiguous HTTP framing and remote input without terminating the process.
- Make test and network requests share the same Clojure dispatch path.
- Keep build, conformance, and benchmark roles distinct and reproducible offline.
- Avoid growing `core_compiled.clj` into a container for unrelated libraries.

## Decision

### 1. Compatibility target and namespace ownership

The first delivered connector is P0/P1:

```text
cljn.http.request
cljn.http.response
cljn.pedestal.chain
cljn.pedestal.connector
cljn.pedestal.route
```

These are compiler-owned Clojure namespaces. They preserve a documented synchronous
subset of Pedestal concepts but do not claim that Pedestal source code was compiled.
The initial manual differential oracle is pinned to the version already recorded by the
project fixture, `io.pedestal/pedestal.http-kit 0.8.2-beta-10`.

P2/P3 remain separate future gates:

- P2 compiles selected upstream namespaces after the language and loader inventory
  proves their dependencies.
- P3 implements the upstream `PedestalConnector` protocol directly and requires its own
  ADR.

The project will not publish namespaces under `io.pedestal.*` for compiler-owned code.
This prevents source and ownership ambiguity.

### 2. Responsibility split

The connector itself is implemented in Clojure. Platform mechanisms remain in C.
Rust owns only compiler and build orchestration.

| Layer | Owns | Must not own |
| --- | --- | --- |
| Compiled Clojure | connector-map validation, request/response semantic validation, interceptor chain, routing, lifecycle orchestration, events, `test-request` | sockets, descriptors, raw parser buffers |
| C runtime | socket lifecycle, `poll`, signal wakeup, bounded request parsing, framing validation, body buffers, response bytes, partial writes, OS errors | routes, interceptor order, application handlers |
| Rust compiler/CLI | static namespace graph, source resolution, module initialization order, diagnostics, runtime link | HTTP semantics or application routing |

The same Clojure function is used for direct dispatch, interceptor dispatch, and
`test-request`. Network parsing and byte serialization wrap that function but do not
replace its validation or chain semantics.

### 3. Clojure owns the service loop

The C provider does not retain or invoke a Clojure closure. This avoids introducing a
general C-to-Clojure callback lifetime before the FFI callback model is stable.

The native connector drives a narrow provider API:

```clojure
(runtime/open-server config)
(runtime/receive! server)
(runtime/respond! server response)
(runtime/reject! server http-error)
(runtime/stop-server! server)
(runtime/close-server! server)
```

`receive!` blocks in C until one of these occurs:

- one complete request has been parsed;
- a protocol-level rejection has been written;
- the listener receives a stop signal;
- a timeout or OS error occurs.

Only one accepted connection may be active inside a server handle. The connection never
escapes into application Clojure. The application receives only a request map.
`respond!` consumes the current connection and closes it after the complete response.

The public P1 lifecycle adds two explicit operations:

```clojure
(serve-one! connector)
(serve! connector)
```

`start!` opens the listener and enters `serve!` only when `:join?` is true. With
`:join? false`, `start!` returns a running connector, but no background execution is
implied; the caller must invoke `serve!` or `serve-one!`. This is a deliberate
single-threaded divergence and must be documented in the public API.

`stop!` marks the handle for shutdown and closes the listener. A blocking `serve!` is
normally stopped by `SIGINT`, `SIGTERM`, a dispatch path that calls `stop!`, or a
provider error. No native or Clojure worker thread is created.

### 4. Native server handle and GC contract

The runtime adds an opaque `T_HTTP_SERVER` object. It contains no Clojure application
function. Its native state owns:

- listener and current connection descriptors;
- parser and response buffers;
- lifecycle state;
- configured limits and timeouts;
- saturating statistics; and
- the resource-registry identity required by ADR-0007.

The handle is registered while open and removed only by `close-server!`. Closing is
idempotent. Standard GC collection is not a correctness mechanism for closing sockets.
A diagnostic shutdown check fails tests when registered handles remain open.

P1 permits one open network server per process. Additional connector values may be used
with `test-request`, but opening a second listener fails with `:server-limit`. Supporting
multiple active listeners requires a provider-level multiplexing decision rather than
hidden threads.

ABI: no descriptor, `FILE *`, parser pointer, or OS address structure becomes a Clojure
integer or collection value.

GC: every C function that allocates a `Value` while constructing a request or error map
roots all intermediate strings, bytes, keywords, maps, and the server handle. Returned
request and response values are owned by generated-code root frames after the ABI call
returns.

### 5. HTTP provider and parser

The first provider is an in-tree C subsystem under
`crates/clojure-codegen/runtime/http/`, amalgamated through the existing runtime build.
It uses POSIX sockets and `poll` on Linux x86_64.

Bind addresses are IPv4 literals plus the explicit alias `localhost`; P1 performs no
DNS lookup. Port zero is supported through `getsockname` after bind.

The first parser is a purpose-built, incremental, bounded HTTP/1.0 and HTTP/1.1 request
parser in C. It is not a general web parser and accepts only the grammar required by the
P1 gate. Keeping it in-tree avoids an additional native dependency and makes the exact
accepted grammar auditable with the runtime.

The parser:

- accepts one request per connection;
- supports only identity framing with zero or one effective `Content-Length`;
- rejects `Transfer-Encoding`, chunked bodies, ambiguous or divergent lengths, folded
  headers, NUL, bare CR/LF, invalid header tokens, invalid versions, and limit overflow;
- reads request line, headers, and body incrementally under fixed limits;
- produces lowercase ASCII header names and preserves header values as bytes until
  validated;
- supports string bodies only after strict UTF-8 conversion; and
- never treats incomplete input as a complete request.

The provider always returns `Connection: close`. It may generate only the
pre-application 400, 408, 413, and 431 responses required to reject malformed or
oversized input. Application 4xx/5xx responses are produced through the Clojure
dispatch path.

The response serializer repeats security-critical validation even after Clojure
normalization: status range, header-token syntax, CR/LF rejection, forbidden
`Transfer-Encoding`, computed `Content-Length`, and partial-write completion. It emits
application headers in lowercase lexical order so byte-level fixtures remain
deterministic; HTTP clients must still treat their order as insignificant.

Moving to a vendored parser, Rust provider, keep-alive, TLS, or HTTP/2 requires a new ADR
with fuzzing and binary-size evidence.

### 6. Signal and blocking model

The provider uses a nonblocking self-pipe observed by `poll`.

- A minimal POSIX signal handler sets a `volatile sig_atomic_t` flag and writes a byte
  to the pipe.
- The handler does not allocate, access the GC, call Clojure, format output, or mutate
  collection state.
- `SIGINT` and `SIGTERM` wake the service loop, which performs normal shutdown on the
  server thread.
- A full self-pipe cannot lose the stop request because the atomic flag remains set.

Tests send signals to subprocesses; they do not install process-global handlers inside
parallel unit tests.

### 7. Error representation for P0/P1

P0/P1 does not wait for typed exception hierarchies. The current runtime can throw any
rooted `Value`, so connector failures are immutable data maps:

```clojure
{:cljn.error/domain :http
 :kind :address-in-use
 :operation :bind
 :host "127.0.0.1"
 :port 8080
 :os-code 98
 :message "Address already in use"}
```

`:cljn.error/domain`, `:kind`, and `:operation` are stable. OS codes and messages are
diagnostic. Interceptor `:error` callbacks receive the thrown value through an internal
context key and may recover by producing a response.

Fatal runtime checks must not be used for remote input, invalid connector configuration,
or expected socket errors. Each such path returns or throws a categorized map.

Typed exception objects and JVM-like `ex-data` integration remain prerequisites for
P2/P3, not for the first native connector.

### 8. Static module loader and built-in source bundle

The connector is not appended to `core_compiled.clj`. Before NC-1 is declared complete,
the compiler gains a static, offline module graph.

Compiler-owned sources live under:

```text
stdlib/cljn/http/request.clj
stdlib/cljn/http/response.clj
stdlib/cljn/pedestal/chain.clj
stdlib/cljn/pedestal/connector.clj
stdlib/cljn/pedestal/route.clj
```

The CLI resolves namespaces from:

1. compiler-owned built-in sources;
2. explicit local `--source-path` roots; and
3. the entry file.

Only literal `ns`/`:require` declarations participate. Resolution is deterministic,
canonicalizes paths, rejects duplicate namespace ownership and dependency cycles, and
never accesses Maven, JARs, Git, or the network. Every source keeps its own source-map
identity and diagnostics.

The loader topologically initializes namespaces and emits one native object. It also
adds top-level data `def` initializers, using supported expressions, backed by permanent
global root slots. Redefinition remains unsupported. This is necessary for route tables,
function references, and module constants. Initializers execute once in dependency
order before `-main`; cycles and access before initialization are build errors.

General dependency resolution remains outside P1. A later project/package decision may
consume `deps.edn`, but it must not alter this offline built-in-module contract.

### 9. Clojure implementation constraints

The P1 namespaces use only the compiled subset and active core functions. They do not
use:

- user-defined macros or syntax quote;
- lazy or infinite sequences;
- Java interop or Java exception classes;
- `core.async`, promises, futures, or threads;
- dynamic loading or runtime `require`; or
- keyword-as-function shorthand when an explicit `get` is sufficient.

If an implementation need exposes a missing general language operation, that operation
is added through its own failing conformance case. HTTP-specific behavior is not hidden
inside the analyzer or codegen.

Compiler-known dynamic Vars and `binding` already exist, but arbitrary Var maps do not.
The Pedestal `:bindings` feature remains explicitly unsupported in P1 and is not a
blocker for the connector, chain, or router.

### 10. Request, response, and routing contracts

`cljn.http.request` and `cljn.http.response` own semantic map validation for both
`test-request` and network requests. Native parser output is not trusted merely because
it came from C.

The initial router is a deterministic immutable linear matcher:

- literal segments and one-segment `:param` captures;
- method matching, route names, 404, and 405;
- canonical ordering independent of set/hash iteration; and
- duplicate or ambiguous route rejection during connector creation.

A tree or trie is introduced only after route-count benchmarks demonstrate that linear
matching is material. Routing remains a normal interceptor; the connector never imports
the route namespace.

### 11. Conformance fixtures are split by compatibility layer

The current fixture:

```text
e.pedestal.hello_world_api
```

continues to import `io.pedestal.*`, HTTP-Kit, and Maven dependencies. It remains
`pending` until P3 and must not be rewritten to make P1 appear compatible with upstream
Pedestal.

A new fixture is added for the native layer:

```text
e.pedestal.native_connector_hello
```

It imports `cljn.pedestal.*`, uses only local/built-in sources, and progresses:

```text
pending -> xfail -> active
```

`test-request` becomes active before network service tests. Loopback network lifecycle
tests belong to the CLI/runtime integration layer and run in isolated subprocesses with
ephemeral ports and timeouts. The conformance runner may gain a declarative `[service]`
section only after its schema has tests for startup readiness, request bytes, response
bytes, shutdown, timeout, and cleanup.

The JVM oracle is manual and applies to the shared interceptor semantics, not to the
`cljn.*` namespace spelling or native transport implementation.

### 12. Benchmark scope

HTTP performance is a separate benchmark suite. It is not added to Cracking, Cormen, or
the language conformance report.

Every Native/JVM comparison uses equivalent request/response checksums and records:

- compiler and Pedestal revisions;
- startup to readiness;
- stripped and unstripped binary size;
- idle and peak RSS;
- wall and CPU time;
- requests per second;
- latency p50, p95, and p99;
- warmup, request count, concurrency, CPU affinity, and repetitions; and
- the single-threaded native limitation.

At least five measured repetitions are required for a published comparison. The first
gate has no throughput-win requirement; correctness, startup, RSS, stable latency, and
absence of leaks are the acceptance criteria.

## Gap-closure sequence

Each phase follows Red–Green–Refactor and is merged independently.

### Gate 0 — Freeze contracts

- Add schema tests for the native connector fixture and future service runner.
- Add request/response and interceptor-chain cases as `xfail`.
- Record the P1/P3 fixture split.
- Keep every connector capability non-active.

### Gate 1 — Static modules and globals

- Add failing loader tests for local and built-in namespace graphs.
- Implement literal `:require`, aliases, cycle diagnostics, source-path isolation, and
  per-file spans.
- Add initialized top-level data `def` forms and permanent global roots.
- Compile a two-namespace local project under GC stress.

Exit criterion: built-in Clojure modules can be compiled separately from core, with
deterministic initialization and no network.

### Gate 2 — In-memory connector

- Implement connector-map, request, and response validation in Clojure.
- Implement categorized thrown data maps.
- Implement `create-connector` without a server handle and `test-request`.
- Cover nil/string/`Bytes` bodies, header normalization, content length, invalid
  responses, and application errors.

Exit criterion: the native P1 Hello World passes through `test-request` in normal and
GC-stress modes.

### Gate 3 — Synchronous interceptor chain

- Implement `:enter`, reverse `:leave`, reverse `:error`, termination, and recovery.
- Add positive, boundary, and invalid-result cases.
- Run the manually pinned differential corpus against Pedestal/JVM.

Exit criterion: the shared observable context/event corpus matches the pinned oracle.

### Gate 4 — Native HTTP provider

- Add `T_HTTP_SERVER`, resource registration, C ABI contracts, and C harness tests.
- Add the strict incremental parser, response serializer, self-pipe signal wakeup,
  limits, timeouts, partial-write loops, lifecycle, and stats.
- Add fuzz targets and request-smuggling regression corpus.
- Run normal, GC-stress, ASan, and UBSan paths.

Exit criterion: 1,000 start/serve/stop subprocess cycles and malformed-input corpus
complete with no open handle, crash, hang, or sanitizer finding.

### Gate 5 — Native service and router

- Implement `start!`, `serve-one!`, `serve!`, `stop!`, restart, events, and stats in
  Clojure.
- Implement the linear route interceptor and deterministic ambiguity checks.
- Activate `e.pedestal.native_connector_hello`.
- Preserve `e.pedestal.hello_world_api` as P3 pending.

Exit criterion: the versioned raw request receives the expected 200 response over an
ephemeral loopback port, and 404/405/error cases are active.

### Gate 6 — Hardening and publication

- Meet coverage ratchets for new Rust, C, and Clojure modules.
- Keep `make quality`, `make coverage`, `make compatibility`, runtime sanitizers, and
  leak accounting green.
- Publish the separate HTTP benchmark with at least five repetitions.
- Update the compatibility matrix without claiming complete Pedestal support.

Exit criterion: the complete first-cut Definition of Done in the connector
specification is satisfied for P1.

### Gate 7 — Upstream inventory

- Pin an upstream source snapshot.
- Inventory every selected namespace and first blocker.
- Decide whether P2 is useful before proposing P3.

The snapshot, namespace inventory, per-namespace first blocker, and the P2
usefulness decision are recorded in
[`specs/PEDESTAL_UPSTREAM_INVENTORY.md`](../PEDESTAL_UPSTREAM_INVENTORY.md): P2 is
not useful until general Java interop, an async model, and user macros exist, and
no P3 protocol-compatibility work is proposed.

No P2/P3 fixture is promoted merely because P1 is active.

## Alternatives considered

| Alternative | Advantages | Disadvantages | Result |
| --- | --- | --- | --- |
| Implement connector, chain, and routing in C | one native component | duplicates Clojure semantics in unsafe code and prevents differential reuse | rejected |
| Add a Rust HTTP library linked into every executable | mature ecosystem options | creates a second runtime/ownership boundary rejected for the initial I/O gate | rejected for P1 |
| Compile upstream Pedestal and HTTP-Kit first | maximum source familiarity | requires JVM interop, Maven, async, and broad stdlib before proving the connector | rejected |
| Put all connector code in `core_compiled.clj` | avoids a loader initially | worsens bootstrap coupling and violates module-maintenance boundaries | rejected |
| Let C retain and invoke a Clojure callback | compact provider loop | introduces callback rooting, lifetime, and reentrancy before general FFI callbacks | rejected for P1 |
| Clojure service loop over a narrow C provider | keeps semantics testable in Clojure and unsafe state in one ABI | requires explicit `serve!` in non-joining mode | **chosen** |
| General Maven/JAR dependency resolution | familiar project model | adds network, classpath, Java artifacts, and nondeterminism not needed for P1 | deferred |
| Linear initial router | small and deterministic | O(routes × segments) dispatch | **chosen until measured otherwise** |

## Consequences

### Positive

- The connector, chain, route composition, and contracts are ordinary compiled Clojure.
- The C surface is limited to mechanisms that require OS access or byte-safe framing.
- No JVM, Java class, Maven resolution, or general FFI is required by the P1 binary.
- The same dispatch behavior is exercised by `test-request` and real network traffic.
- The current single-threaded GC model remains valid.
- P1 delivery cannot be mistaken for compiling upstream Pedestal.
- Static modules become reusable infrastructure for other pure Clojure libraries.

### Costs

- A strict HTTP parser and serializer become security-sensitive C maintained by the
  project.
- The compiler must deliver module loading and rooted top-level initialization before
  the connector can leave the bootstrap core.
- `:join? false` requires explicit `serve!` because no background scheduler exists.
- Typed Pedestal/JVM exception identity is not available in P1.
- The provider is Linux/POSIX-specific until a platform abstraction earns its own ADR.
- The first router favors clarity over large route-table throughput.

### Risks and mitigations

| Risk | Mitigation |
| --- | --- |
| request smuggling or parser memory error | strict grammar, hard limits, fuzzing, adversarial corpus, ASan/UBSan |
| descriptor or buffer leak | external-resource registry, idempotent close, 1,000-cycle gate, shutdown leak assertion |
| GC corruption while constructing requests | explicit ABI/GC comments, C harnesses, GC stress at every dispatch allocation |
| hidden background semantics | explicit `serve!` contract and no thread creation |
| module initialization order bugs | topological graph, cycle diagnostics, one-time global initialization tests |
| false Pedestal compatibility claim | separate P1 and P3 fixtures, namespace policy, pinned matrix |
| benchmark misuse | dedicated HTTP suite, response checksum, repetitions, concurrency disclosure |

## Acceptance

This decision is implemented only when all of the following are true:

1. compiler-owned connector modules compile through the static namespace graph;
2. initialized top-level values are permanently rooted and survive GC stress;
3. `test-request` and network traffic use the same Clojure dispatch function;
4. interceptor order, termination, unwind, and recovery match the pinned manual oracle;
5. raw malformed input cannot abort the process or reach application dispatch;
6. every socket and buffer is closed on normal, error, timeout, and signal paths;
7. loopback service tests pass under explicit timeouts with ephemeral ports;
8. runtime C tests pass normally and under ASan/UBSan;
9. `e.pedestal.native_connector_hello` is active while the upstream fixture remains
   correctly classified;
10. the HTTP benchmark is versioned separately from language conformance and algorithm
    benchmarks; and
11. documentation states P0/P1 support and every deliberate divergence without claiming
    complete Pedestal compatibility.

Any move to threads, async interceptors, a Rust provider, a third-party parser,
keep-alive, TLS, HTTP/2, Maven resolution, or direct upstream protocol implementation
requires a superseding ADR.
