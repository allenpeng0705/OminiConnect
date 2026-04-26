# HeyReach Tools

Provider: `heyreach` | Engine: `nango` | Auth: API Key via Nango

## Overview

These tools wrap the HeyReach API. They allow AI agents to manage outbound campaigns, sequences, prospects, and accounts. HeyReach is a sales engagement platform for LinkedIn and email outreach.

## Authentication

**Nango API Key**:
- User provides API key via Nango Connect
- Key passed in X-Api-Key header
- Token stored in Nango, accessed via `connection_ref`
- Base URL: https://api.heyreach.io/api

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `heyreach_list_campaigns` | List campaigns | GET | /public/campaigns |
| `heyreach_get_campaign` | Get campaign details | GET | /public/campaigns/{id} |
| `heyreach_list_sequences` | List sequences | GET | /public/sequences |
| `heyreach_get_sequence` | Get sequence details | GET | /public/sequences/{id} |
| `heyreach_list_prospects` | List prospects | GET | /public/prospects |
| `heyreach_get_prospect` | Get prospect details | GET | /public/prospects/{id} |
| `heyreach_list_accounts` | List accounts | GET | /public/accounts |
| `heyreach_get_account` | Get account details | GET | /public/accounts/{id} |
| `heyreach_list_tasks` | List tasks | GET | /public/tasks |
| `heyreach_list_stats` | List statistics | GET | /public/stats |

---

## Tool Details

### heyreach_list_campaigns

**What it does**: Lists all campaigns in HeyReach.

**When to use**: Browse outreach campaigns.

**Arguments**: None

**Example LLM prompt**: "List all campaigns"

---

### heyreach_get_campaign

**What it does**: Gets detailed information about a specific campaign.

**When to use**: View campaign settings and performance.

**Arguments**:
- `id` (required): Campaign ID

**Example LLM prompt**: "Get campaign with ID abc123"

---

### heyreach_list_sequences

**What it does**: Lists all sequences in HeyReach.

**When to use**: Browse sequence templates.

**Arguments**: None

**Example LLM prompt**: "List all sequences"

---

### heyreach_get_sequence

**What it does**: Gets detailed information about a specific sequence.

**When to use**: View sequence steps and timing.

**Arguments**:
- `id` (required): Sequence ID

**Example LLM prompt**: "Get sequence with ID def456"

---

### heyreach_list_prospects

**What it does**: Lists all prospects in HeyReach.

**When to use**: Browse leads in outreach.

**Arguments**:
- `campaign_id` (optional): Filter by campaign ID

**Example LLM prompt**: "List all prospects for campaign abc123"

---

### heyreach_get_prospect

**What it does**: Gets detailed information about a specific prospect.

**When to use**: View prospect activities and status.

**Arguments**:
- `id` (required): Prospect ID

**Example LLM prompt**: "Get prospect with ID xyz789"

---

### heyreach_list_accounts

**What it does**: Lists all accounts in HeyReach.

**When to use**: Browse target accounts.

**Arguments**: None

**Example LLM prompt**: "List all accounts"

---

### heyreach_get_account

**What it does**: Gets detailed information about a specific account.

**When to use**: View account and associated prospects.

**Arguments**:
- `id` (required): Account ID

**Example LLM prompt**: "Get account with ID acc123"

---

### heyreach_list_tasks

**What it does**: Lists all tasks in HeyReach.

**When to use**: View pending outreach tasks.

**Arguments**: None

**Example LLM prompt**: "List all tasks"

---

### heyreach_list_stats

**What it does**: Lists campaign statistics in HeyReach.

**When to use**: View campaign performance metrics.

**Arguments**:
- `campaign_id` (optional): Filter by campaign ID

**Example LLM prompt**: "List stats for campaign abc123"

---

## HeyReach API Notes

- **API Base URL**: https://api.heyreach.io/api
- **Auth Mode**: API Key in header
- **Campaigns**: Outreach campaigns with sequences
- **Sequences**: Multi-step outreach workflows
- **Prospects**: Individual leads in campaigns
- **Accounts**: Company accounts for account-based outreach
- **Tasks**: Pending actions for outreach
- **Stats**: Performance metrics for campaigns
