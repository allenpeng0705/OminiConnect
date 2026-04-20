# Postgres Explorer MCP

Read-only tools: list schemas/tables, describe columns, run `SELECT` / `WITH` (guarded).

## One-click

```bash
export DATABASE_URL='postgresql://USER:PASS@127.0.0.1:5432/DB'
pip install -r requirements.txt
python3 server.py
```

## Panda `panda.yaml` snippet

```yaml
mcp:
  enabled: true
  servers:
    - name: postgres_explorer
      command: python3
      args: ["/absolute/path/to/panda-mcp-registry/postgres/server.py"]
      env:
        DATABASE_URL: "postgresql://USER:PASS@127.0.0.1:5432/DB"
```

Use an absolute path to `server.py` so Panda’s working directory does not matter.
