# Bilibili Tools

Provider: `bilibili` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Bilibili API. They allow AI agents to interact with videos, users, subtitles, and favorites on Bilibili, China's leading video sharing platform.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Bilibili
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `video_read`, `video_write`, `user_info`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `bilibili_list_videos` | List user's videos | GET | /video/get_my_videos |
| `bilibili_get_video` | Get video details | GET | /video/{avid} |
| `bilibili_search_videos` | Search for videos | GET | /video/search |
| `bilibili_list_users` | List users | GET | /user/search |
| `bilibili_get_user` | Get user profile | GET | /user/{mid} |
| `bilibili_list_subtitles` | List video subtitles | GET | /video/{avid}/subtitles |
| `bilibili_update_subtitle` | Update subtitle status | PUT | /video/{avid}/subtitles/{subtitle_id} |
| `bilibili_list_favorites` | List favorite videos | GET | /favorite/get |
| `bilibili_get_favorite_folder` | Get favorite folder details | GET | /favorite/{folder_id} |
| `bilibili_get_video_stats` | Get video statistics | GET | /video/{avid}/stats |

---

## Tool Details

### bilibili_list_videos

**What it does**: Returns a paginated list of videos the authenticated user has uploaded with title, description, and view count.

**When to use**: Browse user's uploaded content, find videos to share or analyze.

**Arguments**:
- `page` (optional): Page number (default 1)
- `page_size` (optional): Videos per page (default 30, max 50)

**Example LLM prompt**: "List all videos I've uploaded to Bilibili"

---

### bilibili_get_video

**What it does**: Gets detailed information about a specific video including title, description, duration, view count, and metadata.

**When to use**: Get full video details before sharing or analyzing content.

**Arguments**:
- `avid` (required): Video av ID

**Example LLM prompt**: "Get details for video av123456789"

---

### bilibili_search_videos

**What it does**: Searches for videos by keyword query, returning matching videos with title, author, and statistics.

**When to use**: Find specific videos, discover content, or search by keyword.

**Arguments**:
- `keyword` (required): Search keyword
- `page` (optional): Page number (default 1)
- `page_size` (optional): Videos per page (default 30, max 50)

**Example LLM prompt**: "Search for videos about 'gaming'"

---

### bilibili_list_users

**What it does**: Searches for users by keyword or lists followed users. Returns user profiles with follower counts.

**When to use**: Find creators, influencers, or specific Bilibili users.

**Arguments**:
- `keyword` (required): Search keyword
- `page` (optional): Page number (default 1)
- `page_size` (optional): Users per page (default 30)

**Example LLM prompt**: "Search for users with 'gaming' in their name"

---

### bilibili_get_user

**What it does**: Gets detailed user profile including nickname, avatar, follower count, and bio.

**When to use**: Get user profile information for social analysis or engagement.

**Arguments**:
- `mid` (required): User mid

**Example LLM prompt**: "Get profile info for user mid 12345678"

---

### bilibili_list_subtitles

**What it does**: Lists subtitles for a video with language, timing, and track information.

**When to use**: Check available subtitle languages, find subtitle tracks for a video.

**Arguments**:
- `avid` (required): Video av ID

**Example LLM prompt**: "List subtitles for video av123456789"

---

### bilibili_update_subtitle

**What it does**: Updates subtitle status or settings for a video, including default track selection and visibility.

**When to use**: Set default subtitle language, enable or disable subtitle tracks.

**Arguments**:
- `avid` (required): Video av ID
- `subtitle_id` (required): Subtitle ID
- `is_default` (optional): Set as default subtitle (default false)
- `status` (optional): Subtitle status (active, disabled)

**Example LLM prompt**: "Set English subtitle as default for video av123456789"

---

### bilibili_list_favorites

**What it does**: Lists videos in the user's favorite folder with collection info.

**When to use**: See what videos a user has bookmarked, analyze interests.

**Arguments**:
- `page` (optional): Page number (default 1)
- `page_size` (optional): Videos per page (default 30)

**Example LLM prompt**: "Show my favorite videos on Bilibili"

---

### bilibili_get_favorite_folder

**What it does**: Gets detailed information about a favorite folder including name, description, and video count.

**When to use**: View favorite folder contents, manage video collections.

**Arguments**:
- `folder_id` (required): Folder ID

**Example LLM prompt**: "Get details for favorite folder abc123"

---

### bilibili_get_video_stats

**What it does**: Gets detailed statistics for a video including view count, likes, coins, favorites, and shares.

**When to use**: Analyze video performance, track engagement metrics.

**Arguments**:
- `avid` (required): Video av ID

**Example LLM prompt**: "Get statistics for video av123456789"

---

## Bilibili API Notes

- **Rate Limits**: Varies by endpoint and user tier
- **Video IDs**: Bilibili uses av numbers as identifiers (e.g., av123456789)
- **Subtitles**: Videos can have multiple subtitle tracks in different languages
- **User IDs**: Unique mid values for each user account
- **Favorites**: Users can organize favorites into multiple folders