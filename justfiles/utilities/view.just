# View exported user messages.
# Usage: just view [args...]
# Example: just view --help

view *ARGS:
	node {{invocation_directory()}}/.opencode/view-user-messages.mjs {{ARGS}}
