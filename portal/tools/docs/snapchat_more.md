# Snapchat More Tools

Provider: `snapchat_more` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Snapchat Ads API. They allow AI agents to manage ad accounts, create and monitor advertising campaigns, handle ad creatives, retrieve performance reports, and manage custom audiences for Snapchat advertising.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Snapchat
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `adaccounts:read`, `campaigns:read`, `campaigns:write`, `ads:read`, `ads:write`, `reports:read`, `audiences:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `snapchat_more_list_ad_accounts` | List Snapchat ad accounts | GET | /adaccounts |
| `snapchat_more_get_ad_account` | Get ad account details | GET | /adaccounts/{adaccount_id} |
| `snapchat_more_list_campaigns` | List advertising campaigns | GET | /campaigns |
| `snapchat_more_get_campaign` | Get campaign details | GET | /campaigns/{campaign_id} |
| `snapchat_more_create_campaign` | Create a new campaign | POST | /campaigns |
| `snapchat_more_list_ads` | List ads in a campaign | GET | /ads |
| `snapchat_more_get_ad` | Get ad details | GET | /ads/{ad_id} |
| `snapchat_more_create_ad` | Create a new ad | POST | /ads |
| `snapchat_more_get_campaign_report` | Get campaign performance report | GET | /campaigns/{campaign_id}/stats |
| `snapchat_more_list_audiences` | List custom audiences | GET | /audiences |

---

## Tool Details

### snapchat_more_list_ad_accounts

**What it does**: Lists all Snapchat ad accounts associated with the authenticated user. Returns account details including status, spend limits, and billing information.

**When to use**: View all ad accounts you manage, check account status and billing settings.

**Arguments**:
- `page` (optional): Page number (default: 1)
- `count` (optional): Results per page, max 100 (default: 20)

**Example LLM prompt**: "List all my Snapchat ad accounts"

---

### snapchat_more_get_ad_account

**What it does**: Gets detailed information about a specific Snapchat ad account including status, billing settings, and spending limits.

**When to use**: Review account details before creating campaigns, check spending status.

**Arguments**:
- `adaccount_id` (required): Ad account ID

**Example LLM prompt**: "Get details for ad account 12345"

---

### snapchat_more_list_campaigns

**What it does**: Lists all advertising campaigns in a specific Snapchat ad account. Returns campaign details including status, budget, and delivery metrics.

**When to use**: View all campaigns in an account, filter by status, or find a specific campaign.

**Arguments**:
- `adaccount_id` (required): Ad account ID
- `status` (optional): `ACTIVE`, `PAUSED`, or `ARCHIVED`
- `page` (optional): Page number (default: 1)
- `count` (optional): Results per page (default: 20)

**Example LLM prompt**: "List all active campaigns in ad account 12345"

---

### snapchat_more_get_campaign

**What it does**: Gets detailed information about a specific advertising campaign including budget, scheduling, targeting, and performance data.

**When to use**: Review campaign settings before modifying, check delivery status, analyze performance.

**Arguments**:
- `campaign_id` (required): Campaign ID

**Example LLM prompt**: "Get details for campaign 67890"

---

### snapchat_more_create_campaign

**What it does**: Creates a new advertising campaign in Snapchat Ads. Set campaign objectives, budget, and targeting parameters.

**When to use**: Launch a new advertising initiative, set up campaign structure for a product launch.

**Arguments**:
- `adaccount_id` (required): Ad account ID
- `campaign_name` (required): Campaign name
- `objective` (required): `APP_INSTALL`, `WEB_VIEW`, `VIDEO_VIEW`, `REACH`, `BRAND_AWARENESS`, `AWARENESS`, `CONSIDERATION`, or `CONVERSION`
- `daily_budget` (optional): Daily budget in cents
- `lifetime_budget` (optional): Lifetime budget in cents
- `start_time` (optional): Campaign start time (ISO 8601 format)
- `end_time` (optional): Campaign end time (ISO 8601 format)
- `status` (optional): `ACTIVE` or `PAUSED` (default: PAUSED)

**Example LLM prompt**: "Create a campaign named 'Summer Sale' with APP_INSTALL objective and $100 daily budget"

---

### snapchat_more_list_ads

**What it does**: Lists all ads within a specific campaign or ad set. Use this to view ad performance and creative details.

**When to use**: Monitor ad performance, check which ads are running, review creative variations.

**Arguments**:
- `adaccount_id` (required): Ad account ID
- `campaign_id` (optional): Filter by campaign ID
- `adset_id` (optional): Filter by ad set ID
- `status` (optional): Filter by ad status
- `page` (optional): Page number (default: 1)
- `count` (optional): Results per page (default: 20)

**Example LLM prompt**: "List all active ads in campaign 67890"

---

### snapchat_more_get_ad

**What it does**: Gets detailed information about a specific ad including creative assets, targeting, and delivery metrics.

**When to use**: Review ad creative, check delivery status, analyze ad performance.

**Arguments**:
- `ad_id` (required): Ad ID

**Example LLM prompt**: "Get details for ad 33333"

---

### snapchat_more_create_ad

**What it does**: Creates a new ad in a Snapchat ad set. Define creative content, ad format, and tracking pixels.

**When to use**: Add new creatives to an existing ad set, launch new ad variations for testing.

**Arguments**:
- `adaccount_id` (required): Ad account ID
- `adset_id` (required): Ad set ID to attach the ad to
- `name` (required): Ad name
- `type` (optional): `SNAP_AD`, `FILTER`, `LENS`, `STORY`, or `COLLECTION`
- `headline` (optional): Ad headline (max 50 characters)
- `body` (optional): Ad body text (max 500 characters)
- `image_url` (optional): URL of the ad image
- `video_url` (optional): URL of the ad video
- `call_to_action` (optional): `SHOP_NOW`, `LEARN_MORE`, `WATCH_MORE`, `INSTALL_NOW`, `PLAY_NOW`, `BOOK_NOW`, or `GET_OFFER`
- `webview_url` (optional): Destination URL for webview ads
- `app_install_url` (optional): Deep link for app install ads
- `status` (optional): `ACTIVE` or `PAUSED` (default: PAUSED)

**Example LLM prompt**: "Create a new Snap Ad named 'Product Launch' in ad set 22222 with image from https://example.com/product.jpg"

---

### snapchat_more_get_campaign_report

**What it does**: Gets performance report for a Snapchat campaign including impressions, clicks, spend, and conversions.

**When to use**: Analyze campaign effectiveness, measure ROI, optimize ad spend allocation.

**Arguments**:
- `campaign_id` (required): Campaign ID
- `start_time` (required): Start time (YYYY-MM-DD format)
- `end_time` (required): End time (YYYY-MM-DD format)
- `granularity` (optional): `HOUR`, `DAY`, `WEEK`, or `MONTH` (default: DAY)

**Example LLM prompt**: "Get a daily performance report for campaign 67890 from 2024-01-01 to 2024-01-31"

---

### snapchat_more_list_audiences

**What it does**: Lists custom audiences created for targeted Snapchat advertising. Audiences can be built from customer lists, engagement, or lookalike profiles.

**When to use**: View available audiences for targeting, check audience size and type before selecting.

**Arguments**:
- `adaccount_id` (required): Ad account ID
- `audience_type` (optional): `CUSTOM`, `LOOKALIKE`, `ENGAGEMENT`, `VIDEO_VIEW`, `WEB_VISITOR`, or `CONVERSION`
- `page` (optional): Page number (default: 1)
- `count` (optional): Results per page (default: 20)

**Example LLM prompt**: "List all custom audiences for ad account 12345"

---

## Snapchat Ads API Notes

- **Ad Format**: Snapchat supports various ad types including Snap Ads, Filters, Lenses, and Collections
- **Creative**: High-quality vertical images and videos perform best on mobile
- **Objectives**: Different objectives optimize for different outcomes (app installs, web views, etc.)
- **Audiences**: Custom audiences enable precise targeting based on user data and engagement
- **Budgets**: Snapchat uses cents as currency units; minimum budgets apply
