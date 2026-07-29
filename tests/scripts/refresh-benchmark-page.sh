#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
fixture_root="$(mktemp -d)"
trap 'rm -rf "$fixture_root"' EXIT

fake_make="$fixture_root/make"
printf '%s\n' '#!/usr/bin/env bash' 'printf "%s\n" "$*" >> "$BENCHMARK_PAGE_LOG"' >"$fake_make"
chmod +x "$fake_make"

log="$fixture_root/calls"
MAKE_BIN="$fake_make" BENCHMARK_PAGE_LOG="$log" "$repo_root/scripts/refresh-benchmark-page.sh"
test "$(sed -n '1p' "$log")" = test
test "$(sed -n '2p' "$log")" = benchmarks
test "$(sed -n '3p' "$log")" = benchmarks-charts
