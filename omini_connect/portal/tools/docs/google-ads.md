# Google Ads Tools

Provider: `google-ads` | Engine: `nango` | Auth: OAUTH2 via Nango (alias: google)

## Overview

These tools wrap the Google Ads API. They allow AI agents to manage campaigns, ad groups, ads, keywords, and reports. **Requires Google OAuth2 with Ads permissions.**

## Authentication

**Nango OAUTH2 (Google Ads)**:
- User authenticates via OAuth2 with Google Ads scope
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://googleads.googleapis.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `google_ads_list_campaigns` | List campaigns | GET | /v17/customers/{customer_id}/campaigns |
| `google_ads_get_campaign` | Get campaign details | GET | /v17/customers/{customer_id}/campaigns/{campaign_id} |
| `google_ads_list_ad_groups` | List ad groups | GET | /v17/customers/{customer_id}/campaigns/{campaign_id}/adGroups |
| `google_ads_list_ads` | List ads | GET | /v17/customers/{customer_id}/adGroups/{ad_group_id}/ads |
| `google_ads_list_keywords` | List keywords | GET | /v17/customers/{customer_id}/adGroups/{ad_group_id}/keywords |
| `google_ads_get_keyword` | Get keyword details | GET | /v17/customers/{customer_id}/keywords/{keyword_id} |
| `google_ads_list_reports` | List performance reports | GET | /v17/customers/{customer_id}/campaignStats |
| `google_ads_list_accounts` | List managed accounts | GET | /v17/customers/{customer_id}/managedCustomers |
| `google_ads_get_account` | Get account details | GET | /v17/customers/{customer_id} |
| `google_ads_list_conversions` | List conversion actions | GET | /v17/customers/{customer_id}/conversionActions |

---

## Tool Details

### google_ads_list_campaigns

**What it does**: Lists all campaigns in Google Ads.

**When to use**: Browse advertising campaigns.

**Arguments**:
- `customer_id` (required): Google Ads customer ID

**Example LLM prompt**: "List all campaigns in my Google Ads account"

---

### google_ads_get_campaign

**What it does**: Gets detailed information about a campaign.

**When to use**: Get campaign settings and status.

**Arguments**:
- `customer_id` (required): Google Ads customer ID
- `campaign_id` (required): Campaign ID

**Example LLM prompt**: "Get details for campaign 123"

---

### google_ads_list_ad_groups

**What it does**: Lists ad groups in a campaign.

**When to use**: See ad groups within a campaign.

**Arguments**:
- `customer_id` (required): Google Ads customer ID
- `campaign_id` (required): Campaign ID

**Example LLM prompt**: "List ad groups in campaign 123"

---

### google_ads_list_ads

**What it does**: Lists ads in an ad group.

**When to use**: See ads within an ad group.

**Arguments**:
- `customer_id` (required): Google Ads customer ID
- `ad_group_id` (required): Ad group ID

**Example LLM prompt**: "List ads in ad group 456"

---

### google_ads_list_keywords

**What it does**: Lists keywords in an ad group.

**When to use**: See keywords targeting ads.

**Arguments**:
- `customer_id` (required): Google Ads customer ID
- `ad_group_id` (required): Ad group ID

**Example LLM prompt**: "List keywords in ad group 456"

---

### google_ads_get_keyword

**What it does**: Gets detailed information about a keyword.

**When to use**: Get keyword stats and bids.

**Arguments**:
- `customer_id` (required): Google Ads customer ID
- `keyword_id` (required): Keyword ID

**Example LLM prompt**: "Get details for keyword 789"

---

### google_ads_list_reports

**What it does**: Gets performance reports for campaigns.

**When to use**: Analyze campaign performance.

**Arguments**:
- `customer_id` (required): Google Ads customer ID
- `date_range` (optional): Date range (e.g., LAST_30_DAYS)

**Example LLM prompt**: "Get performance report for the last 30 days"

---

### google_ads_list_accounts

**What it does**: Lists managed Google Ads accounts.

**When to use**: Browse account hierarchy.

**Arguments**:
- `customer_id` (required): Google Ads customer ID

**Example LLM prompt**: "List all managed accounts"

---

### google_ads_get_account

**What it does**: Gets detailed information about an account.

**When to use**: Get account settings and status.

**Arguments**:
- `customer_id` (required): Google Ads customer ID

**Example LLM prompt**: "Get details for account 123"

---

### google_ads_list_conversions

**What it does**: Lists conversion actions.

**When to use**: See tracking conversions.

**Arguments**:
- `customer_id` (required): Google Ads customer ID

**Example LLM prompt**: "List all conversion actions"

---

## Google Ads API Notes

- **Customer ID**: Numeric ID for the Google Ads account
- **Campaign ID**: Numeric ID for campaigns
- **Ad group ID**: Numeric ID for ad groups
- **Keyword ID**: Numeric ID for keywords
- **Date ranges**: LAST_7_DAYS, LAST_30_DAYS, etc.
