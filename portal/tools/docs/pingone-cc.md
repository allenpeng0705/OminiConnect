# PingOne Client Credentials Tools

Provider: `pingone-cc` | Engine: `nango` | Auth: OAuth2 Client Credentials via Nango

## Overview

These tools wrap the PingOne API. They allow AI agents to manage users, groups, applications, populations, and audit events. PingOne is an identity and access management platform. **Requires PingOne OAuth2 Client Credentials authentication.**

## Authentication

**Nango OAuth2 CC**:
- Uses client credentials for token flow
- Token stored in Nango, accessed via `connection_ref`
- Base URL: https://api.pingone.{tld}
- Requires tld in connection config

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `pingone_cc_list_users` | List users | GET | /v1/users |
| `pingone_cc_get_user` | Get user details | GET | /v1/users/{userId} |
| `pingone_cc_list_groups` | List groups | GET | /v1/groups |
| `pingone_cc_get_group` | Get group details | GET | /v1/groups/{groupId} |
| `pingone_cc_list_applications` | List applications | GET | /v1/applications |
| `pingone_cc_get_application` | Get application details | GET | /v1/applications/{applicationId} |
| `pingone_cc_list_populations` | List populations | GET | /v1/populations |
| `pingone_cc_get_population` | Get population details | GET | /v1/populations/{populationId} |
| `pingone_cc_list_events` | List audit events | GET | /v1/events |
| `pingone_cc_get_environment` | Get environment info | GET | /v1/environments/{environmentId} |

---

## Tool Details

### pingone_cc_list_users

**What it does**: Lists all users in the environment.

**When to use**: Browse user directory, find users.

**Arguments**:
- `populationId` (optional): Filter by population
- `status` (optional): Filter by status (active, inactive)

**Example LLM prompt**: "List all active users"

---

### pingone_cc_get_user

**What it does**: Gets detailed information about a specific user.

**When to use**: Get user profile, attributes.

**Arguments**:
- `userId` (required): User ID

**Example LLM prompt**: "Get details for user 12345"

---

### pingone_cc_list_groups

**What it does**: Lists all groups in the environment.

**When to use**: Browse group directory.

**Arguments**: None

**Example LLM prompt**: "List all groups"

---

### pingone_cc_get_group

**What it does**: Gets detailed information about a specific group.

**When to use**: Get group details, membership.

**Arguments**:
- `groupId` (required): Group ID

**Example LLM prompt**: "Get details for group 67890"

---

### pingone_cc_list_applications

**What it does**: Lists all applications in the environment.

**When to use**: Browse SSO applications.

**Arguments**:
- `status` (optional): Filter by status

**Example LLM prompt**: "List all active applications"

---

### pingone_cc_get_application

**What it does**: Gets detailed information about a specific application.

**When to use**: Get application configuration.

**Arguments**:
- `applicationId` (required): Application ID

**Example LLM prompt**: "Get details for application APP-11111"

---

### pingone_cc_list_populations

**What it does**: Lists all populations in the environment.

**When to use**: Browse identity populations.

**Arguments**: None

**Example LLM prompt**: "What populations are configured?"

---

### pingone_cc_get_population

**What it does**: Gets detailed information about a specific population.

**When to use**: Get population details, settings.

**Arguments**:
- `populationId` (required): Population ID

**Example LLM prompt**: "Get details for population POP-22222"

---

### pingone_cc_list_events

**What it does**: Lists audit events in the environment.

**When to use**: Review audit logs, security events.

**Arguments**:
- `startDate` (optional): Start date (YYYY-MM-DD)
- `endDate` (optional): End date (YYYY-MM-DD)
- `userId` (optional): Filter by user

**Example LLM prompt**: "Show audit events for the past week"

---

### pingone_cc_get_environment

**What it does**: Gets environment information.

**When to use**: Get environment settings, configuration.

**Arguments**:
- `environmentId` (required): Environment ID

**Example LLM prompt**: "Get our environment information"

---

## PingOne CC API Notes

- **OAuth2 Client Credentials**: Machine-to-machine authentication
- **Multi-tenant**: Supports different TLDs
- **Rate limits**: Apply to API calls
- **Date formats**: Use YYYY-MM-DD format
