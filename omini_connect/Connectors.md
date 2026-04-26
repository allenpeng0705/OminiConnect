# OminiConnect Connectors

This document defines the connector spec for OminiConnect and platform notes for currently supported global connectors.

## Connector Spec (v1)

OminiConnect supports connectors through two integration engines:

1. **Native** — Built-in OAuth2 / API key connectors (legacy, for Feishu, DingTalk, WeChat Work, QQ Mail, LinkedIn, Facebook, X, Maton)
2. **Nango** — 700+ providers via Nango's integration hub, supporting multiple auth modes

All connectors are configured via the OminiConnect Portal UI and used through `/api/proxy/{platform}/{path}` or `/api/ominiconnect/proxy/{platform}/*path` with an OminiConnect API key.

### Unified Connector Config

Stored model:

- `platform`: unique platform key (`github`, `slack`, `salesforce`, ...)
- `client_id`: OAuth client ID or API key / username field
- `client_secret`: OAuth client secret or password/API key secret field
- `redirect_uri`: OAuth callback URI (auto-generated)
- `scopes`: OAuth scopes list
- `engine`: `omini_connect_native` | `nango`
- `provider_key`: Nango integration key (for Nango-managed connectors)
- `connection_ref`: Nango connection ID (for Nango-managed connectors)
- `enabled`: connector on/off

---

## Supported Auth Modes

OminiConnect supports the following auth modes via the Nango integration hub:

### Tier 1 — Full Support (Direct credential entry or Nango popup)

| Auth Mode | Flow | Count |
|-----------|------|-------|
| **OAuth2** | Nango popup → OAuth authorize → polling for connection | 258 |
| **OAuth1** | Nango popup (legacy OAuth) | 4 |
| **MCP_OAuth2** | Nango popup (OAuth with MCP endpoint) | 12 |
| **Unknown** | Treated as OAuth2 (51 major platforms missing auth_mode in Nango YAML) | 51 |
| **API_KEY** | Direct entry → credentials posted to Nango | 229 |
| **BASIC** | Direct entry (username + password) → posted to Nango | 81 |
| **SIGNATURE** | Direct entry (Emarsys WSSE) → posted to Nango | 1 |

**Total Tier 1: 636 providers**

### Tier 2 — Full Support (Nango Connect popup + polling)

| Auth Mode | Flow | Count |
|-----------|------|-------|
| **OAUTH2_CC** | Direct entry (client_id + client_secret) → posted to Nango | 72 |
| **TWO_STEP** | Nango popup → user completes provider-specific flow | 42 |
| **BILL** | Nango popup (Bill.com custom auth) | 2 |
| **TBA** | Nango popup (NetSuite Token Based Auth) | 1 |

**Total Tier 2: 117 providers**

### Skipped (Deferred)

These auth modes are not yet supported in the portal:

| Auth Mode | Reason | Count |
|-----------|--------|-------|
| JWT | Complex key format, rare | 3 |
| APP | GitHub App auth | 2 |
| CUSTOM | Custom auth per provider | 1 |
| APP_STORE | Apple App Store ecosystem | 1 |
| INSTALL_PLUGIN | Browser plugin install | 1 |
| NONE | Unauthenticated webhook routing | 1 |

**Total skipped: 7 providers**

---

## Integration Engine: Native

For Feishu, DingTalk, WeChat Work, QQ Mail, LinkedIn, Facebook, X, and Maton.

### Type A: Native OAuth2 Connector

Flow:

1. Save connector config (`client_id`, `client_secret`, `scopes`) in Portal
2. Start OAuth from Portal (`GET /oauth/{platform}`)
3. User authorizes in browser
4. Callback (`GET /oauth/{platform}/callback`)
5. Exchange code for token and store in vault
6. Proxy calls use vault token automatically
7. Expired token is auto-refreshed when refresh token exists

### Type B: Native API Key Connector

Flow:

1. Save connector config with key material
2. No OAuth flow and no callback
3. Proxy directly uses configured key from connector secret field

---

## Integration Engine: Nango

For all other providers (700+ integrations via Nango).

