#!/usr/bin/env bash
set -euo pipefail

threshold="${COVERAGE_THRESHOLD:-100}"
package=""

while (( $# > 0 )); do
    case "$1" in
        --crate|--package)
            (( $# >= 2 )) || { printf '%s requires a package name\n' "$1" >&2; exit 2; }
            package="$2"
            shift 2
            ;;
        --threshold)
            (( $# >= 2 )) || { printf '%s requires a percentage\n' "$1" >&2; exit 2; }
            threshold="$2"
            shift 2
            ;;
        *)
            printf 'usage: %s [--crate PACKAGE] [--threshold PERCENT]\n' "$0" >&2
            exit 2
            ;;
    esac
done

[[ "$threshold" =~ ^[0-9]+$ ]] || {
    printf 'coverage threshold must be an integer percentage between 0 and 100\n' >&2
    exit 2
}
(( threshold <= 100 )) || {
    printf 'coverage threshold must be between 0 and 100\n' >&2
    exit 2
}

coverage_command=(cargo llvm-cov --workspace --all-features --fail-under-lines "$threshold")
if [[ -n "$package" ]]; then
    coverage_command+=(--package "$package")
fi

"${coverage_command[@]}"
