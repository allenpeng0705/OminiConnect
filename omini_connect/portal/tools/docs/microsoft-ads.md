# Microsoft Ads Tools

Provider: `microsoft-ads` | Engine: `nango` | Auth: OAuth2 via Nango (alias: microsoft)

## Overview

These tools wrap the Microsoft Ads API. They allow AI agents to manage advertising campaigns, ad groups, ads, and keywords. **Requires Microsoft Ads OAuth2.**

## Authentication

**Nango OAUTH2 (Microsoft Ads)**:
- User authenticates via Nango Connect with Microsoft Ads
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://clientcenter.api.bingads.microsoft.com/Api`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `microsoft_ads_list_campaigns` | List campaigns | GET | /campaignManagement/v13/campaigns |
| `microsoft_ads_get_campaign` | Get campaign details | GET | /campaignManagement/v13/campaigns/{campaignId} |
| `microsoft_ads_list_ad_groups` | List ad groups | GET | /campaignManagement/v13/adgroups |
| `microsoft_ads_get_ad_group` | Get ad group details | GET | /campaignManagement/v13/adgroups/{adGroupId} |
| `microsoft_ads_list_ads` | List ads | GET | /campaignManagement/v13/ads |
| `microsoft_ads_get_ad` | Get ad details | GET | /campaignManagement/v13/ads/{adId} |
| `microsoft_ads_list_keywords` | List keywords | GET | /campaignManagement/v13/keywords |
| `microsoft_ads_get_keyword` | Get keyword details | GET | /campaignManagement/v13/keywords/{keywordId} |
| `microsoft_ads_get_performance` | Get campaign performance | GET | /reporting/v13/reports |
| `microsoft_ads_list_reports` | Generate reports | POST | /reporting/v13/reports |

---

## Tool Details

### microsoft_ads_list_campaigns

**What it does**: Lists all campaigns in Microsoft Ads.

**When to use**: Browse campaigns, check campaign status.

**Arguments**:
- `accountId` (required): Account ID
- `status` (optional): Filter by status

**Example LLM prompt**: "List all campaigns for account 12345"

---

### microsoft_ads_get_campaign

**What it does**: Gets details of a specific campaign.

**When to use**: Check campaign settings, review campaign.

**Arguments**:
- `campaignId` (required): Campaign ID
- `accountId` (required): Account ID

**Example LLM prompt**: "Get details for campaign 12345"

---

### microsoft_ads_list_ad_groups

**What it does**: Lists all ad groups for a campaign.

**When to use**: Manage targeting groups, adjust bids.

**Arguments**:
- `campaignId` (required): Campaign ID
- `accountId` (required): Account ID
- `status` (optional): Filter by status

**Example LLM prompt**: "List all ad groups for campaign 12345"

---

### microsoft_ads_get_ad_group

**What it does**: Gets details of a specific ad group.

**When to use**: Check ad group settings, review targeting.

**Arguments**:
- `adGroupId` (required): Ad Group ID
- `campaignId` (required): Campaign ID

**Example LLM prompt**: "Get details for ad group 12345"

---

### microsoft_ads_list_ads

**What it does**: Lists all ads for an ad group.

**When to use**: Manage individual ads, check ad status.

**Arguments**:
- `adGroupId` (required): Ad Group ID
- `status` (optional): Filter by status

**Example LLM prompt**: "List all ads in ad group 12345"

---

### microsoft_ads_get_ad

**What it does**: Gets details of a specific ad.

**When to use**: Check ad creative, review ad status.

**Arguments**:
- `adId` (required): Ad ID
- `adGroupId` (required): Ad Group ID

**Example LLM prompt**: "Get details for ad 12345"

---

### microsoft_ads_list_keywords

**What it does**: Lists all keywords for an ad group.

**When to use**: Manage keywords, check keyword status.

**Arguments**:
- `adGroupId` (required): Ad Group ID
- `status` (optional): Filter by status

**Example LLM prompt**: "List all keywords for ad group 12345"

---

### microsoft_ads_get_keyword

**What it does**: Gets details of a specific keyword.

**When to use**: Check keyword performance, adjust bids.

**Arguments**:
- `keywordId` (required): Keyword ID

**Example LLM prompt**: "Get details for keyword 12345"

---

### microsoft_ads_get_performance

**What it does**: Gets performance metrics for a campaign.

**When to use**: Analyze campaign ROI, track metrics.

**Arguments**:
- `campaignId` (required): Campaign ID
- `startDate` (optional): Start date (YYYY-MM-DD)
- `endDate` (optional): End date (YYYY-MM-DD)

**Example LLM prompt**: "Get impressions and clicks for campaign 12345"

---

### microsoft_ads_list_reports

**What it does**: Generates or lists available reports.

**When to use**: Generate performance reports.

**Arguments**:
- `reportType` (required): Report type (CampaignPerformance, AdGroupPerformance, etc.)
- `startDate` (optional): Start date
- `endDate` (optional): End date

**Example LLM prompt**: "Generate a CampaignPerformance report for January"

---

## Microsoft Ads Notes

- **Bing Ads**: Microsoft's advertising platform
- **Campaign hierarchy**: Campaign > Ad Group > Ad > Keyword
- **Account ID**: Required for most operations
- **Reporting**: Various report types available
- **Rate limits**: Microsoft Ads has API rate limits
