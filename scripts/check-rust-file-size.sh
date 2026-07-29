#!/usr/bin/env bash
# Enforces gradual Rust source-file size limits and grandfathered baselines.
set -euo pipefail

root="."
baseline="config/rust-file-size-baseline.json"
while (($#)); do
  case "$1" in
    --root) root="${2:-}"; shift 2 ;;
    --baseline) baseline="${2:-}"; shift 2 ;;
    *) printf 'unknown option: %s\n' "$1" >&2; exit 2 ;;
  esac
done

[[ -f "$baseline" ]] || { printf 'missing size baseline: %s\n' "$baseline" >&2; exit 2; }
failed=0
while IFS= read -r -d '' absolute; do
  path="${absolute#"$root"/}"
  if jq -e --arg path "$path" '.exclusions | index($path) != null' "$baseline" >/dev/null; then
    continue
  fi
  case "$path" in
    */examples/generate_suite.rs) kind=generator ;;
    */tests/*|*/examples/*) kind=test ;;
    */src/lib.rs|*/src/main.rs) kind=facade ;;
    *) kind=production ;;
  esac
  lines="$(wc -l <"$absolute")"
  entry="$(jq -r --arg path "$path" '.allowlist[$path] // empty | @base64' "$baseline")"
  if [[ -n "$entry" ]]; then
    decoded="$(printf '%s' "$entry" | base64 -d)"
    allowed="$(jq -r '.lines' <<<"$decoded")"
    owner="$(jq -r '.owner // empty' <<<"$decoded")"
    reason="$(jq -r '.reason // empty' <<<"$decoded")"
    if [[ -z "$owner" || -z "$reason" || "$lines" -gt "$allowed" ]]; then
      printf 'size gate: %s has %s lines; baseline %s (owner=%s)\n' "$path" "$lines" "$allowed" "$owner" >&2
      failed=1
    fi
  else
    limit="$(jq -r --arg kind "$kind" '.limits[$kind]' "$baseline")"
    if [[ "$lines" -gt "$limit" ]]; then
      printf 'size gate: new %s file %s has %s lines; limit %s\n' "$kind" "$path" "$lines" "$limit" >&2
      failed=1
    fi
  fi
done < <(find "$root/crates" -type f -name '*.rs' -print0)

if ((failed)); then exit 1; fi
