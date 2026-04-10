# Bump, tag, and push a release for GitHub Actions publishing.
# Usage: just pub
# Example: just pub

pub:
	#!/usr/bin/env bash
	set -euo pipefail

	if ! git -C "{{invocation_directory()}}" diff --quiet --exit-code || ! git -C "{{invocation_directory()}}" diff --cached --quiet --exit-code; then
		echo "Git working tree must be clean before running just pub"
		exit 1
	fi

	just --justfile "{{invocation_directory()}}/justfile" pub-bump

	NEW_VERSION=$(node -p "require('{{invocation_directory()}}/package.json').version")
	echo "Release version: $NEW_VERSION"

	cargo check --manifest-path "{{invocation_directory()}}/cli/Cargo.toml" --quiet
	npm --prefix "{{invocation_directory()}}" pack --dry-run >/dev/null

	git -C "{{invocation_directory()}}" add package.json cli/Cargo.toml cli/Cargo.lock
	git -C "{{invocation_directory()}}" commit -m "chore(release): bump version to $NEW_VERSION"
	git -C "{{invocation_directory()}}" tag "v$NEW_VERSION"

	if git -C "{{invocation_directory()}}" remote get-url origin >/dev/null 2>&1; then
		git -C "{{invocation_directory()}}" push
		git -C "{{invocation_directory()}}" push --tags
		echo "Pushed release commit and tag v$NEW_VERSION"
	else
		echo "No origin remote configured. Created commit and tag locally only."
	fi
