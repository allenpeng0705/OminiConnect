# GitHub Issue Manager MCP

Uses the **GitHub REST API** with `GITHUB_TOKEN` (fine-grained or classic PAT with `issues` read/write for create).

## One-click

```bash
export GITHUB_TOKEN='ghp_...'
python3 server.py
```

## Tools

- `gh_list_issues` — `owner`, `repo`, optional `per_page`
- `gh_get_issue` — `owner`, `repo`, `number`
- `gh_create_issue` — `owner`, `repo`, `title`, optional `body`

## Panda snippet

```yaml
mcp:
  enabled: true
  servers:
    - name: github
      command: python3
      args: ["/absolute/path/to/panda-mcp-registry/github-issues/server.py"]
      env:
        GITHUB_TOKEN: "${GITHUB_TOKEN}"
```
