#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

min_lines="${COVERAGE_MIN_LINES:-82}"
min_functions="${COVERAGE_MIN_FUNCTIONS:-82}"
min_regions="${COVERAGE_MIN_REGIONS:-82}"
min_file_lines="${COVERAGE_MIN_FILE_LINES:-30}"
baseline="${COVERAGE_BASELINE:-config/coverage-baseline.json}"
report_directory="${COVERAGE_REPORT_DIR:-target/coverage}"
report="$report_directory/coverage.json"
summary="$report_directory/summary.json"

declare -a coverage_command
if cargo llvm-cov --version >/dev/null 2>&1; then
  coverage_command=(cargo llvm-cov)
elif [[ -x "$repo_root/target/coverage-tools/bin/cargo-llvm-cov" ]]; then
  coverage_command=(
    "$repo_root/target/coverage-tools/bin/cargo-llvm-cov"
    llvm-cov
  )
else
  printf '%s\n' \
    "cargo-llvm-cov não está instalado." \
    "Instale com: cargo install cargo-llvm-cov --locked" \
    "E adicione: rustup component add llvm-tools-preview" >&2
  exit 1
fi

printf 'Coverage gates: lines=%s%% functions=%s%% regions=%s%% per-file-lines=%s%%\n' \
  "$min_lines" "$min_functions" "$min_regions" "$min_file_lines"

mkdir -p "$report_directory"

"${coverage_command[@]}" \
  --workspace \
  --json \
  --output-path "$report" \
  --fail-under-lines "$min_lines" \
  --fail-under-functions "$min_functions" \
  --fail-under-regions "$min_regions" \
  --fail-under-file-lines "$min_file_lines" \
  "$@"

scripts/coverage-report.sh summarize \
  --report "$report" --baseline "$baseline" --output "$summary"
scripts/coverage-report.sh check-ratchet --report "$report" --baseline "$baseline"
scripts/coverage-report.sh check-diff \
  --report "$report" --base "${COVERAGE_DIFF_BASE:-origin/master}" \
  --minimum "${COVERAGE_DIFF_MIN_LINES:-90}"

printf 'Coverage reports: %s and %s\n' "$summary" "$report"
