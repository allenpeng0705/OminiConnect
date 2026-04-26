# Klaviyo OAuth Tools

Provider: `klaviyo-oauth` | Engine: `nango` | Auth: OAuth2 via Nango

## Overview

These tools wrap the Klaviyo API. They allow AI agents to manage profiles, lists, campaigns, and metrics. **Requires Klaviyo OAuth2 authentication.**

## Authentication

**OAuth2 via Nango**:
- User authenticates via Nango Connect with Klaviyo
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://a.klaviyo.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `klaviyo_list_profiles` | List profiles | GET | /api/profiles/ |
| `klaviyo_get_profile` | Get profile details | GET | /api/profiles/{profile_id} |
| `klaviyo_create_profile` | Create a profile | POST | /api/profiles/ |
| `klaviyo_update_profile` | Update a profile | PUT | /api/profiles/{profile_id} |
| `klaviyo_list_lists` | List lists | GET | /api/lists/ |
| `klaviyo_get_list` | Get list details | GET | /api/lists/{list_id} |
| `klaviyo_list_campaigns` | List campaigns | GET | /api/campaigns/ |
| `klaviyo_get_campaign` | Get campaign details | GET | /api/campaigns/{campaign_id} |
| `klaviyo_list_metrics` | List metrics | GET | /api/metrics/ |
| `klaviyo_get_metrics_event` | Get metrics event | GET | /api/metrics/{metric_id} |

---

## Tool Details

### klaviyo_list_profiles

**What it does**: Lists all profiles in Klaviyo.

**When to use**: Find profiles, view customer list.

**Arguments**:
- `page` (optional): Page number (default: 1)
- `page_size` (optional): Results per page (default: 20)

**Example LLM prompt**: "List all profiles in Klaviyo"

---

### klaviyo_get_profile

**What it does**: Gets details for a specific profile.

**When to use**: Get profile information, view customer details.

**Arguments**:
- `profile_id` (required): Profile ID

**Example LLM prompt**: "Get details for profile abc123"

---

### klaviyo_create_profile

**What it does**: Creates a new profile.

**When to use**: Add customers to Klaviyo.

**Arguments**:
- `email` (required): Email address
- `first_name` (optional): First name
- `last_name` (optional): Last name
- `properties` (optional): Additional properties

**Example LLM prompt**: "Create a profile for john@example.com"

---

### klaviyo_update_profile

**What it does**: Updates an existing profile.

**When to use**: Modify profile data.

**Arguments**:
- `profile_id` (required): Profile ID
- `properties` (required): Properties to update

**Example LLM prompt**: "Update profile abc123 with new properties"

---

### klaviyo_list_lists

**What it does**: Lists all lists.

**When to use**: View lists, find subscriber groups.

**Arguments**:
- `page` (optional): Page number (default: 1)

**Example LLM prompt**: "List all lists in Klaviyo"

---

### klaviyo_get_list

**What it does**: Gets details for a specific list.

**When to use**: Get list information.

**Arguments**:
- `list_id` (required): List ID

**Example LLM prompt**: "Get details for list xyz789"

---

### klaviyo_list_campaigns

**What it does**: Lists all campaigns.

**When to use**: View campaigns, track emails.

**Arguments**:
- `page` (optional): Page number (default: 1)

**Example LLM prompt**: "List all campaigns in Klaviyo"

---

### klaviyo_get_campaign

**What it does**: Gets details for a specific campaign.

**When to use**: Get campaign information.

**Arguments**:
- `campaign_id` (required): Campaign ID

**Example LLM prompt**: "Get details for campaign c1"

---

### klaviyo_list_metrics

**What it does**: Lists all metrics.

**When to use**: View available metrics, find KPIs.

**Arguments**: None

**Example LLM prompt**: "List all metrics in Klaviyo"

---

### klaviyo_get_metrics_event

**What it does**: Gets a metrics event.

**When to use**: Get metric information.

**Arguments**:
- `metric_id` (required): Metric ID

**Example LLM prompt**: "Get details for metric m1"

---

## Klaviyo API Notes

- **Email Marketing**: E-commerce email marketing platform
- **Profiles**: Customer profiles with properties
- **Lists**: Subscriber lists for campaigns
- **Campaigns**: Email campaigns to send
- **Metrics**: Tracking and analytics
