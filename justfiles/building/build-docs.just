# Build the Agent Sight docs app.
# Usage: just build-docs
# Example: just build-docs

build-docs:
	bun run --cwd {{invocation_directory()}}/docs build
