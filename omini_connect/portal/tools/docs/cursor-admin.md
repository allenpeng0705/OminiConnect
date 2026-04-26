# Cursor Admin Tools

Provider: `cursor-admin` | Engine: `nango` | Auth: BASIC via Nango

## Overview

These tools wrap the Cursor Admin API. They allow AI agents to manage workspace users, teams, projects, and settings. Cursor Admin provides administrative functions for team workspaces.

## Authentication

**Nango BASIC**:
- User provides username (API key) and password via Nango
- Token stored in Nango, accessed via `connection_ref`
- Format: API key as username with empty password

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `cursor_admin_list_users` | List workspace users | GET | /teams/members |
| `cursor_admin_get_user` | Get user details | GET | /teams/members/{user_id} |
| `cursor_admin_list_projects` | List workspace projects | GET | /teams/projects |
| `cursor_admin_get_project` | Get project details | GET | /teams/projects/{project_id} |
| `cursor_admin_list_teams` | List teams | GET | /teams |
| `cursor_admin_get_team` | Get team details | GET | /teams/{team_id} |
| `cursor_admin_list_invitations` | List pending invitations | GET | /teams/invitations |
| `cursor_admin_create_invitation` | Create team invitation | POST | /teams/invitations |
| `cursor_admin_list_settings` | List workspace settings | GET | /teams/settings |
| `cursor_admin_get_settings` | Get workspace settings | GET | /teams/settings/{setting_key} |

---

## Tool Details

### cursor_admin_list_users

**What it does**: Lists all users in the Cursor workspace.

**When to use**: View team members, find specific users, manage user access.

**Arguments**:
- `limit` (optional): Max results (default 50, max 200)

**Example LLM prompt**: "List all users in the workspace"

---

### cursor_admin_get_user

**What it does**: Gets detailed user information including role and account status.

**When to use**: Check user details, verify user permissions, review user activity.

**Arguments**:
- `user_id` (required): User ID

**Example LLM prompt**: "Get details for user u-123"

---

### cursor_admin_list_projects

**What it does**: Lists all projects in the workspace.

**When to use**: Browse projects, find project ownership, view project statistics.

**Arguments**:
- `limit` (optional): Max results (default 50, max 200)

**Example LLM prompt**: "List all workspace projects"

---

### cursor_admin_get_project

**What it does**: Gets detailed project information.

**When to use**: Check project settings, verify ownership, review project configuration.

**Arguments**:
- `project_id` (required): Project ID

**Example LLM prompt**: "Get details for project p-456"

---

### cursor_admin_list_teams

**What it does**: Lists all teams in the organization.

**When to use**: View team structure, find team administrators, manage team hierarchy.

**Arguments**:
- `limit` (optional): Max results (default 50, max 200)

**Example LLM prompt**: "List all teams"

---

### cursor_admin_get_team

**What it does**: Gets detailed team information including member list.

**When to use**: Review team composition, check team settings, manage team resources.

**Arguments**:
- `team_id` (required): Team ID

**Example LLM prompt**: "Get details for team t-789"

---

### cursor_admin_list_invitations

**What it does**: Lists all pending team invitations.

**When to use**: Track pending invites, manage invitation status, verify sent invitations.

**Arguments**: None

**Example LLM prompt**: "List all pending invitations"

---

### cursor_admin_create_invitation

**What it does**: Creates and sends a team invitation email.

**When to use**: Invite new team members, add users to specific teams, onboard new users.

**Arguments**:
- `email` (required): Email address to invite
- `team_id` (required): Team ID to invite to
- `role` (optional): Role - member or admin (default: member)

**Example LLM prompt**: "Invite user@example.com to team t-789 as member"

---

### cursor_admin_list_settings

**What it does**: Lists all workspace settings.

**When to use**: View all configuration options, audit workspace settings.

**Arguments**: None

**Example LLM prompt**: "List all workspace settings"

---

### cursor_admin_get_settings

**What it does**: Gets specific workspace setting value.

**When to use**: Check specific configuration, verify setting values, review security policies.

**Arguments**:
- `setting_key` (required): Setting key to retrieve

**Example LLM prompt**: "Get the workspace security settings"

---

## Cursor Admin API Notes

- **API Key as Username**: The Cursor Admin API uses the API key as the username
- **Admin Functions**: Cursor Admin provides workspace-level administration
- **Teams**: Organizational units for grouping users and projects
- **Invitations**: Email-based user onboarding flow
- **Settings**: Workspace-wide configuration options
