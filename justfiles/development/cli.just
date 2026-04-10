# Run the Rust CLI binary directly.
# Usage: just cli [args...]
# Example: just cli --help

cli *ARGS:
	cargo run --manifest-path {{invocation_directory()}}/cli/Cargo.toml --bin agent-sight -- {{ARGS}}
