#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)"
cd "$repo_root"

fail() {
    printf 'FAIL: %s\n' "$1" >&2
    exit 1
}

assert_contains() {
    local expected="$1"
    local actual="$2"
    grep -F -- "$expected" <<<"$actual" >/dev/null \
        || fail "expected output to contain: $expected"
}

temp_dir="$(mktemp -d)"
trap 'rm -rf "$temp_dir"' EXIT

scripts/check_rust_file_conventions.sh src/main.rs

cat >"$temp_dir/wrong_name.rs" <<'EOF'
pub struct NavigationState;
EOF
if scripts/check_rust_file_conventions.sh "$temp_dir/wrong_name.rs" \
    >"$temp_dir/wrong_name.out" 2>&1; then
    fail 'struct filename convention accepted an invalid filename'
fi
assert_contains 'expected filename navigation_state.rs' "$(cat "$temp_dir/wrong_name.out")"

cat >"$temp_dir/mod.rs" <<'EOF'
pub fn implementation() {}
EOF
if scripts/check_rust_file_conventions.sh "$temp_dir/mod.rs" \
    >"$temp_dir/mod.out" 2>&1; then
    fail 'module aggregator accepted implementation code'
fi
assert_contains 'mod.rs may only declare modules' "$(cat "$temp_dir/mod.out")"

if scripts/check_coverage.sh --threshold 101 >"$temp_dir/coverage.out" 2>&1; then
    fail 'coverage checker accepted a threshold above 100'
fi
assert_contains 'between 0 and 100' "$(cat "$temp_dir/coverage.out")"

assert_contains 'usage:' "$(scripts/publish_crates.sh --help)"
printf '%s\n' 'project script tests passed'
