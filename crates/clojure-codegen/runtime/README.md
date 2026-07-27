# Runtime C modules

[Project README](../../../README.md) ·
[Architecture guide](../../../docs/architecture.md) ·
[Runtime specification](../../../specs/RUNTIME_SPEC.md)

The native runtime is physically split by subsystem but remains one C translation unit.
`clojure-codegen` concatenates the fragments with `include_str!`; `runtime_all.c` and the
compatibility entry point `../runtime.c` provide the same ordering to C-native tools.
This preserves the existing ABI, internal `static` visibility, and cross-subsystem
optimization behavior.

The order is dependency-sensitive:

| Module | Responsibility |
| --- | --- |
| `00_types.c` | headers, tagged values, object layouts and forward declarations |
| `10_gc.c` | shadow stack, slab allocator, tracing and sweeping |
| `20_values_and_functions.c` | strings, cons cells, functions, calls and keywords |
| `30_vector.c` | persistent vector trie |
| `40_hash_collections.c` | sets, hashing, HAMT and maps |
| `50_sorted_collections.c` | persistent LLRB maps and sets |
| `60_records_and_dispatch.c` | records, protocols and generic collection dispatch |
| `70_transients.c` | transient vectors and boxed transient maps/sets |
| `80_core_operations.c` | arithmetic, equality, predicates and sequence operations |
| `90_print.c` | value rendering and string conversion |
| `100_exceptions.c` | native throw, catch and finally |
| `110_multimethods.c` | multimethod registration and invocation |
| `120_test_introspection.c` | GC observations used only by tests |

Fragments are not independent libraries and must not define duplicate runtime state.
When adding or moving a module, update both `runtime_all.c` and the
`embed_runtime_modules!` invocation in `src/lib.rs`, preserving their common order.

Run the dedicated gates with:

```bash
make test-runtime
make test-runtime-sanitize
```

Run these commands from the repository root. The underlying
`scripts/test-runtime-c.sh` entry point remains available for low-level debugging.
