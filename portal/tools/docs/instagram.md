# Instagram Tools

Provider: `instagram` | Engine: `nango` | Auth: OAuth (via Facebook) | Requires: Business or Creator Instagram account

## Overview

These tools wrap the Instagram Graph API. They allow AI agents to manage Instagram business accounts, including media, comments, insights, users, and relationships. Requires a business or creator Instagram account connected via a Facebook Page.

## Authentication

**Nango (recommended for OAuth)**:
- User authenticates via Nango Connect
- Requires Facebook Page with connected Instagram Business Account
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `instagram_basic`, `instagram_content_publish`, `instagram_insights`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `instagram_list_media` | List media items on the account | GET | /{instagram-account-id}/media |
| `instagram_get_media` | Get a specific media item | GET | /{media-id} |
| `instagram_list_comments` | List comments on media | GET | /{media-id}/comments |
| `instagram_get_comment` | Get a specific comment | GET | /{comment-id} |
| `instagram_post_comment` | Post a comment on media | POST | /{media-id}/comments |
| `instagram_get_media_insights` | Get media insights/metrics | GET | /{media-id}/insights |
| `instagram_list_users` | List followers or following | GET | /{instagram-account-id}/{relationship} |
| `instagram_get_user` | Get user profile info | GET | /{user-id} |
| `instagram_list_relationships` | List relationships (followers/following) | GET | /{instagram-account-id}/relationships |
| `instagram_follow_user` | Follow or unfollow a user | POST | /{instagram-account-id}/relationships |

---

## Tool Details

### instagram_list_media

**What it does**: Lists media items (photos, videos, reels) on the authenticated Instagram business account.

**When to use**: Review past content, find media to analyze, or check what has been posted.

**Arguments**:
- `instagram_account_id` (required): Instagram Business Account ID
- `limit` (optional): Max media items (default 10, max 100)
- `fields` (optional): Fields to include (default: id, caption, media_type, timestamp)

**Example LLM prompt**: "List the last 20 posts on my Instagram"

---

### instagram_get_media

**What it does**: Get details of a specific Instagram media item including caption, media URL, and metadata.

**When to use**: Get full details of a specific post including media URLs for download or analysis.

**Arguments**:
- `media_id` (required): Media ID
- `fields` (optional): Fields to include

**Example LLM prompt**: "Get details for Instagram media ID 123456"

---

### instagram_list_comments

**What it does**: Lists comments on an Instagram media item. Returns comment text, author, and timestamp.

**When to use**: Monitor engagement, find comments to respond to, or analyze sentiment.

**Arguments**:
- `media_id` (required): Media ID
- `limit` (optional): Max comments (default 10, max 100)

**Example LLM prompt**: "List all comments on my latest post"

---

### instagram_get_comment

**What it does**: Get a specific Instagram comment by ID including text and author info.

**When to use**: Read comment details before replying or moderating.

**Arguments**:
- `comment_id` (required): Comment ID

**Example LLM prompt**: "Get details for comment ID 789012"

---

### instagram_post_comment

**What it does**: Posts a comment on an Instagram media item.

**When to use**: Engage with followers, respond to questions, or join conversations.

**Arguments**:
- `media_id` (required): Media ID
- `text` (required): Comment text content

**Example LLM prompt**: "Post a thank you comment on the post about our new product"

---

### instagram_get_media_insights

**What it does**: Get insights and metrics for an Instagram media item including reach, impressions, and engagement.

**When to use**: Analyze post performance, track engagement metrics, or generate content reports.

**Arguments**:
- `media_id` (required): Media ID

**Example LLM prompt**: "Show me insights for my last Instagram post"

---

### instagram_list_users

**What it does**: Lists users who are followers or following the Instagram account.

**When to use**: Analyze your audience or find accounts to engage with.

**Arguments**:
- `instagram_account_id` (required): Instagram Business Account ID
- `relationship` (required): followers or follows
- `limit` (optional): Max users (default 10, max 100)

**Example LLM prompt**: "List my Instagram followers"

---

### instagram_get_user

**What it does**: Get Instagram user profile information including username, name, bio, and follower count.

**When to use**: Research an Instagram account before following or engaging.

**Arguments**:
- `user_id` (required): Instagram User ID (use 'me' for authenticated account)
- `fields` (optional): Fields to include

**Example LLM prompt**: "Get profile info for the user with ID 1234567890"

---

### instagram_list_relationships

**What it does**: Lists the Instagram account's relationships (who follows you or who you follow).

**When to use**: Analyze your social network or find accounts to engage with.

**Arguments**:
- `instagram_account_id` (required): Instagram Business Account ID
- `direction` (required): followers or follows
- `limit` (optional): Max results (default 10, max 100)

**Example LLM prompt**: "Show me who I follow on Instagram"

---

### instagram_follow_user

**What it does**: Follow or unfollow an Instagram user.

**When to use**: Grow your network, engage with accounts, or declutter your feed.

**Arguments**:
- `instagram_account_id` (required): Instagram Business Account ID
- `user_id` (required): Target user ID to follow
- `action` (required): follow or unfollow

**Example LLM prompt**: "Follow the user with ID 9876543210 on Instagram"

---

## Instagram API Reference

These tools use the Instagram Graph API. See official docs for full details:
- https://developers.facebook.com/docs/instagram-api
- Rate limits: 200 calls/hour per user per app (varies by endpoint)
- Pagination: Use `limit` parameter
- Insights availability: Media must be at least 24 hours old

## Notes

- Requires an Instagram Business or Creator account
- Must be connected to a Facebook Page for API access
- Content publishing requires `instagram_content_publish` permission
- Insights require `instagram_insights` permission
- Direct messaging requires additional permissions and setup
- Some endpoints only work with the authenticated account (not other users)
