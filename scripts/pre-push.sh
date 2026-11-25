#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
cd "$repo_root"

printf '%s\n' 'Checking Rust formatting...'
cargo fmt --all -- --check
printf '%s\n' 'Running workspace tests...'
cargo test --workspace --all-targets
printf '%s\n' 'Running Clippy with warnings denied...'
cargo clippy --workspace --all-targets -- -D warnings
printf '%s\n' 'Checking Rust file conventions...'
scripts/check_changed_rust_conventions.sh
