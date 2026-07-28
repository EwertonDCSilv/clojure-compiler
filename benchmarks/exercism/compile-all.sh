#!/usr/bin/env bash
set -u

suite_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repo_root="$(cd "$suite_dir/../.." && pwd)"
checkout="${EXERCISM_CLOJURE_CHECKOUT:-$HOME/github/exercism-clojure}"
compiler="$repo_root/target/release/clojure-native"
report="$suite_dir/results/compilation.tsv"
strict=0
scope="references"

usage() {
  printf '%s\n' \
    "Uso: benchmarks/exercism/compile-all.sh [opções]" \
    "" \
    "Compila todas as soluções de referência do checkout exercism/clojure." \
    "Falhas de compatibilidade são catalogadas e não tornam o comando inválido," \
    "a menos que --strict seja informado." \
    "" \
    "  --checkout PATH   Checkout exercism/clojure (padrão: ~/github/exercism-clojure)" \
    "  --compiler PATH   Binário clojure-native" \
    "  --report PATH     Relatório TSV" \
    "  --scope SCOPE     references (101 soluções) ou all (todo .clj/.cljc)" \
    "  --strict          Retorna status não-zero se qualquer exercício falhar" \
    "  -h, --help        Mostra esta ajuda"
}

while (($# > 0)); do
  case "$1" in
    --checkout)
      checkout="${2:-}"
      shift 2
      ;;
    --compiler)
      compiler="${2:-}"
      shift 2
      ;;
    --report)
      report="${2:-}"
      shift 2
      ;;
    --scope)
      scope="${2:-}"
      shift 2
      ;;
    --strict)
      strict=1
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

case "$scope" in
  references|all) ;;
  *)
    printf 'Escopo inválido: %s (use references ou all)\n' "$scope" >&2
    exit 2
    ;;
esac

if [[ ! -d "$checkout/.git" ]]; then
  printf 'Checkout exercism/clojure não encontrado em %s\n' "$checkout" >&2
  exit 2
fi
if [[ ! -d "$checkout/exercises/practice" ]]; then
  printf 'Árvore exercises/practice ausente em %s\n' "$checkout" >&2
  exit 2
fi
if [[ ! -x "$compiler" ]]; then
  printf 'Compilador não encontrado; construindo release...\n' >&2
  cargo build --manifest-path "$repo_root/Cargo.toml" --release -p clojure-native-cli ||
    exit 1
fi

upstream_commit="$(git -C "$checkout" rev-parse HEAD)"
compiler_commit="$(git -C "$repo_root" rev-parse HEAD)"
output_dir="$(mktemp -d /tmp/cljn-exercism-compile.XXXXXX)"
cleanup() {
  if [[ "$output_dir" == /tmp/cljn-exercism-compile.* ]]; then
    rm -rf -- "$output_dir"
  fi
}
trap cleanup EXIT

mkdir -p "$(dirname "$report")"
: >"$report"
printf 'case\trole\tstatus\terror_code\tcategory\tline\tfirst_error\tsource\tupstream_commit\tcompiler_commit\n' \
  >>"$report"

categorize() {
  local message="$1"
  local source_line="$2"
  case "$message" in
    *"literal de regex"*) printf 'reader-regex' ;;
    *"syntax-quote"*) printf 'syntax-quote' ;;
    *"quote ainda"*) printf 'quote' ;;
    *"char ainda"*) printf 'char-codegen' ;;
    *"binding deve ser símbolo simples"*) printf 'destructuring' ;;
    *"def/defn só é permitido"*)
      if [[ "$source_line" == "(defn "* ]]; then
        printf 'destructuring'
      else
        printf 'top-level-def'
      fi
      ;;
    *"fora do slice"*)
      if [[ "$message" == *"Character/"* || "$message" == *"Integer/"* ]]; then
        printf 'java-interop'
      else
        printf 'stdlib-namespace'
      fi
      ;;
    *"função não resolvida:"*)
      case "$message" in
        *": for"|*": condp"|*": letfn"|*": if-let") printf 'core-macro' ;;
        *": IllegalArgumentException.") printf 'exception-constructor' ;;
        *": .compareTo") printf 'java-interop' ;;
        *) printf 'missing-core-function' ;;
      esac
      ;;
    *"símbolo não resolvido:"*) printf 'missing-symbol' ;;
    *"aridade errada"*|*"aridade inválida"*) printf 'missing-arity' ;;
    *"primitiva variádica"*) printf 'variadic-primitive-value' ;;
    *"fn: forma inválida"*) printf 'fn-form' ;;
    *"delimitador inesperado"*)
      if [[ "$source_line" == *"'"* ]]; then
        printf 'symbol-apostrophe'
      else
        printf 'reader-delimiter'
      fi
      ;;
    *) printf 'compiler-other' ;;
  esac
}

