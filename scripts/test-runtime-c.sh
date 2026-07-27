#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

case "${1:-}" in
  "")
    cargo test -p clojure-codegen --test runtime_c
    ;;
  --sanitize)
    CLJN_RUNTIME_C_SANITIZE=1 cargo test -p clojure-codegen --test runtime_c
    ;;
  *)
    printf 'uso: %s [--sanitize]\n' "$0" >&2
    exit 2
    ;;
esac
