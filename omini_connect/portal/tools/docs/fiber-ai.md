# Fiber AI Tools

Provider: `fiber-ai` | Engine: `nango` | Auth: API_KEY via Nango

## Overview

These tools wrap the Fiber AI API. They allow AI agents to manage campaigns, audiences, segments, and analytics. Fiber AI is a marketing analytics and automation platform.

## Authentication

**Nango API_KEY**:
- User provides their Fiber AI API key via Nango Connect
- Key is passed in the apiKey query parameter
- Key stored in Nango, accessed via `connection_ref`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `fiber_ai_get_credits` | Get organization credits | GET | /v1/get-org-credits |
| `fiber_ai_list_campaigns` | List campaigns | GET | /v1/campaigns |
| `fiber_ai_get_campaign` | Get campaign details | GET | /v1/campaigns/{id} |
| `fiber_ai_list_audiences` | List audiences | GET | /v1/audiences |
| `fiber_ai_get_audience` | Get audience details | GET | /v1/audiences/{id} |
| `fiber_ai_list_reports` | List reports | GET | /v1/reports |
| `fiber_ai_get_report` | Get report details | GET | /v1/reports/{id} |
| `fiber_ai_list_segments` | List segments | GET | /v1/segments |
| `fiber_ai_get_analytics` | Get analytics data | GET | /v1/analytics |
| `fiber_ai_search` | Search data | GET | /v1/search |

---

## Tool Details

### fiber_ai_get_credits

**What it does**: Gets organization credit balance.

**When to use**: Check usage, monitor credits.

**Arguments**: None

**Example LLM prompt**: "Get my Fiber AI credits"

---

### fiber_ai_list_campaigns

**What it does**: Lists all marketing campaigns.

**When to use**: Browse campaigns, find campaign IDs.

**Arguments**:
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all campaigns"

---

### fiber_ai_get_campaign

**What it does**: Gets details of a specific campaign.

**When to use**: View campaign configuration, status.

**Arguments**:
- `id` (required): Campaign ID

**Example LLM prompt**: "Get details for campaign abc123"

---

### fiber_ai_list_audiences

**What it does**: Lists all audiences.

**When to use**: Browse audience lists.

**Arguments**: None

**Example LLM prompt**: "List all audiences"

---

### fiber_ai_get_audience

**What it does**: Gets details of a specific audience.

**When to use**: View audience size, composition.

**Arguments**:
- `id` (required): Audience ID

**Example LLM prompt**: "Get details for audience xyz789"

---

### fiber_ai_list_reports

**What it does**: Lists all reports.

**When to use**: Browse available reports.

**Arguments**:
- `campaign_id` (optional): Filter by campaign

**Example LLM prompt**: "List reports for campaign abc123"

---

### fiber_ai_get_report

**What it does**: Gets details of a specific report.

**When to use**: View report data, metrics.

**Arguments**:
- `id` (required): Report ID

**Example LLM prompt**: "Get report def456"

---

### fiber_ai_list_segments

**What it does**: Lists all audience segments.

**When to use**: Browse segments, find targeting groups.

**Arguments**: None

**Example LLM prompt**: "List all segments"

---

### fiber_ai_get_analytics

**What it does**: Gets analytics data for campaigns or audiences.

**When to use**: View performance metrics, trends.

**Arguments**:
- `campaign_id` (optional): Campaign ID
- `start_date` (optional): Start date (YYYY-MM-DD)
- `end_date` (optional): End date (YYYY-MM-DD)

**Example LLM prompt**: "Get analytics for the last 30 days"

---

### fiber_ai_search

**What it does**: Searches across data.

**When to use**: Find specific data points.

**Arguments**:
- `q` (required): Search query

**Example LLM prompt**: "Search for 'conversion'"

---

## Fiber AI API Notes

- **Campaigns**: Marketing campaigns with targeting
- **Audiences**: Groups of users for targeting
- **Segments**: Subdivisions within audiences
- **Reports**: Generated analytics reports
- **Credits**: Usage-based billing system
- **Analytics**: Performance and metrics data
