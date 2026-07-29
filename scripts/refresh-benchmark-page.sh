#!/usr/bin/env bash
# Run the reproducible test, native benchmark, and Pages-chart refresh sequence.
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
make_bin="${MAKE_BIN:-make}"
cd "$repo_root"

printf '%s\n' '==> tests'
"$make_bin" test
printf '%s\n' '==> native benchmarks'
"$make_bin" benchmarks
printf '%s\n' '==> benchmark charts and Pages assets'
"$make_bin" benchmarks-charts
