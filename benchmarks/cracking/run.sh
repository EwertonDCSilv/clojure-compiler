#!/usr/bin/env bash
set -u

default_script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
script_dir="${CLJN_BENCHMARK_SUITE_DIR:-$default_script_dir}"
repo_root="$(cd "$script_dir/../.." && pwd)"
compiler="$repo_root/target/release/clojure-native"
opt_level=""
chapter=""
gc_stress=0
list_only=0
keep_binaries=0
scale=1
csv_path=""

usage() {
  printf '%s\n' \
    "Uso: benchmarks/cracking/run.sh [opções]" \
    "" \
    "  --chapter PREFIX    Executa um capítulo, por exemplo 04" \
    "  --compiler PATH     Usa um binário específico do compilador" \
    "  --opt-level LEVEL   Usa none, speed ou speed-and-size no Cranelift" \
    "  --gc-stress         Executa com CLJN_GC_STRESS=1" \
    "  --extreme           Multiplica a carga interna por 25" \
    "  --scale N           Multiplica a carga interna por N" \
    "  --csv PATH          Também grava o resultado CSV em PATH" \
    "  --keep-binaries     Preserva os executáveis temporários" \
    "  --list              Lista os benchmarks selecionados" \
    "  -h, --help          Mostra esta ajuda"
}

while (($# > 0)); do
  case "$1" in
    --chapter)
      chapter="${2:-}"
      shift 2
      ;;
    --compiler)
      compiler="${2:-}"
      shift 2
      ;;
    --opt-level)
      opt_level="${2:-}"
      shift 2
      ;;
    --gc-stress)
      gc_stress=1
      shift
      ;;
    --extreme)
      scale=25
      shift
      ;;
    --scale)
      scale="${2:-}"
      shift 2
      ;;
    --csv)
      csv_path="${2:-}"
      shift 2
      ;;
    --keep-binaries)
      keep_binaries=1
      shift
      ;;
    --list)
      list_only=1
      shift
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

if [[ ! "$scale" =~ ^[1-9][0-9]*$ ]]; then
  printf 'Escala inválida: %s\n' "$scale" >&2
  exit 2
fi

case "$opt_level" in
  ""|none|speed|speed-and-size) ;;
  *)
    printf 'Nível de otimização inválido: %s\n' "$opt_level" >&2
    exit 2
    ;;
esac

