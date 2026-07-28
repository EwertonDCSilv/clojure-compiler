#!/usr/bin/env bash
# Orchestrate repository checks based on the files staged for commit.
set -uo pipefail

repository="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
mode="staged"

case "${1:-}" in
  "")
    ;;
  --all)
    mode="all"
    ;;
  *)
    printf 'usage: %s [--all]\n' "$0" >&2
    exit 2
    ;;
esac

cd "${repository}"

declare -a files=()
if [[ "${mode}" == "all" ]]; then
  mapfile -d '' -t files < <(
    {
      git ls-files -z
      git ls-files --others --exclude-standard -z
    }
  )
else
  mapfile -d '' -t files < <(
    git diff --cached --name-only --diff-filter=ACMR -z
  )
fi

if ((${#files[@]} == 0)); then
  printf 'pre-commit: no files to check.\n'
  exit 0
fi

check_rust=0
check_c=0
check_clojure=0
check_shell=0
declare -a shell_files=()
declare -a partially_staged=()

if [[ "${mode}" == "all" ]]; then
  check_rust=1
  check_c=1
  check_clojure=1
  check_shell=1
fi

for path in "${files[@]}"; do
  if [[ "${mode}" == "staged" ]] && ! git diff --quiet -- "${path}"; then
    partially_staged+=("${path}")
  fi

  case "${path}" in
    *.rs | Cargo.toml | Cargo.lock | rust-toolchain | rust-toolchain.toml)
      check_rust=1
      ;;
  esac

  case "${path}" in
    *.c | *.h)
      check_c=1
      ;;
  esac

  case "${path}" in
    *.clj | *.cljc | *.cljs | *.edn | .clj-kondo/*)
      check_clojure=1
      ;;
  esac

  case "${path}" in
    *.sh | .githooks/*)
      check_shell=1
      shell_files+=("${path}")
      ;;
  esac

  case "${path}" in
    Makefile | \
      scripts/pre-commit.sh | \
      scripts/check-file-hygiene.sh | \
      scripts/lint-c.sh | \
      scripts/lint-clojure.sh)
      check_rust=1
      check_c=1
      check_clojure=1
      ;;
  esac
done

if ((${#partially_staged[@]} != 0)); then
  printf 'note: checks use working-tree contents for partially staged files:\n'
  printf '  %s\n' "${partially_staged[@]}"
fi

status=0

run_step() {
  local label="$1"
  shift
  printf '\n==> %s\n' "${label}"
  if "$@"; then
    printf 'ok: %s\n' "${label}"
  else
    status=1
    printf 'failed: %s\n' "${label}" >&2
  fi
}

lint_shell_files() {
  local path
  for path in "${shell_files[@]}"; do
    [[ -f "${path}" ]] || continue
    bash -n -- "${path}"
  done
}

run_step "file hygiene" "${repository}/scripts/check-file-hygiene.sh" "${files[@]}"

if ((check_shell != 0)); then
  if [[ "${mode}" == "all" ]]; then
    mapfile -d '' -t shell_files < <(
      {
        git ls-files -z -- '*.sh'
        git ls-files -z -- '.githooks/*'
      }
    )
  fi
  run_step "shell syntax" lint_shell_files
fi

if ((check_rust != 0)); then
  run_step "Rust formatting" make fmt-check
  run_step "Rust Clippy" make lint-rust
fi

if ((check_c != 0)); then
  run_step "C compiler lint" "${repository}/scripts/lint-c.sh"
fi

if ((check_clojure != 0)); then
  run_step "Clojure clj-kondo" "${repository}/scripts/lint-clojure.sh"
fi

if ((status != 0)); then
  printf '\npre-commit checks failed.\n' >&2
  exit "${status}"
fi

printf '\npre-commit checks passed.\n'
