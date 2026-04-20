# MCP starter stack (Docker Compose)

Runs **Panda** with **Node.js** inside the image so `npx @modelcontextprotocol/*` MCP servers can be spawned as **stdio** children (same model as local `panda.yaml`). MCP packages are preinstalled at image build, and runtime uses `npx --no-install`.

## What you get

| Service | Role |
|---------|------|
| `panda` | Gateway on `:8080` with MCP enabled |
| `postgres` | Database for `@modelcontextprotocol/server-postgres` |

Bundled MCP processes (see `panda.mcp-starter.yaml`):

1. **Filesystem** — read-only `./workspace` (create files there for demos).  
2. **GitHub** — needs `GITHUB_PERSONAL_ACCESS_TOKEN`.  
3. **Postgres** — uses `postgresql://panda:panda@postgres:5432/panda`.  
4. **Memory** — knowledge-graph style memory server.  
5. **Fetch** — URL → markdown fetch (handy without OAuth).  

**Google Drive** is documented as an optional swap-in: the official server expects OAuth desktop keys (`gcp-oauth.keys.json`). Uncomment the `gdrive` block in `panda.mcp-starter.yaml`, set `enabled: true`, disable `fetch` if you like, and mount keys:

```yaml
volumes:
  - ./workspace:/workspace:ro
  - ./secrets/gcp-oauth.keys.json:/secrets/gcp-oauth.keys.json:ro
```

## Quick start

From the **repository root**:

```bash
cp deploy/mcp-starters/.env.example deploy/mcp-starters/.env
# edit OPENAI_API_KEY
mkdir -p deploy/mcp-starters/workspace
echo "Hello from MCP workspace" > deploy/mcp-starters/workspace/README.txt
docker compose -f deploy/mcp-starters/docker-compose.yml up --build
```

Health:

```bash
curl -s http://127.0.0.1:8080/health
curl -s http://127.0.0.1:8080/mcp/status
```

## Override config

Mount your own YAML over `/app/panda.yaml`:

```yaml
services:
  panda:
    volumes:
      - ./my-panda.yaml:/app/panda.yaml:ro
```

## Security notes

- `server-fetch` can retrieve arbitrary URLs; use only in trusted environments or disable in `panda.mcp-starter.yaml`.  
- GitHub and OpenAI tokens are **secrets**; do not commit `.env`.  
- MCP packages are installed during image build; runtime avoids ad-hoc package fetches (`npx --no-install`).

## See also

- Community Wasm plugins: [`community-plugins/README.md`](../../community-plugins/README.md)  
- Example stdio MCP (Python): [`examples/mcp_stdio_minimal/`](../../examples/mcp_stdio_minimal/)  
