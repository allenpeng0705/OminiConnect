# Ghost Admin Tools

Provider: `ghost-admin` | Engine: `nango` | Auth: JWT via Nango

## Overview

These tools wrap the Ghost Admin API. They allow AI agents to manage posts, pages, tags, users, and site settings. **Requires Ghost Admin JWT authentication.**

## Authentication

**Nango JWT**:
- Uses private key to generate JWT tokens for Ghost Admin API
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://{adminDomain}/ghost/api/admin/`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `ghost_admin_list_posts` | List posts | GET | /posts |
| `ghost_admin_get_post` | Get post details | GET | /posts/{id} |
| `ghost_admin_create_post` | Create a post | POST | /posts |
| `ghost_admin_update_post` | Update a post | PUT | /posts/{id} |
| `ghost_admin_delete_post` | Delete a post | DELETE | /posts/{id} |
| `ghost_admin_list_pages` | List pages | GET | /pages |
| `ghost_admin_get_page` | Get page details | GET | /pages/{id} |
| `ghost_admin_list_tags` | List tags | GET | /tags |
| `ghost_admin_list_users` | List users | GET | /users |
| `ghost_admin_get_settings` | Get site settings | GET | /settings |

---

## Tool Details

### ghost_admin_list_posts

**What it does**: Lists all posts in Ghost with optional filters.

**When to use**: Browse posts, filter by status.

**Arguments**:
- `status` (optional): Filter by status (draft, published, scheduled)
- `limit` (optional): Number of results (default 20)

**Example LLM prompt**: "List all published posts"

---

### ghost_admin_get_post

**What it does**: Gets detailed information about a specific post.

**When to use**: Get post content, metadata, tags.

**Arguments**:
- `id` (required): Post ID

**Example LLM prompt**: "Get post with id abc123"

---

### ghost_admin_create_post

**What it does**: Creates a new post in Ghost.

**When to use**: Publish new blog content.

**Arguments**:
- `title` (required): Post title
- `html` (optional): Post content in HTML
- `status` (optional): Post status (draft, published)
- `tags` (optional): Tag names to assign

**Example LLM prompt**: "Create a new post titled 'Hello World' with content '<p>Hello!</p>'"

---

### ghost_admin_update_post

**What it does**: Updates an existing post.

**When to use**: Edit post content or change status.

**Arguments**:
- `id` (required): Post ID
- `title` (optional): New title
- `html` (optional): New content
- `status` (optional): New status

**Example LLM prompt**: "Update post abc123 to published status"

---

### ghost_admin_delete_post

**What it does**: Deletes a post from Ghost.

**When to use**: Remove unwanted posts.

**Arguments**:
- `id` (required): Post ID

**Example LLM prompt**: "Delete post abc123"

---

### ghost_admin_list_pages

**What it does**: Lists all pages in Ghost.

**When to use**: Browse static pages.

**Arguments**:
- `status` (optional): Filter by status

**Example LLM prompt**: "List all pages"

---

### ghost_admin_get_page

**What it does**: Gets detailed information about a specific page.

**When to use**: Get page content and metadata.

**Arguments**:
- `id` (required): Page ID

**Example LLM prompt**: "Get page abc123"

---

### ghost_admin_list_tags

**What it does**: Lists all tags in Ghost.

**When to use**: View available tags for organizing content.

**Arguments**: None

**Example LLM prompt**: "List all tags"

---

### ghost_admin_list_users

**What it does**: Lists all users in Ghost.

**When to use**: View site authors and contributors.

**Arguments**: None

**Example LLM prompt**: "List all users"

---

### ghost_admin_get_settings

**What it does**: Gets Ghost site settings.

**When to use**: View site configuration.

**Arguments**: None

**Example LLM prompt**: "Get site settings"

---

## Ghost Admin API Notes

- **Post status**: draft, published, scheduled
- **Pages**: Static pages (not in feed)
- **Tags**: Used for organizing posts
- **Users**: Authors and admin users
- **JWT tokens**: Expire after 5 minutes, auto-refreshed
