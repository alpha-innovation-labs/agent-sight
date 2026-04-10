# View exported user messages.
# Usage: just view [args...]
# Example: just view --help

view *ARGS:
	node ../../.opencode/view-user-messages.mjs {{ARGS}}
