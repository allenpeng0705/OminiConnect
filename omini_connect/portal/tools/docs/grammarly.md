# Grammarly Tools

Provider: `grammarly` | Engine: `nango` | Auth: OAuth2 Client Credentials via Nango

## Overview

These tools wrap the Grammarly API. They allow AI agents to manage users, teams, organizations, roles, and view analytics. Grammarly is an AI-powered writing assistant platform.

## Authentication

**Nango OAuth2 Client Credentials**:
- Uses client credentials flow for server-to-server authentication
- Token stored in Nango, accessed via `connection_ref`
- Scope separator: comma

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `grammarly_get_user` | Get user info | GET | /v1/user |
| `grammarly_list_teams` | List teams | GET | /v1/ecosystem/teams |
| `grammarly_get_team` | Get team details | GET | /v1/ecosystem/teams/{teamId} |
| `grammarly_list_members` | List team members | GET | /v1/ecosystem/members |
| `grammarly_get_member` | Get member details | GET | /v1/ecosystem/members/{memberId} |
| `grammarly_list_orgs` | List organizations | GET | /v1/ecosystem/orgs |
| `grammarly_get_org` | Get org details | GET | /v1/ecosystem/orgs/{orgId} |
| `grammarly_list_roles` | List roles | GET | /v1/ecosystem/roles |
| `grammarly_get_role` | Get role details | GET | /v1/ecosystem/roles/{roleId} |
| `grammarly_list_analytics` | List analytics | GET | /v2/analytics/users |

---

## Tool Details

### grammarly_get_user

**What it does**: Gets information about the authenticated user.

**When to use**: Check current user context, verify authentication.

**Arguments**: None

**Example LLM prompt**: "Get current Grammarly user info"

---

### grammarly_list_teams

**What it does**: Lists all teams in the organization.

**When to use**: Browse available teams, find team by name.

**Arguments**: None

**Example LLM prompt**: "List all Grammarly teams"

---

### grammarly_get_team

**What it does**: Gets detailed information about a specific team.

**When to use**: View team settings and statistics.

**Arguments**:
- `teamId` (required): Team ID

**Example LLM prompt**: "Get team with ID abc123"

---

### grammarly_list_members

**What it does**: Lists all members in the organization or team.

**When to use**: View team composition, check member roles.

**Arguments**:
- `team_id` (optional): Filter by team ID

**Example LLM prompt**: "List all members of team abc123"

---

### grammarly_get_member

**What it does**: Gets detailed information about a specific member.

**When to use**: View member profile and stats.

**Arguments**:
- `memberId` (required): Member ID

**Example LLM prompt**: "Get member with ID xyz789"

---

### grammarly_list_orgs

**What it does**: Lists all organizations.

**When to use**: View organization structure.

**Arguments**: None

**Example LLM prompt**: "List all organizations"

---

### grammarly_get_org

**What it does**: Gets detailed information about a specific organization.

**When to use**: Check org settings and limits.

**Arguments**:
- `orgId` (required): Org ID

**Example LLM prompt**: "Get org with ID org123"

---

### grammarly_list_roles

**What it does**: Lists all available roles.

**When to use**: View role definitions and permissions.

**Arguments**: None

**Example LLM prompt**: "List all available roles"

---

### grammarly_get_role

**What it does**: Gets detailed information about a specific role.

**When to use**: View role permissions.

**Arguments**:
- `roleId` (required): Role ID

**Example LLM prompt**: "Get role with ID role456"

---

### grammarly_list_analytics

**What it does**: Lists analytics data for the organization.

**When to use**: View usage statistics, check writing metrics.

**Arguments**:
- `start_date` (optional): Start date (YYYY-MM-DD)
- `end_date` (optional): End date (YYYY-MM-DD)

**Example LLM prompt**: "Show analytics for the past month"

---

## Grammarly API Notes

- **API Base URL**: https://api.grammarly.com/ecosystem/api
- **Auth Mode**: OAuth2 Client Credentials (server-to-server)
- **Scope Separator**: comma
- **Organizations**: Top-level entity containing teams and members
- **Teams**: Groups of members within an organization
- **Roles**: Define permissions for members
