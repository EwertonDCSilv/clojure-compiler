#!/usr/bin/env bash
# Run tests, comparative benchmarks, and the deterministic Pages refresh sequence.
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
make_bin="${MAKE_BIN:-make}"
page_data_renderer="${BENCHMARK_PAGE_DATA_RENDERER:-$repo_root/scripts/render-benchmark-page-data.sh}"
cd "$repo_root"

mkdir -p target
measurement_dir="${BENCHMARK_PAGE_WORK_DIR:-$(mktemp -d "$repo_root/target/benchmark-page-refresh.XXXXXX")}"
mkdir -p "$measurement_dir"
trap 'rm -rf "$measurement_dir"' EXIT
cracking_csv="$measurement_dir/cracking.csv"
cormen_csv="$measurement_dir/cormen.csv"
exercism_csv="$measurement_dir/exercism.csv"
page_data="$measurement_dir/data.js"
published_cracking_csv="${BENCHMARK_PAGE_CRACKING_CSV:-benchmarks/cracking/results/extreme.csv}"
published_cormen_csv="${BENCHMARK_PAGE_CORMEN_CSV:-benchmarks/cormen/results/extreme.csv}"
published_exercism_csv="${BENCHMARK_PAGE_EXERCISM_CSV:-benchmarks/exercism/results/extreme.csv}"
published_page_data="${BENCHMARK_PAGE_DATA_OUTPUT:-docs/assets/benchmarks/data.js}"

printf '%s\n' '==> tests'
"$make_bin" test
printf '%s\n' '==> native and Clojure/JVM comparative benchmarks'
"$make_bin" benchmarks-compare \
  "CRACKING_COMPARISON_CSV=$cracking_csv" \
  "CORMEN_COMPARISON_CSV=$cormen_csv" \
  "EXERCISM_COMPARISON_CSV=$exercism_csv"
printf '%s\n' '==> benchmark data consumed by Pages'
"$page_data_renderer" \
  "$cracking_csv" \
  "$cormen_csv" \
  "$exercism_csv" \
  "$page_data"
printf '%s\n' '==> publish validated benchmark artifacts'
mv "$cracking_csv" "$published_cracking_csv"
mv "$cormen_csv" "$published_cormen_csv"
mv "$exercism_csv" "$published_exercism_csv"
mv "$page_data" "$published_page_data"
printf '%s\n' '==> benchmark charts and Pages assets'
"$make_bin" benchmarks-charts
