# Categorization

This repo contains `agent-sight`, a local history/query CLI for user messages from two sources: OpenCode SQLite history and Claude Code local prompt history. The installed app is `agent-sight`, and the main development entrypoint is `just cli ...`, which runs the Rust binary in `cli/target/debug/cli-rs`.

## Agent Operating Rules

- Prefer the Rust CLI under `cli/` over older prototypes or ad hoc scripts.
- Treat OpenCode and Claude as separate adapters.
- Keep Claude-specific code under `cli/src/claude/`.
- Keep OpenCode-specific code under `cli/src/opencode/`.
- Keep shared output shaping in common modules, not provider-specific files.
- Do not reintroduce `opencode db path` subprocess calls; OpenCode DB path is resolved directly.
- Preserve the current command contract exposed through `agent-sight` and `just cli`.

## Environment and Version Constraints

- Rust crate manifest: `cli/Cargo.toml`
- Binary invoked by Just: `cli/target/debug/cli-rs`
- OpenCode DB default path logic matches OpenCode source:
  - `PROMSIGHT_OPENCODE_DB` if set
  - else `XDG_DATA_HOME/opencode/opencode.db`
  - else `~/.local/share/opencode/opencode.db`
- Claude history source file:
  - `~/.claude/history.jsonl`

## Quick Task Playbooks

### Add or change a CLI command

1. Update parsing in `cli/src/args.rs`.
2. Wire command dispatch in `cli/src/main.rs`.
3. Reuse source adapters instead of duplicating query logic.
4. Run `cargo fmt`, `cargo build`, and a `just cli ...` smoke test.

### Change OpenCode query behavior

1. Edit `cli/src/opencode/db.rs`.
2. Keep the time-first query shape.
3. Filter JSON fields in Rust instead of `json_extract(...)` in SQL when possible.
4. Verify verbose timing with `just cli query --since 24h --source opencode --verbose`.

### Change Claude history behavior

1. Edit `cli/src/claude/history.rs`.
2. Stream `history.jsonl` line-by-line.
3. Treat grouping as heuristic, not canonical sessions.
4. Verify with `just cli query --since 24h --source claude`.

## Getting Started

- Build the Rust CLI:

```bash
cargo build --manifest-path cli/Cargo.toml
```

- Run the default CLI:

```bash
just cli query --since 24h
```

- Run against Claude history:

```bash
just cli query --since 24h --source claude
```

## Workspace Overview

- `cli/`
  Rust implementation of the CLI.
- `cli/src/args.rs`
  Command-line parsing and command/source definitions.
- `cli/src/main.rs`
  Top-level command dispatch and output selection.
- `cli/src/opencode/`
  OpenCode SQLite adapter.
- `cli/src/claude/`
  Claude local history adapter.
- `cli/src/output.rs`
  Shared grouping and compact/full output shaping.
- `justfile`
  Developer entrypoints.

## Sources

### OpenCode

- Reads directly from the local SQLite database.
- Current fast path:
  - query recent `message` rows by `time_created`
  - filter `role == user` in Rust
  - fetch `part` rows by `message_id`
  - filter `type == text` in Rust
  - group by session in Rust

### Claude

- Reads from `~/.claude/history.jsonl`.
- Each line is treated as one user prompt.
- Conversations are inferred by:
  - exact project path
  - time gaps between prompts

## Usage Cards

### Query Recent History

Use when
Query user messages from a recent time window.

Enable/Install
Build the Rust binary with `cargo build --manifest-path cli/Cargo.toml`.

Import/Invoke
`just cli query --since 24h`

Minimal flow
1. Choose a source with `--source` if needed.
2. Pass a `--since` window.
3. Optionally restrict with `--directory`.
4. Add `--full` for expanded conversation objects.

Key APIs
- `query`
- `--since`
- `--source`
- `--directory`
- `--full`

Pitfalls
- OpenCode queries can regress badly if SQL-side JSON filtering is reintroduced.
- Claude results are inferred conversations, not true thread IDs.

Source
`cli/src/main.rs`, `cli/src/opencode/db.rs`, `cli/src/claude/history.rs`

### Query One OpenCode Session

Use when
Inspect user messages for one specific OpenCode session.

Enable/Install
Build the Rust binary with `cargo build --manifest-path cli/Cargo.toml`.

Import/Invoke
`just cli session --id <session-id>`

Minimal flow
1. Pass a session ID.
2. Keep source as OpenCode.
3. Add `--full` if you need metadata.

Key APIs
- `session`
- `--id`
- `--full`

Pitfalls
- This command does not support Claude because Claude local history has no canonical session ID.

Source
`cli/src/main.rs`, `cli/src/opencode/db.rs`

### Filter Message Text

Use when
Find user messages containing a specific text fragment.

Enable/Install
Build the Rust binary with `cargo build --manifest-path cli/Cargo.toml`.

Import/Invoke
`just cli filter "rust" --since 24h`

Minimal flow
1. Provide filter text as the first positional argument.
2. Provide `--since`.
3. Optionally set `--source` and `--directory`.

Key APIs
- `filter <text>`
- `--since`
- `--source`
- `--directory`
- `--full`

Pitfalls
- Filter text must come immediately after `filter`.
- Matching is case-insensitive substring matching, not regex.

Source
`cli/src/args.rs`, `cli/src/main.rs`, `cli/src/output.rs`, `cli/src/claude/history.rs`

## API Reference

- `just cli query --since <duration> [--source <source>] [--directory <path>] [--full] [--verbose]`
- `just cli session --id <session-id> [--source opencode] [--full] [--verbose]`
- `just cli filter <text> --since <duration> [--source <source>] [--directory <path>] [--full] [--verbose]`

Supported sources:

- `opencode` (default)
- `claude`

## Common Pitfalls

- Do not assume Claude has real session IDs; grouping is heuristic.
- Do not move Claude parsing logic into shared OpenCode modules.
- Do not switch `just cli` away from the Rust implementation unless requested.
- Do not depend on `opencode` subprocesses for DB path discovery.
- Avoid `json_extract(...)` on hot SQLite scans unless there is a strong reason.

## Optional

- Installed CLI examples:

```bash
agent-sight --version
agent-sight query --since 24h --source claude
```

- Useful verification command:

```bash
just cli query --since 24h --source opencode --verbose
```
