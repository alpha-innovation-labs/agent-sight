# Bump the patch version for npm and Rust.
# Usage: just pub-bump
# Example: just pub-bump

pub-bump:
	#!/usr/bin/env bash
	set -euo pipefail

	CURRENT_VERSION=$(node -p "require('{{invocation_directory()}}/package.json').version")
	if [[ -z "${CURRENT_VERSION:-}" ]]; then
		echo "Could not read current version from package.json"
		exit 1
	fi

	IFS='.' read -r MAJOR MINOR PATCH <<< "$CURRENT_VERSION"
	NEW_VERSION="${MAJOR}.${MINOR}.$((PATCH + 1))"

	tmp_package=$(mktemp)
	node -e "const fs=require('fs'); const path='{{invocation_directory()}}/package.json'; const pkg=JSON.parse(fs.readFileSync(path,'utf8')); pkg.version='${NEW_VERSION}'; fs.writeFileSync('${tmp_package}', JSON.stringify(pkg, null, 2) + '\n');"
	mv "$tmp_package" "{{invocation_directory()}}/package.json"

	npm --prefix "{{invocation_directory()}}" run version:sync
	cargo check --manifest-path "{{invocation_directory()}}/cli/Cargo.toml" --quiet

	echo "Current version: $CURRENT_VERSION"
	echo "New version: $NEW_VERSION"
