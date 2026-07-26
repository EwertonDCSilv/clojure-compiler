#!/usr/bin/env bash
set -u

default_script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
script_dir="${CLJN_BENCHMARK_SUITE_DIR:-$default_script_dir}"
repo_root="$(cd "$script_dir/../.." && pwd)"
native_runner="$script_dir/run.sh"
compiler=""
opt_level=""
chapter=""
scale=25
csv_path="$script_dir/results/extreme.csv"
clojure_version="1.12.5"
clojure_classpath=""

usage() {
  printf '%s\n' \
    "Uso: benchmarks/cracking/compare-clojure.sh [opções]" \
    "" \
    "Compara os executáveis nativos com Clojure/JVM AOT na mesma carga." \
    "" \
    "  --chapter PREFIX         Executa um capítulo, por exemplo 04" \
    "  --compiler PATH          Usa um binário específico do compilador nativo" \
    "  --opt-level LEVEL        Usa none, speed ou speed-and-size no Cranelift" \
    "  --scale N                Multiplica a carga interna (padrão: 25)" \
    "  --csv PATH               Grava o CSV neste caminho" \
    "  --clojure-classpath CP   Usa um runtime Clojure/JVM já instalado" \
    "  -h, --help               Mostra esta ajuda"
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
    --scale)
      scale="${2:-}"
      shift 2
      ;;
    --csv)
      csv_path="${2:-}"
      shift 2
      ;;
    --clojure-classpath)
      clojure_classpath="${2:-}"
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

if [[ ! -x /usr/bin/time ]]; then
  printf 'GNU time não encontrado em /usr/bin/time\n' >&2
  exit 1
fi

if ! command -v java >/dev/null 2>&1; then
  printf 'Java não encontrado; ele é necessário para a referência Clojure/JVM.\n' >&2
  exit 1
fi

download_jar() {
  local url="$1"
  local destination="$2"
  local partial="$destination.part.$$"

  if [[ -f "$destination" ]]; then
    return
  fi
  if ! command -v curl >/dev/null 2>&1; then
    printf 'curl não encontrado para baixar %s\n' "$url" >&2
    exit 1
  fi

  printf 'Baixando %s...\n' "$(basename "$destination")" >&2
  if ! curl -fsSL "$url" -o "$partial"; then
    rm -f -- "$partial"
    exit 1
  fi
  mv -- "$partial" "$destination"
}

if [[ -z "$clojure_classpath" ]]; then
  runtime_dir="$repo_root/target/benchmark-clojure/clojure-$clojure_version"
  mkdir -p "$runtime_dir"

  clojure_jar="$runtime_dir/clojure-$clojure_version.jar"
  spec_jar="$runtime_dir/spec.alpha-0.5.238.jar"
  core_specs_jar="$runtime_dir/core.specs.alpha-0.4.74.jar"

  download_jar \
    "https://repo.maven.apache.org/maven2/org/clojure/clojure/$clojure_version/clojure-$clojure_version.jar" \
    "$clojure_jar"
  download_jar \
    "https://repo.maven.apache.org/maven2/org/clojure/spec.alpha/0.5.238/spec.alpha-0.5.238.jar" \
    "$spec_jar"
  download_jar \
    "https://repo.maven.apache.org/maven2/org/clojure/core.specs.alpha/0.4.74/core.specs.alpha-0.4.74.jar" \
    "$core_specs_jar"

  clojure_classpath="$clojure_jar:$spec_jar:$core_specs_jar"
fi

detected_clojure_version="$(
  java -cp "$clojure_classpath" clojure.main \
    -e '(print (clojure-version))' 2>/dev/null
)"
if [[ -z "$detected_clojure_version" ]]; then
  printf 'Não foi possível iniciar Clojure com o classpath informado.\n' >&2
  exit 1
fi
clojure_version="$detected_clojure_version"

