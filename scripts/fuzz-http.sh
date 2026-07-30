#!/usr/bin/env bash
# libFuzzer harness for the strict HTTP request parser (ADR-0013 Gate 4).
#
# Usage: scripts/fuzz-http.sh [max_total_time_seconds]
# Requires clang with libFuzzer. A short run smoke-tests the target; CI runs it
# continuously against the persisted corpus under src/compiler/clojure-codegen/fuzz-corpus/.
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

cc="${CLANG:-clang}"
if ! "$cc" --version >/dev/null 2>&1; then
  printf 'clang indisponível; pulando o fuzzing HTTP\n' >&2
  exit 0
fi

max_time="${1:-30}"
src="src/compiler/clojure-codegen/tests/fuzz/http_parse_fuzz.c"
corpus="src/compiler/clojure-codegen/fuzz-corpus/http"
work="$(mktemp -d)"
trap 'rm -rf "$work"' EXIT
mkdir -p "$corpus"

# Seed the corpus with a couple of well-formed requests on first run.
if [ -z "$(ls -A "$corpus" 2>/dev/null)" ]; then
  printf 'GET / HTTP/1.1\r\nhost: x\r\n\r\n' >"$corpus/seed-get"
  printf 'POST /x HTTP/1.1\r\ncontent-length: 3\r\n\r\nabc' >"$corpus/seed-post"
fi

"$cc" -std=c11 -g -O1 -fsanitize=fuzzer,address,undefined -fno-omit-frame-pointer \
  -I "src/compiler/clojure-codegen" "$src" -o "$work/http_fuzz"

ASAN_OPTIONS="${ASAN_OPTIONS:-detect_leaks=0}" \
  "$work/http_fuzz" -max_total_time="$max_time" -print_final_stats=1 "$corpus"
