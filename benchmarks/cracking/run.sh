#!/usr/bin/env bash
set -u

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repo_root="$(cd "$script_dir/../.." && pwd)"
compiler="$repo_root/target/release/clojure-native"
chapter=""
gc_stress=0
list_only=0
keep_binaries=0

usage() {
  printf '%s\n' \
    "Uso: benchmarks/cracking/run.sh [opções]" \
    "" \
    "  --chapter PREFIX    Executa um capítulo, por exemplo 04" \
    "  --compiler PATH     Usa um binário específico do compilador" \
    "  --gc-stress         Executa com CLJN_GC_STRESS=1" \
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
    --gc-stress)
      gc_stress=1
      shift
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
printf 'benchmark,compile_ms,run_ms,checksum,expected,status\n'

for source in "${sources[@]}"; do
  relative="${source#"$script_dir/"}"
  safe_name="${relative//\//_}"
  executable="$output_dir/${safe_name%.clj}"

  compile_start="$(date +%s%N)"
  if ! build_output="$("$compiler" build "$source" -o "$executable" 2>&1)"; then
    compile_end="$(date +%s%N)"
    compile_ms=$(((compile_end - compile_start) / 1000000))
    printf '%s,%s,0,,%s,BUILD_FAIL\n' \
      "$relative" "$compile_ms" "${expected[$relative]:-missing}"
    printf '%s\n' "$build_output" >&2
    failures=$((failures + 1))
    continue
  fi
  compile_end="$(date +%s%N)"

  run_start="$(date +%s%N)"
  if ((gc_stress)); then
    run_output="$(CLJN_GC_STRESS=1 "$executable" 2>&1)"
    run_status=$?
  else
    run_output="$("$executable" 2>&1)"
    run_status=$?
  fi
  run_end="$(date +%s%N)"

  compile_ms=$(((compile_end - compile_start) / 1000000))
  run_ms=$(((run_end - run_start) / 1000000))
  checksum="${run_output//$'\n'/}"
  wanted="${expected[$relative]:-missing}"

  if ((run_status != 0)); then
    status="RUN_FAIL"
    failures=$((failures + 1))
  elif [[ "$checksum" != "$wanted" ]]; then
    status="MISMATCH"
    failures=$((failures + 1))
  else
    status="OK"
  fi

  printf '%s,%s,%s,%s,%s,%s\n' \
    "$relative" "$compile_ms" "$run_ms" "$checksum" "$wanted" "$status"
done

if ((failures > 0)); then
  printf '%s benchmark(s) falharam.\n' "$failures" >&2
  exit 1
fi
