# Zeplin Tools

Provider: `zeplin` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Zeplin API. They allow AI agents to browse projects, screens, users, comments, styleguides, and specs. Zeplin is a design handoff platform for designers and developers.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Zeplin
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `projects:read`, `screens:read`, `comments:read`, `users:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `zeplin_list_projects` | List all projects | GET | /api/v1/projects |
| `zeplin_get_project` | Get project details | GET | /api/v1/projects/{project_id} |
| `zeplin_list_screens` | List screens in a project | GET | /api/v1/projects/{project_id}/screens |
| `zeplin_get_screen` | Get screen details | GET | /api/v1/screens/{screen_id} |
| `zeplin_list_users` | List project users | GET | /api/v1/projects/{project_id}/members |
| `zeplin_get_user` | Get user details | GET | /api/v1/users/{user_id} |
| `zeplin_list_comments` | List comments on a screen | GET | /api/v1/screens/{screen_id}/comments |
| `zeplin_get_comment` | Get comment details | GET | /api/v1/comments/{comment_id} |
| `zeplin_get_styleguide` | Get styleguide for a project | GET | /api/v1/projects/{project_id}/styleguide |
| `zeplin_get_spec` | Get spec for a screen | GET | /api/v1/screens/{screen_id}/spec |

---

## Tool Details

### zeplin_list_projects

**What it does**: Lists all Zeplin projects the user has access to.

**When to use**: Browse available projects, find specific design systems, or explore team workspaces.

**Arguments**:
- `limit` (optional): Number of results (default 20)
- `offset` (optional): Pagination offset

**Example LLM prompt**: "List all my Zeplin projects"

---

### zeplin_get_project

**What it does**: Gets details about a specific Zeplin project including stories and members.

**When to use**: Understand project structure, see connected stories, or review team composition.

**Arguments**:
- `project_id` (required): Project ID

**Example LLM prompt**: "Get details for project abc123"

---

### zeplin_list_screens

**What it does**: Lists all screens in a Zeplin project or story with thumbnails and metadata.

**When to use**: Browse screens in a project, find specific designs, or extract asset information.

**Arguments**:
- `project_id` (required): Project ID
- `story_id` (optional): Story ID to filter by
- `limit` (optional): Number of results (default 50)
- `offset` (optional): Pagination offset

**Example LLM prompt**: "List all screens in project abc123"

---

### zeplin_get_screen

**What it does**: Gets details about a specific screen including its layers, assets, and metadata.

**When to use**: Get detailed screen information, extract layer data, or review asset specifications.

**Arguments**:
- `screen_id` (required): Screen ID

**Example LLM prompt**: "Get details for screen xyz"

---

### zeplin_list_users

**What it does**: Lists all users in a Zeplin project with their roles.

**When to use**: See who has access to a project, find collaborators, or check team composition.

**Arguments**:
- `project_id` (required): Project ID
- `limit` (optional): Number of results (default 50)

**Example LLM prompt**: "List all users in project abc123"

---

### zeplin_get_user

**What it does**: Gets details about a specific Zeplin user including their profile and activity.

**When to use**: Get user information, see their projects, or find contact details.

**Arguments**:
- `user_id` (required): User ID

**Example LLM prompt**: "Get details for user xyz"

---

### zeplin_list_comments

**What it does**: Lists all comments on a Zeplin screen.

**When to use**: Read design feedback, review discussions, or understand stakeholder comments.

**Arguments**:
- `screen_id` (required): Screen ID
- `limit` (optional): Number of results (default 50)

**Example LLM prompt**: "What comments are on screen abc123?"

---

### zeplin_get_comment

**What it does**: Gets details about a specific comment on a screen including author and timestamp.

**When to use**: Read full comment details, see annotation location, or check reply threads.

**Arguments**:
- `comment_id` (required): Comment ID

**Example LLM prompt**: "Get details for comment abc123"

---

### zeplin_get_styleguide

**What it does**: Gets the styleguide for a Zeplin project including colors, typography, spacing, and components.

**When to use**: Extract design tokens, review style specifications, or understand the design system.

**Arguments**:
- `project_id` (required): Project ID

**Example LLM prompt**: "Get the styleguide for project abc123"

---

### zeplin_get_spec

**What it does**: Gets the specification for a screen including dimensions, spacing, and asset references.

**When to use**: Extract precise measurements, get CSS values, or review technical specifications.

**Arguments**:
- `screen_id` (required): Screen ID

**Example LLM prompt**: "Get the spec for screen abc123"

---

## Zeplin API Notes

- **Projects**: Contain screens organized into stories for different platforms (iOS, Android, Web).
- **Screens**: Design assets with layers, assets, and metadata for developer handoff.
- **Styleguide**: Centralized design tokens including colors, typography, spacing, and components.
- **Spec**: Technical specifications for screens including dimensions, margins, and asset details.
- **Comments**: Feedback attached to specific locations on screens.
- **Users**: Project members with roles (owner, admin, member, viewer).
- **Pagination**: Use offset-based pagination for large result sets.
