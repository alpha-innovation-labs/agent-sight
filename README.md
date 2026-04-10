# Agent Sight

Agent Sight is a Rust CLI for querying local OpenCode SQLite history and Claude Code history, distributed as an npm package with native prebuilt binaries.

## Install

```bash
npm install -g agent-sight
```

## Usage

```bash
agent-sight --version
agent-sight query --since 24h
agent-sight query --since 24h --source claude
agent-sight session --id ses_123
```

## Development

```bash
npm run build:native
just cli --help
```

`package.json` is the single version source of truth. Run `npm run version:sync` before building or publishing if you changed the npm version.
