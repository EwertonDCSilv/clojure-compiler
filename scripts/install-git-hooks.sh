#!/usr/bin/env bash
# Point this checkout at the repository-owned Git hooks without copying files.
set -euo pipefail

repository="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
force=0

case "${1:-}" in
  "")
    ;;
  --force)
    force=1
    ;;
  *)
    printf 'usage: %s [--force]\n' "$0" >&2
    exit 2
    ;;
esac

cd "${repository}"
if ! git rev-parse --is-inside-work-tree >/dev/null 2>&1; then
  printf 'not inside a Git worktree: %s\n' "${repository}" >&2
  exit 1
fi

current="$(git config --local --get core.hooksPath || true)"
if [[ -n "${current}" && "${current}" != ".githooks" && "${force}" -eq 0 ]]; then
  printf 'core.hooksPath is already set to %s.\n' "${current}" >&2
  printf 'Re-run with --force only if replacing that hook directory is intentional.\n' >&2
  exit 1
fi

git config --local core.hooksPath .githooks
printf 'Git hooks enabled for %s\n' "${repository}"
printf 'pre-commit: staged hygiene plus language-specific format/lint checks\n'
printf 'pre-push: complete quality gate plus C runtime sanitizers\n'
