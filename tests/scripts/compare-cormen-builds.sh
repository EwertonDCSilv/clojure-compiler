#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
runner="$repo_root/benchmarks/cormen/compare-ir.sh"

"$runner" --help | rg -q -- '--control-compiler PATH'
if "$runner" --compiler /bin/true --candidate-compiler /missing/compiler > /tmp/cormen-build-gate.out 2>&1; then
  echo "expected a missing candidate compiler to be rejected" >&2
  exit 1
fi
rg -q 'Compilador não encontrado: /missing/compiler' /tmp/cormen-build-gate.out
