# TikTok Ads Tools

Provider: `tiktok-ads` | Engine: `nango` | Auth: OAuth2 via Nango

## Overview

TikTok Ads API for managing advertising campaigns. **Requires oauth2 via nango.**

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with TikTok Ads
- Token stored in Nango, accessed via `connection_ref`
- Scopes: advertising.read, advertising.write, campaign.read, campaign.write

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `tiktok_list_ad_accounts` | List all ad accounts | GET | /advertiser/get |
| `tiktok_list_campaigns` | List all campaigns | GET | /campaign/list |
| `tiktok_create_campaign` | Create a new campaign | POST | /campaign/create |
| `tiktok_update_campaign` | Update a campaign | POST | /campaign/update |
| `tiktok_list_ad_groups` | List ad groups in a campaign | GET | /adgroup/list |
| `tiktok_create_ad_group` | Create a new ad group | POST | /adgroup/create |
| `tiktok_list_ads` | List all ads | GET | /ad/list |
| `tiktok_create_ad` | Create a new ad | POST | /ad/create |
| `tiktok_get_ad_report` | Get performance report for an ad | GET | /report/get |
| `tiktok_get_campaign_report` | Get performance report for a campaign | GET | /campaign/report |

---

## Tool Details

### tiktok_list_ad_accounts

**What it does**: List all ad accounts

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

### tiktok_list_ad_groups

**What it does**: List ad groups in a campaign

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tiktok_create_ad_group

**What it does**: Create a new ad group

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tiktok_list_ads

**What it does**: List all ads

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

### tiktok_get_ad_report

**What it does**: Get performance report for an ad

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tiktok_get_campaign_report

**What it does**: Get performance report for a campaign

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---


## API Notes

- Base URL: `https://business-api.tiktok.com/open_api/v1.3/`
- Docs: https://nango.dev/docs/integrations/all/tiktok-ads
