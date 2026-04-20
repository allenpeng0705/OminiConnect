# Local file search (RAG-lite) MCP

Indexes `.txt` files under a directory and runs **keyword overlap** scoring (no vectors, no deps).

## One-click

```bash
export RAG_LITE_ROOT="$HOME/notes"   # optional auto-index on startup
python3 server.py
```

Then use tools `rag_index`, `rag_search`, `rag_stats`.

## Panda snippet

```yaml
mcp:
  enabled: true
  servers:
    - name: rag_lite
      command: python3
      args: ["/absolute/path/to/panda-mcp-registry/local-rag-lite/server.py"]
      env:
        RAG_LITE_ROOT: "/path/to/docs"
```

For mixed formats, extend `glob` (e.g. `**/*.md`) via the `rag_index` tool arguments.
