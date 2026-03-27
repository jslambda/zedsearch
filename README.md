# zed-srcsearch (Phase 1)

A thin Zed slash-command extension that runs the local [`srcsearch`](https://github.com/jslambda/srcsearch) CLI for repository indexing and search.

## What this extension does

- Local repository indexing from inside Zed
- BM25-style ranked search through `srcsearch`
- Documentation-only search scope
- Manual index refresh

This extension intentionally keeps all indexing and retrieval logic in `srcsearch`.

## Requirements

- [Zed](https://zed.dev)
- `srcsearch` installed locally and available on `PATH`
- Extension process execution capability for running `srcsearch`

## Commands

- `/srcindex` — build an index for the active workspace
- `/srcsearch <query>` — search across indexed code + docs
- `/srcdocs <query>` — search only documentation-oriented fields (`--scope doc`)
- `/srcupdate` — refresh index (Phase 1: full rebuild fallback)

## Index location

The index is stored in:

```text
<repo-root>/.zed/srcsearch-index/
```

Where `<repo-root>` is the active Zed worktree root.

## Setup (dev extension)

1. Clone or copy this extension directory locally.
2. Ensure `srcsearch` is installed and available from your shell path:
   ```bash
   srcsearch --help
   ```
3. Install/load as a Zed dev extension.
4. Open a project folder and run `/srcindex`.

## Known limitations (Phase 1)

- Manual indexing/update only
- No custom search panel UI
- No automatic file watching or auto-refresh on save
- Depends on local `srcsearch` installation
