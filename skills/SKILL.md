---
name: agent-sight
description: Query local OpenCode and Claude prompt history from the command line. Use when you need to inspect recent user messages, retrieve one OpenCode session, or filter local prompt history by text.
allowed-tools: Bash(agent-sight:*), Bash(just cli *)
---

# Agent Sight

`agent-sight` is a local CLI for querying prompt history from two sources:

- `opencode` for OpenCode SQLite history
- `claude` for Claude Code `~/.claude/history.jsonl`

Agents already know the harness they are running in, so they should choose the source directly in their command calls when needed.

Examples:

```bash
agent-sight query --since 24h --source opencode
agent-sight query --since 24h --source claude
agent-sight filter "rust sqlite" --since 7d --source claude
agent-sight session --id ses_123 --source opencode
```

## When To Use

Use `agent-sight` when you need to:

- inspect recent local prompt history
- compare prompt activity across OpenCode and Claude
- retrieve all user messages from one OpenCode session
- search local prompt history for a text fragment

Do not use it for:

- editing prompt history
- querying remote hosted services
- retrieving Claude session IDs, which do not exist as canonical IDs

## Core Commands

### Query recent history

```bash
agent-sight query --since <duration> [--source <source>] [--directory <path>] [--full] [--verbose]
```

Use this to fetch recent user messages from a local source.

Examples:

```bash
agent-sight query --since 24h
agent-sight query --since 7d --source claude
agent-sight query --since 24h --source opencode --directory /Users/alex/work/project
```

### Query one OpenCode session

```bash
agent-sight session --id <session-id> [--source opencode] [--full] [--verbose]
```

Use this to inspect one specific OpenCode session.

Examples:

```bash
agent-sight session --id ses_123
agent-sight session --id ses_123 --full
```

`session` is OpenCode-only.

### Filter by text

```bash
agent-sight filter <text> --since <duration> [--source <source>] [--directory <path>] [--full] [--verbose]
```

Use this to find recent user messages containing a text fragment.

Examples:

```bash
agent-sight filter "bug report" --since 24h
agent-sight filter "rust sqlite" --since 7d --source claude
```

## Sources

- `opencode`: queries the local OpenCode SQLite database
- `claude`: queries `~/.claude/history.jsonl`

If `--source` is omitted, the default is `opencode`.

## Important Options

- `--source <source>`: choose `opencode` or `claude`
- `--since <duration>`: required for `query` and `filter`; accepts values like `24h` or `7d`
- `--directory <path>`: restrict results to a specific project directory
- `--id <session-id>`: required for `session`
- `--full`: return expanded conversation objects instead of compact output
- `--verbose`: print step-by-step progress and timing

## Output Behavior

- Default output is compact JSON.
- `--full` returns richer conversation objects with metadata.
- `session` in compact mode returns an array of message strings when exactly one conversation is found.

## Recommended Agent Workflow

1. Start with `query` when you need broad recent context.
2. Use `--source` explicitly when the harness context already tells you which provider matters.
3. Use `--directory` to narrow to one project when the user is asking about a specific repo.
4. Use `filter` when you already know a likely phrase or topic.
5. Use `session` only for OpenCode when you already have a session ID.
6. Add `--full` only when you need metadata, not for quick scans.

## Common Pitfalls

- `session` does not work for Claude.
- `filter` requires the text immediately after the command name.
- `query` and `filter` require `--since`.
- Claude conversation grouping is heuristic, not based on true session IDs.

## Examples For Agents

```bash
# Recent Claude prompts
agent-sight query --since 24h --source claude

# Recent OpenCode prompts in the current repo
agent-sight query --since 24h --source opencode --directory /path/to/project

# Search Claude prompts for a topic
agent-sight filter "postgres migration" --since 7d --source claude

# Inspect one OpenCode session in detail
agent-sight session --id ses_123 --source opencode --full
```
