# Klaviyo Tools

Provider: `klaviyo` | Engine: `nango` | Auth: API Key via Nango

## Overview

These tools wrap the Klaviyo API. They allow AI agents to manage profiles, lists, segments, and marketing campaigns. Klaviyo is a leading email marketing and SMS platform designed for e-commerce brands.

## Authentication

**Nango API Key**:
- User provides Klaviyo Private API Key
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `profiles`, `lists`, `segments`, `campaigns`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `klaviyo_list_profiles` | List all profiles | GET | /api/profiles/ |
| `klaviyo_get_profile` | Get profile by ID | GET | /api/profiles/{profile_id}/ |
| `klaviyo_create_profile` | Create a new profile | POST | /api/profiles/ |
| `klaviyo_list_lists` | List all lists | GET | /api/lists/ |
| `klaviyo_get_list` | Get list by ID | GET | /api/lists/{list_id}/ |
| `klaviyo_add_profile_to_list` | Add a profile to a list | POST | /api/lists/{list_id}/relationships/profiles/ |
| `klaviyo_list_segments` | List all segments | GET | /api/segments/ |
| `klaviyo_get_segment` | Get segment by ID | GET | /api/segments/{segment_id}/ |
| `klaviyo_list_campaigns` | List all campaigns | GET | /api/campaigns/ |
| `klaviyo_get_campaign` | Get campaign by ID | GET | /api/campaigns/{campaign_id}/ |

---

## Tool Details

### klaviyo_list_profiles

**What it does**: Retrieves all profiles in your Klaviyo account. Profiles contain customer data including email, name, and custom properties.

**When to use**: Browse your customer database, search for contacts, or prepare bulk operations.

**Arguments**:
- `page` (optional): Cursor for pagination
- `page_size` (optional): Number of results per page (default 50, max 100)
- `attributes` (optional): Filter profiles by attributes

**Example LLM prompt**: "List the first 50 profiles in our Klaviyo account"

---

### klaviyo_get_profile

**What it does**: Gets a specific profile by its ID including all profile attributes and subscription status.

**When to use**: Look up individual customer details, check subscription preferences, or view customer data.

**Arguments**:
- `profile_id` (required): Profile ID (UUID format)

**Example LLM prompt**: "Get profile details for profile abc123-def456-..."

---

### klaviyo_create_profile

**What it does**: Creates a new profile in Klaviyo with email, phone, name, and custom properties.

**When to use**: Add new customers, capture signups from your app, or create profiles from transactions.

**Arguments**:
- `email` (optional): Profile email address
- `phone_number` (optional): Profile phone number
- `first_name` (optional): First name
- `last_name` (optional): Last name
- `organization` (optional): Company or organization name
- `properties` (optional): Custom profile properties

**Example LLM prompt**: "Create a profile for john@example.com with first name John, last name Doe, and phone +1234567890"

---

### klaviyo_list_lists

**What it does**: Gets all lists in your Klaviyo account. Lists are static groups of profiles for targeted campaigns.

**When to use**: See available lists, check audience size, or prepare campaigns.

**Arguments**:
- `page` (optional): Cursor for pagination
- `page_size` (optional): Number of results per page (default 50, max 100)

**Example LLM prompt**: "List all our Klaviyo lists"

---

### klaviyo_get_list

**What it does**: Gets details about a specific list including profile count and settings.

**When to use**: Check list details, verify audience before adding profiles or sending campaigns.

**Arguments**:
- `list_id` (required): List ID (UUID format)

**Example LLM prompt**: "Get details for list abc123-def456-..."

---

### klaviyo_add_profile_to_list

**What it does**: Adds an existing profile to a list. The profile must already exist in your Klaviyo account.

**When to use**: Subscribe customers to lists, organize contacts by interest or behavior.

**Arguments**:
- `list_id` (required): List ID to add profile to
- `profiles` (required): Array of profile objects with id property

**Example LLM prompt**: "Add profile abc123 to list xyz789"

---

### klaviyo_list_segments

**What it does**: Gets all segments in your Klaviyo account. Segments are dynamic groups based on profile criteria.

**When to use**: Browse segments, understand audience criteria, find target groups for campaigns.

**Arguments**:
- `page` (optional): Cursor for pagination
- `page_size` (optional): Number of results per page (default 50, max 100)

**Example LLM prompt**: "List all Klaviyo segments"

---

### klaviyo_get_segment

**What it does**: Gets details about a specific segment including member count and the conditions that define it.

**When to use**: Understand segment criteria, check segment size, or verify targeting before campaigns.

**Arguments**:
- `segment_id` (required): Segment ID (UUID format)

**Example LLM prompt**: "Get details for segment abc123-def456-..."

---

### klaviyo_list_campaigns

**What it does**: Gets all campaigns in your Klaviyo account including sent, scheduled, and draft campaigns.

**When to use**: Review campaign history, check campaign statuses, or prepare new campaigns.

**Arguments**:
- `page` (optional): Cursor for pagination
- `page_size` (optional): Number of results per page (default 50, max 100)
- `status` (optional): Filter by status: draft, scheduled, sent, cancelled

**Example LLM prompt**: "List all sent campaigns from the last 30 days"

---

### klaviyo_get_campaign

**What it does**: Gets details about a specific campaign including subject, content, send status, and tracking metrics.

**When to use**: Check campaign performance, review send details, or investigate campaign issues.

**Arguments**:
- `campaign_id` (required): Campaign ID (UUID format)

**Example LLM prompt**: "Get details for campaign abc123-def456-..."

---

## Klaviyo API Notes

- **Profiles**: Core entity representing a customer with email, phone, name, and custom properties
- **Lists vs Segments**: Lists are static collections; segments dynamically update based on profile criteria
- **Subscriptions**: Profiles can have different subscription statuses per channel (email, SMS)
- **ID Format**: Profile IDs are UUIDs typically starting with person_ or similar prefixes
- **Properties**: Custom key-value pairs for storing additional customer data
- **E-commerce Integration**: Klaviyo integrates with Shopify, WooCommerce, and other platforms for automatic data sync
- **Metrics**: Klaviyo tracks opens, clicks, bounces, and revenue attribution per campaign
