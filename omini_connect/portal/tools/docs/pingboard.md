# Pingboard Tools

Provider: `pingboard` | Engine: `nango` | Auth: OAuth2 Client Credentials via Nango

## Overview

These tools wrap the Pingboard API. They allow AI agents to manage users, accounts, statuses, skills, departments, and badges. Pingboard is an employee directory and org chart platform. **Requires Pingboard OAuth2 Client Credentials authentication.**

## Authentication

**Nango OAuth2 CC**:
- Uses client credentials for token flow
- Token stored in Nango, accessed via `connection_ref`
- Base URL: https://app.pingboard.com/api/v2

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `pingboard_list_users` | List users | GET | /v2/users |
| `pingboard_get_user` | Get user details | GET | /v2/users/{userId} |
| `pingboard_list_accounts` | List accounts | GET | /v2/accounts |
| `pingboard_get_account` | Get account details | GET | /v2/accounts/{accountId} |
| `pingboard_list_statuses` | List statuses | GET | /v2/statuses |
| `pingboard_list_skills` | List skills | GET | /v2/skills |
| `pingboard_list_departments` | List departments | GET | /v2/departments |
| `pingboard_get_directory` | Get directory | GET | /v2/directory |
| `pingboard_list_badges` | List badges | GET | /v2/badges |
| `pingboard_get_user_status` | Get user status | GET | /v2/users/{userId}/status |

---

## Tool Details

### pingboard_list_users

**What it does**: Lists all users in the organization.

**When to use**: Browse employee directory.

**Arguments**:
- `status` (optional): Filter by status (active, inactive)

**Example LLM prompt**: "List all active users"

---

### pingboard_get_user

**What it does**: Gets detailed information about a specific user.

**When to use**: Get user profile, contact info.

**Arguments**:
- `userId` (required): User ID

**Example LLM prompt**: "Get details for user 12345"

---

### pingboard_list_accounts

**What it does**: Lists all accounts in the organization.

**When to use**: Browse account information.

**Arguments**: None

**Example LLM prompt**: "List all accounts"

---

### pingboard_get_account

**What it does**: Gets detailed information about a specific account.

**When to use**: Get account details.

**Arguments**:
- `accountId` (required): Account ID

**Example LLM prompt**: "Get details for account 67890"

---

### pingboard_list_statuses

**What it does**: Lists all user statuses.

**When to use**: Browse available statuses.

**Arguments**: None

**Example LLM prompt**: "What statuses are available?"

---

### pingboard_list_skills

**What it does**: Lists all skills in the organization.

**When to use**: Browse skill directory.

**Arguments**: None

**Example LLM prompt**: "What skills are in our directory?"

---

### pingboard_list_departments

**What it does**: Lists all departments in the organization.

**When to use**: Browse organizational structure.

**Arguments**: None

**Example LLM prompt**: "What departments do we have?"

---

### pingboard_get_directory

**What it does**: Gets the company directory.

**When to use**: Get full org chart, directory.

**Arguments**: None

**Example LLM prompt**: "Get our company directory"

---

### pingboard_list_badges

**What it does**: Lists all badges in the organization.

**When to use**: Browse achievement badges.

**Arguments**: None

**Example LLM prompt**: "What badges are available?"

---

### pingboard_get_user_status

**What it does**: Gets the current status of a user.

**When to use**: Check if user is available, busy, etc.

**Arguments**:
- `userId` (required): User ID

**Example LLM prompt**: "What's the status of user 12345?"

---

## Pingboard API Notes

- **OAuth2 Client Credentials**: Machine-to-machine authentication
- **Org Chart**: Useful for organizational structure
- **User Status**: Real-time availability status
