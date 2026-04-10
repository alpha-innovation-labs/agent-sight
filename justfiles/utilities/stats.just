# Show user message statistics.
# Usage: just stats [args...]
# Example: just stats --help

stats *ARGS:
	node ../../.opencode/message-stats.mjs {{ARGS}}
