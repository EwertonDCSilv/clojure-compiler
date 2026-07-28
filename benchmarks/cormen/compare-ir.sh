#!/usr/bin/env bash
set -u

suite_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repo_root="$(cd "$suite_dir/../.." && pwd)"
runner="$suite_dir/run.sh"
compiler="$repo_root/target/release/clojure-native"
analyzer_source="$repo_root/benchmarks/analyze-ir-ab.rs"
analyzer="$repo_root/target/benchmark-tools/analyze-ir-ab"
repetitions=7
scale=25
raw_path="$suite_dir/results/ir-ab-raw.csv"
report_path="$suite_dir/results/ir-ab-report.md"
chapter=""
control_ir_opt="none"
candidate_ir_opt="safe"
candidate_experiment="none"

usage() {
  printf '%s\n' \
    "Uso: benchmarks/cormen/compare-ir.sh [opções]" \
    "" \
    "Compara dois perfis nativos no mesmo commit." \
    "A ordem control/candidate é alternada para reduzir viés temporal." \
    "" \
    "  --repetitions N    Pares por caso (padrão: 7; mínimo do gate: 7)" \
    "  --scale N          Multiplicador da carga (padrão: 25)" \
    "  --chapter PREFIX   Restringe a um capítulo; não satisfaz o gate global" \
    "  --compiler PATH    Usa um compilador release específico" \
    "  --control-ir-opt M IR do controle: none ou safe (padrão: none)" \
    "  --candidate-ir-opt M IR candidata: none ou safe (padrão: safe)" \
    "  --candidate-experiment ID  Experimento candidato: none ou adr15" \
    "  --raw PATH         CSV de amostras brutas" \
    "  --report PATH      Relatório Markdown agregado" \
    "  -h, --help         Mostra esta ajuda"
}

while (($# > 0)); do
  case "$1" in
    --repetitions)
      repetitions="${2:-}"
      shift 2
      ;;
    --scale)
      scale="${2:-}"
      shift 2
      ;;
    --chapter)
      chapter="${2:-}"
      shift 2
      ;;
    --compiler)
      compiler="${2:-}"
      shift 2
      ;;
    --control-ir-opt)
      control_ir_opt="${2:-}"
      shift 2
      ;;
    --candidate-ir-opt)
      candidate_ir_opt="${2:-}"
      shift 2
      ;;
    --candidate-experiment)
      candidate_experiment="${2:-}"
      shift 2
      ;;
    --raw)
      raw_path="${2:-}"
      shift 2
      ;;
    --report)
      report_path="${2:-}"
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

if [[ ! "$repetitions" =~ ^[1-9][0-9]*$ ]]; then
  printf 'Número de repetições inválido: %s\n' "$repetitions" >&2
  exit 2
fi
if [[ ! "$scale" =~ ^[1-9][0-9]*$ ]]; then
  printf 'Escala inválida: %s\n' "$scale" >&2
  exit 2
fi
case "$control_ir_opt:$candidate_ir_opt:$candidate_experiment" in
  none:none:none|none:safe:none|safe:none:none|safe:safe:none|safe:safe:adr15) ;;
  *)
    printf 'Combinação de perfis IR inválida\n' >&2
    exit 2
    ;;
esac

if [[ ! -x "$compiler" ]]; then
  cargo build --manifest-path "$repo_root/Cargo.toml" --release \
    -p clojure-native-cli || exit 1
fi

mkdir -p "$(dirname "$analyzer")" "$(dirname "$raw_path")" "$(dirname "$report_path")"
rustc --edition=2021 -D warnings -O "$analyzer_source" -o "$analyzer" || exit 1
metadata_path="${raw_path%.csv}.metadata.txt"

