# FindyMail Tools

Provider: `findymail` | Engine: `nango` | Auth: API_KEY via Nango

## Overview

These tools wrap the FindyMail API. They allow AI agents to manage prospects, campaigns, sequences, and email marketing. FindyMail is a cold email and prospecting platform.

## Authentication

**Nango API_KEY**:
- User provides their FindyMail API key via Nango Connect
- Key is passed in the Authorization header as Bearer token
- Key stored in Nango, accessed via `connection_ref`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `findymail_list_prospects` | List prospects | GET | /api/v1/prospects |
| `findymail_get_prospect` | Get prospect details | GET | /api/v1/prospects/{id} |
| `findymail_create_prospect` | Create a prospect | POST | /api/v1/prospects |
| `findymail_update_prospect` | Update prospect details | PUT | /api/v1/prospects/{id} |
| `findymail_list_campaigns` | List campaigns | GET | /api/v1/campaigns |
| `findymail_get_campaign` | Get campaign details | GET | /api/v1/campaigns/{id} |
| `findymail_enrich_prospect` | Enrich prospect data | POST | /api/v1/prospects/{id}/enrich |
| `findymail_list_sequences` | List sequences | GET | /api/v1/sequences |
| `findymail_get_sequence` | Get sequence details | GET | /api/v1/sequences/{id} |
| `findymail_get_stats` | Get campaign stats | GET | /api/v1/campaigns/{id}/stats |

---

## Tool Details

### findymail_list_prospects

**What it does**: Lists all prospects in the account.

**When to use**: Browse prospects, find specific leads.

**Arguments**:
- `page` (optional): Page number (default 1)
- `limit` (optional): Number of results (default 20)

**Example LLM prompt**: "List all my prospects"

---

### findymail_get_prospect

**What it does**: Gets details of a specific prospect.

**When to use**: View prospect information, contact details.

**Arguments**:
- `id` (required): Prospect ID

**Example LLM prompt**: "Get details for prospect abc123"

---

### findymail_create_prospect

**What it does**: Creates a new prospect.

**When to use**: Add new leads to the system.

**Arguments**:
- `email` (required): Email address
- `first_name` (optional): First name
- `last_name` (optional): Last name
- `company` (optional): Company name

**Example LLM prompt**: "Create a prospect for john@company.com"

---

### findymail_update_prospect

**What it does**: Updates prospect details.

**When to use**: Modify prospect information.

**Arguments**:
- `id` (required): Prospect ID
- `first_name` (optional): First name
- `last_name` (optional): Last name
- `company` (optional): Company name

**Example LLM prompt**: "Update prospect abc123 with new company info"

---

### findymail_list_campaigns

**What it does**: Lists all email campaigns.

**When to use**: Browse campaign history.

**Arguments**: None

**Example LLM prompt**: "List all campaigns"

---

### findymail_get_campaign

**What it does**: Gets details of a specific campaign.

**When to use**: View campaign configuration, status.

**Arguments**:
- `id` (required): Campaign ID

**Example LLM prompt**: "Get details for campaign xyz789"

---

### findymail_enrich_prospect

**What it does**: Enriches prospect with additional data.

**When to use**: Get more info about a lead.

**Arguments**:
- `id` (required): Prospect ID

**Example LLM prompt**: "Enrich prospect abc123"

---

### findymail_list_sequences

**What it does**: Lists all email sequences.

**When to use**: Browse automation sequences.

**Arguments**: None

**Example LLM prompt**: "List all sequences"

---

### findymail_get_sequence

**What it does**: Gets details of a specific sequence.

**When to use**: View sequence steps, settings.

**Arguments**:
- `id` (required): Sequence ID

**Example LLM prompt**: "Get sequence def456"

---

### findymail_get_stats

**What it does**: Gets campaign statistics.

**When to use**: View campaign performance.

**Arguments**:
- `id` (required): Campaign ID

**Example LLM prompt**: "Get stats for campaign xyz789"

---

## FindyMail API Notes

- **Prospects**: Leads and contacts in the system
- **Campaigns**: Email marketing campaigns
- **Sequences**: Automated email sequences
- **Enrichment**: Additional data lookup for prospects
- **Stats**: Campaign open rates, click rates, etc.
