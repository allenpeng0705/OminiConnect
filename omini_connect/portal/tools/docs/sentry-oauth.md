# Sentry (Public Integrations) Tools

Provider: `sentry-oauth` | Engine: `nango` | Auth: OAUTH2

## Overview

These tools wrap the Sentry (Public Integrations) API. They allow AI agents to interact with Sentry (Public Integrations) functionality. **Requires OAUTH2 authentication.**

## Authentication

**OAuth2 Authentication**:
- User authenticates via OAuth2 authorization code flow
- Nango manages the OAuth handshake and token refresh
- Default scopes depend on the provider configuration

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `list_projects` | List all Sentry projects | GET | /projects |
| `list_issues` | List issues across projects | GET | /issues |
| `get_issue` | Get details for a specific issue | GET | /issues/{id} |
| `list_events` | List events for an issue | GET | /events |
| `list_teams` | List teams in organization | GET | /teams |
| `list_members` | List organization members | GET | /members |
| `list_releases` | List releases | GET | /releases |
| `get_org_stats` | Get organization statistics | GET | /stats |
| `list_projects_by_team` | List projects for a team | GET | /teams/{team_id}/projects |
| `get_event_counts` | Get event count statistics | GET | /event-counts |

---

## Tool Details

### list_projects

**What it does**: List all Sentry projects

**When to use**: Use this tool when you need to list all sentry projects.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list projects to..."

---

### list_issues

**What it does**: List issues across projects

**When to use**: Use this tool when you need to list issues across projects.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list issues to..."

---

### get_issue

**What it does**: Get details for a specific issue

**When to use**: Use this tool when you need to get details for a specific issue.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get issue to..."

---

### list_events

**What it does**: List events for an issue

**When to use**: Use this tool when you need to list events for an issue.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list events to..."

---

### list_teams

**What it does**: List teams in organization

**When to use**: Use this tool when you need to list teams in organization.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list teams to..."

---

### list_members

**What it does**: List organization members

**When to use**: Use this tool when you need to list organization members.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list members to..."

---

### list_releases

**What it does**: List releases

**When to use**: Use this tool when you need to list releases.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list releases to..."

---

### get_org_stats

**What it does**: Get organization statistics

**When to use**: Use this tool when you need to get organization statistics.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get org stats to..."

---

### list_projects_by_team

**What it does**: List projects for a team

**When to use**: Use this tool when you need to list projects for a team.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list projects by team to..."

---

### get_event_counts

**What it does**: Get event count statistics

**When to use**: Use this tool when you need to get event count statistics.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get event counts to..."

---

## Sentry (Public Integrations) API Notes

- **Auth mode**: OAUTH2
- **Base URL**: https://{hostname}/api
- **API prefix**: /
- **Rate limits**: Check provider documentation for specific limits
