# Export user messages to a JSON file.
# Usage: just dev [output] [args...]
# Example: just dev .opencode/user-messages.json --pretty

dev OUTPUT=".opencode/user-messages.json" *ARGS:
	node {{invocation_directory()}}/.opencode/export-user-messages.mjs --output {{OUTPUT}} {{ARGS}}
