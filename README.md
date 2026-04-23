# OminiConnect

**A self-hosted API gateway that lets AI agents use connected services via OAuth.**

OminiConnect acts as a unified proxy between AI agents and external APIs. Instead of managing OAuth flows directly, AI agents use OminiConnect API keys to call the proxy, which automatically injects stored OAuth tokens and forwards requests to the native APIs.

## How It Works

```
AI Agent / Client
     │
     │ Bearer $OMNICONNECT_API_KEY
     ▼
OminiConnect Portal
     │
     ├─ User authenticates via OAuth (e.g., LinkedIn, Facebook)
     ├─ Tokens stored securely in database (persisted)
     ├─ Auto-refresh tokens before expiry
     │
     │  POST /api/proxy/linkedin/v2/ugcPosts
     │
     ▼
LinkedIn API (with stored OAuth token injected automatically)
```

## Features

- **OAuth2 Management**: Connect to platforms via standard OAuth2 flows
- **Token Persistence**: OAuth tokens stored in database, survive server restarts
- **Auto Token Refresh**: Tokens automatically refreshed before expiry
- **API Key Auth**: Simple Bearer token authentication for AI agents
- **Passthrough Proxy**: Forward requests to native APIs with tokens injected automatically
- **Multi-Platform**: Support for LinkedIn, Facebook, Feishu, DingTalk, WeChat Work, and more

## Quick Start

### 1. Start the Portal

```bash
cd omini_connect/portal
PORTAL_BASE_URL=https://your-domain.com cargo run -p omini-connect-portal
```

### 2. Access the Web UI

Open `http://localhost:9000` (or your configured domain).

Default login: `admin` / `admin`

### 3. Connect a Platform

1. Go to **Dashboard** → Click **Connect** on a platform
2. Enter your OAuth app credentials (Client ID, Client Secret)
3. Click **Connect OAuth** to authorize
4. Token is stored automatically

### 4. Generate an API Key

1. Go to **API Keys** in the header
2. Click **Create Key**
3. Copy the raw key immediately (shown only once)

### 5. Use the Proxy

```bash
# Get LinkedIn user info
curl -X POST https://your-domain.com/api/proxy/linkedin/v2/userinfo \
  -H "Authorization: Bearer YOUR_API_KEY"

# Create a LinkedIn post
curl -X POST https://your-domain.com/api/proxy/linkedin/v2/ugcPosts \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "author": "urn:li:person:USER_ID",
    "lifecycleState": "PUBLISHED",
    "specificContent": {
      "com.linkedin.ugc.ShareContent": {
        "shareCommentary": {"text": "Hello from OminiConnect!"},
        "shareMediaCategory": "NONE"
      }
    },
    "visibility": {
      "com.linkedin.ugc.MemberNetworkVisibility": "PUBLIC"
    }
  }'
```

## Supported Platforms

| Platform | Type | Status | Notes |
|----------|------|--------|-------|
| LinkedIn | OAuth2 | ✅ | Posting, user info |
| Facebook | OAuth2 | ✅ | Page access tokens (App Review required for Page posting) |
| X / Twitter | OAuth2 | ✅ | Requires PKCE, credit-based API access |
| Feishu / Lark | OAuth2 | ✅ | Custom Apps only (Enterprise Internal Apps not supported) |
| DingTalk | OAuth2 | ✅ | OAuth2 supported |
| WeChat Work | OAuth2 | ✅ | OAuth2 supported |
| Maton.ai | API Key | ✅ | Agent connector |
| QQ Enterprise Mail | API Key | ✅ | Email API |

### Platform-Specific Notes

- **Facebook**: `pages_manage_posts` and `pages_read_engagement` require [App Review](https://developers.facebook.com/docs/app-review) before use in production. Development mode works with app roles.
- **X**: Uses PKCE (RFC 7636) for OAuth2 - no client secret needed in callback. X has credit-based API access with monthly limits.
- **Feishu**: Enterprise Internal Apps do not support user OAuth - use Custom App type.

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `PORTAL_BASE_URL` | `http://localhost:8090` | Base URL for OAuth callbacks |
| `DATABASE_URL` | `sqlite:portal.db` | Database connection URL |

### Database Examples

```bash
# SQLite (default)
DATABASE_URL=sqlite:portal.db

# MySQL
DATABASE_URL=mysql://user:pass@localhost/omini_connect_portal

# PostgreSQL
DATABASE_URL=postgres://user:pass@localhost/omini_connect_portal

# Absolute path for SQLite
DATABASE_URL=sqlite:///absolute/path/to/portal.db
```

## Project Structure

```
third_party/
└── nango/              # Optional vendored upstream (if used)

omini_connect/
├── portal/              # Web UI and API server (Rust/Axum)
│   ├── frontend/       # React frontend
│   └── src/
│       ├── api/        # REST API routes (connectors, proxy)
│       ├── auth/       # Authentication (login, API keys, sessions)
│       └── oauth/      # OAuth handlers
├── oauth_vault/        # OAuth2 token storage and refresh
│   └── src/
│       ├── platforms/  # Platform-specific OAuth implementations
│       └── pkce.rs     # PKCE helpers for X OAuth2
└── connectors/         # MCP connector implementations (optional)
```

See `docs/omini_connect_nango_option_b.md` and `docs/nango_upstream_strategy.md` for the Option B architecture and upstream sync strategy.

To vendor Nango locally as a submodule:

```bash
chmod +x scripts/bootstrap_nango_submodule.sh
./scripts/bootstrap_nango_submodule.sh
```

Copy `.env.example` to `.env` and fill in secrets for local development.

## API Reference

### Proxy Endpoint

```
POST /api/proxy/{platform}/{path}
GET  /api/proxy/{platform}/{path}
```

Authentication: `Authorization: Bearer YOUR_API_KEY`

The proxy automatically:
1. Validates your OminiConnect API key
2. Retrieves the stored OAuth token for the platform
3. Injects the token into the request
4. Forwards to the native platform API
5. Returns the raw response

### Example Requests

```bash
# LinkedIn - get user info
curl -X POST http://localhost:9000/api/proxy/linkedin/v2/userinfo \
  -H "Authorization: Bearer YOUR_KEY"

# Facebook - get user info
curl -X POST http://localhost:9000/api/proxy/facebook/me?fields=id,name,email \
  -H "Authorization: Bearer YOUR_KEY"

# Feishu - get user info
curl -X POST http://localhost:9000/api/proxy/feishu/open-apis/contact/v3/users/me \
  -H "Authorization: Bearer YOUR_KEY"
```

## Security Notes

- OAuth tokens are stored encrypted via bcrypt hashing
- API keys are bcrypt-hashed before storage
- Session cookies use `HttpOnly`, `Secure`, `SameSite=Lax`
- Client secrets are never returned by the API (only masked indicator)

## License

Private - All rights reserved
