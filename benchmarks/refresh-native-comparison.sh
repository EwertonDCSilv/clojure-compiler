#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
suite=""
scale=25
opt_level="none"
compiler="${repo_root}/target/release/clojure-native"
csv_path=""

usage() {
  printf '%s\n' \
    "Uso: benchmarks/refresh-native-comparison.sh --suite SUITE [opções]" \
    "" \
    "Atualiza somente as métricas nativas e preserva a referência Clojure/JVM." \
    "" \
    "  --suite SUITE       cracking ou cormen" \
    "  --scale N           Multiplicador interno (padrão: 25)" \
    "  --opt-level LEVEL   none, speed ou speed-and-size (padrão: none)" \
    "  --compiler PATH     Binário clojure-native" \
    "  --csv PATH          CSV comparativo existente e destino da atualização" \
    "  -h, --help          Mostra esta ajuda"
}

while (($# > 0)); do
  case "$1" in
    --suite)
      suite="${2:-}"
      shift 2
      ;;
    --scale)
      scale="${2:-}"
      shift 2
      ;;
    --opt-level)
      opt_level="${2:-}"
      shift 2
      ;;
    --compiler)
      compiler="${2:-}"
      shift 2
      ;;
    --csv)
      csv_path="${2:-}"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      printf 'Opção desconhecida: %s\n' "$1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

case "${suite}" in
  cracking|cormen) ;;
  *)
    printf 'Informe --suite cracking ou --suite cormen.\n' >&2
    exit 2
    ;;
esac

if [[ ! "${scale}" =~ ^[1-9][0-9]*$ ]]; then
  printf 'Escala inválida: %s\n' "${scale}" >&2
  exit 2
fi

case "${opt_level}" in
  none|speed|speed-and-size) ;;
  *)
    printf 'Nível de otimização inválido: %s\n' "${opt_level}" >&2
    exit 2
    ;;
esac

if [[ -z "${csv_path}" ]]; then
  csv_path="${repo_root}/benchmarks/${suite}/results/extreme.csv"
fi
if [[ ! -f "${csv_path}" ]]; then
  printf 'CSV comparativo não encontrado: %s\n' "${csv_path}" >&2
  exit 1
fi
if [[ ! -x "${compiler}" ]]; then
  printf 'Compilador não encontrado: %s\n' "${compiler}" >&2
  exit 1
fi

work_dir="$(mktemp -d "${TMPDIR:-/tmp}/clojure-native-comparison.XXXXXXXX")"
trap 'rm -rf -- "${work_dir}"' EXIT
baseline_csv="${work_dir}/baseline.csv"
native_csv="${work_dir}/native.csv"
merged_csv="${work_dir}/merged.csv"
cp -- "${csv_path}" "${baseline_csv}"

"${repo_root}/benchmarks/${suite}/run.sh" \
  --scale "${scale}" \
  --opt-level "${opt_level}" \
  --compiler "${compiler}" \
  --csv "${native_csv}"

awk -F, -v OFS=, '
  NR == FNR {
    if (FNR == 1) {
      comparison_header = $0
      next
    }
    previous[$1] = $0
    previous_count++
    next
  }

  FNR == 1 {
    print comparison_header
    next
  }

  {
    benchmark = $1
    if (!(benchmark in previous)) {
      printf "Benchmark novo sem referência JVM: %s\n", benchmark > "/dev/stderr"
      failed = 1
      next
    }

    split(previous[benchmark], old, FS)
    if (($3 + 0) != (old[2] + 0)) {
      printf "Escala incompatível em %s: nativo=%s, JVM preservada=%s\n",
        benchmark, $3, old[2] > "/dev/stderr"
      failed = 1
      next
    }

    wall_ratio = ""
    cpu_ratio = ""
    rss_ratio = ""
    status = "OK"
    if ($13 != "OK") {
      status = "NATIVE_" $13
    } else if ($11 != old[19]) {
      status = "CHECKSUM_MISMATCH"
    } else {
      if (($5 + 0) > 0) {
        wall_ratio = sprintf("%.3f", (old[12] + 0) / ($5 + 0))
      }
      if (($8 + 0) > 0) {
        cpu_ratio = sprintf("%.3f", (old[15] + 0) / ($8 + 0))
      }
      if (($10 + 0) > 0) {
        rss_ratio = sprintf("%.3f", (old[17] + 0) / ($10 + 0))
      }
    }

    if (status != "OK") {
      printf "%s: %s\n", benchmark, status > "/dev/stderr"
      failed = 1
    }

    print benchmark, $3, $4, $5, $6, $7, $8, $9, $10,
      old[10], old[11], old[12], old[13], old[14], old[15], old[16], old[17],
      $11, old[19], wall_ratio, cpu_ratio, rss_ratio, status
    delete previous[benchmark]
    native_count++
  }

  END {
    for (benchmark in previous) {
      printf "Referência JVM sem benchmark nativo: %s\n", benchmark > "/dev/stderr"
      failed = 1
    }
    if (native_count != previous_count) {
      printf "Quantidade incompatível: nativo=%d, referência=%d\n",
        native_count, previous_count > "/dev/stderr"
      failed = 1
    }
    if (failed) {
      exit 1
    }
  }
' "${baseline_csv}" "${native_csv}" >"${merged_csv}"

mv -- "${merged_csv}" "${csv_path}"
printf 'Comparação atualizada: %s\n' "${csv_path}"
printf 'As colunas clojure_* e clojure_checksum foram preservadas.\n'
