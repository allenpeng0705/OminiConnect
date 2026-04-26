# TikTok for Business (Accounts) Tools

Provider: `tiktok-accounts` | Engine: `nango` | Auth: OAuth2 via Nango

## Overview

TikTok for Business API for managing ad accounts. **Requires oauth2 via nango.**

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with TikTok for Business (Accounts)
- Token stored in Nango, accessed via `connection_ref`
- Scopes: user.info.basic, advertising.read, advertising.write

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `tiktok_list_ad_accounts` | List all ad accounts | GET | /advertiser/get |
| `tiktok_get_ad_account` | Get ad account details | GET | /advertiser/info |
| `tiktok_list_campaigns` | List all campaigns | GET | /campaign/list |
| `tiktok_get_campaign` | Get campaign details | GET | /campaign/get |
| `tiktok_create_campaign` | Create a new campaign | POST | /campaign/create |
| `tiktok_update_campaign` | Update a campaign | POST | /campaign/update |
| `tiktok_list_ads` | List all ads in a campaign | GET | /ad/list |
| `tiktok_get_ad` | Get ad details | GET | /ad/get |
| `tiktok_create_ad` | Create a new ad | POST | /ad/create |
| `tiktok_get_user_info` | Get basic user info | GET | /user/info |

---

## Tool Details

### tiktok_list_ad_accounts

**What it does**: List all ad accounts

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tiktok_get_ad_account

**What it does**: Get ad account details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tiktok_list_campaigns

**What it does**: List all campaigns

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tiktok_get_campaign

**What it does**: Get campaign details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tiktok_create_campaign

**What it does**: Create a new campaign

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tiktok_update_campaign

**What it does**: Update a campaign

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tiktok_list_ads

**What it does**: List all ads in a campaign

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tiktok_get_ad

**What it does**: Get ad details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tiktok_create_ad

**What it does**: Create a new ad

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tiktok_get_user_info

**What it does**: Get basic user info

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---


## API Notes

- Base URL: `https://business-api.tiktok.com/open_api/v1.3/`
- Docs: https://nango.dev/docs/integrations/all/tiktok-accounts
