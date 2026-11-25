#!/usr/bin/env bash
set -euo pipefail

usage() {
    printf 'usage: %s [--dry-run] [cargo publish options...]\n' "$0"
}

dry_run=0
publish_args=()
while (( $# > 0 )); do
    case "$1" in
        --dry-run)
            dry_run=1
            shift
            ;;
        -h|--help)
            usage
            exit 0
            ;;
        *)
            publish_args+=("$1")
            shift
            ;;
    esac
done

command=(cargo publish --workspace --locked)
if (( dry_run == 1 )); then
    command+=(--dry-run)
fi
command+=("${publish_args[@]}")

"${command[@]}"
