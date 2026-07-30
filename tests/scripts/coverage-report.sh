#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
fixture_root="$(mktemp -d)"
trap 'rm -rf "$fixture_root"' EXIT

report="$fixture_root/report.json"
baseline="$fixture_root/baseline.json"

cat >"$report" <<'EOF'
{
  "data": [{"files": [
    {"filename": "/workspace/src/compiler/clojure-reader/src/lib.rs", "summary": {"lines": {"count": 10, "covered": 9}, "functions": {"count": 4, "covered": 3}, "regions": {"count": 12, "covered": 10}}, "segments": [[10,1,1,true,true,false], [11,1,0,true,true,false]]},
    {"filename": "/workspace/src/compiler/clojure-reader/src/token.rs", "summary": {"lines": {"count": 5, "covered": 5}, "functions": {"count": 2, "covered": 2}, "regions": {"count": 5, "covered": 5}}, "segments": [[4,1,1,true,true,false]]}
  ]}]
}
EOF

cat >"$baseline" <<'EOF'
{
  "version": 1,
  "crates": {
    "clojure-reader": {"lines": 90, "functions": 75, "regions": 83}
  },
  "modules": {
    "src/compiler/clojure-reader/src/lib.rs": {"lines": 90, "functions": 75, "regions": 83}
  }
}
EOF

summary="$fixture_root/summary.json"
"$repo_root/scripts/coverage-report.sh" summarize \
  --report "$report" --baseline "$baseline" --output "$summary"

jq -e '.crates["clojure-reader"].lines == 93.33' "$summary" >/dev/null
jq -e '.modules["src/compiler/clojure-reader/src/lib.rs"].functions == 75' "$summary" >/dev/null

cat >"$baseline" <<'EOF'
{"version":1,"crates":{"clojure-reader":{"lines":94,"functions":75,"regions":83}},"modules":{}}
EOF
if "$repo_root/scripts/coverage-report.sh" check-ratchet \
  --report "$report" --baseline "$baseline" >"$fixture_root/ratchet.stderr" 2>&1; then
  echo "expected the crate coverage ratchet to reject a regression" >&2
  exit 1
fi
rg -q 'crate clojure-reader lines' "$fixture_root/ratchet.stderr"
