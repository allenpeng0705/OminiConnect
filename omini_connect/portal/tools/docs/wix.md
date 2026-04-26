# Wix Tools

Provider: `wix` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Wix API. They allow AI agents to manage sites, pages, members, and blog content in a Wix account. Wix is a popular website building platform with built-in e-commerce, membership, and content management features.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Wix
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `site:read`, `site:write`, `page:read`, `page:write`, `members:read`, `blog:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `wix_list_sites` | List all sites | GET | /sites |
| `wix_get_site` | Get site details | GET | /sites/{site_id} |
| `wix_list_pages` | List pages in a site | GET | /sites/{site_id}/pages |
| `wix_get_page` | Get page details | GET | /sites/{site_id}/pages/{page_id} |
| `wix_list_members` | List site members | GET | /sites/{site_id}/members |
| `wix_get_member` | Get member details | GET | /sites/{site_id}/members/{member_id} |
| `wix_list_blog_posts` | List blog posts | GET | /sites/{site_id}/blog/posts |
| `wix_get_blog_post` | Get blog post details | GET | /sites/{site_id}/blog/posts/{post_id} |
| `wix_get_site_settings` | Get site settings | GET | /sites/{site_id}/settings |
| `wix_update_site_settings` | Update site settings | PATCH | /sites/{site_id}/settings |

---

## Tool Details

### wix_list_sites

**What it does**: Lists all sites associated with your Wix account.

**When to use**: View all sites, find sites by pagination.

**Arguments**:
- `limit` (optional): Maximum number of sites to return (default 20, max 100)
- `offset` (optional): Number of sites to skip for pagination

**Example LLM prompt**: "List all sites in my Wix account"

---

### wix_get_site

**What it does**: Gets detailed information about a specific Wix site by site ID.

**When to use**: Check site settings, view site configuration.

**Arguments**:
- `site_id` (required): Unique identifier of the site

**Example LLM prompt**: "Get details for site abc-123"

---

### wix_list_pages

**What it does**: Lists all pages within a specific Wix site.

**When to use**: Browse site structure, find specific pages.

**Arguments**:
- `site_id` (required): Unique identifier of the site

**Example LLM prompt**: "List all pages in site abc-123"

---

### wix_get_page

**What it does**: Gets detailed information about a specific page including its hierarchy and settings.

**When to use**: Get page details, check page structure.

**Arguments**:
- `site_id` (required): Unique identifier of the site
- `page_id` (required): Unique identifier of the page

**Example LLM prompt**: "Get page details for page xyz-456 in site abc-123"

---

### wix_list_members

**What it does**: Lists all members (registered users) of a Wix site.

**When to use**: View site members, find members by pagination.

**Arguments**:
- `site_id` (required): Unique identifier of the site
- `limit` (optional): Maximum number of members to return (default 20)
- `offset` (optional): Number of members to skip for pagination

**Example LLM prompt**: "List all members in site abc-123"

---

### wix_get_member

**What it does**: Gets detailed information about a specific site member including profile and contact info.

**When to use**: Check member profile, view member activity.

**Arguments**:
- `site_id` (required): Unique identifier of the site
- `member_id` (required): Unique identifier of the member

**Example LLM prompt**: "Get details for member mem-789 in site abc-123"

---

### wix_list_blog_posts

**What it does**: Lists all blog posts published on a Wix site.

**When to use**: Browse blog content, find posts by pagination.

**Arguments**:
- `site_id` (required): Unique identifier of the site
- `limit` (optional): Maximum number of posts to return (default 20)
- `offset` (optional): Number of posts to skip for pagination

**Example LLM prompt**: "List all blog posts in site abc-123"

---

### wix_get_blog_post

**What it does**: Gets detailed content and metadata of a specific blog post.

**When to use**: Get blog post content, check post metadata.

**Arguments**:
- `site_id` (required): Unique identifier of the site
- `post_id` (required): Unique identifier of the blog post

**Example LLM prompt**: "Get blog post details for post post-111 in site abc-123"

---

### wix_get_site_settings

**What it does**: Gets global settings and configuration for a Wix site.

**When to use**: Check site settings, view site configuration.

**Arguments**:
- `site_id` (required): Unique identifier of the site

**Example LLM prompt**: "Get settings for site abc-123"

---

### wix_update_site_settings

**What it does**: Updates global settings and configuration for a Wix site.

**When to use**: Modify site settings, update site configuration.

**Arguments**:
- `site_id` (required): Unique identifier of the site
- `site_name` (optional): Updated name for the site
- `language` (optional): Primary language code (e.g., en, es, fr)

**Example LLM prompt**: "Update site name to 'New Site Name' for site abc-123"

---

## Wix API Notes

- **Sites**: Wix sites are identified by unique site_id
- **Members**: Wix membership system for site visitors (registered users)
- **Blog**: Wix blog posts with rich content and metadata
- **Rate Limits**: Varies by Wix plan tier
- **Webhooks**: Wix supports webhooks for real-time updates (future expansion)
