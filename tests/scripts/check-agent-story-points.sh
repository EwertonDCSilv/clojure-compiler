#!/usr/bin/env bash
set -euo pipefail

repository="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../.." && pwd)"
guard="${repository}/scripts/check-agent-story-points.sh"
fixture_bin="${repository}/tests/scripts/fixtures"
failures=0

run_guard() {
  local expected_status="$1"
  local expected_text="$2"
  local row="$3"
  shift 3

  local output
  local status
  set +e
  output="$(
    PATH="${fixture_bin}:${PATH}" \
      FAKE_GH_ROW="${row}" \
      FAKE_GH_EXIT="${FAKE_GH_EXIT:-0}" \
      "${guard}" "$@" 2>&1
  )"
  status=$?
  set -e

  if [[ "${status}" != "${expected_status}" ]]; then
    printf 'expected status %s, got %s: %s\n' \
      "${expected_status}" "${status}" "${output}" >&2
    failures=$((failures + 1))
  fi
  if [[ "${output}" != *"${expected_text}"* ]]; then
    printf 'expected output containing %q, got: %s\n' \
      "${expected_text}" "${output}" >&2
    failures=$((failures + 1))
  fi
}

run_guard 0 "aprovada com 8 story points" \
  $'8\tarea: reader\tReader task\thttps://example.invalid/101' \
  --issue 101 --branch feature/101-reader-task

run_guard 1 "implementação bloqueada" \
  $'13\tarea: reader\tLarge reader task\thttps://example.invalid/102' \
  --issue 102 --branch feature/102-large-reader-task

run_guard 1 "implementação bloqueada" \
  $'8.5\tarea: runtime\tLarge runtime task\thttps://example.invalid/103' \
  --issue 103 --branch feature/103-large-runtime-task

run_guard 1 "não possui Story points" \
  $'UNESTIMATED\tarea: reader\tUnestimated task\thttps://example.invalid/104' \
  --issue 104 --branch feature/104-unestimated-task

run_guard 1 "não está no clojure-compiler Roadmap" "" \
  --issue 105 --branch feature/105-untracked-task

run_guard 1 "épicos e roll-ups não podem ser implementados" \
  $'5\tepic,area: reader\tEpic: reader work\thttps://example.invalid/106' \
  --issue 106 --branch feature/106-reader-epic

run_guard 0 "branch não é de feature" "" \
  --branch docs/update-guides

run_guard 0 "issue #101 aprovada com 3 story points" \
  $'3\tdocumentation\tAgent guard\thttps://example.invalid/101' \
  --branch feature/101-agent-story-point-guard

FAKE_GH_EXIT=1 run_guard 1 "não foi possível consultar o Roadmap" "" \
  --issue 107 --branch feature/107-network-failure
unset FAKE_GH_EXIT

run_guard 1 "não corresponde à issue #108 da branch" \
  $'3\tdocumentation\tWrong branch\thttps://example.invalid/107' \
  --issue 107 --branch feature/108-other-task

if ((failures != 0)); then
  printf '%s\n' "agent story-point guard tests failed: ${failures}" >&2
  exit 1
fi

printf '%s\n' "agent story-point guard tests passed"
