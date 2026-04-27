# Figma Additional Tools

Provider: `figma_more` | Engine: `nango` | Auth: OAuth (via Nango)

## Overview

These tools wrap the Figma REST API. They allow AI agents to interact with files, comments, components, and styles in Figma's collaborative design platform.

## Authentication

**Nango (recommended for OAuth)**:
- User authenticates via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `files:read`, `files:write`, `comments:read`, `comments:write`, `library:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `figma_more_list_files` | List files in a project | GET | /v1/projects/{projectId}/files |
| `figma_more_get_file` | Get a specific file | GET | /v1/files/{fileKey} |
| `figma_more_get_file_nodes` | Get specific nodes from a file | GET | /v1/files/{fileKey}/nodes |
| `figma_more_list_comments` | List comments on a file | GET | /v1/files/{fileKey}/comments |
| `figma_more_get_comment` | Get a specific comment | GET | /v1/files/{fileKey}/comments/{commentId} |
| `figma_more_post_comment` | Post a comment on a file | POST | /v1/files/{fileKey}/comments |
| `figma_more_list_components` | List components in a team library | GET | /v1/components/{teamId} |
| `figma_more_get_component` | Get a specific component | GET | /v1/components/{key} |
| `figma_more_list_styles` | List styles in a team library | GET | /v1/styles/{teamId} |
| `figma_more_get_style` | Get a specific style | GET | /v1/styles/{key} |

---

## Tool Details

### figma_more_list_files

**What it does**: Returns a list of all files in a specific project. Files contain designs, prototypes, and specifications.

**When to use**: Browse a project to find specific design files.

**Arguments**:
- `projectId` (required): Project ID
- `cursor` (optional): Pagination cursor for next page
- `limit` (optional): Number of results per page (max 50, default 30)

**Example LLM prompt**: "List all files in the Mobile App project"

---

### figma_more_get_file

**What it does**: Returns a specific Figma file including its document structure, canvas, and metadata. Returns the full file content including all pages and elements.

**When to use**: Load a complete design file for analysis or export.

**Arguments**:
- `fileKey` (required): File key (found in Figma URL)
- `version` (optional): Specific version to retrieve
- `ids` (optional): Comma-separated list of node IDs to include
- `depth` (optional): Maximum depth to traverse (1-4)

**Example LLM prompt**: "Get the Figma file with key ABC123xyz"

---

### figma_more_get_file_nodes

**What it does**: Returns specific nodes from a Figma file by their IDs. Use this to retrieve selected components, frames, or elements without loading the entire file.

**When to use**: Access specific design elements for targeted analysis or documentation.

**Arguments**:
- `fileKey` (required): File key
- `ids` (required): Comma-separated list of node IDs to retrieve
- `version` (optional): Specific version to retrieve

**Example LLM prompt**: "Get nodes 1:234 and 1:456 from file ABC123xyz"

---

### figma_more_list_comments

**What it does**: Lists all comments on a specific Figma file. Comments include author, message, and position information.

**When to use**: Review feedback and discussion on a design.

**Arguments**:
- `fileKey` (required): File key

**Example LLM prompt**: "Show me all comments on the Homepage design"

---

### figma_more_get_comment

**What it does**: Returns details of a specific comment including the message, author, timestamp, and position.

**When to use**: Read full comment details for review or follow-up.

**Arguments**:
- `fileKey` (required): File key
- `commentId` (required): Comment ID

**Example LLM prompt**: "Get details for comment abc-123"

---

### figma_more_post_comment

**What it does**: Posts a new comment on a Figma file. Comments can include text and optional position coordinates to point to specific areas.

**When to use**: Leave feedback, ask questions, or start discussions on designs.

**Arguments**:
- `fileKey` (required): File key
- `message` (required): Comment message
- `client_meta` (optional): Position for the comment (x, y coordinates or node_id)

**Example LLM prompt**: "Post a comment on the Homepage design asking about the color scheme"

---

### figma_more_list_components

**What it does**: Lists all components in a team library. Components are reusable design elements like buttons, cards, and icons.

**When to use**: Explore available components for maintaining design consistency.

**Arguments**:
- `teamId` (required): Team ID
- `page_size` (optional): Number of results per page (max 50, default 30)
- `after` (optional): Cursor for pagination

**Example LLM prompt**: "List all components in the Design System team library"

---

### figma_more_get_component

**What it does**: Returns details of a specific component including its name, description, and containing component set.

**When to use**: Understand a component's structure and usage.

**Arguments**:
- `key` (required): Component key
- `depth` (optional): Maximum depth to traverse (1-4)

**Example LLM prompt**: "Get details for component Button:Primary"

---

### figma_more_list_styles

**What it does**: Lists all styles in a team library. Styles include colors, typography, effects, and grids.

**When to use**: Explore available styles for maintaining design consistency.

**Arguments**:
- `teamId` (required): Team ID
- `page_size` (optional): Number of results per page (max 50, default 30)
- `after` (optional): Cursor for pagination

**Example LLM prompt**: "What color styles are available in the Design System?"

---

### figma_more_get_style

**What it does**: Returns details of a specific style including its name, description, and style properties.

**When to use**: Understand a style's values and how to apply it.

**Arguments**:
- `key` (required): Style key

**Example LLM prompt**: "Get details for style Brand/Primary/Blue"

---

## Figma API Reference

These tools use the Figma REST API. See official docs for full details:
- https://www.figma.com/developers/api
- Rate limits: 90 requests per minute for regular users
- Pagination: Use cursor/after for efficient traversal
- All dates: ISO 8601 format
