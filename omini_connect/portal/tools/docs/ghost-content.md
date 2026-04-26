# Ghost Content API Tools

Provider: `ghost-content` | Engine: `nango` | Auth: API_KEY via Nango

## Overview

These tools wrap the Ghost Content API. They allow AI agents to read published posts, pages, tags, authors, and site settings. This is a read-only API for public content. **Requires Ghost Content API key.**

## Authentication

**Nango API_KEY**:
- User provides their Ghost Content API key
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://{adminDomain}/ghost/api/content/`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `ghost_content_list_posts` | List published posts | GET | /posts |
| `ghost_content_get_post` | Get post by slug | GET | /posts/{slug} |
| `ghost_content_list_pages` | List published pages | GET | /pages |
| `ghost_content_get_page` | Get page by slug | GET | /pages/{slug} |
| `ghost_content_list_tags` | List tags | GET | /tags |
| `ghost_content_get_tag` | Get tag by slug | GET | /tags/{slug} |
| `ghost_content_list_authors` | List authors | GET | /authors |
| `ghost_content_get_author` | Get author by slug | GET | /authors/{slug} |
| `ghost_content_list_settings` | Get site settings | GET | /settings |
| `ghost_content_list_tiers` | List membership tiers | GET | /tiers |

---

## Tool Details

### ghost_content_list_posts

**What it does**: Lists all published posts from Ghost.

**When to use**: Browse blog posts, paginate through content.

**Arguments**:
- `limit` (optional): Number of results (default 20)
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List the latest 10 posts"

---

### ghost_content_get_post

**What it does**: Gets a specific post by its slug.

**When to use**: Read full post content and metadata.

**Arguments**:
- `slug` (required): Post slug

**Example LLM prompt**: "Get the post with slug hello-world"

---

### ghost_content_list_pages

**What it does**: Lists all published pages from Ghost.

**When to use**: Browse static pages on the site.

**Arguments**:
- `limit` (optional): Number of results (default 20)

**Example LLM prompt**: "List all pages"

---

### ghost_content_get_page

**What it does**: Gets a specific page by its slug.

**When to use**: Read full page content.

**Arguments**:
- `slug` (required): Page slug

**Example LLM prompt**: "Get the page with slug about-us"

---

### ghost_content_list_tags

**What it does**: Lists all tags from Ghost.

**When to use**: Browse available tags.

**Arguments**:
- `limit` (optional): Number of results (default 20)

**Example LLM prompt**: "List all tags"

---

### ghost_content_get_tag

**What it does**: Gets a specific tag by its slug.

**When to use**: Get tag details and description.

**Arguments**:
- `slug` (required): Tag slug

**Example LLM prompt**: "Get the tag with slug technology"

---

### ghost_content_list_authors

**What it does**: Lists all authors from Ghost.

**When to use**: Browse site contributors.

**Arguments**:
- `limit` (optional): Number of results (default 20)

**Example LLM prompt**: "List all authors"

---

### ghost_content_get_author

**What it does**: Gets a specific author by their slug.

**When to use**: Get author bio and profile.

**Arguments**:
- `slug` (required): Author slug

**Example LLM prompt**: "Get the author with slug john"

---

### ghost_content_list_settings

**What it does**: Gets public site settings from Ghost.

**When to use**: Get site title, description, URL.

**Arguments**: None

**Example LLM prompt**: "Get site settings"

---

### ghost_content_list_tiers

**What it does**: Lists membership tiers from Ghost.

**When to use**: View available membership options.

**Arguments**: None

**Example LLM prompt**: "List membership tiers"

---

## Ghost Content API Notes

- **Read-only API**: Only exposes published content
- **Slug-based lookup**: Posts and pages identified by URL slug
- **Public content**: No authentication required for public posts
- **Tiers**: Membership/paid content tiers
