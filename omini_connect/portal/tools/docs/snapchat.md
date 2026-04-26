# Snapchat Tools

Provider: `snapchat` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Snapchat Marketing API. They allow AI agents to manage advertising campaigns, create ads, track performance, and access Spectacles (AR lens) content. Snapchat is a major platform for reaching younger audiences through visual, engaging ad formats.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Snapchat
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `snapchat-marketing-api`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `snapchat_list_ad_accounts` | List advertising accounts | GET | /v1/adaccounts |
| `snapchat_get_ad_account` | Get ad account details | GET | /v1/adaccounts/{adaccountId} |
| `snapchat_list_campaigns` | List campaigns in an ad account | GET | /v1/campaigns |
| `snapchat_get_campaign` | Get campaign details | GET | /v1/campaigns/{campaignId} |
| `snapchat_create_campaign` | Create a new campaign | POST | /v1/campaigns |
| `snapchat_list_ads` | List ads in a campaign | GET | /v1/ads |
| `snapchat_get_ad` | Get ad details | GET | /v1/ads/{adId} |
| `snapchat_create_ad` | Create a new ad | POST | /v1/ads |
| `snapchat_list_spectacles` | List Spectacles content | GET | /v1/spectacles |
| `snapchat_get_spectacle` | Get Spectacle details | GET | /v1/spectacles/{spectacleId} |

---

## Tool Details

### snapchat_list_ad_accounts

**What it does**: Retrieves all advertising accounts the authenticated user has access to.

**When to use**: Find available ad accounts before creating campaigns or ads.

**Arguments**:
- `limit` (optional): Max results (default 100)

**Example LLM prompt**: "List all my Snapchat ad accounts"

---

### snapchat_get_ad_account

**What it does**: Gets detailed information about a specific advertising account including budget, status, and settings.

**When to use**: Check account status, budget remaining, or account settings.

**Arguments**:
- `adaccountId` (required): Ad account ID

**Example LLM prompt**: "Get details for ad account abc123"

---

### snapchat_list_campaigns

**What it does**: Lists all campaigns within an advertising account.

**When to use**: Review existing campaigns, check status, or find campaign IDs.

**Arguments**:
- `adaccountId` (required): Ad account ID
- `limit` (optional): Max results (default 100)

**Example LLM prompt**: "List all campaigns in my ad account"

---

### snapchat_get_campaign

**What it does**: Gets detailed information about a specific campaign including budget, schedule, and performance metrics.

**When to use**: Review campaign details before modifying or checking performance.

**Arguments**:
- `campaignId` (required): Campaign ID

**Example LLM prompt**: "Get campaign details for campaign xyz456"

---

### snapchat_create_campaign

**What it does**: Creates a new advertising campaign within an ad account.

**When to use**: Launch a new advertising initiative, set up seasonal promotions, or start A/B testing.

**Arguments**:
- `adaccountId` (required): Ad account ID
- `name` (required): Campaign name
- `status` (optional): Campaign status (ACTIVE, PAUSED)
- `daily_budget` (optional): Daily budget in cents
- `start_time` (optional): Start time (ISO 8601)
- `end_time` (optional): End time (ISO 8601)

**Example LLM prompt**: "Create a new campaign named 'Spring Sale' with a daily budget of $50"

---

### snapchat_list_ads

**What it does**: Lists all ads within a specific campaign.

**When to use**: Review ads in a campaign, check which ads are running.

**Arguments**:
- `campaignId` (required): Campaign ID
- `limit` (optional): Max results (default 100)

**Example LLM prompt**: "List all ads in campaign xyz456"

---

### snapchat_get_ad

**What it does**: Gets detailed information about a specific ad including creative content and status.

**When to use**: Review ad details, check performance, or prepare modifications.

**Arguments**:
- `adId` (required): Ad ID

**Example LLM prompt**: "Get details for ad abc789"

---

### snapchat_create_ad

**What it does**: Creates a new ad within a campaign.

**When to use**: Add new creatives to a campaign, launch new variations, or test new messaging.

**Arguments**:
- `campaignId` (required): Campaign ID
- `name` (required): Ad name
- `ad_type` (required): Ad type (SNAP_AD, STORY, FILTER, etc.)
- `creative` (optional): Ad creative object

**Example LLM prompt**: "Create a new SNAP_AD in campaign xyz456 called 'Summer Collection'"

---

### snapchat_list_spectacles

**What it does**: Lists all Spectacles (AR lenses) the user has access to.

**When to use**: Browse available AR lenses for use in campaigns.

**Arguments**:
- `limit` (optional): Max results (default 100)

**Example LLM prompt**: "List all Spectacles I have access to"

---

### snapchat_get_spectacle

**What it does**: Gets detailed information about a specific Spectacle including lens ID and availability.

**When to use**: Check a specific lens before using it in an ad.

**Arguments**:
- `spectacleId` (required): Spectacle ID

**Example LLM prompt**: "Get details for Spectacle lens abc123"

---

## Snapchat API Notes

- **Budgets**: Snapchat uses cents as the base unit (e.g., $50 = 5000 cents)
- **Ad Types**: SNAP_AD is the standard full-screen vertical ad format
- **Spectacles**: AR lenses created through Lens Studio, primarily for organic content
- **Campaign Structure**: Ad Account > Campaign > Ad Set > Ad
- **Targeting**: Snapchat offers demographic and interest-based targeting options
