# Gong OAuth Tools

Provider: `gong-oauth` | Engine: `nango` | Auth: OAUTH2 via Nango

## Overview

These tools wrap the Gong API. They allow AI agents to manage calls, transcripts, scorecards, users, and teams. **Requires Gong OAuth2 authentication.**

## Authentication

**Nango OAUTH2**:
- User authenticates via OAuth2 with Gong
- Token stored in Nango, accessed via `connection_ref`
- Base URL: Customer-specific API base URL from token response

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `gong_oauth_list_calls` | List calls | GET | /v2/calls |
| `gong_oauth_get_call` | Get call details | GET | /v2/calls/{call_id} |
| `gong_oauth_list_transcripts` | List call transcripts | GET | /v2/calls/{call_id}/transcript |
| `gong_oauth_list_scorecards` | List scorecards | GET | /v2/scorecards |
| `gong_oauth_get_scorecard` | Get scorecard details | GET | /v2/scorecards/{scorecard_id} |
| `gong_oauth_list_users` | List users | GET | /v2/users |
| `gong_oauth_get_user` | Get user details | GET | /v2/users/{user_id} |
| `gong_oauth_list_teams` | List teams | GET | /v2/teams |
| `gong_oauth_get_team` | Get team details | GET | /v2/teams/{team_id} |
| `gong_oauth_list_workspace_stats` | List workspace stats | GET | /v2/workspace/stats |

---

## Tool Details

### gong_oauth_list_calls

**What it does**: Lists calls recorded in Gong.

**When to use**: Browse recorded calls, filter by date.

**Arguments**:
- `from_date` (optional): Start date (YYYY-MM-DD)
- `to_date` (optional): End date (YYYY-MM-DD)
- `page_size` (optional): Number of results (default 20)

**Example LLM prompt**: "List all calls from last week"

---

### gong_oauth_get_call

**What it does**: Gets detailed information about a call.

**When to use**: Get call metadata, participants, duration.

**Arguments**:
- `call_id` (required): Call ID

**Example LLM prompt**: "Get details for call abc123"

---

### gong_oauth_list_transcripts

**What it does**: Lists transcripts for a call.

**When to use**: Read the full transcript of a call.

**Arguments**:
- `call_id` (required): Call ID

**Example LLM prompt**: "Get the transcript for call abc123"

---

### gong_oauth_list_scorecards

**What it does**: Lists scorecards for calls.

**When to use**: View call evaluations.

**Arguments**:
- `call_id` (optional): Filter by call ID

**Example LLM prompt**: "List scorecards for call abc123"

---

### gong_oauth_get_scorecard

**What it does**: Gets detailed information about a scorecard.

**When to use**: Get scorecard ratings and feedback.

**Arguments**:
- `scorecard_id` (required): Scorecard ID

**Example LLM prompt**: "Get details for scorecard abc123"

---

### gong_oauth_list_users

**What it does**: Lists users in Gong.

**When to use**: View team members who use Gong.

**Arguments**: None

**Example LLM prompt**: "List all Gong users"

---

### gong_oauth_get_user

**What it does**: Gets detailed information about a user.

**When to use**: Get user details and settings.

**Arguments**:
- `user_id` (required): User ID

**Example LLM prompt**: "Get details for user abc123"

---

### gong_oauth_list_teams

**What it does**: Lists teams in Gong.

**When to use**: View Gong team structure.

**Arguments**: None

**Example LLM prompt**: "List all teams"

---

### gong_oauth_get_team

**What it does**: Gets detailed information about a team.

**When to use**: Get team members and settings.

**Arguments**:
- `team_id` (required): Team ID

**Example LLM prompt**: "Get details for team abc123"

---

### gong_oauth_list_workspace_stats

**What it does**: Gets workspace statistics and metrics.

**When to use**: Get aggregate metrics for calls.

**Arguments**:
- `from_date` (optional): Start date (YYYY-MM-DD)
- `to_date` (optional): End date (YYYY-MM-DD)

**Example LLM prompt**: "Get stats for this month"

---

## Gong API Notes

- **Call recording**: Revenue intelligence platform
- **Transcripts**: Full call transcripts with speaker separation
- **Scorecards**: Call evaluation forms
- **Teams**: Group users for reporting
- **Customer-specific URL**: API base URL varies per customer