### How Nango Managed Connectors Work

1. User selects a provider from the catalog in the Portal UI
2. Based on auth mode, either:
   - **Direct flow**: User enters credentials in the portal form → sent directly to Nango `/connections`
   - **Popup flow**: Portal opens Nango Connect UI in a popup → user authenticates with the provider → Nango notifies portal via WebSocket → portal polls for the new connection
3. Nango stores the connection credentials
4. OminiConnect proxies API calls through Nango using the stored connection

### Proxy Contract

Client calls:

```bash
POST /api/ominiconnect/proxy/{platform}/{path}
Authorization: Bearer YOUR_OMNICONNECT_API_KEY
Content-Type: application/json

{"key": "value"}
```

OminiConnect responsibilities:

1. Validate OminiConnect API key
2. Resolve platform credential (via Nango connection or native vault)
3. Inject provider auth header (Bearer token, API key, or Basic auth)
4. Forward request to provider base URL
5. Return raw upstream response

---

## Platform Notes

## LinkedIn (OAuth2)

### Setup

1. Create app in [LinkedIn Developer Console](https://www.linkedin.com/developers/apps)
2. Add callback URI:
   ```
   https://your-domain.com/oauth/linkedin/callback
   ```
3. Enable required products/scopes in your app

### Suggested Scopes

- `openid` - OpenID Connect
- `profile` - Basic profile
- `email` - Email address
- `w_member_social` - Post on behalf of members (required for sharing)

### Proxy Base URL

- `https://api.linkedin.com`

### API Examples

#### Get User Info
```bash
curl -X POST https://your-domain.com/api/proxy/linkedin/v2/userinfo \
  -H "Authorization: Bearer YOUR_OMNICONNECT_API_KEY" \
  -H "x-linkedin-version: 202404"
```

#### Share a Post (requires `w_member_social`)
```bash
curl -X POST https://your-domain.com/api/proxy/linkedin/v2/ugcPosts \
  -H "Authorization: Bearer YOUR_OMNICONNECT_API_KEY" \
  -H "x-linkedin-version: 202404" \
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
    "visibility": {"memberNetworkVisibility": "PUBLIC"}
  }'
```

### Notes

- LinkedIn has **geo restrictions** - API calls may fail with `451` from some regions
- `w_member_social` requires LinkedIn approval
- Image upload (`w_asset_upload`) requires additional LinkedIn approval and multi-step upload flow

---

## Facebook (OAuth2)

### Setup

1. Go to [Meta Developer Console](https://developers.facebook.com/apps)
2. Click **Create App** → Select **Consumer** or **Business**
3. Add **Facebook Login** product:
   - Go to **Facebook Login** → **Settings**
   - Set Valid OAuth Redirect URI:
     ```
     https://your-domain.com/oauth/facebook/callback
     ```
4. Add permissions in **Permissions and Features** (required for Page posting)

### Suggested Scopes

| Scope | Description | App Review Required |
|-------|-------------|-------------------|
| `public_profile` | Basic profile info | No (dev mode) |
| `email` | Email address | No (dev mode) |
| `pages_manage_posts` | Create posts on Pages | Yes |
| `pages_read_engagement` | Read Page metrics | Yes |

**Note**: In Development mode, only basic permissions work without review. For `pages_manage_posts` and `pages_read_engagement`, you must add them in **Permissions and Features** before requesting.

### Proxy Base URL

- `https://graph.facebook.com/v21.0`

### OminiConnect Facebook Token Flow

OminiConnect automatically fetches a **Page access token** (not user token) after OAuth:
1. Exchange authorization code for user token
2. Extend user token to long-lived token (60 days)
3. Fetch Page access token via `/me/accounts`
4. Store Page token for API calls

This means after connecting Facebook via OAuth, OminiConnect can post to your Page directly.

### API Examples

#### Get User Info
```bash
curl -X POST https://your-domain.com/api/proxy/facebook/me \
  -H "Authorization: Bearer YOUR_OMNICONNECT_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{}'
```

#### Get User's Pages
```bash
curl -X GET https://your-domain.com/api/proxy/facebook/me/accounts \
  -H "Authorization: Bearer YOUR_OMNICONNECT_API_KEY"
```

#### Post to Page
```bash
curl -X POST https://your-domain.com/api/proxy/facebook/{page-id}/feed \
  -H "Authorization: Bearer YOUR_OMNICONNECT_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{"message":"Posted via OminiConnect API!","published":true}'
```

#### Upload Photo to Page
```bash
curl -X POST https://your-domain.com/api/proxy/facebook/{page-id}/photos \
  -H "Authorization: Bearer YOUR_OMNICONNECT_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{"url":"https://example.com/photo.jpg","caption":"Photo caption"}'
```

### Notes

- **Page Permissions**: User must be admin of the Page and grant `pages_manage_posts`
- **App Review**: Required for Live apps. Development mode allows testing with app roles.
- **Token Expiration**: User tokens auto-refreshed to long-lived (60 days)
- **Rate Limits**: Facebook API has rate limits - monitor at developer.facebook.com

---

## X / Twitter (OAuth2)

### Setup

1. Go to [X Developer Portal](https://developer.x.com/console)
2. Create a project and app
3. Set **OAuth 2.0** settings:
   - Callback URL: `https://your-domain.com/oauth/x/callback`
   - App permissions: **Read and Write** (for posting)
4. Copy **API Key** (Client ID) and **API Secret** (Client Secret)

### Suggested Scopes

- `tweet.read` - Read tweets
- `users.read` - Read user profile
- `tweet.write` - Post tweets
- `offline.access` - Refresh token (for token renewal)

### Proxy Base URL

- `https://api.x.com/2`

### OminiConnect X Token Flow

X OAuth 2.0 requires **PKCE** (RFC 7636). OminiConnect generates `code_challenge` / `code_verifier` automatically - no manual configuration needed.

### API Examples

#### Get User Info
```bash
curl -X GET https://your-domain.com/api/proxy/x/users/me \
  -H "Authorization: Bearer YOUR_OMNICONNECT_API_KEY"
```

#### Post a Tweet
```bash
curl -X POST https://your-domain.com/api/proxy/x/tweets \
  -H "Authorization: Bearer YOUR_OMNICONNECT_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{"text":"Hello from OminiConnect! 🐦"}'
```

#### Delete a Tweet
```bash
curl -X DELETE https://your-domain.com/api/proxy/x/tweets/{tweet_id} \
  -H "Authorization: Bearer YOUR_OMNICONNECT_API_KEY"
```

### Notes

- X OAuth requires **Basic Auth header** with client credentials for token exchange
- **App permissions** must be set to Read and Write in X Developer Portal
- X has **credit-based API access** - free tier has monthly limits (500-1500 posts depending on tier)
- `offline.access` scope enables automatic token refresh
- Callback URI must match exactly - including HTTPS

---

## Feishu / Lark (OAuth2)

### Setup

1. Create app at [Feishu Open Platform](https://open.feishu.cn/)
2. Go to **App Credentials** → Get `App ID` and `App Secret`
3. Set Redirect URI:
   ```
   https://your-domain.com/oauth/feishu/callback
   ```
4. Configure required scopes in app settings

### Suggested Scopes

- `contact:user.base:readonly` - Read user basic info

### Proxy Base URL

- `https://open.feishu.cn/open-apis`

### API Examples

#### Get User Info
```bash
curl -X POST https://your-domain.com/api/proxy/feishu/contact/v3/users/me \
  -H "Authorization: Bearer YOUR_OMNICONNECT_API_KEY" \
  -H "Content-Type: application/json"
```

### Notes

- Feishu supports **Custom Apps** (recommended) and Enterprise Internal Apps
- **Enterprise Internal Apps do not support user OAuth** - use Custom App type
- Scopes must be enabled in app configuration before requesting

---

## DingTalk (OAuth2)

### Setup

1. Create app at [DingTalk Open Platform](https://developers.dingtalk.com/)
2. Get `AppKey` (Client ID) and `AppSecret` (Client Secret)
3. Set Redirect URI in app settings

### Proxy Base URL

- `https://api.dingtalk.com`

### API Examples

#### Get User Info
```bash
curl -X POST https://your-domain.com/api/proxy/dingtalk/topapi/v2/user/get \
  -H "Authorization: Bearer YOUR_OMNICONNECT_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{"userid": "USER_ID"}'
```

---

## WeChat Work (OAuth2)

### Overview

WeChat Work (企业微信) is Tencent's enterprise communication platform. It uses OAuth2 with a QR code-based authorization flow similar to QQ login.

### Setup

1. Go to [WeChat Work Admin Console](https://work.weixin.qq.com/)
2. Log in with your enterprise account
3. Navigate to **Apps** → **Create App** (or select existing app)
4. Get your **Corp ID** (CorpID) and **App Secret** (AppSecret)
5. Set Redirect URI in app settings:
   ```
   https://your-domain.com/oauth/wechatwork/callback
   ```

### Suggested Scopes

- `snsapi_base` - Get user userid (basic, no consent required)
- `snsapi_privateinfo` - Get detailed user info (requires user consent)

### Proxy Base URL

- `https://qyapi.weixin.qq.com`

### OminiConnect WeChat Work Token Flow

WeChat Work OAuth2 works differently from other platforms:

1. User is redirected to WeChat authorization page (QR code on web)
2. After scanning and confirming, callback receives an authorization `code`
3. Exchange code for user access token
4. User tokens don't have refresh tokens - re-authentication needed when expired

### API Examples

#### Get User Info (by userid)
```bash
curl -X GET "https://your-domain.com/api/proxy/wechatwork/cgi-bin/user/get?userid=USER_ID" \
  -H "Authorization: Bearer YOUR_OMNICONNECT_API_KEY"
```

#### Get User Info (using OAuth code)
```bash
# First get userid from OAuth token
curl -X GET "https://your-domain.com/api/proxy/wechatwork/cgi-bin/oauth2/getuserinfo?code=AUTH_CODE" \
  -H "Authorization: Bearer YOUR_OMNICONNECT_API_KEY"
```

### Notes

- WeChat Work OAuth shows a **QR code** that users scan with their WeChat Work app
- **User access tokens** don't have refresh tokens - when they expire, users need to re-authenticate
- **App access tokens** (obtained via corpsecret) are different from user tokens
- WeChat Work has different APIs for internal apps vs third-party apps
- Geographic restrictions may apply outside China

---

## API Key Connectors

### Maton.ai

API Key-based connector for Maton.ai agents.

```bash
curl -X POST https://your-domain.com/api/proxy/maton/endpoint \
  -H "Authorization: Bearer YOUR_OMNICONNECT_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{"key": "value"}'
```

### QQ Enterprise Mail

API Key-based connector for QQ Enterprise Mail.

```bash
curl -X POST https://your-domain.com/api/proxy/qqmail/message/send \
  -H "Authorization: Bearer YOUR_OMNICONNECT_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{"to": "user@example.com", "subject": "Hello", "content": "Message"}'
```

---

## Common Issues

### OAuth Redirect URI Mismatch

**Error**: Redirect URI doesn't match registered URI

**Solution**: Set `PORTAL_BASE_URL` environment variable to the same URL users access the portal from:
```bash
PORTAL_BASE_URL=https://your-domain.com cargo run -p omini-connect-portal
```

### Token Not Available

**Error**: `no token available`

**Solution**:
1. Verify OAuth flow completed successfully
2. Check connector is marked as "Connected" in Portal
3. Try reconnecting OAuth

### Geographic Restrictions

Some APIs (LinkedIn, Facebook) have geographic restrictions. If API calls fail with location-related errors:
- Use a VPN or proxy in an allowed region
- Deploy OminiConnect in a supported region

### Platform-Specific Issues

- **Facebook Page permissions**: Requires App Review for Live apps, test in Development mode
- **X API credits**: Free tier has monthly limits; upgrade for higher limits
- **LinkedIn image upload**: Requires additional `w_asset_upload` scope and approval
