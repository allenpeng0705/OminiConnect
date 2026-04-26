# Circle.so Tools

Provider: `circle-so` | Engine: `nango` | Auth: OAuth (via Nango)

## Overview

These tools wrap the Circle.so API for community management. They allow AI agents to interact with communities, members, posts, comments, and events.

## Authentication

**Nango (recommended for OAuth)**:
- User authenticates via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `communities:read`, `members:read`, `posts:read`, `posts:create`, `comments:read`, `events:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `circle_so_list_communities` | List communities | GET | /communities |
| `circle_so_get_community` | Get details of a community | GET | /communities/{community_id} |
| `circle_so_list_members` | List members in a community | GET | /communities/{community_id}/members |
| `circle_so_get_member` | Get details of a member | GET | /communities/{community_id}/members/{member_id} |
| `circle_so_list_posts` | List posts in a community | GET | /communities/{community_id}/posts |
| `circle_so_get_post` | Get details of a post | GET | /communities/{community_id}/posts/{post_id} |
| `circle_so_create_post` | Create a new post | POST | /communities/{community_id}/posts |
| `circle_so_list_comments` | List comments on a post | GET | /communities/{community_id}/posts/{post_id}/comments |
| `circle_so_get_comment` | Get details of a comment | GET | /communities/{community_id}/posts/{post_id}/comments/{comment_id} |
| `circle_so_list_events` | List events in a community | GET | /communities/{community_id}/events |

---

## Tool Details

### circle_so_list_communities

**What it does**: Returns a paginated list of communities the authenticated user has access to.

**When to use**: Find available communities before browsing content or members.

**Arguments**:
- `page` (optional): Page number (default 1)
- `per_page` (optional): Results per page (default 20)

**Example LLM prompt**: "List all communities I have access to"

---

### circle_so_get_community

**What it does**: Get details of a specific community including name, description, member count, and settings.

**When to use**: Understand a community's focus before browsing or posting content.

**Arguments**:
- `community_id` (required): Community ID

**Example LLM prompt**: "Get details for community abc-123"

---

### circle_so_list_members

**What it does**: Returns a paginated list of members in a specific community. Filter by role or join date.

**When to use**: Find community members, check member roster, or identify active participants.

**Arguments**:
- `community_id` (required): Community ID
- `page` (optional): Page number (default 1)
- `per_page` (optional): Results per page (default 20)
- `sort` (optional): Sort order - join_date, name, created_at (default: join_date)

**Example LLM prompt**: "List all members of the product community"

---

### circle_so_get_member

**What it does**: Get details of a specific community member including name, email, role, and join date.

**When to use**: Look up a specific member's profile or role in the community.

**Arguments**:
- `community_id` (required): Community ID
- `member_id` (required): Member ID

**Example LLM prompt**: "Get details for member xyz-456 in community abc-123"

---

### circle_so_list_posts

**What it does**: Returns a paginated list of posts in a community. Filter by author, tag, or content type.

**When to use**: Browse discussions, find relevant content, or research community topics.

**Arguments**:
- `community_id` (required): Community ID
- `page` (optional): Page number (default 1)
- `per_page` (optional): Results per page (default 20)
- `author_id` (optional): Filter by author member ID
- `tag_id` (optional): Filter by tag ID

**Example LLM prompt**: "Show me recent posts in the engineering community"

---

### circle_so_get_post

**What it does**: Get full details of a specific post including title, body, author, likes, and comments count.

**When to use**: Read a specific post before commenting or engaging with it.

**Arguments**:
- `community_id` (required): Community ID
- `post_id` (required): Post ID

**Example LLM prompt**: "Get details for post 789 in community abc-123"

---

### circle_so_create_post

**What it does**: Create a new post in a community. Include title, body content, and optional tags.

**When to use**: Start a new discussion, share an update, or ask a question to the community.

**Arguments**:
- `community_id` (required): Community ID to create post in
- `title` (required): Post title
- `body` (required): Post body content (HTML or markdown)
- `tag_ids` (optional): Array of tag IDs to assign
- `pinned` (optional): Pin the post to top of community (default: false)

**Example LLM prompt**: "Create a post in the engineering community titled 'API v2 Discussion' with body 'Let's discuss the new API changes'"

---

### circle_so_list_comments

**What it does**: Returns a paginated list of comments on a specific post. Includes author and timestamps.

**When to use**: Read discussion on a post, find answers, or understand community sentiment.

**Arguments**:
- `community_id` (required): Community ID
- `post_id` (required): Post ID
- `page` (optional): Page number (default 1)
- `per_page` (optional): Results per page (default 20)

**Example LLM prompt**: "Show me all comments on post 789"

---

### circle_so_get_comment

**What it does**: Get details of a specific comment including text, author, and creation date.

**When to use**: Read a specific comment in full.

**Arguments**:
- `community_id` (required): Community ID
- `post_id` (required): Post ID
- `comment_id` (required): Comment ID

**Example LLM prompt**: "Get details for comment 999 in post 789"

---

### circle_so_list_events

**What it does**: Returns a paginated list of events in a community. Filter by date, type, or status.

**When to use**: Find upcoming community events, webinars, or meetups.

**Arguments**:
- `community_id` (required): Community ID
- `page` (optional): Page number (default 1)
- `per_page` (optional): Results per page (default 20)
- `status` (optional): Filter by status - upcoming, past, all (default: upcoming)

**Example LLM prompt**: "Show me all upcoming events in the community"

---

## Circle.so API Reference

These tools use the Circle.so API. See official docs for full details:
- https://developers.circle.so
- Rate limits: Follow your Circle.so plan limits
- Pagination: Use `page` and `per_page` parameters
- All dates: ISO 8601 format
