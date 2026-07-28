#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
repository="$(cd -- "$script_dir/.." && pwd)"

cd "$repository"

echo "Checking changelog contract..."
if [[ ! -s CHANGELOG.md ]]; then
  echo "missing CHANGELOG.md" >&2
  exit 1
fi
if ! grep -Fq '## [Unreleased]' CHANGELOG.md; then
  echo "CHANGELOG.md does not contain an Unreleased section" >&2
  exit 1
fi

echo "Checking AI contributor instructions..."
for required_rule in \
  AGENTS.md \
  CLAUDE.md \
  .cursor/rules/compiler-documentation-and-tdd.mdc \
  .github/copilot-instructions.md \
  specs/DOCUMENTATION_STYLE.md \
  specs/TDD_WORKFLOW.md; do
  if [[ ! -s "${required_rule}" ]]; then
    echo "missing AI contributor rule: ${required_rule}" >&2
    exit 1
  fi
done

for adapter in \
  CLAUDE.md \
  .cursor/rules/compiler-documentation-and-tdd.mdc \
  .github/copilot-instructions.md; do
  if ! grep -Fq "AGENTS.md" "${adapter}"; then
    echo "AI adapter does not reference AGENTS.md: ${adapter}" >&2
    exit 1
  fi
done

for policy in specs/DOCUMENTATION_STYLE.md specs/TDD_WORKFLOW.md; do
  if ! grep -Fq "${policy}" AGENTS.md; then
    echo "AGENTS.md does not reference mandatory policy: ${policy}" >&2
    exit 1
  fi
done

echo "Checking Rust module documentation..."
module_failures=0
while IFS= read -r source; do
  if ! awk '
    /^[[:space:]]*$/ { next }
    /^#!\[/ { next }
    /^\/\/!/ { found = 1; exit }
    { exit 1 }
    END { if (!found) exit 1 }
  ' "$source"; then
    echo "missing //! module documentation: $source" >&2
    module_failures=1
  fi
done < <(rg --files crates -g '*.rs' | sort)
if ((module_failures != 0)); then
  exit 1
fi

echo "Checking C subsystem and ABI contracts..."
c_header_failures=0
while IFS= read -r source; do
  if ! awk '
    /^[[:space:]]*$/ { next }
    /^\/\*/ { found = 1; exit }
    { exit 1 }
    END { if (!found) exit 1 }
  ' "$source"; then
    echo "missing C subsystem header: $source" >&2
    c_header_failures=1
  fi
done < <(
  {
    printf '%s\n' crates/clojure-codegen/runtime.c
    rg --files crates/clojure-codegen/runtime -g '*.c'
  } | sort
)
if ((c_header_failures != 0)); then
  exit 1
fi

perl -0777 -e '
  my $failed = 0;
  for my $file (@ARGV) {
    open my $handle, "<", $file or die "cannot read $file: $!\n";
    local $/;
    my $source = <$handle>;
    while ($source =~ /^(?:[A-Za-z_][A-Za-z0-9_]*[ \t]+)+(?:\*?[ \t]*)?(cljn_[A-Za-z0-9_]+)[ \t]*\([^;{}]*?\)[ \t]*\{/mg) {
      my ($name, $start) = ($1, $-[0]);
      my $prefix = substr($source, 0, $start);
      if ($prefix !~ m{/\*.*?\*/[ \t\r\n]*\z}s) {
        my $line = 1 + (substr($source, 0, $start) =~ tr/\n//);
        warn "missing ABI contract comment: $file:$line ($name)\n";
        $failed = 1;
      }
    }
  }
  exit $failed;
' crates/clojure-codegen/runtime/*.c

echo "Checking compiled core docstrings..."
perl -0777 -e '
  my ($file) = @ARGV;
  open my $handle, "<", $file or die "cannot read $file: $!\n";
  local $/;
  my $source = <$handle>;
  my $failed = 0;
  while ($source =~ /\(defn[ \t\r\n]+([^\s\[\(]+)/g) {
    my ($name, $after_name) = ($1, $+[0]);
    my $tail = substr($source, $after_name);
    if ($tail !~ /^[ \t\r\n]+"(?:\\.|[^"\\])*"/s) {
      warn "missing Clojure docstring: $file ($name)\n";
      $failed = 1;
    }
  }
  exit $failed;
' crates/clojure-native-cli/src/core_compiled.clj

echo "Building rustdoc with documentation warnings denied..."
RUSTDOCFLAGS="${RUSTDOCFLAGS:-} -D missing-docs -D rustdoc::broken_intra_doc_links" \
  CARGO_NET_OFFLINE=true \
  cargo doc --locked --workspace --no-deps --document-private-items

echo "Running workspace doctests..."
CARGO_NET_OFFLINE=true cargo test --locked --workspace --doc
