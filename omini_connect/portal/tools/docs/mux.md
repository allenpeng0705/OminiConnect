# Mux Tools

Provider: `mux` | Engine: `nango` | Auth: OAuth (via Nango)

## Overview

These tools wrap the Mux Video API. They allow AI agents to manage videos, assets, livestreams, and uploads on behalf of the authenticated user.

## Authentication

**Nango (recommended for OAuth)**:
- User authenticates via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `videos:read`, `videos:write`, `livestreams:read`, `livestreams:write`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `mux_list_videos` | List videos in your Mux account | GET | /videos |
| `mux_get_video` | Get details of a specific video | GET | /videos/{video_id} |
| `mux_create_video` | Create a new video asset | POST | /videos |
| `mux_delete_video` | Delete a video | DELETE | /videos/{video_id} |
| `mux_list_assets` | List all assets | GET | /assets |
| `mux_get_asset` | Get details of a specific asset | GET | /assets/{asset_id} |
| `mux_list_livestreams` | List livestreams | GET | /livestreams |
| `mux_get_livestream` | Get details of a specific livestream | GET | /livestreams/{livestream_id} |
| `mux_create_upload` | Create a direct upload URL | POST | /uploads |
| `mux_get_upload` | Get status of an upload | GET | /uploads/{upload_id} |

---

## Tool Details

### mux_list_videos

**What it does**: Returns a paginated list of all videos in your Mux account with ID, title, status, and metadata.

**When to use**: Browse your video library to find videos before getting details or checking status.

**Arguments**:
- `limit` (optional, max 100): default 25
- `page` (optional): default 1

**Example LLM prompt**: "List all my Mux videos"

---

### mux_get_video

**What it does**: Gets details of a specific video including title, duration, status, and playback URLs.

**When to use**: Check video processing status, get playback URLs, or view video metadata.

**Arguments**:
- `video_id` (required): Mux video ID

**Example LLM prompt**: "What's the status of video abc123?"

---

### mux_create_video

**What it does**: Creates a new video asset in Mux. Returns an upload URL for direct file upload.

**When to use**: Programmatically create video content and get an upload URL for the user.

**Arguments**:
- `title` (optional): Video title
- `description` (optional): Video description
- `playback_policy` (optional): Array containing "public" or "signed" (default: public)
- `cors_origin` (optional): CORS origin domain for signed playback

**Example LLM prompt**: "Create a new video asset titled 'Product Demo'"

---

### mux_delete_video

**What it does**: Deletes a video and all associated assets from Mux. This action is irreversible.

**When to use**: Remove unwanted video content from your Mux account.

**Arguments**:
- `video_id` (required): Mux video ID to delete

**Example LLM prompt**: "Delete video abc123 from my Mux account"

---

### mux_list_assets

**What it does**: Lists all assets in your Mux account. Assets are the underlying media files that power videos and livestreams.

**When to use**: Browse your asset library to find playable videos.

**Arguments**:
- `limit` (optional, max 100): default 25
- `page` (optional): default 1

**Example LLM prompt**: "List all Mux assets in my account"

---

### mux_get_asset

**What it does**: Gets details of a specific asset including status, duration, and playback IDs.

**When to use**: Check asset processing status or retrieve playback information.

**Arguments**:
- `asset_id` (required): Mux asset ID

**Example LLM prompt**: "Show me the playback IDs for asset xyz789"

---

### mux_list_livestreams

**What it does**: Lists all livestreams in your Mux account. Returns stream keys, status, and configuration.

**When to use**: Monitor active streams or find streams to view.

**Arguments**:
- `limit` (optional, max 100): default 25
- `page` (optional): default 1

**Example LLM prompt**: "Show me all my livestreams"

---

### mux_get_livestream

**What it does**: Gets details of a specific livestream including stream key, status, and playback URL.

**When to use**: Monitor stream health or get embed URLs.

**Arguments**:
- `livestream_id` (required): Mux livestream ID

**Example LLM prompt**: "Get the status and playback URL for livestream abc123"

---

### mux_create_upload

**What it does**: Creates a direct upload URL for Mux. Returns an upload URL and ID for tracking.

**When to use**: Enable direct file uploads without going through Mux's UI.

**Arguments**:
- `title` (optional): Upload title
- `description` (optional): Upload description
- `cors_origin` (optional): CORS origin for the upload
- `new_asset_settings` (optional): Asset settings including playback_policy
- `test` (optional): Create a test upload (won't count against limits)

**Example LLM prompt**: "Create an upload URL for a new video with public playback"

---

### mux_get_upload

**What it does**: Gets the status of an upload including progress, asset ID, and any errors.

**When to use**: Track upload completion and get the resulting video ID.

**Arguments**:
- `upload_id` (required): Mux upload ID

**Example LLM prompt**: "Check the status of upload xyz789"

---

## Mux API Reference

These tools use the Mux Video API. See official docs for full details:
- https://docs.mux.com
- Rate limits: Vary by endpoint and plan
- Pagination: Use `limit` and `page` parameters
- All dates: ISO 8601 format
