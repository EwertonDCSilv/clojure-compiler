#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
fixture_root="$(mktemp -d)"
trap 'rm -rf "$fixture_root"' EXIT

fake_make="$fixture_root/make"
printf '%s\n' \
  '#!/usr/bin/env bash' \
  'printf "%s\n" "$*" >> "$BENCHMARK_PAGE_LOG"' \
  'if [[ "$1" = benchmarks-compare ]]; then' \
  '  count_file="${BENCHMARK_PAGE_COUNT_FILE:?}"' \
  '  count=0' \
  '  [[ ! -f "$count_file" ]] || read -r count < "$count_file"' \
  '  count=$((count + 1))' \
  '  printf "%s\n" "$count" > "$count_file"' \
  '  if [[ "${BENCHMARK_PAGE_FAIL_ROUND:-0}" = "$count" ]]; then exit 1; fi' \
  'fi' \
  'for argument in "$@"; do' \
  '  case "$argument" in' \
  '    *_COMPARISON_CSV=*) touch "${argument#*=}" ;;' \
  '  esac' \
  'done' \
  >"$fake_make"
chmod +x "$fake_make"

fake_aggregator="$fixture_root/aggregate-runs"
printf '%s\n' \
  '#!/usr/bin/env bash' \
  'printf "aggregate" >> "$BENCHMARK_PAGE_LOG"' \
  'printf " %s" "$@" >> "$BENCHMARK_PAGE_LOG"' \
  'printf "\n" >> "$BENCHMARK_PAGE_LOG"' \
  'touch "$1"' >"$fake_aggregator"
chmod +x "$fake_aggregator"

fake_renderer="$fixture_root/render-page-data"
printf '%s\n' \
  '#!/usr/bin/env bash' \
  'printf "%s\n" "$*" >> "$BENCHMARK_PAGE_LOG"' \
  'touch "$4"' >"$fake_renderer"
chmod +x "$fake_renderer"

log="$fixture_root/calls"
count_file="$fixture_root/count"
work_dir="$fixture_root/work"
published_dir="$fixture_root/published"
mkdir -p "$published_dir"
MAKE_BIN="$fake_make" \
  BENCHMARK_PAGE_AGGREGATOR="$fake_aggregator" \
  BENCHMARK_PAGE_DATA_RENDERER="$fake_renderer" \
  BENCHMARK_PAGE_WORK_DIR="$work_dir" \
  BENCHMARK_PAGE_CRACKING_CSV="$published_dir/cracking.csv" \
  BENCHMARK_PAGE_CORMEN_CSV="$published_dir/cormen.csv" \
  BENCHMARK_PAGE_EXERCISM_CSV="$published_dir/exercism.csv" \
  BENCHMARK_PAGE_DATA_OUTPUT="$published_dir/data.js" \
  BENCHMARK_PAGE_COUNT_FILE="$count_file" \
  BENCHMARK_PAGE_LOG="$log" \
  "$repo_root/scripts/refresh-benchmark-page.sh"
test "$(sed -n '1p' "$log")" = test
for run in $(seq -w 1 10); do
  line=$((10#$run + 1))
  test "$(sed -n "${line}p" "$log")" = \
    "benchmarks-compare CRACKING_COMPARISON_CSV=$work_dir/round-$run/cracking.csv CORMEN_COMPARISON_CSV=$work_dir/round-$run/cormen.csv EXERCISM_COMPARISON_CSV=$work_dir/round-$run/exercism.csv"
  test -f "$work_dir/round-$run/cracking.csv"
done
test "$(sed -n '12p' "$log")" = \
  "aggregate $work_dir/cracking.csv $work_dir/round-01/cracking.csv $work_dir/round-02/cracking.csv $work_dir/round-03/cracking.csv $work_dir/round-04/cracking.csv $work_dir/round-05/cracking.csv $work_dir/round-06/cracking.csv $work_dir/round-07/cracking.csv $work_dir/round-08/cracking.csv $work_dir/round-09/cracking.csv $work_dir/round-10/cracking.csv"
test "$(sed -n '15p' "$log")" = \
  "$work_dir/cracking.csv $work_dir/cormen.csv $work_dir/exercism.csv $work_dir/data.js"
test "$(sed -n '16p' "$log")" = benchmarks-charts
test "$(cat "$count_file")" = 10
test -f "$published_dir/cracking.csv"
test -f "$published_dir/cormen.csv"
test -f "$published_dir/exercism.csv"
test -f "$published_dir/data.js"

failure_work_dir="$fixture_root/failure-work"
failure_count_file="$fixture_root/failure-count"
printf '%s\n' sentinel >"$published_dir/cracking.csv"
if MAKE_BIN="$fake_make" \
  BENCHMARK_PAGE_AGGREGATOR="$fake_aggregator" \
  BENCHMARK_PAGE_DATA_RENDERER="$fake_renderer" \
  BENCHMARK_PAGE_WORK_DIR="$failure_work_dir" \
  BENCHMARK_PAGE_CRACKING_CSV="$published_dir/cracking.csv" \
  BENCHMARK_PAGE_CORMEN_CSV="$published_dir/cormen.csv" \
  BENCHMARK_PAGE_EXERCISM_CSV="$published_dir/exercism.csv" \
  BENCHMARK_PAGE_DATA_OUTPUT="$published_dir/data.js" \
  BENCHMARK_PAGE_FAIL_ROUND=4 \
  BENCHMARK_PAGE_COUNT_FILE="$failure_count_file" \
  BENCHMARK_PAGE_LOG="$log" \
  "$repo_root/scripts/refresh-benchmark-page.sh"; then
  printf '%s\n' "refresh accepted a failed benchmark comparison" >&2
  exit 1
fi
test "$(cat "$failure_count_file")" = 4
test "$(cat "$published_dir/cracking.csv")" = sentinel
