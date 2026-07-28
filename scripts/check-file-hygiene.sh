#!/usr/bin/env bash
# Check maintained text files without rewriting staged content.
set -euo pipefail

declare -a paths=()
if [[ "${1:-}" == "--tracked" ]]; then
  mapfile -d '' -t paths < <(git ls-files -z)
else
  paths=("$@")
fi

if ((${#paths[@]} == 0)); then
  exit 0
fi

failed=0

is_exact_fixture_or_external() {
  case "$1" in
    .lsp/* | \
      benchmarks/exercism/01-practice/* | \
      benchmarks/exercism/02-concept/* | \
      tests/conformance/*/input.* | \
      tests/conformance/*/expected.* | \
      tests/conformance/*/stdin.* | \
      tests/conformance/*/work.before/* | \
      tests/conformance/*/work.after/*)
      return 0
      ;;
    *)
      return 1
      ;;
  esac
}

is_maintained_text() {
  case "$1" in
    .editorconfig | .githooks/* | Makefile | \
      *.c | *.h | \
      *.clj | *.cljc | *.cljs | *.edn | \
      *.json | *.md | *.mdc | *.rs | *.sh | *.toml | *.yaml | *.yml)
      return 0
      ;;
    *)
      return 1
      ;;
  esac
}

report_matching_lines() {
  local path="$1"
  local pattern="$2"
  LC_ALL=C grep -nE "${pattern}" -- "${path}" | cut -d: -f1 | paste -sd, -
}

for path in "${paths[@]}"; do
  [[ -f "${path}" ]] || continue

  # grep -I treats files containing NUL bytes as binary. Empty files are text
  # for the purposes of the checks below.
  if [[ -s "${path}" ]] && ! LC_ALL=C grep -Iq . -- "${path}"; then
    continue
  fi

  if LC_ALL=C grep -qE '^(<<<<<<<([[:space:]]|$)|=======$|>>>>>>>[[:space:]])' -- "${path}"; then
    lines="$(report_matching_lines \
      "${path}" \
      '^(<<<<<<<([[:space:]]|$)|=======$|>>>>>>>[[:space:]])')"
    printf 'merge-conflict marker: %s (lines %s)\n' "${path}" "${lines}" >&2
    failed=1
  fi

  if is_exact_fixture_or_external "${path}" || ! is_maintained_text "${path}"; then
    continue
  fi

  if LC_ALL=C grep -qE '[[:blank:]]+$' -- "${path}"; then
    lines="$(report_matching_lines "${path}" '[[:blank:]]+$')"
    printf 'trailing whitespace: %s (lines %s)\n' "${path}" "${lines}" >&2
    failed=1
  fi

  if [[ -s "${path}" ]] && [[ "$(tail -c 1 -- "${path}" | wc -l)" -eq 0 ]]; then
    printf 'missing final newline: %s\n' "${path}" >&2
    failed=1
  fi

  if [[ "${path}" == *.sh ]]; then
    mode="$(git ls-files --stage -- "${path}" | awk 'NR == 1 { print $1 }')"
    if [[ -n "${mode}" && "${mode}" != "100755" ]]; then
      printf 'shell script is not executable: %s (mode %s)\n' \
        "${path}" "${mode}" >&2
      failed=1
    fi
  fi
done

if ((failed != 0)); then
  printf 'file hygiene failed; fix the files above and stage them again.\n' >&2
  exit 1
fi
