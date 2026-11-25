#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)"
cd "$repo_root"

temp_dir="$(mktemp -d)"
trap 'rm -rf "$temp_dir"' EXIT

scripts/check_coverage.sh --crate rats_core >"$temp_dir/package.out" 2>&1
scripts/check_coverage.sh --threshold 0 >"$temp_dir/workspace.out" 2>&1
printf '%s\n' 'package-scoped coverage tests passed'
