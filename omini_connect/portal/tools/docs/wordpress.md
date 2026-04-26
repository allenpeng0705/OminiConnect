# WordPress Tools

Provider: `wordpress` | Engine: `nango` | Auth: OAuth2 via Nango

## Overview

WordPress is a content management system for websites and blogs. **Requires oauth2 via nango.**

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with WordPress
- Token stored in Nango, accessed via `connection_ref`
- Scopes: read, write

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `wordpress_get_current_user` | Get current authenticated user | GET | /me |
| `wordpress_list_sites` | List all sites for the user | GET | /me/sites |
| `wordpress_get_site` | Get site details | GET | /sites/{site_id} |
| `wordpress_list_posts` | List posts on a site | GET | /sites/{site_id}/posts |
| `wordpress_get_post` | Get post details | GET | /sites/{site_id}/posts/{post_id} |
| `wordpress_create_post` | Create a new post | POST | /sites/{site_id}/posts/new |
| `wordpress_update_post` | Update an existing post | POST | /sites/{site_id}/posts/{post_id} |
| `wordpress_delete_post` | Delete a post | POST | /sites/{site_id}/posts/{post_id}/delete |
| `wordpress_list_comments` | List comments on a site | GET | /sites/{site_id}/comments |
| `wordpress_list_media` | List media library items | GET | /sites/{site_id}/media |

---

## Tool Details

### wordpress_get_current_user

**What it does**: Get current authenticated user

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### wordpress_list_sites

**What it does**: List all sites for the user

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### wordpress_get_site

**What it does**: Get site details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### wordpress_list_posts

**What it does**: List posts on a site

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### wordpress_get_post

**What it does**: Get post details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### wordpress_create_post

**What it does**: Create a new post

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### wordpress_update_post

**What it does**: Update an existing post

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### wordpress_delete_post

**What it does**: Delete a post

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### wordpress_list_comments

**What it does**: List comments on a site

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### wordpress_list_media

**What it does**: List media library items

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---


## API Notes

- Base URL: `https://public-api.wordpress.com/rest/v1`
- Docs: https://nango.dev/docs/integrations/all/wordpress
