# OmniConnect Connectors

This document defines the connector spec for OmniConnect and platform notes for currently supported global connectors.

## Connector Spec (v1)

OmniConnect supports two connector types:

1. OAuth2 connectors
2. API key connectors

All connectors are configured in `/api/connectors` and used through `/api/proxy/{platform}/{path}` with an OmniConnect API key.

### Unified Connector Config

Stored model:

- `platform`: unique platform key (`linkedin`, `facebook`, `x`, `maton`, `qqmail`, ...)
- `client_id`: OAuth client ID or API key ID field
- `client_secret`: OAuth client secret or API key secret field
- `redirect_uri`: OAuth callback URI (empty for API key connectors)
- `scopes`: OAuth scopes list (empty for API key connectors)
- `enabled`: connector on/off

### Type A: OAuth2 Connector

Flow:

1. Save connector config (`client_id`, `client_secret`, `redirect_uri`, `scopes`)
2. Start OAuth (`GET /oauth/{platform}`)
3. Callback (`GET /oauth/{platform}/callback`)
4. Exchange code for token and store in vault
5. Proxy calls use vault token automatically
6. Expired token is auto-refreshed when refresh token exists

Token requirements:

- Must store `access_token`
- Should store `refresh_token` when available
- Must store `expires_at`

### Type B: API Key Connector

Flow:

1. Save connector config with key material
2. No OAuth flow and no callback
3. Proxy directly uses configured key from connector secret field

Token requirements:

- No refresh
- No token exchange

### Proxy Contract

Client calls:

```bash
POST /api/proxy/{platform}/{native_api_path}
Authorization: Bearer YOUR_OMNICONNECT_API_KEY
```

OmniConnect responsibilities:

1. Validate OmniConnect API key
2. Resolve platform credential (OAuth token or API key)
3. Inject provider auth header
4. Forward request to provider base URL
5. Return raw upstream response

## Platform Notes

## LinkedIn (OAuth2)

### Setup

1. Create app in [LinkedIn Developer Console](https://www.linkedin.com/developers/apps)
2. Add callback URI:
   `https://your-domain.com/oauth/linkedin/callback`
3. Enable required products/scopes

### Suggested Scopes

- `openid`
- `profile`
- `email`
- `w_member_social`

### Proxy Base URL

- `https://api.linkedin.com`

### Example

```bash
curl -X POST https://your-domain.com/api/proxy/linkedin/v2/userinfo \
  -H "Authorization: Bearer YOUR_OMNICONNECT_API_KEY"
```

### Notes

- LinkedIn geo restrictions can cause `451` from some regions.
- `w_asset_upload` requires additional LinkedIn approval.

## Facebook (OAuth2)

### Setup

1. Go to [Meta Developer Console](https://developers.facebook.com/apps)
2. Click **Create App** → Select **Consumer** (for personal posts) or **Business** (for Page management)
3. Add **Facebook Login** product:
   - Go to **Facebook Login** → **Settings**
   - Set Valid OAuth Redirect URI:
     ```
     https://your-domain.com/oauth/facebook/callback
     ```
4. Configure permissions (see Suggested Scopes below)

### Suggested Scopes

| Scope | Description | Requires Review |
|-------|-------------|----------------|
| `public_profile` | Basic profile info | No |
| `email` | Email address | No |
| `pages_manage_posts` | Create posts on Pages | Yes |
| `pages_read_engagement` | Read Page metrics | Yes |
| `publish_video` | Publish videos | Yes |
| `instagram_content_publish` | Publish to Instagram | Yes |

**Note**: `public_profile` and `email` are available immediately. Other scopes require Facebook app review.

### Proxy Base URL

- `https://graph.facebook.com/v21.0`

### API Examples

#### Get User Info
```bash
curl -X POST https://your-domain.com/api/proxy/facebook/me?fields=id,name,email \
  -H "Authorization: Bearer YOUR_OMNICONNECT_API_KEY"
```

#### Create Post to Timeline
```bash
curl -X POST https://your-domain.com/api/proxy/facebook/me/feed \
  -H "Authorization: Bearer YOUR_OMNICONNECT_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{"message":"Hello from OmniConnect!"}'
```

#### Get User's Pages
```bash
curl -X POST https://your-domain.com/api/proxy/facebook/me/accounts \
  -H "Authorization: Bearer YOUR_OMNICONNECT_API_KEY"
```

#### Post to Page
```bash
curl -X POST https://your-domain.com/api/proxy/facebook/{page-id}/feed \
  -H "Authorization: Bearer YOUR_OMNICONNECT_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{"message":"Posted via OmniConnect API!","published":true}'
```

#### Upload Photo (from URL)
```bash
curl -X POST https://your-domain.com/api/proxy/facebook/me/photos \
  -H "Authorization: Bearer YOUR_OMNICONNECT_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{"url":"https://public-image-url.com/photo.jpg","caption":"Photo caption"}'
```

### Notes

- **App Review**: Most permissions beyond `public_profile`/`email` require Facebook app review
- **Page Posting**: User must be Page admin and grant `pages_manage_posts`
- **Token Expiration**: Short-lived tokens (2h) auto-refreshed to long-lived (60d) when possible
- **Rate Limits**: Facebook API has rate limits - monitor at developer.facebook.com

## X / Twitter (OAuth2)

### Setup

1. Create app in X Developer Portal
2. Add callback URI:
   `https://your-domain.com/oauth/x/callback`
3. Configure app permissions/scopes for read/write as needed

### Suggested Scopes

- `tweet.read`
- `users.read`
- `tweet.write`
- `offline.access` (required for refresh-token style usage)

### Proxy Base URL

- `https://api.x.com/2`

### Notes

- X OAuth 2.0 **requires PKCE** (RFC 7636). OmniConnect generates `code_challenge` / `code_verifier` automatically when you use **Connect OAuth** from the portal; you do not configure PKCE manually.
- Callback URI and scopes must match your X app settings exactly.
- Set `PORTAL_BASE_URL` to the same origin users use in the browser (for example `http://localhost:9000` if the portal listens on port 9000), otherwise the redirect URI sent to X will not match what you registered.
