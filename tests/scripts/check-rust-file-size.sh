#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
fixture_root="$(mktemp -d)"
trap 'rm -rf "$fixture_root"' EXIT

mkdir -p "$fixture_root/src/compiler/demo/src" "$fixture_root/src/compiler/demo/tests"
printf '%s\n' 'fn main() {}' >"$fixture_root/src/compiler/demo/src/lib.rs"
printf '%s\n' 'fn test() {}' >"$fixture_root/src/compiler/demo/tests/e2e.rs"

cat >"$fixture_root/baseline.json" <<'EOF'
{
  "version": 1,
  "limits": {"production": 800, "facade": 500, "test": 1000, "generator": 1200},
  "allowlist": {"src/compiler/demo/src/lib.rs": {"lines": 1, "owner": "#109", "reason": "fixture debt"}},
  "exclusions": ["src/compiler/demo/tests/e2e.rs"]
}
EOF

"$repo_root/scripts/check-rust-file-size.sh" \
  --root "$fixture_root" --baseline "$fixture_root/baseline.json"

printf '%s\n' 'fn grew() {}' >>"$fixture_root/src/compiler/demo/src/lib.rs"
if "$repo_root/scripts/check-rust-file-size.sh" \
  --root "$fixture_root" --baseline "$fixture_root/baseline.json" >"$fixture_root/error" 2>&1; then
  echo "expected grandfathered file growth to fail" >&2
  exit 1
fi
rg -q 'baseline 1' "$fixture_root/error"
