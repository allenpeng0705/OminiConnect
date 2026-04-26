# Streak Tools

Provider: `streak` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Streak CRM API. They allow AI agents to manage boxes (deals), pipelines, users, and files within Streak's Gmail-integrated CRM. Streak is built around the concept of "boxes" which represent deals or companies being tracked.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Streak
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `read`, `write`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `streak_list_boxes` | List boxes (deals) | GET | /boxes |
| `streak_get_box` | Get box details | GET | /boxes/{box_id} |
| `streak_create_box` | Create a box | POST | /boxes |
| `streak_update_box` | Update a box | PUT | /boxes/{box_id} |
| `streak_list_pipelines` | List pipelines | GET | /pipelines |
| `streak_get_pipeline` | Get pipeline details | GET | /pipelines/{pipeline_id} |
| `streak_list_users` | List team users | GET | /users |
| `streak_get_user` | Get user details | GET | /users/{user_id} |
| `streak_list_files` | List files | GET | /files |
| `streak_get_file` | Get file details | GET | /files/{file_id} |

---

## Tool Details

### streak_list_boxes

**What it does**: Lists all boxes (deals) with optional filters for pipeline, stage, assignee, or search term.

**When to use**: Browse active deals, find deals in a specific pipeline, search by deal name.

**Arguments**:
- `pipeline_id` (optional): Filter by pipeline ID
- `stage_id` (optional): Filter by stage ID
- `assigned_to` (optional): Filter by user ID
- `search` (optional): Search in box name or notes
- `page_size` (optional): Number of results (default 20, max 100)

**Example LLM prompt**: "List all boxes in the Sales pipeline"

---

### streak_get_box

**What it does**: Gets detailed information about a specific box including contacts, notes, and stage.

**When to use**: Read full box details, see deal history and associated contacts.

**Arguments**:
- `box_id` (required): Box ID

**Example LLM prompt**: "Get details for box 12345"

---

### streak_create_box

**What it does**: Creates a new box (deal) in a pipeline.

**When to use**: Add new deals to track, create boxes for new opportunities.

**Arguments**:
- `name` (required): Box/deal name
- `pipeline_id` (required): Pipeline ID
- `stage_id` (optional): Stage ID within pipeline
- `assigned_to` (optional): Array of user IDs
- `contact_ids` (optional): Array of contact IDs
- `notes` (optional): Initial notes

**Example LLM prompt**: "Create a new box called 'Acme Corp Deal' in the Sales pipeline"

---

### streak_update_box

**What it does**: Updates an existing box — can change name, stage, assignee, notes, contacts.

**When to use**: Move deals through pipeline stages, reassign deals, update notes.

**Arguments**:
- `box_id` (required): Box ID
- `name` (optional): New name
- `stage_id` (optional): New stage ID
- `assigned_to` (optional): New array of user IDs
- `contact_ids` (optional): New array of contact IDs
- `notes` (optional): New notes

**Example LLM prompt**: "Move box 12345 to the 'Negotiation' stage"

---

### streak_list_pipelines

**What it does**: Lists all pipelines in the Streak organization.

**When to use**: See available pipelines, understand deal stages.

**Arguments**:
- `page_size` (optional): Number of results (default 20, max 100)

**Example LLM prompt**: "List all available pipelines"

---

### streak_get_pipeline

**What it does**: Gets detailed information about a pipeline including all its stages.

**When to use**: Understand pipeline structure, see stage progression.

**Arguments**:
- `pipeline_id` (required): Pipeline ID

**Example LLM prompt**: "Get details for the Sales pipeline"

---

### streak_list_users

**What it does**: Lists all team members in the Streak organization.

**When to use**: Find team members to assign boxes, see team structure.

**Arguments**:
- `page_size` (optional): Number of results (default 20, max 100)

**Example LLM prompt**: "List all team members"

---

### streak_get_user

**What it does**: Gets detailed information about a specific user.

**When to use**: Get user profile, email, and role information.

**Arguments**:
- `user_id` (required): User ID

**Example LLM prompt**: "Get details for user 12345"

---

### streak_list_files

**What it does**: Lists files attached to boxes, optionally filtered by box.

**When to use**: Find attachments, see files associated with deals.

**Arguments**:
- `box_id` (optional): Filter by box ID
- `page_size` (optional): Number of results (default 20, max 100)

**Example LLM prompt**: "List all files attached to box 12345"

---

### streak_get_file

**What it does**: Gets detailed information about a specific file including metadata.

**When to use**: Get file details, size, and download information.

**Arguments**:
- `file_id` (required): File ID

**Example LLM prompt**: "Get file details for 67890"

---

## Streak API Notes

- **Boxes are deals**: In Streak, a "box" represents a deal or company being tracked
- **Pipelines**: Each pipeline has multiple stages (e.g., Lead, Qualified, Proposal, Negotiation, Won, Lost)
- **Gmail integration**: Streak works natively with Gmail, tracking email threads on boxes
- **Pagination**: Default page_size is 20, max is 100
- **File attachments**: Files can be attached to boxes for document storage
