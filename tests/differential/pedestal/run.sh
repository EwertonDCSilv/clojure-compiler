#!/usr/bin/env bash
# ADR-0013 Gate 3 — interceptor-chain differential vs pinned Pedestal/JVM.
#
# Builds and runs the native corpus (native_chain.clj, compiled cljn.pedestal.chain)
# and the JVM corpus (jvm/src/jvm_chain.clj, pinned io.pedestal.interceptor.chain),
# then diffs their normalized per-scenario output. Identical output proves the
# native interceptor order, termination, unwind, and recovery match the manual
# oracle (ADR-0013 acceptance #4). Any divergence fails the run.
#
# Like the JVM conformance oracle and the HTTP benchmark, this needs a JVM, the
# Clojure CLI, and network access to Clojars/Maven Central, so it runs on demand
# outside CI. The native binary runs under an address-space ulimit and a timeout.
#
# Usage: tests/differential/pedestal/run.sh
set -euo pipefail

here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repo_root="$(cd "$here/../../.." && pwd)"
compiler="${COMPILER:-$repo_root/target/release/clojure-native}"

if command -v clojure >/dev/null 2>&1; then
  clojure_bin="clojure"
elif [ -x "${HOME}/.clojure-cli/bin/clojure" ]; then
  clojure_bin="${HOME}/.clojure-cli/bin/clojure"
else
  printf 'clojure CLI not found; install it to resolve the pinned Pedestal deps\n' >&2
  exit 1
fi

if [ ! -x "$compiler" ]; then
  printf 'building the native compiler...\n' >&2
  (cd "$repo_root" && cargo build --release --locked -p clojure-native-cli >/dev/null)
fi

work="$(mktemp -d)"
trap 'rm -rf "$work"' EXIT
results_dir="$here/results"
mkdir -p "$results_dir"

printf 'building the native corpus...\n' >&2
native_bin="$work/native_chain"
"$compiler" build "$here/native_chain.clj" -o "$native_bin" >/dev/null

printf 'running the native corpus...\n' >&2
( ulimit -v 2000000; exec timeout 30 "$native_bin" ) >"$work/native.txt"

printf 'running the pinned Pedestal corpus...\n' >&2
(cd "$here/jvm" && "$clojure_bin" -M -m jvm-chain) >"$work/jvm.txt"

if diff -u "$work/jvm.txt" "$work/native.txt" >"$work/diff.txt"; then
  cp "$work/native.txt" "$results_dir/observed.txt"
  scenarios="$(wc -l <"$results_dir/observed.txt" | tr -d ' ')"
  printf 'GATE 3 PASS: %s scenarios match the pinned Pedestal/JVM oracle\n' "$scenarios"
  printf '# ADR-0013 Gate 3 differential — interceptor chain\n\nNative cljn.pedestal.chain output equals pinned Pedestal/JVM (0.8.2-beta-10) for every scenario (order, termination, unwind, recovery). Regenerate with `tests/differential/pedestal/run.sh`.\n\n```\n' >"$results_dir/summary.md"
  cat "$results_dir/observed.txt" >>"$results_dir/summary.md"
  printf '```\n' >>"$results_dir/summary.md"
  exit 0
else
  printf 'GATE 3 FAIL: native output diverges from the pinned oracle:\n' >&2
  cat "$work/diff.txt" >&2
  exit 1
fi
