# Meta Marketing API Tools

Provider: `meta-marketing-api` | Engine: `nango` | Auth: OAuth2 via Nango (alias: facebook)

## Overview

These tools wrap the Meta Marketing API. They allow AI agents to manage Facebook/Meta ad campaigns, ad sets, ads, and analytics. **Requires Meta Marketing API OAuth2.**

## Authentication

**Nango OAUTH2**:
- User authenticates via Nango Connect with Meta
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://graph.facebook.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `meta_list_ad_accounts` | List advertising accounts | GET | /me/adaccounts |
| `meta_get_ad_account` | Get ad account details | GET | /act_{accountId} |
| `meta_list_campaigns` | List campaigns | GET | /act_{accountId}/campaigns |
| `meta_get_campaign` | Get campaign details | GET | /{campaignId} |
| `meta_list_ad_sets` | List ad sets | GET | /act_{accountId}/adsets |
| `meta_get_ad_set` | Get ad set details | GET | /{adsetId} |
| `meta_list_ads` | List ads | GET | /act_{accountId}/ads |
| `meta_get_ad` | Get ad details | GET | /{adId} |
| `meta_list_adsets_insights` | Get ad set insights | GET | /{adsetId}/insights |
| `meta_list_campaign_insights` | Get campaign insights | GET | /{campaignId}/insights |

---

## Tool Details

### meta_list_ad_accounts

**What it does**: Lists all advertising accounts accessible to the user.

**When to use**: Find ad account IDs, check account status.

**Arguments**:
- `fields` (optional): Fields to return (default "name,account_id,account_status")

**Example LLM prompt**: "List all my Meta ad accounts"

---

### meta_get_ad_account

**What it does**: Gets details of a specific ad account.

**When to use**: Check account settings, view account balance.

**Arguments**:
- `accountId` (required): Ad account ID
- `fields` (optional): Fields to return

**Example LLM prompt**: "Get details for ad account act_123456789"

---

### meta_list_campaigns

**What it does**: Lists all campaigns for an ad account.

**When to use**: Browse campaigns, check campaign status.

**Arguments**:
- `accountId` (required): Ad account ID
- `status` (optional): Filter by status (1=active, 2=paused)
- `objective` (optional): Filter by objective
- `fields` (optional): Fields to return

**Example LLM prompt**: "List all active campaigns for account act_123456789"

---

### meta_get_campaign

**What it does**: Gets details of a specific campaign.

**When to use**: Check campaign settings, review campaign performance.

**Arguments**:
- `campaignId` (required): Campaign ID
- `fields` (optional): Fields to return

**Example LLM prompt**: "Get details for campaign 123456789"

---

### meta_list_ad_sets

**What it does**: Lists all ad sets for a campaign or account.

**When to use**: Manage targeting groups, adjust budgets.

**Arguments**:
- `accountId` (required): Ad account ID
- `campaign_id` (optional): Filter by campaign ID
- `status` (optional): Filter by status
- `fields` (optional): Fields to return

**Example LLM prompt**: "List all ad sets for campaign 123456789"

---

### meta_get_ad_set

**What it does**: Gets details of a specific ad set.

**When to use**: Check ad set settings, review targeting.

**Arguments**:
- `adsetId` (required): Ad set ID
- `fields` (optional): Fields to return

**Example LLM prompt**: "Get details for ad set 123456789"

---

### meta_list_ads

**What it does**: Lists all ads for an ad set or account.

**When to use**: Manage individual ads, check ad status.

**Arguments**:
- `accountId` (required): Ad account ID
- `adset_id` (optional): Filter by ad set ID
- `status` (optional): Filter by status
- `fields` (optional): Fields to return

**Example LLM prompt**: "List all ads in ad set 123456789"

---

### meta_get_ad

**What it does**: Gets details of a specific ad.

**When to use**: Check ad creative, review ad status.

**Arguments**:
- `adId` (required): Ad ID
- `fields` (optional): Fields to return

**Example LLM prompt**: "Get details for ad 123456789"

---

### meta_list_adsets_insights

**What it does**: Gets insights data for an ad set.

**When to use**: Analyze ad set performance, track metrics.

**Arguments**:
- `adsetId` (required): Ad set ID
- `date_preset` (optional): Date range preset (today, yesterday, last_7d, etc.)
- `fields` (optional): Insights fields to return

**Example LLM prompt**: "Get impressions and clicks for ad set 123456789"

---

### meta_list_campaign_insights

**What it does**: Gets insights data for a campaign.

**When to use**: Analyze campaign ROI, track performance.

**Arguments**:
- `campaignId` (required): Campaign ID
- `date_preset` (optional): Date range preset
- `fields` (optional): Insights fields to return

**Example LLM prompt**: "Get cost per result for campaign 123456789"

---

## Meta Marketing API Notes

- **Ad account ID format**: act_123456789
- **Campaign status**: 1=active, 2=paused, etc.
- **Insights**: Performance metrics (impressions, clicks, spend)
- **Object hierarchy**: Campaign > Ad Set > Ad
- **Rate limits**: Meta has strict API rate limits
