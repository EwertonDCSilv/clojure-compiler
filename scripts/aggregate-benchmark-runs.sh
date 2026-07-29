#!/usr/bin/env bash
# Aggregate ten compatible benchmark CSVs into per-case medians.
set -euo pipefail

if [[ "$#" -ne 11 ]]; then
  printf '%s\n' \
    "Uso: scripts/aggregate-benchmark-runs.sh SAIDA.csv RODADA-01.csv ... RODADA-10.csv" >&2
  exit 2
fi

output="$1"
shift
output_dir="$(dirname "$output")"
mkdir -p "$output_dir"
temporary="$(mktemp "$output_dir/.benchmark-median.XXXXXX")"
trap 'rm -f "$temporary"' EXIT

expected_header='benchmark,scale,native_compile_wall_ms,native_wall_time_s,native_cpu_user_s,native_cpu_system_s,native_cpu_total_s,native_cpu_percent,native_max_rss_kb,clojure_version,clojure_compile_wall_ms,clojure_wall_time_s,clojure_cpu_user_s,clojure_cpu_system_s,clojure_cpu_total_s,clojure_cpu_percent,clojure_max_rss_kb,native_checksum,clojure_checksum,wall_speedup_vs_clojure,cpu_speedup_vs_clojure,rss_ratio_clojure_over_native,status'

awk -F, -v OFS=, -v expected_header="$expected_header" '
  function reject(message) {
    printf "%s\n", message > "/dev/stderr"
    failed = 1
    exit 1
  }
  function is_metric(column) {
    return (column >= 3 && column <= 9) || (column >= 11 && column <= 17)
  }
  function numeric(value) {
    return value ~ /^[-+]?[0-9]+([.][0-9]+)?([eE][-+]?[0-9]+)?$/
  }
  function median(row, column, sorted, count, i, j, current) {
    count = file_count
    for (i = 1; i <= count; i++)
      sorted[i] = samples[row, column, i]
    for (i = 2; i <= count; i++) {
      current = sorted[i]
      j = i - 1
      while (j >= 1 && sorted[j] > current) {
        sorted[j + 1] = sorted[j]
        j--
      }
      sorted[j + 1] = current
    }
    return (sorted[count / 2] + sorted[count / 2 + 1]) / 2
  }
  function metric_text(column, value) {
    if (column == 8 || column == 16)
      return sprintf("%.3f%%", value)
    return sprintf("%.6f", value)
  }
  FNR == 1 {
    file_count++
    if ($0 != expected_header)
      reject(sprintf("schema de benchmark inesperado em %s", FILENAME))
    next
  }
  {
    if (NF != 23)
      reject(sprintf("linha CSV inválida em %s:%d", FILENAME, FNR))

    row = FNR - 1
    row_counts[file_count]++
    if ($23 != "OK")
      reject(sprintf("benchmark %s tem status %s em %s", $1, $23, FILENAME))

    if (file_count == 1) {
      stable[row, 1] = $1
      stable[row, 2] = $2
      stable[row, 10] = $10
      stable[row, 18] = $18
      stable[row, 19] = $19
      stable[row, 23] = $23
    } else if ($1 != stable[row, 1] || $2 != stable[row, 2] ||
               $10 != stable[row, 10] || $18 != stable[row, 18] ||
               $19 != stable[row, 19] || $23 != stable[row, 23]) {
      reject(sprintf("caso, escala, versão, checksum ou status divergente em %s:%d",
                     FILENAME, FNR))
    }

    for (column = 1; column <= 23; column++) {
      if (!is_metric(column))
        continue
      value = $column
      if (column == 8 || column == 16) {
        if (value !~ /%$/)
          reject(sprintf("percentual inválido em %s:%d", FILENAME, FNR))
        sub(/%$/, "", value)
      }
      if (!numeric(value))
        reject(sprintf("métrica não numérica em %s:%d", FILENAME, FNR))
      samples[row, column, file_count] = value + 0
    }
  }
  END {
    if (failed)
      exit 1
    if (file_count != 10)
      reject(sprintf("esperadas 10 rodadas, recebidas %d", file_count))
    rows = row_counts[1]
    if (rows == 0)
      reject("as rodadas de benchmark não contêm casos")
    for (run = 2; run <= file_count; run++) {
      if (row_counts[run] != rows)
        reject(sprintf("quantidade de casos divergente na rodada %d", run))
    }

    print expected_header
    for (row = 1; row <= rows; row++) {
      native_wall = median(row, 4)
      native_cpu = median(row, 7)
      native_rss = median(row, 9)
      clojure_wall = median(row, 12)
      clojure_cpu = median(row, 15)
      clojure_rss = median(row, 17)

      line = ""
      for (column = 1; column <= 23; column++) {
        if (is_metric(column))
          field = metric_text(column, median(row, column))
        else if (column == 20)
          field = native_wall == 0 ? "" : sprintf("%.3f", clojure_wall / native_wall)
        else if (column == 21)
          field = native_cpu == 0 ? "" : sprintf("%.3f", clojure_cpu / native_cpu)
        else if (column == 22)
          field = native_rss == 0 ? "" : sprintf("%.3f", clojure_rss / native_rss)
        else
          field = stable[row, column]
        line = line (column == 1 ? "" : OFS) field
      }
      print line
    }
  }
' "$@" >"$temporary"

mv "$temporary" "$output"
trap - EXIT
