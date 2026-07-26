#!/usr/bin/env bash
set -u

suite_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
export CLJN_BENCHMARK_SUITE_DIR="$suite_dir"
exec "$suite_dir/../cracking/run.sh" "$@"
