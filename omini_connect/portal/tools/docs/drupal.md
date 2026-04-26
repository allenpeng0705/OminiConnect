# Drupal Tools

Provider: `drupal` | Engine: `nango` | Auth: Two-Step via Nango

## Overview

These tools wrap the Drupal API. They allow AI agents to manage content nodes, users, taxonomy terms, and comments. Drupal is a content management system (CMS).

## Authentication

**Nango Two-Step**:
- Uses client credentials with username/password
- Token stored in Nango, accessed via `connection_ref`
- Base URL configured per connection

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `drupal_list_nodes` | List content nodes | GET | /node |
| `drupal_get_node` | Get node details | GET | /node/{node_id} |
| `drupal_create_node` | Create a node | POST | /node |
| `drupal_update_node` | Update a node | PUT | /node/{node_id} |
| `drupal_list_users` | List users | GET | /user |
| `drupal_get_user` | Get user details | GET | /user/{user_id} |
| `drupal_list_taxonomies` | List taxonomy terms | GET | /taxonomy_term |
| `drupal_get_taxonomy` | Get taxonomy details | GET | /taxonomy_term/{term_id} |
| `drupal_list_comments` | List comments | GET | /comment |
| `drupal_get_comment` | Get comment details | GET | /comment/{comment_id} |

---

## Tool Details

### drupal_list_nodes

**What it does**: Lists all content nodes with optional type filtering.

**When to use**: Browse website content, find articles, manage content inventory.

**Arguments**:
- `limit` (optional): Max results (default 20, max 100)
- `type` (optional): Filter by content type (e.g., article, page)

**Example LLM prompt**: "List all article nodes"

---

### drupal_get_node

**What it does**: Gets detailed node information.

**When to use**: Read article content, check node details, review content metadata.

**Arguments**:
- `node_id` (required): Node ID

**Example LLM prompt**: "Get details for node n-123"

---

### drupal_create_node

**What it does**: Creates a new content node.

**When to use**: Publish new articles, create pages, add content to website.

**Arguments**:
- `type` (optional): Content type (default: article)
- `title` (required): Node title
- `body` (optional): Body content

**Example LLM prompt**: "Create a new article titled 'Welcome' with body content"

---

### drupal_update_node

**What it does**: Updates an existing content node.

**When to use**: Edit articles, update page content, modify existing content.

**Arguments**:
- `node_id` (required): Node ID
- `title` (optional): Node title
- `body` (optional): Body content

**Example LLM prompt**: "Update node n-123 with new title"

---

### drupal_list_users

**What it does**: Lists all users in the Drupal site.

**When to use**: Browse user directory, find site members, manage user accounts.

**Arguments**:
- `limit` (optional): Max results (default 50, max 100)

**Example LLM prompt**: "List all Drupal users"

---

### drupal_get_user

**What it does**: Gets detailed user information.

**When to use**: Check user profile, verify user permissions, review user activity.

**Arguments**:
- `user_id` (required): User ID

**Example LLM prompt**: "Get details for user u-456"

---

### drupal_list_taxonomies

**What it does**: Lists all taxonomy terms with optional vocabulary filtering.

**When to use**: Browse categories, find tags, manage content classification.

**Arguments**:
- `limit` (optional): Max results (default 50, max 100)
- `vocabulary` (optional): Filter by vocabulary name

**Example LLM prompt**: "List all tags (taxonomy terms)"

---

### drupal_get_taxonomy

**What it does**: Gets detailed taxonomy term information.

**When to use**: Check term details, understand term hierarchy, review term usage.

**Arguments**:
- `term_id` (required): Term ID

**Example LLM prompt**: "Get details for taxonomy term t-789"

---

### drupal_list_comments

**What it does**: Lists all comments with optional node filtering.

**When to use**: View discussions, moderate comments, track comment activity.

**Arguments**:
- `limit` (optional): Max results (default 50, max 100)
- `node_id` (optional): Filter by node ID

**Example LLM prompt**: "List all comments on node n-123"

---

### drupal_get_comment

**What it does**: Gets detailed comment information.

**When to use**: Review comment content, check comment status, moderate discussions.

**Arguments**:
- `comment_id` (required): Comment ID

**Example LLM prompt**: "Get details for comment c-101"

---

## Drupal API Notes

- **Two-Step Auth**: Uses client credentials flow with username/password
- **Content Nodes**: Articles, pages, and other content types
- **Users**: Site members and administrators
- **Taxonomies**: Categories, tags, and content classification
- **Comments**: User discussions on content nodes
- **Base URL**: Configured per connection for self-hosted Drupal
