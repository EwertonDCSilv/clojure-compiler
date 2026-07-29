#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
fixture_root="$(mktemp -d)"
trap 'rm -rf "$fixture_root"' EXIT

fake_make="$fixture_root/make"
printf '%s\n' \
  '#!/usr/bin/env bash' \
  'printf "%s\n" "$*" >> "$BENCHMARK_PAGE_LOG"' \
  'for argument in "$@"; do' \
  '  case "$argument" in' \
  '    *_COMPARISON_CSV=*) touch "${argument#*=}" ;;' \
  '  esac' \
  'done' \
  'if [[ "${BENCHMARK_PAGE_FAIL_COMPARE:-0}" = 1 && "$1" = benchmarks-compare ]]; then exit 1; fi' \
  >"$fake_make"
chmod +x "$fake_make"

fake_renderer="$fixture_root/render-page-data"
printf '%s\n' \
  '#!/usr/bin/env bash' \
  'printf "%s\n" "$*" >> "$BENCHMARK_PAGE_LOG"' \
  'touch "$4"' >"$fake_renderer"
chmod +x "$fake_renderer"

log="$fixture_root/calls"
work_dir="$fixture_root/work"
published_dir="$fixture_root/published"
mkdir -p "$work_dir"
mkdir -p "$published_dir"
MAKE_BIN="$fake_make" \
  BENCHMARK_PAGE_DATA_RENDERER="$fake_renderer" \
  BENCHMARK_PAGE_WORK_DIR="$work_dir" \
  BENCHMARK_PAGE_CRACKING_CSV="$published_dir/cracking.csv" \
  BENCHMARK_PAGE_CORMEN_CSV="$published_dir/cormen.csv" \
  BENCHMARK_PAGE_EXERCISM_CSV="$published_dir/exercism.csv" \
  BENCHMARK_PAGE_DATA_OUTPUT="$published_dir/data.js" \
  BENCHMARK_PAGE_LOG="$log" \
  "$repo_root/scripts/refresh-benchmark-page.sh"
test "$(sed -n '1p' "$log")" = test
test "$(sed -n '2p' "$log")" = \
  "benchmarks-compare CRACKING_COMPARISON_CSV=$work_dir/cracking.csv CORMEN_COMPARISON_CSV=$work_dir/cormen.csv EXERCISM_COMPARISON_CSV=$work_dir/exercism.csv"
test "$(sed -n '3p' "$log")" = \
  "$work_dir/cracking.csv $work_dir/cormen.csv $work_dir/exercism.csv $work_dir/data.js"
test "$(sed -n '4p' "$log")" = benchmarks-charts
test -f "$published_dir/cracking.csv"
test -f "$published_dir/cormen.csv"
test -f "$published_dir/exercism.csv"
test -f "$published_dir/data.js"

failure_work_dir="$fixture_root/failure-work"
printf '%s\n' sentinel >"$published_dir/cracking.csv"
if MAKE_BIN="$fake_make" \
  BENCHMARK_PAGE_DATA_RENDERER="$fake_renderer" \
  BENCHMARK_PAGE_WORK_DIR="$failure_work_dir" \
  BENCHMARK_PAGE_CRACKING_CSV="$published_dir/cracking.csv" \
  BENCHMARK_PAGE_CORMEN_CSV="$published_dir/cormen.csv" \
  BENCHMARK_PAGE_EXERCISM_CSV="$published_dir/exercism.csv" \
  BENCHMARK_PAGE_DATA_OUTPUT="$published_dir/data.js" \
  BENCHMARK_PAGE_FAIL_COMPARE=1 \
  BENCHMARK_PAGE_LOG="$log" \
  "$repo_root/scripts/refresh-benchmark-page.sh"; then
  printf '%s\n' "refresh accepted a failed benchmark comparison" >&2
  exit 1
fi
test "$(cat "$published_dir/cracking.csv")" = sentinel
