# OmniConnect

**A self-hosted API gateway that lets AI agents use connected services via OAuth.**

OmniConnect acts as a unified proxy between AI agents and external APIs. Instead of managing OAuth flows directly, AI agents use OmniConnect API keys to call the proxy, which automatically injects stored OAuth tokens and forwards requests to the native APIs.

## How It Works

```
AI Agent / Client
     │
     │ Bearer $OMNICONNECT_API_KEY
     ▼
OmniConnect Portal
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
cd omni/portal
PORTAL_BASE_URL=https://your-domain.com cargo run -p omni-portal
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
        "shareCommentary": {"text": "Hello from OmniConnect!"},
        "shareMediaCategory": "NONE"
      }
    },
    "visibility": {
      "com.linkedin.ugc.MemberNetworkVisibility": "PUBLIC"
    }
  }'
```

## Supported Platforms

| Platform | Status | Notes |
|----------|--------|-------|
| LinkedIn | ✅ | Posting, user info |
| Facebook | ✅ | Basic API access |
| Feishu | ✅ | Enterprise internal apps not supported |
| DingTalk | ✅ | OAuth2 supported |
| WeChat Work | ✅ | OAuth2 supported |
| Maton.ai | ✅ | API key based |
| QQ Mail | ✅ | API key based |

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
DATABASE_URL=mysql://user:pass@localhost/omni_portal

# PostgreSQL
DATABASE_URL=postgres://user:pass@localhost/omni_portal

# Absolute path for SQLite
DATABASE_URL=sqlite:///absolute/path/to/portal.db
```

## Project Structure

```
omni/
├── portal/              # Web UI and API server (Rust/Axum)
│   ├── frontend/       # React frontend
│   └── src/
│       ├── api/        # REST API routes (connectors, proxy)
│       ├── auth/       # Authentication (login, API keys, sessions)
│       └── oauth/      # OAuth handlers
├── oauth_vault/        # OAuth2 token storage and refresh
│   └── src/platforms/  # Platform-specific OAuth implementations
└── connectors/         # MCP connector implementations (optional)
```

## API Reference

### Proxy Endpoint

```
POST /api/proxy/{platform}/{path}
GET  /api/proxy/{platform}/{path}
```

Authentication: `Authorization: Bearer YOUR_API_KEY`

The proxy automatically:
1. Validates your OmniConnect API key
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
