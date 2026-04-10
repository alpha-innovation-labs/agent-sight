# Show user message statistics.
# Usage: just stats [args...]
# Example: just stats --help

stats *ARGS:
	node {{invocation_directory()}}/.opencode/message-stats.mjs {{ARGS}}
