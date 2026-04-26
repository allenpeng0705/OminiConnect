# Instantly Tools

Provider: `instantly` | Engine: `nango` | Auth: API Key via Nango

## Overview

These tools wrap the Instantly API. They allow AI agents to manage campaigns, leads, sequences, and view analytics. Instantly is a cold email outreach platform.

## Authentication

**Nango API Key**:
- User provides API key via Nango Connect
- Key passed in Authorization header as Bearer token
- Token stored in Nango, accessed via `connection_ref`
- Base URL: https://api.instantly.ai/api

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `instantly_list_campaigns` | List campaigns | GET | /campaigns |
| `instantly_get_campaign` | Get campaign details | GET | /campaigns/{id} |
| `instantly_list_leads` | List leads | GET | /leads |
| `instantly_get_lead` | Get lead details | GET | /leads/{id} |
| `instantly_list_sequences` | List sequences | GET | /sequences |
| `instantly_get_sequence` | Get sequence details | GET | /sequences/{id} |
| `instantly_list_workspaces` | List workspaces | GET | /workspaces |
| `instantly_get_workspace` | Get workspace details | GET | /workspaces/{id} |
| `instantly_list_stats` | List statistics | GET | /stats |
| `instantly_list_analytics` | List analytics | GET | /analytics |

---

## Tool Details

### instantly_list_campaigns

**What it does**: Lists all campaigns in Instantly.

**When to use**: Browse email outreach campaigns.

**Arguments**: None

**Example LLM prompt**: "List all campaigns"

---

### instantly_get_campaign

**What it does**: Gets detailed information about a specific campaign.

**When to use**: View campaign settings and performance.

**Arguments**:
- `id` (required): Campaign ID

**Example LLM prompt**: "Get campaign with ID abc123"

---

### instantly_list_leads

**What it does**: Lists all leads in Instantly.

**When to use**: Browse lead database.

**Arguments**:
- `campaign_id` (optional): Filter by campaign ID

**Example LLM prompt**: "List all leads for campaign abc123"

---

### instantly_get_lead

**What it does**: Gets detailed information about a specific lead.

**When to use**: View lead profile and activity.

**Arguments**:
- `id` (required): Lead ID

**Example LLM prompt**: "Get lead with ID xyz789"

---

### instantly_list_sequences

**What it does**: Lists all sequences in Instantly.

**When to use**: Browse email sequences.

**Arguments**: None

**Example LLM prompt**: "List all sequences"

---

### instantly_get_sequence

**What it does**: Gets detailed information about a specific sequence.

**When to use**: View sequence steps and timing.

**Arguments**:
- `id` (required): Sequence ID

**Example LLM prompt**: "Get sequence with ID seq456"

---

### instantly_list_workspaces

**What it does**: Lists all workspaces in Instantly.

**When to use**: Browse workspaces.

**Arguments**: None

**Example LLM prompt**: "List all workspaces"

---

### instantly_get_workspace

**What it does**: Gets detailed information about a specific workspace.

**When to use**: View workspace settings and users.

**Arguments**:
- `id` (required): Workspace ID

**Example LLM prompt**: "Get workspace with ID wsp789"

---

### instantly_list_stats

**What it does**: Lists campaign statistics in Instantly.

**When to use**: View email engagement metrics.

**Arguments**:
- `campaign_id` (optional): Filter by campaign ID

**Example LLM prompt**: "List stats for campaign abc123"

---

### instantly_list_analytics

**What it does**: Lists analytics data in Instantly.

**When to use**: View overall engagement analytics.

**Arguments**: None

**Example LLM prompt**: "List analytics"

---

## Instantly API Notes

- **API Base URL**: https://api.instantly.ai/api
- **Auth Mode**: API Key in Bearer token
- **Campaigns**: Email outreach campaigns
- **Leads**: Individual email prospects
- **Sequences**: Multi-step email sequences
- **Workspaces**: Account workspaces
- **Stats**: Campaign performance metrics
- **Analytics**: Engagement analytics
