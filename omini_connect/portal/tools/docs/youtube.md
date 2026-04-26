# YouTube Tools

Provider: `youtube` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the YouTube Data API. They allow AI agents to search videos, manage playlists, browse channels, and view comments. YouTube is the world's largest video platform.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Google/YouTube
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `youtube.readonly`, `youtube`, `youtube.search`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `youtube_list_videos` | List videos (search by chart or filter) | GET | /youtube/v3/videos |
| `youtube_get_video` | Get video details by ID | GET | /youtube/v3/videos |
| `youtube_search_videos` | Search for videos by keyword | GET | /youtube/v3/search |
| `youtube_list_channels` | List channels | GET | /youtube/v3/channels |
| `youtube_get_channel` | Get channel details by ID | GET | /youtube/v3/channels |
| `youtube_list_playlists` | List playlists for authenticated user | GET | /youtube/v3/playlists |
| `youtube_get_playlist` | Get playlist details by ID | GET | /youtube/v3/playlists |
| `youtube_add_video_to_playlist` | Add a video to a playlist | POST | /youtube/v3/playlistItems |
| `youtube_list_comments` | List comments on a video | GET | /youtube/v3/commentThreads |
| `youtube_get_comment` | Get comment details by ID | GET | /youtube/v3/comments |

---

## Tool Details

### youtube_list_videos

**What it does**: Lists videos from YouTube. Returns most popular videos (chart) or videos matching filters.

**When to use**: Discover trending content or browse videos by category.

**Arguments**:
- `chart` (optional): Chart type: mostPopular
- `region_code` (optional): ISO 3166-1 alpha-2 country code for regional content (default US)
- `video_category_id` (optional): Filter by video category ID
- `max_results` (optional): Maximum results (max 50, default 10)

**Example LLM prompt**: "Show me the most popular videos this week"

---

### youtube_get_video

**What it does**: Gets details of a specific video by its ID. Returns title, description, view count, likes, and metadata.

**When to use**: Get detailed information about a video.

**Arguments**:
- `id` (required): Video ID
- `part` (optional): Comma-separated list of parts: snippet,contentDetails,statistics,player (default snippet,statistics)

**Example LLM prompt**: "Get details for video dQw4w9WgXcQ"

---

### youtube_search_videos

**What it does**: Searches for videos by keyword. Returns video IDs, titles, and thumbnails matching the query.

**When to use**: Find specific content or research topics.

**Arguments**:
- `q` (required): Search query
- `type` (optional): Resource type: video, playlist, or channel (default video)
- `channel_id` (optional): Limit search to specific channel
- `max_results` (optional): Maximum results (max 50, default 10)
- `order` (optional): Sort order: date, rating, relevance, title, viewCount (default relevance)

**Example LLM prompt**: "Search for funny cat videos on YouTube"

---

### youtube_list_channels

**What it does**: Lists channels. Returns channel information for authenticated user or specific channel IDs.

**When to use**: Find channel details or manage subscriptions.

**Arguments**:
- `mine` (optional): Set to true to list authenticated user's channel (default false)
- `id` (optional): Comma-separated list of channel IDs
- `max_results` (optional): Maximum results (max 50, default 10)

**Example LLM prompt**: "List my YouTube channel info"

---

### youtube_get_channel

**What it does**: Gets details of a specific channel by its ID. Returns channel name, description, subscriber count, and branding.

**When to use**: Learn about a creator's channel.

**Arguments**:
- `id` (required): Channel ID
- `part` (optional): Comma-separated list of parts: snippet,contentDetails,statistics,brandingSettings (default snippet,statistics)

**Example LLM prompt**: "Get details for channel UC_x5XG1OV2P6uZZ5FSM9Ttw"

---

### youtube_list_playlists

**What it does**: Lists playlists for the authenticated user. Returns playlists created or curated by the user.

**When to use**: Organize videos or find saved collections.

**Arguments**:
- `channel_id` (optional): Channel ID (defaults to authenticated user)
- `mine` (optional): Set to true for authenticated user's playlists (default true)
- `max_results` (optional): Maximum results (max 50, default 10)

**Example LLM prompt**: "List all my YouTube playlists"

---

### youtube_get_playlist

**What it does**: Gets details of a specific playlist by its ID. Returns playlist title, description, and video count.

**When to use**: View playlist contents or learn about a collection.

**Arguments**:
- `id` (required): Playlist ID
- `part` (optional): Comma-separated list of parts: snippet,status (default snippet)

**Example LLM prompt**: "Get details for playlist PLrAXtmEr4O"

---

### youtube_add_video_to_playlist

**What it does**: Adds a video to a playlist. Specify the video ID and playlist ID to add an item.

**When to use**: Save videos or organize content into collections.

**Arguments**:
- `playlist_id` (required): Playlist ID
- `video_id` (required): Video ID to add
- `position` (optional): Position in playlist (0 = first)

**Example LLM prompt**: "Add video dQw4w9WgXcQ to my Favorites playlist"

---

### youtube_list_comments

**What it does**: Lists comments on a video. Returns comment threads with replies and metadata.

**When to use**: Read discussions, feedback, or community sentiment.

**Arguments**:
- `video_id` (required): Video ID to get comments for
- `max_results` (optional): Maximum results (max 100, default 20)
- `order` (optional): Sort order: time, relevance (default relevance)

**Example LLM prompt**: "List comments on video dQw4w9WgXcQ"

---

### youtube_get_comment

**What it does**: Gets details of a specific comment by its ID. Returns comment text, author, likes, and moderation status.

**When to use**: Read a specific comment in a thread.

**Arguments**:
- `id` (required): Comment ID
- `part` (optional): Part: snippet (default snippet)

**Example LLM prompt**: "Get details for comment abc123"

---

## YouTube API Notes

- **Quota**: API has daily quota limits (10,000 units per day for free tier)
- **Rate Limits**: 50 requests per second per project
- **Video Upload**: Videos must be accessible via URL for upload
- **Thumbnails**: Videos have multiple thumbnail sizes available
- **Analytics**: Detailed analytics require YouTube Analytics API (future expansion)
