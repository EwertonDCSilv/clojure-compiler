#!/usr/bin/env bash
set -euo pipefail

if (($# != 1)); then
  printf 'Uso: benchmarks/render-benchmark-summary.sh CAMINHO.csv\n' >&2
  exit 2
fi

csv_path="$1"
if [[ ! -f "${csv_path}" ]]; then
  printf 'CSV não encontrado: %s\n' "${csv_path}" >&2
  exit 1
fi

awk -F, '
  function delta(native_value, clojure_value) {
    if ((clojure_value + 0) == 0) {
      return "n/a"
    }
    difference = 100 * ((native_value + 0) - (clojure_value + 0))
    return sprintf("%+.1f%%", difference / (clojure_value + 0))
  }

  BEGIN {
    print "| Caso | Tempo N/J (s) | Δ tempo | CPU N/J (s) | Δ CPU | RSS N/J (MiB) | Δ RSS |"
    print "| --- | ---: | ---: | ---: | ---: | ---: | ---: |"
  }

  FNR == 1 {
    if ($1 != "benchmark" || $4 != "native_wall_time_s" ||
        $7 != "native_cpu_total_s" || $9 != "native_max_rss_kb" ||
        $12 != "clojure_wall_time_s" || $15 != "clojure_cpu_total_s" ||
        $17 != "clojure_max_rss_kb" || $23 != "status") {
      print "Cabeçalho comparativo incompatível em " FILENAME > "/dev/stderr"
      exit 1
    }
    next
  }

  {
    if ($23 != "OK") {
      printf "%s possui status %s\n", $1, $23 > "/dev/stderr"
      failed = 1
    }
    printf "| `%s` | %.2f / %.2f | %s | %.2f / %.2f | %s | %.1f / %.1f | %s |\n",
      $1, $4, $12, delta($4, $12), $7, $15, delta($7, $15),
      $9 / 1024, $17 / 1024, delta($9, $17)
  }

  END {
    if (failed) {
      exit 1
    }
  }
' "${csv_path}"
