# Dropbox More Tools

Provider: `dropbox_more` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap additional Dropbox API endpoints for Paper docs, shared folders, and team management. They complement the base Dropbox tools with collaboration and administration features. Dropbox is a leading cloud storage platform for file sync and sharing.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Dropbox
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `files.content.read`, `files.content.write`, `sharing.read`, `members.read`, `groups.read`, `teams.read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `dropbox_more_list_paper_docs` | List Paper documents | POST | /paper/docs/list |
| `dropbox_more_get_paper_doc` | Get Paper doc metadata | POST | /paper/docs/get |
| `dropbox_more_create_paper_doc` | Create a Paper document | POST | /paper/docs/create |
| `dropbox_more_list_shared_folders` | List shared folders | POST | /sharing/list_folders |
| `dropbox_more_get_shared_folder` | Get shared folder metadata | POST | /sharing/get_folder_metadata |
| `dropbox_more_list_team_members` | List team members | POST | /team/members/list |
| `dropbox_more_get_team_member` | Get team member details | POST | /team/members/get |
| `dropbox_more_list_team_groups` | List team groups | POST | /team/groups/list |
| `dropbox_more_get_team_group` | Get team group details | POST | /team/groups/get |
| `dropbox_more_get_team_info` | Get team info | POST | /team/get_info |

---

## Tool Details

### dropbox_more_list_paper_docs

**What it does**: Lists Paper documents (now called Dropbox Paper) in the user's account.

**When to use**: Find and browse Paper documents, see recently created or modified docs.

**Arguments**:
- `limit` (optional): Maximum number of docs to return (default 100)
- `cursor` (optional): Cursor for pagination
- `filter` (optional): Filter by doc modified date (`none`, `client_modified`, `server_modified`)

**Example LLM prompt**: "List my Paper documents to find the meeting notes from last week"

---

### dropbox_more_get_paper_doc

**What it does**: Retrieves metadata and content of a specific Paper document.

**When to use**: Get details of a specific Paper doc including its creation date and sharing status.

**Arguments**:
- `doc_id` (required): The Paper doc ID

**Example LLM prompt**: "Get metadata for Paper doc abc-123"

---

### dropbox_more_create_paper_doc

**What it does**: Creates a new Paper document with optional initial content.

**When to use**: Create new meeting notes, project docs, or collaborative documents.

**Arguments**:
- `title` (required): Title of the new document
- `content` (optional): Initial markdown content for the document

**Example LLM prompt**: "Create a new Paper doc titled 'Q2 Planning' with the agenda template content"

---

### dropbox_more_list_shared_folders

**What it does**: Lists all shared folders the user has access to.

**When to use**: Find folders shared with you or that you manage.

**Arguments**:
- `limit` (optional): Maximum number of folders to return (default 100)
- `cursor` (optional): Cursor for pagination
- `direct_only` (optional): Only list directly shared folders (default false)

**Example LLM prompt**: "List all shared folders to find the 'Project Alpha' folder"

---

### dropbox_more_get_shared_folder

**What it does**: Gets detailed information about a specific shared folder including members and settings.

**When to use**: See who has access to a shared folder, check folder settings and member roles.

**Arguments**:
- `shared_folder_id` (required): The shared folder ID

**Example LLM prompt**: "Get details for shared folder xyz-789 to see all members"

---

### dropbox_more_list_team_members

**What it does**: Lists members in the Dropbox team (requires Dropbox Business).

**When to use**: View team directory, find team members for sharing or collaboration.

**Arguments**:
- `limit` (optional): Maximum number of members to return (default 100)
- `cursor` (optional): Cursor for pagination

**Example LLM prompt**: "List all team members to find contact info for Alice"

---

### dropbox_more_get_team_member

**What it does**: Gets detailed information about a specific team member.

**When to use**: Get a team member's email, name, and account status.

**Arguments**:
- `member_id` (required): The team member ID

**Example LLM prompt**: "Get details for team member member-456"

---

### dropbox_more_list_team_groups

**What it does**: Lists groups in the Dropbox team (requires Dropbox Business).

**When to use**: Find team groups for bulk sharing or permission management.

**Arguments**:
- `limit` (optional): Maximum number of groups to return (default 100)
- `cursor` (optional): Cursor for pagination

**Example LLM prompt**: "List all team groups to find the 'Engineering' group"

---

### dropbox_more_get_team_group

**What it does**: Gets detailed information about a specific team group including members.

**When to use**: See all members of a group, manage group-based sharing.

**Arguments**:
- `group_id` (required): The team group ID

**Example LLM prompt**: "Get details for group group-123 to see all its members"

---

### dropbox_more_get_team_info

**What it does**: Retrieves general information about the Dropbox team including name, admin count, and settings.

**When to use**: Get team overview and configuration for team-based workflows.

**Arguments**: None

**Example LLM prompt**: "Get info about our Dropbox team to see team name and member count"