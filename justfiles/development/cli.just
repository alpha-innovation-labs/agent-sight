# Run the Rust CLI binary directly.
# Usage: just cli [args...]
# Example: just cli --help

cli *ARGS:
	cargo run --manifest-path ../../src/Cargo.toml --bin cli-rs -- {{ARGS}}
