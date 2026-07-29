#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
repository="$(cd -- "$script_dir/.." && pwd)"
command="${1:-verify}"

cd "$repository"

case "$command" in
  verify)
    CARGO_NET_OFFLINE=true cargo build --release --locked \
      -p clojure-native-cli \
      -p clojure-test-support
    ;;
  list | oracle | reader-coverage | help | -h | --help)
    cargo build --release --locked -p clojure-test-support
    ;;
  *)
    echo "unknown conformance command: $command" >&2
    echo "use: verify, list, reader-coverage, oracle --check, or oracle --bless" >&2
    exit 2
    ;;
esac

exec "$repository/target/release/clojure-conformance" "$@"
