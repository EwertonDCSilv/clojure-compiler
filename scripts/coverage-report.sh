#!/usr/bin/env bash
# Produces and validates deterministic coverage reports from cargo-llvm-cov JSON.
set -euo pipefail

usage() {
  printf '%s\n' \
    'use: coverage-report.sh summarize --report REPORT --baseline BASELINE --output OUTPUT' \
    '     coverage-report.sh check-ratchet --report REPORT --baseline BASELINE' \
    '     coverage-report.sh check-diff --report REPORT [--base GIT_REV] [--minimum PERCENT]'
}

die() {
  printf 'coverage report: %s\n' "$*" >&2
  exit 2
}

command_name="${1:-}"
[[ -n "$command_name" ]] || { usage >&2; exit 2; }
shift

report=""
baseline=""
output=""
base="origin/master"
minimum="90"
while (($#)); do
  case "$1" in
    --report) report="${2:-}"; shift 2 ;;
    --baseline) baseline="${2:-}"; shift 2 ;;
    --output) output="${2:-}"; shift 2 ;;
    --base) base="${2:-}"; shift 2 ;;
    --minimum) minimum="${2:-}"; shift 2 ;;
    *) die "unknown option \`$1\`" ;;
  esac
done

[[ -f "$report" ]] || die "coverage JSON report not found: $report"

summarize() {
  [[ -f "$baseline" ]] || die "coverage baseline not found: $baseline"
  [[ -n "$output" ]] || die '--output is required for summarize'

  jq --slurpfile baseline "$baseline" '
    def percent($covered; $count):
      if $count == 0 then 100 else (($covered * 10000 / $count | round) / 100) end;
    def metrics($files): {
      lines: percent(($files | map(.summary.lines.covered) | add // 0); ($files | map(.summary.lines.count) | add // 0)),
      functions: percent(($files | map(.summary.functions.covered) | add // 0); ($files | map(.summary.functions.count) | add // 0)),
      regions: percent(($files | map(.summary.regions.covered) | add // 0); ($files | map(.summary.regions.count) | add // 0))
    };
    [ .data[].files[]
      | select(.filename | test("/src/compiler/[^/]+/src/"))
      | . + {path: (.filename | capture(".*/(?<path>src/compiler/[^/]+/src/.*)$").path)}
      | . + {crate: (.path | capture("^src/compiler/(?<crate>[^/]+)/").crate)}
    ] as $files
    | {
        version: 1,
        crates: ($files | group_by(.crate) | map({key: .[0].crate, value: metrics(.)}) | from_entries),
        modules: ($files | map({key: .path, value: metrics([.])}) | from_entries),
        baseline: $baseline[0]
      }
  ' "$report" >"$output"
}

check_ratchet() {
  [[ -f "$baseline" ]] || die "coverage baseline not found: $baseline"
  local summary
  summary="$(mktemp)"
  trap 'rm -f "$summary"' RETURN
  "$0" summarize --report "$report" --baseline "$baseline" --output "$summary"
  local failures
  failures="$(jq -r '
    def regressions($actual; $expected; $kind):
      [ $expected | to_entries[]
        | .key as $name
        | .value as $minimum
        | ($actual[$name] // {}) as $current
        | ["lines", "functions", "regions"][] as $metric
        | select(($minimum[$metric] // 0) > ($current[$metric] // -1))
        | "\($kind) \($name) \($metric): \($current[$metric] // "missing")% < baseline \($minimum[$metric])%"
      ];
    (regressions(.crates; .baseline.crates; "crate") + regressions(.modules; .baseline.modules; "module")) as $failures
    | $failures[]
  ' "$summary")"
  if [[ -n "$failures" ]]; then
    printf '%s\n' "$failures" >&2
    return 1
  fi
}

check_diff() {
  git rev-parse --verify --quiet "$base^{commit}" >/dev/null || die "cannot resolve diff base: $base"
  local changed
  changed="$(mktemp)"
  trap 'rm -f "$changed"' RETURN
  git diff --no-ext-diff --unified=0 "$base"...HEAD -- '*.rs' |
    awk '
      /^\+\+\+ b\// { file = substr($0, 7); next }
      /^@@ / {
        split($0, fields, " "); split(fields[3], span, ",");
        start = substr(span[1], 2); count = span[2] == "" ? 1 : span[2];
        for (line = start; line < start + count; line++) print file ":" line;
      }
    ' >"$changed"
  [[ -s "$changed" ]] || return 0

  jq --rawfile changed "$changed" --argjson minimum "$minimum" '
    ($changed | split("\n") | map(select(length > 0) | split(":"))
      | map({path: .[0], line: (.[1] | tonumber)})) as $changed_lines
    | [ .data[].files[]
        | select(.filename | test("/src/compiler/[^/]+/src/"))
        | . + {path: (.filename | capture(".*/(?<path>src/compiler/[^/]+/src/.*)$").path)}
      ] as $files
    | [ $changed_lines[] as $changed_line
        | $files[] | select(.path == $changed_line.path)
        | .segments[]?
        | select(.[0] == $changed_line.line and .[3] == true)
        | {line: $changed_line, covered: (.[2] > 0)}
      ] | unique_by(.line.path, .line.line) as $executable
    | { executable: ($executable | length), covered: ($executable | map(select(.covered)) | length) }
    | . + {percent: (if .executable == 0 then 100 else ((.covered * 10000 / .executable | round) / 100) end)}
    | if .percent < $minimum then error("Rust diff coverage \(.percent)% is below \($minimum)% (\(.covered)/\(.executable) executable lines)") else . end
  ' "$report"
}

case "$command_name" in
  summarize) summarize ;;
  check-ratchet) check_ratchet ;;
  check-diff) check_diff ;;
  *) usage >&2; exit 2 ;;
esac
