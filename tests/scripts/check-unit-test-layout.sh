#!/usr/bin/env bash
# Verifies that Rust unit-test modules live under the mirrored tests/unit tree.
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
status=0

while IFS= read -r -d '' source; do
  relative="${source#"${repo_root}/"}"
  if rg -q '^#\[cfg\(test\)\]$' "${source}" && rg -q '^#\[path = "\.\./tests/unit/.+/mod\.rs"\]$' "${source}"; then
    path_line="$(rg '^#\[path = "' "${source}" | head -n 1)"
    path_value="${path_line#*\"}"
    path_value="${path_value%\"*}"
    expected="$(dirname "${source}")/${path_value}"
    if [[ ! -f "${expected}" ]]; then
      printf 'unit-test layout: missing mirrored file for %s: %s\n' "${relative}" "${expected#"${repo_root}/"}" >&2
      status=1
    fi
  elif rg -q '^#\[cfg\(test\)\]$' "${source}" && rg -q '^mod tests([; ]|$)' "${source}"; then
    printf 'unit-test layout: inline or unpathed test module remains in %s\n' "${relative}" >&2
    status=1
  fi
done < <(find "${repo_root}/src/compiler" -path '*/src/*.rs' -print0)

exit "${status}"
