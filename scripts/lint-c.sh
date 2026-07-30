#!/usr/bin/env bash
# Compile every maintained C entry point in syntax-only mode with strict warnings.
set -euo pipefail

repository="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
c_compiler="${CC:-cc}"

if ! command -v "${c_compiler}" >/dev/null 2>&1; then
  printf 'C compiler not found: %s\n' "${c_compiler}" >&2
  printf 'Set CC to a GCC- or Clang-compatible C compiler.\n' >&2
  exit 1
fi

flags=(
  -std=c11
  -Wall
  -Wextra
  -Werror
  -Wshadow
  -Wstrict-prototypes
  -Wformat=2
  -Wundef
  -Wwrite-strings
  -Wcast-align
  -Wpointer-arith
  # The current amalgamated runtime has one intentionally compact statement;
  # all other enabled warnings remain errors.
  -Wno-misleading-indentation
  -fsyntax-only
)

sources=(
  src/compiler/clojure-codegen/runtime.c
  src/compiler/clojure-codegen/tests/c/runtime_unit.c
  src/compiler/clojure-codegen/tests/c/runtime_abi.c
  src/compiler/clojure-codegen/tests/c/runtime_errors.c
)

cd "${repository}"
for source in "${sources[@]}"; do
  printf 'C lint: %s\n' "${source}"
  "${c_compiler}" "${flags[@]}" "${source}"
done
