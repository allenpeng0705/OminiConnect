# Artfol Tools

Provider: `artfol` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Artfol API. They allow AI agents to browse and manage posts, follow users, interact with content through likes and comments, and keep up with notifications. Artfol is a visual social platform popular among artists and creators.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Artfol
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `posts:read`, `posts:write`, `users:read`, `users:write`, `notifications:read`, `comments:write`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `artfol_list_posts` | List posts from feed or user | GET | /v1/posts |
| `artfol_get_post` | Get post details | GET | /v1/posts/{post_id} |
| `artfol_create_post` | Create a new post | POST | /v1/posts |
| `artfol_like_post` | Like a post | POST | /v1/posts/{post_id}/like |
| `artfol_comment_on_post` | Add comment to a post | POST | /v1/posts/{post_id}/comments |
| `artfol_list_users` | List users by relation or search | GET | /v1/users |
| `artfol_get_user` | Get user profile | GET | /v1/users/{user_id} |
| `artfol_follow_user` | Follow a user | POST | /v1/users/{user_id}/follow |
| `artfol_unfollow_user` | Unfollow a user | DELETE | /v1/users/{user_id}/follow |
| `artfol_list_notifications` | List notifications | GET | /v1/notifications |

---

## Tool Details

### artfol_list_posts

**What it does**: Lists posts from the authenticated user's feed or from a specific user. Returns posts with metadata including images, descriptions, likes, and comments count.

**When to use**: Browse posts from artists you follow, or filter posts by a specific user's profile.

**Arguments**:
- `user_id` (optional): Filter posts by user ID
- `feed_type` (optional): `following` or `global` (default: following)
- `page` (optional): Page number (default: 1)
- `per_page` (optional): Results per page, max 50 (default: 20)

**Example LLM prompt**: "Show me posts from my friends' feed"

---

### artfol_get_post

**What it does**: Gets detailed information about a specific post including images, description, like count, and comments.

**When to use**: Read the full content of a post before interacting with it, or view engagement metrics.

**Arguments**:
- `post_id` (required): Post ID

**Example LLM prompt**: "Get details for post 12345"

---

### artfol_create_post

**What it does**: Creates a new post with one or more images and an optional description. Publish artwork or photos to your profile.

**When to use**: Share new artwork, photography, or visual content with your followers.

**Arguments**:
- `images` (required): Array of image URLs or base64 encoded images (max 5)
- `description` (optional): Post description (max 2000 characters)
- `tags` (optional): Array of tags to add
- `sensitivity` (optional): `none`, `mature`, or `explicit` (default: none)

**Example LLM prompt**: "Post a new artwork with two images at https://example.com/art1.jpg and https://example.com/art2.jpg with the tag #digitalart"

---

### artfol_like_post

**What it does**: Likes a specific post. Liking increases visibility and shows appreciation to the creator.

**When to use**: Engage with content and support other artists.

**Arguments**:
- `post_id` (required): Post ID to like

**Example LLM prompt**: "Like post 12345"

---

### artfol_comment_on_post

**What it does**: Adds a comment to a post. Comments are visible to all users.

**When to use**: Engage in discussions or give feedback on artwork.

**Arguments**:
- `post_id` (required): Post ID to comment on
- `content` (required): Comment text (max 500 characters)
- `parent_id` (optional): Parent comment ID for replies

**Example LLM prompt**: "Comment 'Beautiful work!' on post 12345"

---

### artfol_list_users

**What it does**: Lists users by relationship (followers, following) or searches by username.

**When to use**: Find artists, view followers, or discover new accounts.

**Arguments**:
- `user_id` (optional): Get followers/following for a specific user
- `relation` (optional): `followers` or `following`
- `search` (optional): Search users by username or display name
- `page` (optional): Page number (default: 1)
- `per_page` (optional): Results per page (default: 20)

**Example LLM prompt**: "Show me users who follow artist_john"

---

### artfol_get_user

**What it does**: Gets a user's profile including bio, avatar, post count, follower count, and following count.

**When to use**: Learn more about an artist before following or interacting.

**Arguments**:
- `user_id` (required): User ID (use `me` for authenticated user)

**Example LLM prompt**: "Get the profile for user 67890"

---

### artfol_follow_user

**What it does**: Follows a user. Their posts will appear in your feed.

**When to use**: Stay updated on an artist's new work.

**Arguments**:
- `user_id` (required): User ID to follow

**Example LLM prompt**: "Follow user 67890"

---

### artfol_unfollow_user

**What it does**: Unfollows a user. Their posts will no longer appear in your feed.

**When to use**: Stop receiving updates from an artist.

**Arguments**:
- `user_id` (required): User ID to unfollow

**Example LLM prompt**: "Unfollow user 67890"

---

### artfol_list_notifications

**What it does**: Lists notifications for the authenticated user including likes, comments, follows, and mentions.

**When to use**: Stay updated on interactions with your posts and profile.

**Arguments**:
- `type` (optional): `like`, `comment`, `follow`, `mention`, or `all` (default: all)
- `unread_only` (optional): Only show unread notifications (default: false)
- `page` (optional): Page number (default: 1)
- `per_page` (optional): Results per page (default: 20)

**Example LLM prompt**: "Show me all my unread notifications"

---

## Artfol API Notes

- **Media**: Artfol is image-focused; posts require image URLs or base64 encoded images
- **Social**: Following drives the feed; engagement (likes, comments) increases visibility
- **Discovery**: Tags help categorize content and reach new audiences
- **Rate Limits**: Pagination helps manage API usage effectively
