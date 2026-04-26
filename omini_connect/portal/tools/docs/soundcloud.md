# SoundCloud Tools

Provider: `soundcloud` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the SoundCloud API. They allow AI agents to interact with tracks, users, playlists, and comments on SoundCloud, a leading audio distribution platform.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with SoundCloud
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `tracks-read`, `tracks-write`, `users-read`, `playlist-read`, `playlist-write`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `soundcloud_list_tracks` | List tracks | GET | /tracks |
| `soundcloud_get_track` | Get track details | GET | /tracks/{track_id} |
| `soundcloud_upload_track` | Upload a track | POST | /tracks |
| `soundcloud_list_users` | List users | GET | /users |
| `soundcloud_get_user` | Get user profile | GET | /users/{user_id} |
| `soundcloud_list_playlists` | List playlists | GET | /playlists |
| `soundcloud_get_playlist` | Get playlist details | GET | /playlists/{playlist_id} |
| `soundcloud_list_comments` | List track comments | GET | /tracks/{track_id}/comments |
| `soundcloud_get_comment` | Get comment details | GET | /comments/{comment_id} |
| `soundcloud_search_tracks` | Search for tracks | GET | /tracks |

---

## Tool Details

### soundcloud_list_tracks

**What it does**: Lists tracks for the authenticated user or filters by a specific user. Returns track details including title, duration, and play count.

**When to use**: Browse uploaded content, list user's tracks, or analyze audio.

**Arguments**:
- `user_id` (optional): Filter by user ID
- `limit` (optional): Max tracks (default 50, max 100)
- `offset` (optional): Offset for pagination (default 0)

**Example LLM prompt**: "List all tracks by user 12345678"

---

### soundcloud_get_track

**What it does**: Gets detailed information about a specific track including title, description, duration, waveform, and statistics.

**When to use**: Get full track details before playing, downloading, or analyzing.

**Arguments**:
- `track_id` (required): Track ID

**Example LLM prompt**: "Get details for track 123456789"

---

### soundcloud_upload_track

**What it does**: Uploads a new track to SoundCloud with title and optional metadata.

**When to use**: Share new audio content, distribute music or podcasts.

**Arguments**:
- `title` (required): Track title
- `description` (optional): Track description
- `tag_list` (optional): Comma-separated tags
- `genre` (optional): Track genre
- `sharing` (optional): Sharing mode (default public)

**Example LLM prompt**: "Upload a new track called 'My New Song'"

---

### soundcloud_list_users

**What it does**: Lists users by search query or browse. Returns user profiles with follower counts.

**When to use**: Find creators, influencers, or specific SoundCloud users.

**Arguments**:
- `q` (optional): Search query
- `limit` (optional): Max users (default 50, max 100)
- `offset` (optional): Offset for pagination (default 0)

**Example LLM prompt**: "Search for users named 'DJ'"

---

### soundcloud_get_user

**What it does**: Gets detailed user profile including username, avatar, bio, and social statistics.

**When to use**: Get user profile information, analyze creator stats.

**Arguments**:
- `user_id` (required): User ID

**Example LLM prompt**: "Get profile for user 12345678"

---

### soundcloud_list_playlists

**What it does**: Lists playlists for the authenticated user or filtered by user.

**When to use**: Browse playlists, find sets, or manage collections.

**Arguments**:
- `user_id` (optional): Filter by user ID
- `limit` (optional): Max playlists (default 50, max 100)
- `offset` (optional): Offset for pagination (default 0)

**Example LLM prompt**: "List all my playlists on SoundCloud"

---

### soundcloud_get_playlist

**What it does**: Gets detailed information about a playlist including title, description, and track list.

**When to use**: Get playlist metadata and contents before playing or sharing.

**Arguments**:
- `playlist_id` (required): Playlist ID

**Example LLM prompt**: "Get details for playlist 1234567890"

---

### soundcloud_list_comments

**What it does**: Lists comments on a track with text, timestamps, and user info.

**When to use**: Read listener feedback, analyze engagement, or find discussions.

**Arguments**:
- `track_id` (required): Track ID
- `limit` (optional): Max comments (default 50, max 100)
- `offset` (optional): Offset for pagination (default 0)

**Example LLM prompt**: "Show comments on track 123456789"

---

### soundcloud_get_comment

**What it does**: Gets detailed information about a specific comment.

**When to use**: Read full comment content, view timestamp and user.

**Arguments**:
- `comment_id` (required): Comment ID

**Example LLM prompt**: "Get details for comment 987654321"

---

### soundcloud_search_tracks

**What it does**: Searches for tracks by keyword query, returning matching results with title, artist, and stats.

**When to use**: Find specific songs, discover new music, or search by keyword.

**Arguments**:
- `q` (required): Search query
- `limit` (optional): Max results (default 50, max 100)
- `offset` (optional): Offset for pagination (default 0)

**Example LLM prompt**: "Search for tracks with 'electronic' in the title"

---

## SoundCloud API Notes

- **Rate Limits**: Varies by endpoint and user tier
- **Waveform**: Tracks include waveform data for visualization
- **Comments**: Support timestamps linking to specific moments in audio
- **Upload**: Requires audio file URL or direct upload capability
- **Privacy**: Tracks can be public, private, or password-protected