sanitize_field() {
  printf '%s' "$1" | tr '\t\r\n' '   '
}

source_role() {
  local relative="$1"
  case "$relative" in
    */.meta/example.clj) printf 'reference' ;;
    */.meta/generator.clj) printf 'generator' ;;
    */test/*.clj|*/test/*.cljc) printf 'test' ;;
    */src/*.clj|*/src/*.cljc) printf 'source' ;;
    project.clj|*/project.clj) printf 'project' ;;
    *) printf 'other' ;;
  esac
}

passes=0
failures=0
declare -A categories=()
declare -a sources=()
if [[ "$scope" == "references" ]]; then
  while IFS= read -r source; do
    sources+=("$source")
  done < <(find "$checkout/exercises/practice" \
    -path '*/.meta/example.clj' -type f | sort)
else
  while IFS= read -r source; do
    sources+=("$source")
  done < <(find "$checkout" -type f \
    \( -name '*.clj' -o -name '*.cljc' \) | sort)
fi

if ((${#sources[@]} == 0)); then
  printf 'Nenhum fonte Clojure encontrado para o escopo %s.\n' "$scope" >&2
  exit 2
fi

for source in "${sources[@]}"; do
  exercise="$(basename "$(dirname "$(dirname "$source")")")"
  relative="${source#"$checkout/"}"
  role="$(source_role "$relative")"
  if [[ "$scope" == "references" ]]; then
    case_id="$exercise"
  else
    case_id="$relative"
  fi
  safe_name="${case_id//\//_}"
  safe_name="${safe_name//./_}"
  log="$output_dir/$safe_name.log"
  executable="$output_dir/$safe_name"

  if "$compiler" build "$source" -o "$executable" >"$log" 2>&1; then
    printf '%s\t%s\tPASS\t\t\t\t\t%s\t%s\t%s\n' \
      "$case_id" "$role" "$relative" "$upstream_commit" "$compiler_commit" >>"$report"
    printf 'PASS  %s\n' "$case_id"
    passes=$((passes + 1))
    continue
  fi

  first_error="$(sed -n '1p' "$log")"
  error_code="$(
    printf '%s\n' "$first_error" |
      sed -n 's/^error\[\([^]]*\)\].*/\1/p'
  )"
  message="$(
    printf '%s\n' "$first_error" |
      sed 's/^error\[[^]]*\]:[[:space:]]*//'
  )"
  location="$(sed -n '2p' "$log")"
  line="$(
    printf '%s\n' "$location" |
      sed -nE 's#.*:([0-9]+):[0-9]+$#\1#p'
  )"
  source_line=""
  if [[ "$line" =~ ^[0-9]+$ ]]; then
    source_line="$(sed -n "${line}p" "$source" | sed 's/^[[:space:]]*//')"
  fi
  category="$(categorize "$message" "$source_line")"
  message="$(sanitize_field "$message")"
  categories["$category"]=$((${categories["$category"]:-0} + 1))
  printf '%s\t%s\tFAIL\t%s\t%s\t%s\t%s\t%s\t%s\t%s\n' \
    "$case_id" "$role" "$error_code" "$category" "$line" "$message" "$relative" \
    "$upstream_commit" "$compiler_commit" >>"$report"
  printf 'FAIL  %-48s %s: %s\n' "$case_id" "$category" "$message"
  failures=$((failures + 1))
done

printf '\nExercism %s (%s): %d PASS, %d FAIL, %d total\n' \
  "${upstream_commit:0:12}" "$scope" "$passes" "$failures" "${#sources[@]}"
printf 'Relatório: %s\n' "$report"
if ((${#categories[@]} > 0)); then
  printf '\nPrimeiros bloqueadores:\n'
  for category in "${!categories[@]}"; do
    printf '%d\t%s\n' "${categories[$category]}" "$category"
  done | sort -nr
fi

if ((strict && failures > 0)); then
  exit 1
fi