{
  printf 'schema=clojure-compiler-ir-ab-v1\n'
  printf 'git_head=%s\n' "$(git -C "$repo_root" rev-parse HEAD)"
  if [[ -n "$(git -C "$repo_root" status --short)" ]]; then
    printf 'git_worktree=dirty\n'
  else
    printf 'git_worktree=clean\n'
  fi
  printf 'recorded_at_utc=%s\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)"
  printf 'kernel=%s\n' "$(uname -srmo)"
  printf 'cpu=%s\n' "$(sed -n 's/^model name[[:space:]]*: //p' /proc/cpuinfo | head -n 1)"
  printf 'affinity=%s\n' "$(taskset -pc $$ 2>/dev/null | sed 's/^.*: //')"
  if compgen -G '/sys/devices/system/cpu/cpu*/cpufreq/scaling_governor' >/dev/null; then
    printf 'cpu_governor=%s\n' "$(
      head -n 1 /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor | head -n 1
    )"
  else
    printf 'cpu_governor=unavailable\n'
  fi
  printf 'rustc=%s\n' "$(rustc --version)"
  printf 'cc=%s\n' "$(cc --version | head -n 1)"
  printf 'cranelift_codegen=%s\n' "$(
    awk '/name = "cranelift-codegen"/{found=1; next}
         found && /version = /{gsub(/"/, "", $3); print $3; exit}' "$repo_root/Cargo.lock"
  )"
  printf 'compiler_sha256=%s\n' "$(sha256sum "$compiler" | awk '{print $1}')"
  printf 'repetitions=%s\n' "$repetitions"
  printf 'scale=%s\n' "$scale"
  printf 'chapter=%s\n' "${chapter:-all}"
  printf 'control=--ir-opt %s --opt-level none\n' "$control_ir_opt"
  printf 'candidate=--ir-opt %s --ir-experiment %s --opt-level none\n' \
    "$candidate_ir_opt" "$candidate_experiment"
} > "$metadata_path"

temporary="$(mktemp -d)"
cleanup() {
  if [[ "$temporary" == /tmp/* ]]; then
    rm -rf -- "$temporary"
  fi
}
trap cleanup EXIT

printf '%s\n' \
  "benchmark,repetition,order,profile,wall_time_s,cpu_total_s,max_rss_kb,checksum,status" \
  > "$raw_path"

append_samples() {
  local csv="$1"
  local repetition="$2"
  local order="$3"
  local profile="$4"
  while IFS=, read -r benchmark _mode _scale _compile wall _user _system cpu \
    _cpu_percent rss checksum _expected status; do
    if [[ "$benchmark" == "benchmark" ]]; then
      continue
    fi
    printf '%s,%s,%s,%s,%s,%s,%s,%s,%s\n' \
      "$benchmark" "$repetition" "$order" "$profile" "$wall" "$cpu" \
      "$rss" "$checksum" "$status" >> "$raw_path"
  done < "$csv"
}

run_profile() {
  local profile="$1"
  local repetition="$2"
  local order="$3"
  local ir_opt="$4"
  local experiment="$5"
  local csv="$temporary/$repetition-$order-$profile.csv"
  local -a arguments=(--scale "$scale" --ir-opt "$ir_opt" --csv "$csv")
  if [[ "$experiment" != "none" ]]; then
    arguments+=(--ir-experiment "$experiment")
  fi
  if [[ -n "$chapter" ]]; then
    arguments+=(--chapter "$chapter")
  fi
  printf 'Repetição %s/%s, ordem %s: %s (IR %s, experimento %s)\n' \
    "$repetition" "$repetitions" "$order" "$profile" "$ir_opt" "$experiment" >&2
  "$runner" "${arguments[@]}" >"$temporary/$repetition-$order-$profile.log" || return 1
  append_samples "$csv" "$repetition" "$order" "$profile"
}

for ((repetition = 1; repetition <= repetitions; repetition++)); do
  if ((repetition % 2 == 1)); then
    run_profile control "$repetition" 1 "$control_ir_opt" none || exit 1
    run_profile candidate "$repetition" 2 "$candidate_ir_opt" \
      "$candidate_experiment" || exit 1
  else
    run_profile candidate "$repetition" 1 "$candidate_ir_opt" \
      "$candidate_experiment" || exit 1
    run_profile control "$repetition" 2 "$control_ir_opt" none || exit 1
  fi
done

declare -a analyzer_args=(
  "$raw_path"
  "$report_path"
  "--repetitions"
  "$repetitions"
  "--scale"
  "$scale"
  "--control-profile"
  "control"
  "--candidate-profile"
  "candidate"
  "--control-label"
  "--ir-opt $control_ir_opt --opt-level none"
  "--candidate-label"
  "--ir-opt $candidate_ir_opt --ir-experiment $candidate_experiment --opt-level none"
)
if [[ -n "$chapter" ]]; then
  analyzer_args+=("--partial")
fi
"$analyzer" "${analyzer_args[@]}"