declare -a sources=()
while IFS= read -r source; do
  relative="${source#"$script_dir/"}"
  if [[ -z "$chapter" || "$relative" == "$chapter"-*/* ]]; then
    sources+=("$source")
  fi
done < <(find "$script_dir" -mindepth 2 -maxdepth 2 -type f -name '*.clj' | sort)

if ((${#sources[@]} == 0)); then
  printf 'Nenhum benchmark encontrado para o capítulo %s\n' "$chapter" >&2
  exit 2
fi

if ((list_only)); then
  for source in "${sources[@]}"; do
    printf '%s\n' "${source#"$script_dir/"}"
  done
  exit 0
fi

if [[ ! -x "$compiler" ]]; then
  printf 'Compilador não encontrado; construindo release...\n' >&2
  if ! cargo build --manifest-path "$repo_root/Cargo.toml" --release -p clojure-native-cli; then
    exit 1
  fi
fi

if [[ ! -x /usr/bin/time ]]; then
  printf 'GNU time não encontrado em /usr/bin/time\n' >&2
  exit 1
fi

declare -A expected=()
while IFS=$'\t' read -r path checksum; do
  expected["$path"]="$checksum"
done < "$script_dir/expected.tsv"

output_dir="$(mktemp -d)"
cleanup() {
  if ((keep_binaries)); then
    printf 'Binários preservados em %s\n' "$output_dir" >&2
  elif [[ "$output_dir" == /tmp/* ]]; then
    rm -rf -- "$output_dir"
  fi
}
trap cleanup EXIT

failures=0
mode="normal"
if ((scale > 1)); then
  mode="extreme"
fi
if ((gc_stress)); then
  mode="$mode-gc-stress"
fi

if [[ -n "$csv_path" ]]; then
  mkdir -p "$(dirname "$csv_path")"
  : > "$csv_path"
  exec 3>>"$csv_path"
fi

emit_row() {
  printf '%s\n' "$1"
  if [[ -n "$csv_path" ]]; then
    printf '%s\n' "$1" >&3
  fi
}

header="benchmark,mode,scale,compile_wall_ms,wall_time_s,cpu_user_s,cpu_system_s,cpu_total_s,cpu_percent,max_rss_kb,checksum,expected,status"
emit_row "$header"

for source in "${sources[@]}"; do
  relative="${source#"$script_dir/"}"
  safe_name="${relative//\//_}"
  executable="$output_dir/${safe_name%.clj}"
  build_source="$source"
  wanted="${expected[$relative]:-missing}"

  if ((scale > 1)); then
    build_source="$output_dir/${safe_name%.clj}.scaled.clj"
    sed -E "s/\\(benchmark ([0-9]+)\\)/\\(benchmark (* \\1 $scale)\\)/" \
      "$source" > "$build_source"
    if cmp -s "$source" "$build_source"; then
      row="$(printf '%s,%s,%s,0,,,,,,,,%s,SCALE_FAIL' \
        "$relative" "$mode" "$scale" "$wanted")"
      emit_row "$row"
      failures=$((failures + 1))
      continue
    fi
    wanted="not-recorded"
  fi

  declare -a build_args=(build "$build_source" -o "$executable")
  if [[ -n "$opt_level" ]]; then
    build_args+=(--opt-level "$opt_level")
  fi

  compile_start="$(date +%s%N)"
  if ! build_output="$("$compiler" "${build_args[@]}" 2>&1)"; then
    compile_end="$(date +%s%N)"
    compile_ms=$(((compile_end - compile_start) / 1000000))
    row="$(printf '%s,%s,%s,%s,,,,,,,,%s,BUILD_FAIL' \
      "$relative" "$mode" "$scale" "$compile_ms" "$wanted")"
    emit_row "$row"
    printf '%s\n' "$build_output" >&2
    failures=$((failures + 1))
    continue
  fi
  compile_end="$(date +%s%N)"

  metrics_file="$output_dir/${safe_name%.clj}.metrics"
  stdout_file="$output_dir/${safe_name%.clj}.stdout"
  stderr_file="$output_dir/${safe_name%.clj}.stderr"
  if ((gc_stress)); then
    /usr/bin/time -f '%e\t%U\t%S\t%P\t%M' -o "$metrics_file" \
      env CLJN_GC_STRESS=1 "$executable" >"$stdout_file" 2>"$stderr_file"
    run_status=$?
  else
    /usr/bin/time -f '%e\t%U\t%S\t%P\t%M' -o "$metrics_file" \
      "$executable" >"$stdout_file" 2>"$stderr_file"
    run_status=$?
  fi

  compile_ms=$(((compile_end - compile_start) / 1000000))
  IFS=$'\t' read -r wall_time user_cpu system_cpu cpu_percent max_rss < "$metrics_file"
  cpu_total="$(awk -v usr="$user_cpu" -v sys="$system_cpu" \
    'BEGIN { printf "%.6f", usr + sys }')"
  run_output="$(<"$stdout_file")"
  checksum="${run_output//$'\n'/}"

  if ((run_status != 0)); then
    status="RUN_FAIL"
    failures=$((failures + 1))
    printf '%s\n' "$(<"$stderr_file")" >&2
  elif [[ ! "$checksum" =~ ^-?[0-9]+$ ]]; then
    status="INVALID_OUTPUT"
    failures=$((failures + 1))
  elif ((scale == 1)) && [[ "$checksum" != "$wanted" ]]; then
    status="MISMATCH"
    failures=$((failures + 1))
  else
    status="OK"
  fi

  row="$(printf '%s,%s,%s,%s,%s,%s,%s,%s,%s,%s,%s,%s,%s' \
    "$relative" "$mode" "$scale" "$compile_ms" "$wall_time" \
    "$user_cpu" "$system_cpu" "$cpu_total" "$cpu_percent" "$max_rss" \
    "$checksum" "$wanted" "$status")"
  emit_row "$row"
done

if ((failures > 0)); then
  printf '%s benchmark(s) falharam.\n' "$failures" >&2
  exit 1
fi
