#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
cd "$repo_root"

base_ref="${BASE_REF:-origin/main}"
if git rev-parse --verify "$base_ref" >/dev/null 2>&1; then
    mapfile -t rust_files < <(git diff --name-only "$base_ref...HEAD" -- '*.rs')
else
    mapfile -t rust_files < <(git ls-files '*.rs')
fi

if (( ${#rust_files[@]} == 0 )); then
    printf '%s\n' 'No changed Rust files to check.'
    exit 0
fi

scripts/check_rust_file_conventions.sh "${rust_files[@]}"
