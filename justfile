# rats_navigation task runner
# Automation lives in scripts/ so CI, hooks, and local commands share behavior.

set shell := ["bash", "-eu", "-o", "pipefail", "-c"]

# Show available recipes.
default:
    @just --list

# Build every workspace crate.
build:
    cargo build --workspace

# Run workspace tests with the configured coverage threshold.
test *args:
    scripts/check_coverage.sh {{args}}

# Run workspace tests without coverage instrumentation.
test-local:
    cargo test --workspace --all-targets

# Check Rust formatting.
fmt-check:
    cargo fmt --all -- --check

# Format Rust sources in place.
fmt:
    cargo fmt --all

# Run Clippy with warnings denied.
lint:
    cargo clippy --workspace --all-targets -- -D warnings

# Check changed Rust file conventions.
rust-conventions:
    scripts/check_changed_rust_conventions.sh

# Run all pre-push checks.
pre-push:
    scripts/pre-push.sh

# Run the full local quality gate.
ci: fmt-check test-local lint rust-conventions

# Verify that all workspace crates can be published.
publish *args:
    scripts/publish_crates.sh {{args}}

# Install the repository's Git hooks.
install-hooks:
    lefthook install

# Remove Rust build artifacts.
clean:
    cargo clean
