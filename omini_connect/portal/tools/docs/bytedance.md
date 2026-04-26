# ByteDance (TikTok Business) Tools

Provider: `bytedance` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the TikTok Business API. They allow AI agents to manage advertising campaigns, create and monitor ads, retrieve performance reports, and manage custom audiences. This is the ad management API for TikTok and ByteDance platforms.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with TikTok Business
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `advertiser:read`, `advertiser:write`, `reports:read`, `audience:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `bytedance_list_ad_campaigns` | List advertising campaigns | GET | /campaign/list |
| `bytedance_get_ad_campaign` | Get campaign details | GET | /campaign/get |
| `bytedance_create_ad_campaign` | Create a new campaign | POST | /campaign/create |
| `bytedance_list_ad_groups` | List ad groups | GET | /adgroup/list |
| `bytedance_get_ad_group` | Get ad group details | GET | /adgroup/get |
| `bytedance_create_ad` | Create a new ad | POST | /ad/create |
| `bytedance_list_ads` | List ads in an ad group | GET | /ad/list |
| `bytedance_get_ad` | Get ad details | GET | /ad/get |
| `bytedance_get_campaign_report` | Get campaign performance report | GET | /reports/get |
| `bytedance_list_audiences` | List custom audiences | GET | /audience/list |

---

## Tool Details

### bytedance_list_ad_campaigns

**What it does**: Lists all advertising campaigns in the user's TikTok Ads account. Returns campaign details including status, budget, and performance metrics.

**When to use**: View all campaigns in an account, check campaign status, or find a specific campaign by name.

**Arguments**:
- `advertiser_id` (required): Advertiser account ID
- `status` (optional): `CAMPAIGN_STATUS_ENABLE` or `CAMPAIGN_STATUS_DISABLE`
- `page` (optional): Page number (default: 1)
- `page_size` (optional): Results per page, max 100 (default: 20)

**Example LLM prompt**: "List all active campaigns for advertiser 12345"

---

### bytedance_get_ad_campaign

**What it does**: Gets detailed information about a specific advertising campaign including budget, schedule, targeting, and current status.

**When to use**: Review campaign settings before modifying or pausing, or check detailed performance.

**Arguments**:
- `advertiser_id` (required): Advertiser account ID
- `campaign_id` (required): Campaign ID to retrieve

**Example LLM prompt**: "Get details for campaign 67890"

---

### bytedance_create_ad_campaign

**What it does**: Creates a new advertising campaign in TikTok Ads. Set budget, scheduling, and campaign objectives.

**When to use**: Launch a new advertising initiative, set up campaign structure for a product launch.

**Arguments**:
- `advertiser_id` (required): Advertiser account ID
- `campaign_name` (required): Campaign name
- `objective` (required): `REACH`, `VIDEO_VIEWS`, `TRAFFIC`, `APP_INSTALL`, `CONVERSION`, or `LEAD_GENERATION`
- `budget_mode` (optional): `BUDGET_MODE_DAY` or `BUDGET_MODE_TOTAL`
- `budget` (optional): Campaign budget (minimum 50)
- `status` (optional): `CAMPAIGN_STATUS_ENABLE` or `CAMPAIGN_STATUS_DISABLE` (default: ENABLE)

**Example LLM prompt**: "Create a new campaign named 'Summer Sale' with REACH objective and $500 daily budget"

---

### bytedance_list_ad_groups

**What it does**: Lists all ad groups within a specific campaign. Ad groups contain ads and define targeting, placement, and scheduling.

**When to use**: View ad group structure within a campaign, check targeting settings.

**Arguments**:
- `advertiser_id` (required): Advertiser account ID
- `campaign_id` (optional): Filter by campaign ID
- `status` (optional): Filter by ad group status
- `page` (optional): Page number (default: 1)
- `page_size` (optional): Results per page (default: 20)

**Example LLM prompt**: "List all ad groups in campaign 67890"

---

### bytedance_get_ad_group

**What it does**: Gets detailed information about a specific ad group including targeting, bidding, placement, and performance data.

**When to use**: Review ad group settings, check targeting criteria, or analyze ad group performance.

**Arguments**:
- `advertiser_id` (required): Advertiser account ID
- `adgroup_id` (required): Ad group ID to retrieve

**Example LLM prompt**: "Get details for ad group 11111"

---

### bytedance_create_ad

**What it does**: Creates a new ad within an ad group. Define creative assets, ad copy, and targeting.

**When to use**: Add new creatives to an existing ad group, launch new ad variations.

**Arguments**:
- `advertiser_id` (required): Advertiser account ID
- `adgroup_id` (required): Ad group ID to add the ad to
- `ad_name` (required): Ad name
- `creative_type` (optional): `VIDEO` or `IMAGE`
- `video_id` (optional): Video ID for video ads
- `image_ids` (optional): Image IDs for image ads
- `ad_text` (optional): Ad copy text
- `call_to_action` (optional): `DOWNLOAD`, `SHOP_NOW`, `LEARN_MORE`, `SIGN_UP`, or `CONTACT_US`

**Example LLM prompt**: "Create a new ad named 'Summer Ad' in ad group 11111 with video 22222"

---

### bytedance_list_ads

**What it does**: Lists all ads within a specific ad group. Use this to view ad performance and status.

**When to use**: Monitor ad performance, check which ads are running, find underperforming ads.

**Arguments**:
- `advertiser_id` (required): Advertiser account ID
- `adgroup_id` (optional): Filter by ad group ID
- `status` (optional): Filter by ad status
- `page` (optional): Page number (default: 1)
- `page_size` (optional): Results per page (default: 20)

**Example LLM prompt**: "List all active ads in ad group 11111"

---

### bytedance_get_ad

**What it does**: Gets detailed information about a specific ad including creative assets, targeting, and real-time performance metrics.

**When to use**: Review ad creative, check delivery status, analyze ad performance.

**Arguments**:
- `advertiser_id` (required): Advertiser account ID
- `ad_id` (required): Ad ID to retrieve

**Example LLM prompt**: "Get details for ad 33333"

---

### bytedance_get_campaign_report

**What it does**: Gets performance report for a specific campaign including impressions, clicks, spend, and conversions.

**When to use**: Analyze campaign effectiveness, measure ROI, generate performance summaries.

**Arguments**:
- `advertiser_id` (required): Advertiser account ID
- `campaign_id` (required): Campaign ID to get report for
- `level` (optional): `CAMPAIGN`, `ADGROUP`, or `AD` (default: CAMPAIGN)
- `start_date` (required): Start date (YYYY-MM-DD format)
- `end_date` (required): End date (YYYY-MM-DD format)
- `page` (optional): Page number (default: 1)
- `page_size` (optional): Results per page (default: 20)

**Example LLM prompt**: "Get a daily performance report for campaign 67890 from 2024-01-01 to 2024-01-31"

---

### bytedance_list_audiences

**What it does**: Lists custom audiences created for targeted advertising. Audiences can be based on user interactions, customer lists, or lookalike profiles.

**When to use**: View available audiences for targeting, check audience size and type.

**Arguments**:
- `advertiser_id` (required): Advertiser account ID
- `audience_type` (optional): `CUSTOM`, `LOOKALIKE`, `ENGAGEMENT`, `VIDEO_VIEW`, or `WEB_VISITOR`
- `page` (optional): Page number (default: 1)
- `page_size` (optional): Results per page (default: 20)

**Example LLM prompt**: "List all custom audiences for advertiser 12345"

---

## ByteDance API Notes

- **Advertising Focus**: This is the TikTok Business API for ad management, not the consumer TikTok API
- **Campaign Structure**: Campaigns contain Ad Groups, which contain Ads
- **Objectives**: Different objectives (REACH, CONVERSION, etc.) optimize for different outcomes
- **Budgets**: Minimum budget requirements apply; daily budgets are recommended for most campaigns
- **Audiences**: Custom audiences enable precise targeting based on user data
