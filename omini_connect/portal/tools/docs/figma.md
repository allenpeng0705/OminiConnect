# Figma Tools

Provider: `figma` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Figma API. They allow AI agents to browse files, interact with comments, access components, and retrieve styles on behalf of the authenticated user. Figma is the leading collaborative design platform.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Figma
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `file_comments:read`, `file_comments:write`, `library:read`, `library:write`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `figma_list_files` | List files in a project | GET | /projects/{project_id}/files |
| `figma_get_file` | Get file details | GET | /files/{file_key} |
| `figma_get_file_nodes` | Get specific nodes from a file | GET | /files/{file_key}/nodes |
| `figma_list_comments` | List comments on a file | GET | /files/{file_key}/comments |
| `figma_get_comment` | Get comment details | GET | /comments/{comment_id} |
| `figma_post_comment` | Post a comment on a file | POST | /files/{file_key}/comments |
| `figma_list_components` | List components in a team | GET | /teams/{team_id}/components |
| `figma_get_component` | Get component details | GET | /components/{key} |
| `figma_list_styles` | List styles in a team | GET | /teams/{team_id}/styles |
| `figma_get_style` | Get style details | GET | /styles/{key} |

---

## Tool Details

### figma_list_files

**What it does**: Lists all files in a Figma project with names, keys, and thumbnails.

**When to use**: Browse available designs, find specific files, or explore project structure.

**Arguments**:
- `project_id` (required): Figma project ID

**Example LLM prompt**: "What design files are in the Marketing project?"

---

### figma_get_file

**What it does**: Gets full details of a Figma file including pages, frames, components, and styles.

**When to use**: Get comprehensive file data, extract design structure, or analyze frame hierarchy.

**Arguments**:
- `file_key` (required): Figma file key
- `version` (optional): Specific version to retrieve
- `ids` (optional): Node IDs to retrieve specific elements
- `depth` (optional): Depth of the node tree to retrieve

**Example LLM prompt**: "Get the landing page design file"

---

### figma_get_file_nodes

**What it does**: Gets specific nodes from a Figma file by their IDs.

**When to use**: Extract specific elements like buttons, icons, or frames from a design.

**Arguments**:
- `file_key` (required): Figma file key
- `ids` (required): Array of node IDs to retrieve

**Example LLM prompt**: "Get the header component from file abc123"

---

### figma_list_comments

**What it does**: Lists all comments on a Figma file with author information and timestamps.

**When to use**: Read design feedback, review discussions, or understand stakeholder comments.

**Arguments**:
- `file_key` (required): Figma file key

**Example LLM prompt**: "What comments are on the landing page design?"

---

### figma_get_comment

**What it does**: Gets details about a specific comment on a Figma file.

**When to use**: Read full comment details, see attachment info, or check comment metadata.

**Arguments**:
- `comment_id` (required): Comment ID

**Example LLM prompt**: "Get details for comment abc123"

---

### figma_post_comment

**What it does**: Posts a new comment on a Figma file. Optionally includes position or region metadata.

**When to use**: Add design feedback, ask questions, or start discussions about a design.

**Arguments**:
- `file_key` (required): Figma file key
- `message` (required): Comment text
- `client_meta` (optional): Position or region for the comment

**Example LLM prompt**: "Add a comment saying 'Consider using a warmer color palette here' on the hero section"

---

### figma_list_components

**What it does**: Lists all components in a Figma team library. Returns component names, keys, and descriptions.

**When to use**: Discover reusable design components, find component definitions, or explore the team's design system.

**Arguments**:
- `team_id` (required): Figma team ID
- `page_size` (optional): Number of components to return (default 50)
- `after` (optional): Pagination cursor

**Example LLM prompt**: "What components are available in the company design library?"

---

### figma_get_component

**What it does**: Gets details about a specific Figma component including its properties and variants.

**When to use**: Understand component structure, see variant options, or review component documentation.

**Arguments**:
- `key` (required): Component key
- `team_id` (optional): Figma team ID

**Example LLM prompt**: "Get details for the Button component"

---

### figma_list_styles

**What it does**: Lists all styles in a Figma team library. Returns style names, keys, and types (color, text, effects).

**When to use**: Discover design tokens, find reusable styles, or explore the team's style library.

**Arguments**:
- `team_id` (required): Figma team ID
- `page_size` (optional): Number of styles to return (default 50)

**Example LLM prompt**: "What styles are available in the company design library?"

---

### figma_get_style

**What it does**: Gets details about a specific Figma style including its properties and metadata.

**When to use**: Understand style configuration, see applied values, or review style documentation.

**Arguments**:
- `key` (required): Style key
- `team_id` (optional): Figma team ID

**Example LLM prompt**: "Get details for the Primary Blue color style"

---

## Figma API Notes

- **Files**: Figma files contain pages, frames, and design elements organized hierarchically.
- **Components**: Reusable design elements with variants that can be published to a team library.
- **Styles**: Named design attributes (colors, text styles, effects) that can be shared across designs.
- **Comments**: Design feedback attached to specific locations in a file.
- **Rate Limits**: 60 requests per minute per token.
- **Pagination**: Use cursor-based pagination with `after` parameter.
