#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
fixture_root="$(mktemp -d)"
trap 'rm -rf "$fixture_root"' EXIT

header="benchmark,scale,native_compile_wall_ms,native_wall_time_s,native_cpu_user_s,native_cpu_system_s,native_cpu_total_s,native_cpu_percent,native_max_rss_kb,clojure_version,clojure_compile_wall_ms,clojure_wall_time_s,clojure_cpu_user_s,clojure_cpu_system_s,clojure_cpu_total_s,clojure_cpu_percent,clojure_max_rss_kb,native_checksum,clojure_checksum,wall_speedup_vs_clojure,cpu_speedup_vs_clojure,rss_ratio_clojure_over_native,status"

for suite in cracking cormen exercism; do
  {
    printf '%s\n' "$header"
    printf '%s\n' "first,1,0,1.25,0,0,2.5,0,100,1.12.5,0,4.5,0,0,5.5,0,400,a,a,0,0,0,OK"
    printf '%s\n' "second,1,0,2.75,0,0,3.5,0,300,1.12.5,0,5.5,0,0,6.5,0,800,b,b,0,0,0,OK"
  } >"$fixture_root/$suite.csv"
done

output="$fixture_root/data.js"
"$repo_root/scripts/render-benchmark-page-data.sh" \
  "$fixture_root/cracking.csv" \
  "$fixture_root/cormen.csv" \
  "$fixture_root/exercism.csv" \
  "$output"

grep -F '"wall": [4.00, 10.00]' "$output"
grep -F '"cpu": [6.00, 12.00]' "$output"
grep -F '"memory": [0.2, 0.6]' "$output"

sed -i 's/,OK$/,MISMATCH/' "$fixture_root/cormen.csv"
if "$repo_root/scripts/render-benchmark-page-data.sh" \
  "$fixture_root/cracking.csv" \
  "$fixture_root/cormen.csv" \
  "$fixture_root/exercism.csv" \
  "$output"; then
  printf '%s\n' "renderer accepted a non-OK benchmark row" >&2
  exit 1
fi
