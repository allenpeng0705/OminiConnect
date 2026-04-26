# lemlist Tools

Provider: `lemlist` | Engine: `nango` | Auth: BASIC via Nango

## Overview

These tools wrap the lemlist API. They allow AI agents to manage campaigns, leads, and email marketing automation. **Requires lemlist API key.**

## Authentication

**Basic Auth via Nango**:
- User provides lemlist API key (as password)
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://api.lemlist.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `lemlist_get_team` | Get team info | GET | /api/team |
| `lemlist_list_campaigns` | List campaigns | GET | /api/campaigns |
| `lemlist_get_campaign` | Get campaign details | GET | /api/campaigns/{campaign_id} |
| `lemlist_list_leads` | List leads | GET | /api/campaigns/{campaign_id}/leads |
| `lemlist_add_lead` | Add a lead | POST | /api/campaigns/{campaign_id}/leads |
| `lemlist_update_lead` | Update a lead | PUT | /api/campaigns/{campaign_id}/leads/{lead_id} |
| `lemlist_list_unsubscribed` | List unsubscribed leads | GET | /api/unsubscribed |
| `lemlist_list_blacklisted` | List blacklisted leads | GET | /api/blacklisted |
| `lemlist_list_stats` | Get stats | GET | /api/stats |
| `lemlist_get_campaign_stats` | Get campaign stats | GET | /api/campaigns/{campaign_id}/stats |

---

## Tool Details

### lemlist_get_team

**What it does**: Gets the current team information.

**When to use**: Verify authentication, get account info.

**Arguments**: None

**Example LLM prompt**: "Get my lemlist team info"

---

### lemlist_list_campaigns

**What it does**: Lists all campaigns.

**When to use**: View campaigns, find email sequences.

**Arguments**: None

**Example LLM prompt**: "List all campaigns in lemlist"

---

### lemlist_get_campaign

**What it does**: Gets details for a specific campaign.

**When to use**: Get campaign information.

**Arguments**:
- `campaign_id` (required): Campaign ID

**Example LLM prompt**: "Get details for campaign abc123"

---

### lemlist_list_leads

**What it does**: Lists all leads in a campaign.

**When to use**: View leads, find recipients.

**Arguments**:
- `campaign_id` (required): Campaign ID

**Example LLM prompt**: "List all leads in campaign abc123"

---

### lemlist_add_lead

**What it does**: Adds a lead to a campaign.

**When to use**: Add recipients to email campaigns.

**Arguments**:
- `campaign_id` (required): Campaign ID
- `email` (required): Email address
- `first_name` (optional): First name
- `last_name` (optional): Last name

**Example LLM prompt**: "Add john@example.com to campaign abc123"

---

### lemlist_update_lead

**What it does**: Updates an existing lead.

**When to use**: Modify lead data.

**Arguments**:
- `campaign_id` (required): Campaign ID
- `lead_id` (required): Lead ID
- `first_name` (optional): First name
- `last_name` (optional): Last name

**Example LLM prompt**: "Update lead xyz789 in campaign abc123"

---

### lemlist_list_unsubscribed

**What it does**: Lists all unsubscribed leads.

**When to use**: View unsubscribe list.

**Arguments**: None

**Example LLM prompt**: "List all unsubscribed leads in lemlist"

---

### lemlist_list_blacklisted

**What it does**: Lists all blacklisted leads.

**When to use**: View blacklist.

**Arguments**: None

**Example LLM prompt**: "List all blacklisted leads in lemlist"

---

### lemlist_list_stats

**What it does**: Gets overall statistics.

**When to use**: View performance metrics.

**Arguments**: None

**Example LLM prompt**: "Get my lemlist stats"

---

### lemlist_get_campaign_stats

**What it does**: Gets statistics for a specific campaign.

**When to use**: Track campaign performance.

**Arguments**:
- `campaign_id` (required): Campaign ID

**Example LLM prompt**: "Get stats for campaign abc123"

---

## lemlist API Notes

- **Email Marketing**: Cold email marketing automation
- **Campaigns**: Email sequences and outreach
- **Leads**: Recipients in campaigns
- **Unsubscribed**: People who opted out
- **Blacklisted**: Blocked recipients
