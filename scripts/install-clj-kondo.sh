#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
version="2026.07.24"
archive="clj-kondo-${version}-linux-amd64.zip"
expected_checksum="62fe3ff0573c42b63aa00a56dd858332f33f1d0b5958dbf1a132586562652f33"
install_dir="${repo_root}/target/tools/clj-kondo/${version}"
binary="${install_dir}/clj-kondo"

if [[ -x "${binary}" ]]; then
  printf '%s\n' "${binary}"
  exit 0
fi

if [[ "$(uname -s)" != "Linux" || "$(uname -m)" != "x86_64" ]]; then
  printf 'Automatic clj-kondo installation currently supports Linux x86_64 only.\n' >&2
  printf 'Install clj-kondo manually or set CLJ_KONDO_BIN to its executable.\n' >&2
  exit 1
fi

for command_name in curl unzip sha256sum; do
  if ! command -v "${command_name}" >/dev/null 2>&1; then
    printf 'Required command not found: %s\n' "${command_name}" >&2
    exit 1
  fi
done

download_dir="$(mktemp -d "${TMPDIR:-/tmp}/clj-kondo.XXXXXXXX")"
trap 'rm -rf -- "${download_dir}"' EXIT
download_path="${download_dir}/${archive}"
download_url="https://github.com/clj-kondo/clj-kondo/releases/download/v${version}/${archive}"

printf 'Downloading clj-kondo v%s...\n' "${version}" >&2
curl --fail --location --retry 3 --silent --show-error \
  --output "${download_path}" \
  "${download_url}"

actual_checksum="$(sha256sum "${download_path}" | cut -d ' ' -f 1)"
if [[ "${actual_checksum}" != "${expected_checksum}" ]]; then
  printf 'clj-kondo checksum mismatch: expected %s, got %s\n' \
    "${expected_checksum}" "${actual_checksum}" >&2
  exit 1
fi

mkdir -p "${install_dir}"
unzip -q "${download_path}" -d "${install_dir}"
chmod +x "${binary}"
printf '%s\n' "${binary}"
