# Start the Agent Sight docs app.
# Usage: just docs
# Example: just docs

docs:
	bun run --cwd {{invocation_directory()}}/docs dev
