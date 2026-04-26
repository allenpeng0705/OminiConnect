# Tumblr Tools

Provider: `tumblr` | Engine: `nango` | Auth: OAuth2 via Nango

## Overview

Tumblr is a microblogging and social media platform. **Requires oauth2 via nango.**

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Tumblr
- Token stored in Nango, accessed via `connection_ref`
- Scopes: read, write

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `tumblr_get_user_info` | Get authenticated user information | GET | /v2/user/info |
| `tumblr_list_blogs` | List all blogs for the user | GET | /v2/user/following |
| `tumblr_get_blog_info` | Get blog details | GET | /v2/blog/{blog_name}/info |
| `tumblr_list_posts` | List posts on a blog | GET | /v2/blog/{blog_name}/posts |
| `tumblr_create_post` | Create a new blog post | POST | /v2/blog/{blog_name}/post |
| `tumblr_edit_post` | Edit an existing post | PUT | /v2/blog/{blog_name}/post/{post_id} |
| `tumblr_delete_post` | Delete a post | DELETE | /v2/blog/{blog_name}/post/{post_id} |
| `tumblr_like_post` | Like a post | POST | /v2/user/like |
| `tumblr_unlike_post` | Unlike a post | POST | /v2/user/unlike |
| `tumblr_follow_blog` | Follow a blog | POST | /v2/user/follow |

---

## Tool Details

### tumblr_get_user_info

**What it does**: Get authenticated user information

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tumblr_list_blogs

**What it does**: List all blogs for the user

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tumblr_get_blog_info

**What it does**: Get blog details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tumblr_list_posts

**What it does**: List posts on a blog

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tumblr_create_post

**What it does**: Create a new blog post

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tumblr_edit_post

**What it does**: Edit an existing post

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tumblr_delete_post

**What it does**: Delete a post

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tumblr_like_post

**What it does**: Like a post

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tumblr_unlike_post

**What it does**: Unlike a post

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tumblr_follow_blog

**What it does**: Follow a blog

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---


## API Notes

- Base URL: `https://api.tumblr.com/v2`
- Docs: https://nango.dev/docs/integrations/all/tumblr