output_dir="$(mktemp -d)"
cleanup() {
  if [[ "$output_dir" == /tmp/* ]]; then
    rm -rf -- "$output_dir"
  fi
}
trap cleanup EXIT

native_csv="$output_dir/native.csv"
declare -a native_args=(--scale "$scale" --csv "$native_csv")
if [[ -n "$chapter" ]]; then
  native_args+=(--chapter "$chapter")
fi
if [[ -n "$compiler" ]]; then
  native_args+=(--compiler "$compiler")
fi
if [[ -n "$opt_level" ]]; then
  native_args+=(--opt-level "$opt_level")
fi

if ! "$native_runner" "${native_args[@]}" >"$output_dir/native.log"; then
  printf 'A medição nativa falhou:\n' >&2
  cat "$output_dir/native.log" >&2
  exit 1
fi

mkdir -p "$(dirname "$csv_path")"
: >"$csv_path"
exec 3>>"$csv_path"

emit_row() {
  printf '%s\n' "$1"
  printf '%s\n' "$1" >&3
}

ratio() {
  awk -v numerator="$1" -v denominator="$2" \
    'BEGIN {
       if (denominator + 0 > 0) {
         printf "%.3f", (numerator + 0) / (denominator + 0)
       }
     }'
}

header="benchmark,scale,native_compile_wall_ms,native_wall_time_s,native_cpu_user_s,native_cpu_system_s,native_cpu_total_s,native_cpu_percent,native_max_rss_kb,clojure_version,clojure_compile_wall_ms,clojure_wall_time_s,clojure_cpu_user_s,clojure_cpu_system_s,clojure_cpu_total_s,clojure_cpu_percent,clojure_max_rss_kb,native_checksum,clojure_checksum,wall_speedup_vs_clojure,cpu_speedup_vs_clojure,rss_ratio_clojure_over_native,status"
emit_row "$header"

failures=0
while IFS=, read -r benchmark _mode row_scale \
  native_compile_ms native_wall native_user native_system native_cpu \
  native_cpu_percent native_rss native_checksum _expected native_status; do
  if [[ "$benchmark" == "benchmark" ]]; then
    continue
  fi

  source="$script_dir/$benchmark"
  safe_name="${benchmark//\//_}"
  build_source="$source"
  if ((scale > 1)); then
    build_source="$output_dir/${safe_name%.clj}.scaled.clj"
    sed -E "s/\\(benchmark ([0-9]+)\\)/\\(benchmark (* \\1 $scale)\\)/" \
      "$source" >"$build_source"
  fi

  namespace="$(sed -nE 's/^\(ns[[:space:]]+([^[:space:])]+).*/\1/p' \
    "$build_source" | head -n 1)"
  namespace_path="${namespace//./\/}"
  namespace_path="${namespace_path//-/_}"
  clojure_source_root="$output_dir/${safe_name%.clj}.clojure-src"
  clojure_classes="$output_dir/${safe_name%.clj}.clojure-classes"
  clojure_source="$clojure_source_root/$namespace_path.clj"
  mkdir -p "$(dirname "$clojure_source")" "$clojure_classes"

  clojure_compile_ms=""
  clojure_wall=""
  clojure_user=""
  clojure_system=""
  clojure_cpu=""
  clojure_cpu_percent=""
  clojure_rss=""
  clojure_checksum=""
  clojure_status="BUILD_FAIL"

  if [[ -z "$namespace" || "$(tail -n 1 "$build_source")" != "(-main)" ]]; then
    printf '%s: namespace ou chamada final -main inválida\n' "$benchmark" >&2
  else
    sed '$d' "$build_source" >"$clojure_source"
    compile_start="$(date +%s%N)"
    java -Dclojure.compile.path="$clojure_classes" \
      -cp "$clojure_classpath:$clojure_source_root" \
      clojure.lang.Compile "$namespace" \
      >"$output_dir/${safe_name%.clj}.clojure-compile.stdout" \
      2>"$output_dir/${safe_name%.clj}.clojure-compile.stderr"
    clojure_compile_status=$?
    compile_end="$(date +%s%N)"
    clojure_compile_ms=$(((compile_end - compile_start) / 1000000))

    if ((clojure_compile_status != 0)); then
      printf '%s: compilação Clojure/JVM falhou\n' "$benchmark" >&2
      cat "$output_dir/${safe_name%.clj}.clojure-compile.stderr" >&2
    else
      metrics_file="$output_dir/${safe_name%.clj}.clojure.metrics"
      stdout_file="$output_dir/${safe_name%.clj}.clojure.stdout"
      stderr_file="$output_dir/${safe_name%.clj}.clojure.stderr"
      /usr/bin/time -f '%e\t%U\t%S\t%P\t%M' -o "$metrics_file" \
        java -cp "$clojure_classpath:$clojure_classes" \
        clojure.main -m "$namespace" \
        >"$stdout_file" 2>"$stderr_file"
      clojure_run_status=$?

      if ((clojure_run_status != 0)); then
        clojure_status="RUN_FAIL"
        printf '%s: execução Clojure/JVM falhou\n' "$benchmark" >&2
        cat "$stderr_file" >&2
      else
        IFS=$'\t' read -r clojure_wall clojure_user clojure_system \
          clojure_cpu_percent clojure_rss <"$metrics_file"
        clojure_cpu="$(awk -v usr="$clojure_user" -v sys="$clojure_system" \
          'BEGIN { printf "%.6f", usr + sys }')"
        clojure_output="$(<"$stdout_file")"
        clojure_checksum="${clojure_output//$'\n'/}"

        if [[ ! "$clojure_checksum" =~ ^-?[0-9]+$ ]]; then
          clojure_status="INVALID_OUTPUT"
        elif [[ "$clojure_checksum" != "$native_checksum" ]]; then
          clojure_status="CHECKSUM_MISMATCH"
        else
          clojure_status="OK"
        fi
      fi
    fi
  fi

  wall_speedup=""
  cpu_speedup=""
  rss_ratio=""
  if [[ "$native_status" == "OK" && "$clojure_status" == "OK" ]]; then
    wall_speedup="$(ratio "$clojure_wall" "$native_wall")"
    cpu_speedup="$(ratio "$clojure_cpu" "$native_cpu")"
    rss_ratio="$(ratio "$clojure_rss" "$native_rss")"
    status="OK"
  elif [[ "$native_status" != "OK" ]]; then
    status="NATIVE_$native_status"
  else
    status="CLOJURE_$clojure_status"
  fi

  if [[ "$status" != "OK" ]]; then
    failures=$((failures + 1))
  fi

  row="$(printf '%s,%s,%s,%s,%s,%s,%s,%s,%s,%s,%s,%s,%s,%s,%s,%s,%s,%s,%s,%s,%s,%s,%s' \
    "$benchmark" "$row_scale" "$native_compile_ms" "$native_wall" \
    "$native_user" "$native_system" "$native_cpu" "$native_cpu_percent" \
    "$native_rss" "$clojure_version" "$clojure_compile_ms" "$clojure_wall" \
    "$clojure_user" "$clojure_system" "$clojure_cpu" "$clojure_cpu_percent" \
    "$clojure_rss" "$native_checksum" "$clojure_checksum" "$wall_speedup" \
    "$cpu_speedup" "$rss_ratio" "$status")"
  emit_row "$row"
done <"$native_csv"

if ((failures > 0)); then
  printf '%s comparação(ões) falharam.\n' "$failures" >&2
  exit 1
fi
