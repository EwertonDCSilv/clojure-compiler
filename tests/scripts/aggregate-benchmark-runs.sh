#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
fixture_root="$(mktemp -d)"
trap 'rm -rf "$fixture_root"' EXIT

header='benchmark,scale,native_compile_wall_ms,native_wall_time_s,native_cpu_user_s,native_cpu_system_s,native_cpu_total_s,native_cpu_percent,native_max_rss_kb,clojure_version,clojure_compile_wall_ms,clojure_wall_time_s,clojure_cpu_user_s,clojure_cpu_system_s,clojure_cpu_total_s,clojure_cpu_percent,clojure_max_rss_kb,native_checksum,clojure_checksum,wall_speedup_vs_clojure,cpu_speedup_vs_clojure,rss_ratio_clojure_over_native,status'

inputs=()
for run in $(seq 1 10); do
  input="$fixture_root/run-$run.csv"
  inputs+=("$input")
  sample="$run"
  if [[ "$run" -eq 10 ]]; then
    sample=100
  fi
  {
    printf '%s\n' "$header"
    printf 'case.clj,25,%d,%d,%d,%d,%d,%d%%,%d,1.12.5,%d,%d,%d,%d,%d,%d%%,%d,native-ok,clojure-ok,999,999,999,OK\n' \
      "$sample" "$sample" "$sample" "$sample" "$sample" "$((sample * 10))" \
      "$((sample * 100))" "$((sample * 2))" "$((sample * 2))" "$((sample * 2))" \
      "$((sample * 2))" "$((sample * 2))" "$((sample * 20))" "$((sample * 300))"
  } >"$input"
done

output="$fixture_root/median.csv"
"$repo_root/scripts/aggregate-benchmark-runs.sh" "$output" "${inputs[@]}"

test "$(sed -n '1p' "$output")" = "$header"
# LC_ALL=C: under a locale with a comma decimal separator, awk's numeric string
# coercion (e.g. `$3 + 0`) can stop at the dot in "5.500000" and yield 5 instead
# of 5.5, so this assertion must not depend on the host's locale.
LC_ALL=C awk -F, '
  NR == 2 {
    if ($1 != "case.clj" || $2 != 25 || $10 != "1.12.5" ||
        $18 != "native-ok" || $19 != "clojure-ok" || $23 != "OK")
      exit 1
    if (($3 + 0) != 5.5 || ($4 + 0) != 5.5 || ($7 + 0) != 5.5 ||
        ($8 + 0) != 55 || ($9 + 0) != 550)
      exit 1
    if (($11 + 0) != 11 || ($12 + 0) != 11 || ($15 + 0) != 11 ||
        ($16 + 0) != 110 || ($17 + 0) != 1650)
      exit 1
    if (($20 + 0) != 2 || ($21 + 0) != 2 || ($22 + 0) != 3)
      exit 1
    found = 1
  }
  END { exit !found }
' "$output"

printf '%s\n' sentinel >"$output"
sed -i '2s/native-ok/native-mismatch/' "${inputs[9]}"
if "$repo_root/scripts/aggregate-benchmark-runs.sh" "$output" "${inputs[@]}"; then
  printf '%s\n' "aggregation accepted mismatched checksums" >&2
  exit 1
fi
test "$(cat "$output")" = sentinel
