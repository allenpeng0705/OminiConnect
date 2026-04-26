# Miro SCIM Tools

Provider: `miro-scim` | Engine: `nango` | Auth: API_KEY via Nango (SCIM token)

## Overview

These tools wrap the Miro SCIM API. They allow AI agents to manage users and teams via SCIM provisioning. **Requires Miro SCIM token.**

## Authentication

**Nango SCIM API_KEY**:
- User provides Miro SCIM token via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://miro.com/api`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `miro_scim_list_users` | List all users (SCIM) | GET | /v1/scim/Users |
| `miro_scim_get_user` | Get user details (SCIM) | GET | /v1/scim/Users/{id} |
| `miro_scim_create_user` | Create a new user (SCIM) | POST | /v1/scim/Users |
| `miro_scim_update_user` | Update a user (SCIM) | PUT | /v1/scim/Users/{id} |
| `miro_scim_delete_user` | Delete a user (SCIM) | DELETE | /v1/scim/Users/{id} |
| `miro_scim_list_teams` | List all teams (SCIM) | GET | /v1/scim/Teams |
| `miro_scim_get_team` | Get team details (SCIM) | GET | /v1/scim/Teams/{id} |
| `miro_scim_create_team` | Create a team (SCIM) | POST | /v1/scim/Teams |
| `miro_scim_update_team` | Update a team (SCIM) | PUT | /v1/scim/Teams/{id} |
| `miro_scim_delete_team` | Delete a team (SCIM) | DELETE | /v1/scim/Teams/{id} |

---

## Tool Details

### miro_scim_list_users

**What it does**: Lists all user accounts in Miro via SCIM API.

**When to use**: Provision users, audit user list, find specific users.

**Arguments**:
- `filter` (optional): Filter expression (e.g., userName eq 'john@example.com')
- `startIndex` (optional): Starting index for pagination (default 1)
- `count` (optional): Number of results to return (default 100)

**Example LLM prompt**: "List all users in Miro SCIM"

---

### miro_scim_get_user

**What it does**: Gets detailed information about a specific user.

**When to use**: Get user details, verify user exists.

**Arguments**:
- `id` (required): User ID (SCIM id)

**Example LLM prompt**: "Get details for user with ID 12345"

---

### miro_scim_create_user

**What it does**: Creates a new user account in Miro via SCIM API.

**When to use**: Provision new team members, sync users from IdP.

**Arguments**:
- `userName` (required): Username (email)
- `name` (optional): User name object with givenName, familyName
- `emails` (optional): Email addresses
- `active` (optional): Whether user is active (default true)

**Example LLM prompt**: "Create a new user john@example.com in Miro"

---

### miro_scim_update_user

**What it does**: Updates an existing user account via SCIM API.

**When to use**: Update user details, activate/deactivate users.

**Arguments**:
- `id` (required): User ID (SCIM id)
- `userName` (optional): Username (email)
- `name` (optional): User name object
- `active` (optional): Whether user is active

**Example LLM prompt**: "Update user 12345 to be active"

---

### miro_scim_delete_user

**What it does**: Deletes a user account via SCIM API.

**When to use**: Deprovision users, remove former employees.

**Arguments**:
- `id` (required): User ID (SCIM id)

**Example LLM prompt**: "Delete user 12345 from Miro"

---

### miro_scim_list_teams

**What it does**: Lists all teams in Miro via SCIM API.

**When to use**: List teams, find teams for user assignment.

**Arguments**:
- `filter` (optional): Filter expression
- `startIndex` (optional): Starting index for pagination
- `count` (optional): Number of results to return

**Example LLM prompt**: "List all teams in Miro SCIM"

---

### miro_scim_get_team

**What it does**: Gets detailed information about a specific team.

**When to use**: Get team members, verify team exists.

**Arguments**:
- `id` (required): Team ID (SCIM id)

**Example LLM prompt**: "Get details for team engineering"

---

### miro_scim_create_team

**What it does**: Creates a new team in Miro via SCIM API.

**When to use**: Create teams, set up departments.

**Arguments**:
- `displayName` (required): Team display name
- `members` (optional): List of member user IDs

**Example LLM prompt**: "Create a team called engineering"

---

### miro_scim_update_team

**What it does**: Updates an existing team via SCIM API.

**When to use**: Rename teams, update team membership.

**Arguments**:
- `id` (required): Team ID (SCIM id)
- `displayName` (optional): Team display name
- `members` (optional): List of member user IDs

**Example LLM prompt**: "Add user 12345 to team engineering"

---

### miro_scim_delete_team

**What it does**: Deletes a team via SCIM API.

**When to use**: Remove teams, clean up unused groups.

**Arguments**:
- `id` (required): Team ID (SCIM id)

**Example LLM prompt**: "Delete the old-contractors team"

---

## Miro SCIM Notes

- **SCIM version**: Uses SCIM 2.0 protocol
- **User IDs**: Stable SCIM IDs
- **Teams**: Miro workspace teams
- **Team membership**: Managed via members array
- **Rate limits**: Implement backoff for bulk operations
