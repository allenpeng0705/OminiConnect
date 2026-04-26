# Vimeo Tools

Provider: `vimeo` | Engine: `nango` | Auth: OAuth (via Nango)

## Overview

These tools wrap the Vimeo API. They allow AI agents to manage videos, albums, users, and groups on behalf of the authenticated user.

## Authentication

**Nango (recommended for OAuth)**:
- User authenticates via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `videos:read`, `videos:write`, `users:read`, `groups:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `vimeo_list_videos` | List videos for the authenticated user | GET | /me/videos |
| `vimeo_get_video` | Get details of a specific video | GET | /videos/{video_id} |
| `vimeo_upload_video` | Upload a new video | POST | /me/videos |
| `vimeo_delete_video` | Delete a video | DELETE | /videos/{video_id} |
| `vimeo_list_users` | List users (following or followers) | GET | /me/following |
| `vimeo_get_user` | Get user profile information | GET | /users/{user_id} |
| `vimeo_list_albums` | List user's albums | GET | /me/albums |
| `vimeo_get_album` | Get album details and videos | GET | /albums/{album_id} |
| `vimeo_list_groups` | List user's groups | GET | /me/groups |
| `vimeo_get_group` | Get group details and members | GET | /groups/{group_id} |

---

## Tool Details

### vimeo_list_videos

**What it does**: Lists videos for the authenticated user. Returns video metadata including title, description, duration, and thumbnail URLs.

**When to use**: Browse your video library to find videos before getting details or managing them.

**Arguments**:
- `user_id` (optional, default 'me'): User ID
- `sort` (optional): date, alphabetical, modified
- `direction` (optional): asc or desc
- `per_page` (optional, max 100): default 25
- `page` (optional): default 1

**Example LLM prompt**: "List my recent Vimeo videos"

---

### vimeo_get_video

**What it does**: Gets details of a specific video including title, description, duration, and embed code.

**When to use**: Retrieve video metadata or generate embed links.

**Arguments**:
- `video_id` (required): Vimeo video ID

**Example LLM prompt**: "Get the embed code for video 123456789"

---

### vimeo_upload_video

**What it does**: Gets an upload ticket and URL for uploading a video to Vimeo.

**When to use**: Programmatically upload video content.

**Arguments**:
- `name` (optional): Video title
- `description` (optional): Video description
- `privacy` (optional): Privacy setting: anybody, nobody, contacts (default: anybody)
- `locale` (optional): Video locale/language code

**Example LLM prompt**: "Get an upload URL for a new video titled 'My Video'"

---

### vimeo_delete_video

**What it does**: Deletes a video from Vimeo. This action is irreversible.

**When to use**: Remove unwanted video content.

**Arguments**:
- `video_id` (required): Vimeo video ID to delete

**Example LLM prompt**: "Delete video 123456789 from Vimeo"

---

### vimeo_list_users

**What it does**: Lists users that the authenticated user follows or who follow them.

**When to use**: See connections and social network.

**Arguments**:
- `filter` (optional): following or followers (default: following)
- `per_page` (optional, max 100): default 25
- `page` (optional): default 1

**Example LLM prompt**: "List all users I follow on Vimeo"

---

### vimeo_get_user

**What it does**: Gets profile information for a specific user including name, bio, location, and stats.

**When to use**: Get context about a user before interacting.

**Arguments**:
- `user_id` (optional, default 'me'): Vimeo user ID or 'me' for authenticated user

**Example LLM prompt**: "Get profile info for user abc123"

---

### vimeo_list_albums

**What it does**: Lists albums (collections) for the authenticated user or a specific user. Albums group videos together for organization and sharing.

**When to use**: Browse album collections.

**Arguments**:
- `user_id` (optional, default 'me'): User ID
- `per_page` (optional, max 100): default 25
- `page` (optional): default 1

**Example LLM prompt**: "List all my Vimeo albums"

---

### vimeo_get_album

**What it does**: Gets details of a specific album including name, description, and video count.

**When to use**: Retrieve album metadata or get videos within an album.

**Arguments**:
- `album_id` (required): Vimeo album ID

**Example LLM prompt**: "Get details for album abc123"

---

### vimeo_list_groups

**What it does**: Lists groups that the authenticated user belongs to. Groups are communities around shared interests or topics.

**When to use**: Find groups you participate in or discover video communities.

**Arguments**:
- `per_page` (optional, max 100): default 25
- `page` (optional): default 1

**Example LLM prompt**: "List all groups I'm a member of"

---

### vimeo_get_group

**What it does**: Gets details of a specific group including name, description, member count, and video count.

**When to use**: Learn about a group before joining or browsing content.

**Arguments**:
- `group_id` (required): Vimeo group ID

**Example LLM prompt**: "Get details for group abc123"

---

## Vimeo API Reference

These tools use the Vimeo API. See official docs for full details:
- https://developer.vimeo.com/api
- Rate limits: 5,000 requests per hour for authenticated users
- Pagination: Use `page` and `per_page` parameters
- All dates: ISO 8601 format
