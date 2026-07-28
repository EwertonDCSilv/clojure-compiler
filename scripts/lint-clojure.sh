#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

if [[ -n "${CLJ_KONDO_BIN:-}" ]]; then
  clj_kondo="${CLJ_KONDO_BIN}"
elif command -v clj-kondo >/dev/null 2>&1; then
  clj_kondo="$(command -v clj-kondo)"
else
  clj_kondo="$("${repo_root}/scripts/install-clj-kondo.sh")"
fi

if [[ ! -x "${clj_kondo}" ]]; then
  printf 'clj-kondo executable not found: %s\n' "${clj_kondo}" >&2
  exit 1
fi

lint_scope() {
  "${clj_kondo}" \
    --lint "$@" \
    --cache false \
    --config-dir "${repo_root}/.clj-kondo" \
    --fail-level warning
}

cd "${repo_root}"

# The two bootstrap cores intentionally define the same clojure.core vars with
# different supported arities. Isolated runs prevent one implementation from
# contaminating the other's analysis.
lint_scope crates/clojure-interp/src/core.clj
lint_scope crates/clojure-native-cli/src/core_compiled.clj

# Conformance case inputs include deliberate syntax errors and expected failures.
# Only the maintained JVM oracle belongs in the lint gate.
lint_scope \
  examples \
  benchmarks/cracking \
  benchmarks/cormen \
  tests/conformance/oracle/runner.clj

# The pinned Exercism reference solution for two-fer intentionally names its
# parameter `name`. Preserve the external source and scope that one upstream
# shadowing convention away without weakening the project's own Clojure lint.
"${clj_kondo}" \
  --lint benchmarks/exercism/01-practice \
  --cache false \
  --config-dir "${repo_root}/.clj-kondo" \
  --config '{:linters {:shadowed-var {:level :off}}}' \
  --fail-level warning
