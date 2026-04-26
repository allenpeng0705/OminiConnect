# Facebook Pages Tools

Provider: `facebook` | Engine: `nango` | Auth: OAuth (via Nango) | Requires: Page management permissions

## Overview

These tools wrap the Facebook Graph API for Page management. They allow AI agents to manage Facebook pages, posts, comments, insights, messaging, and events on behalf of the authenticated user.

## Authentication

**Nango (recommended for OAuth)**:
- User authenticates via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Requires Facebook Page management permissions
- Scopes: `pages_read_user_access`, `pages_write_user_access`, etc.

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `facebook_list_pages` | List pages managed by the user | GET | /me/accounts |
| `facebook_get_page` | Get page info | GET | /{page-id} |
| `facebook_list_posts` | List page posts | GET | /{page-id}/posts |
| `facebook_get_post` | Get a specific post | GET | /{post-id} |
| `facebook_create_post` | Create a post on the page | POST | /{page-id}/feed |
| `facebook_list_comments` | List comments on a post | GET | /{post-id}/comments |
| `facebook_get_comment` | Get a specific comment | GET | /{comment-id} |
| `facebook_get_page_insights` | Get page insights/metrics | GET | /{page-id}/insights |
| `facebook_send_message` | Send a message via page | POST | /{page-id}/messages |
| `facebook_list_page_events` | List page events | GET | /{page-id}/events |

---

## Tool Details

### facebook_list_pages

**What it does**: Lists all Facebook pages managed by the authenticated user. Returns page IDs, names, and access status.

**When to use**: Find which pages you can manage before performing page operations.

**Arguments**:
- `limit` (optional): Max pages to return (default 10, max 100)

**Example LLM prompt**: "List all the Facebook pages I manage"

---

### facebook_get_page

**What it does**: Get Facebook page information including name, description, follower count, and cover photo.

**When to use**: Get context about a page before posting or analyzing its performance.

**Arguments**:
- `page_id` (required): Page ID
- `fields` (optional): Fields to include (default: id, name, about, followers_count)

**Example LLM prompt**: "Get details for my Facebook page"

---

### facebook_list_posts

**What it does**: Lists posts from a Facebook page with post content, metrics, and timestamps.

**When to use**: Review page activity, analyze engagement, or find a specific post to interact with.

**Arguments**:
- `page_id` (required): Page ID
- `limit` (optional): Max posts (default 10, max 100)
- `fields` (optional): Fields to include (default: id, message, created_time, shares, likes)

**Example LLM prompt**: "Show me the last 20 posts on my Facebook page"

---

### facebook_get_post

**What it does**: Get a specific Facebook post by ID with full details including message, metrics, and media.

**When to use**: Read post details before responding, analyzing, or deleting.

**Arguments**:
- `post_id` (required): Post ID
- `fields` (optional): Fields to include

**Example LLM prompt**: "Get details for Facebook post ID 123456"

---

### facebook_create_post

**What it does**: Creates a post on a Facebook page. Supports text, links, and media.

**When to use**: Publish content, share updates, or announce news on a Facebook page.

**Arguments**:
- `page_id` (required): Page ID
- `message` (required): Post content text
- `link` (optional): URL to attach

**Example LLM prompt**: "Create a Facebook post announcing our new product launch"

---

### facebook_list_comments

**What it does**: Lists comments on a Facebook post. Returns comment text, author, and timestamp.

**When to use**: Monitor engagement, find comments to respond to, or analyze sentiment.

**Arguments**:
- `post_id` (required): Post ID
- `limit` (optional): Max comments (default 10, max 100)
- `fields` (optional): Fields to include

**Example LLM prompt**: "List all comments on the post about our sale"

---

### facebook_get_comment

**What it does**: Get a specific Facebook comment by ID including text, author, and likes.

**When to use**: Read comment details before replying or moderating.

**Arguments**:
- `comment_id` (required): Comment ID
- `fields` (optional): Fields to include

**Example LLM prompt**: "Get details for comment ID 789012"

---

### facebook_get_page_insights

**What it does**: Get insights and metrics for a Facebook page including page views, reach, and engagement.

**When to use**: Track page performance, analyze audience engagement, or generate reports.

**Arguments**:
- `page_id` (required): Page ID
- `metric` (optional): Metric name (e.g., page_views, page_impressions)
- `period` (optional): Period: day, week, days_28 (default: week)

**Example LLM prompt**: "Show me page insights for the last month"

---

### facebook_send_message

**What it does**: Sends a direct message to a user via Facebook page messaging.

**When to use**: Respond to customer inquiries, send automated responses, or communicate with page followers.

**Arguments**:
- `page_id` (required): Page ID
- `recipient` (required): Recipient user ID or PSID
- `message` (required): Message text content

**Example LLM prompt**: "Send a message to user 123 saying thanks for reaching out"

---

### facebook_list_page_events

**What it does**: Lists events associated with a Facebook page including event name, date, and attendance.

**When to use**: Manage page events, check attendance, or find events to promote.

**Arguments**:
- `page_id` (required): Page ID
- `limit` (optional): Max events (default 10, max 100)
- `fields` (optional): Fields to include

**Example LLM prompt**: "List upcoming events on my Facebook page"

---

## Facebook API Reference

These tools use the Facebook Graph API. See official docs for full details:
- https://developers.facebook.com/docs/graph-api
- Rate limits: 200 calls/hour per user per app (varies by endpoint)
- Pagination: Use `limit` and `after` parameters
- All dates: ISO 8601 format

## Notes

- Facebook API access requires app review for certain permissions
- Page management requires admin access to the page
- Messaging requires the Page to have messaging enabled
- Insights have different availability based on page role and age
- Some endpoints require a Facebook Business verified account
