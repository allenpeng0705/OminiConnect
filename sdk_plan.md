# OminiConnect SDK & REST API Plan

> Status: **Implementation — Python + TypeScript SDKs + REST API**

## Design Decisions (agreed)

- **Philosophy**: Maton-simple — connect once, call any API directly
- **Primary API**: `client.call(platform, method, path, params?, body?)` — Maton-style passthrough
- **Tools**: Optional discoverability + scope-checking layer on top
- **API keys**: Named/labled per agent/use case via `client.api_keys.create(label)`
- **Two paths**: Fast direct call OR structured tool execute

---

## REST API Changes Needed

### New: `POST /api/call/{platform}`
MATON-STYLE GATEWAY CALL — the primary API.

```
POST /api/call/{platform}
Authorization: Bearer sk-xxxxx
Content-Type: application/json

{ "method": "GET|POST|PUT|DELETE|PATCH", "path": "/user/repos", "params": {...}, "body": {...} }
```

- `method`: HTTP verb
- `path`: the platform API path (e.g., `/user/repos`)
- `params`: query string parameters (optional)
- `body`: request body for POST/PUT/PATCH (optional)

**Response**: Raw upstream body + status code forwarded as-is.

This replaces the need for separate GET/POST proxy routes.

### Unchanged (already exists)
- `GET /api/connectors`
- `POST /api/connectors` / `DELETE /api/connectors/:platform`
- `GET /api/tools`
- `GET /api/tools/search`
- `POST /api/tools/execute` → should wrap response as `{ ok, body }` or `{ ok, error }`
- `POST /auth/apikey`, `GET /auth/apikey`, `DELETE /auth/apikey/:key_hash`
- `POST /api/mcp`, `GET /api/mcp/sse`

---

## Implementation Order

1. **Add `POST /api/call/{platform}`** to the Rust backend
2. **Fix Python SDK** — add `call()`, connectors, errors, TypedDict models
3. **Fix TypeScript SDK** — add `call()`, connectors, api_keys, proper types
4. **Update `sdk/README.md`** with Maton-style usage examples

---

## Files to Modify

| File | Change |
|------|--------|
| `src/api/call.rs` | **New** — `POST /api/call/{platform}` handler |
| `src/api/mod.rs` | Add `call::router()` |
| `sdk/python/ominiconnect/__init__.py` | Export `OminiConnect`, `errors` |
| `sdk/python/ominiconnect/client.py` | Add `.call()`, `.connectors`, `.api_keys`, structured errors |
| `sdk/python/ominiconnect/tools.py` | Fix `list()` response shape, `execute()` platform required |
| `sdk/python/ominiconnect/connectors.py` | **New** |
| `sdk/python/ominiconnect/errors.py` | **New** |
| `sdk/python/ominiconnect/models.py` | **New** — TypedDict types |
| `sdk/js/src/index.ts` | Rewrite with proper types, `.call()`, connectors, api_keys |
| `sdk/README.md` | Maton-style examples |

---

## Verification

1. `cargo build` — backend compiles
2. `curl -X POST http://localhost:9000/api/call/github -H "Authorization: Bearer KEY" -d '{"method":"GET","path":"/user"}'` → GitHub user JSON
3. `cd sdk/python && pip install -e . && python -c "from ominiconnect import OminiConnect; c = OminiConnect('test'); print(c.call('github','GET','/user'))"` → no import error
4. `cd sdk/js && npm run build && npx tsc --noEmit` → TypeScript compiles clean

---

*Last updated: 2026-04-27*
