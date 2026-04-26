# TikTok Tools

Provider: `tiktok` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the TikTok API. They allow AI agents to manage videos, browse user profiles, view engagement stats, and interact with content through comments. TikTok is the leading short-form video platform.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with TikTok
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `user.info.basic`, `video.read`, `video.upload`, `video.comment.read`, `video.comment.write`, `video.analytics`, `user.analytics`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `tiktok_list_videos` | List videos for authenticated user | GET | /v2/user/book/list/ |
| `tiktok_get_video` | Get video details by ID | GET | /v2/video/get/ |
| `tiktok_upload_video` | Upload a video (if permitted) | POST | /v2/video/upload/ |
| `tiktok_get_user_info` | Get user profile information | GET | /v2/user/info/ |
| `tiktok_search_users` | Search for users by keyword | GET | /v2/user/search/ |
| `tiktok_list_user_videos` | List videos for a specific user | GET | /v2/user/book/list/ |
| `tiktok_get_video_comments` | Get comments on a video | GET | /v2/video/comment/list/ |
| `tiktok_post_comment` | Post a comment on a video | POST | /v2/video/comment/ |
| `tiktok_get_video_analytics` | Get analytics for a video | GET | /v2/video/stats/ |
| `tiktok_get_user_analytics` | Get analytics for a user | GET | /v2/user/stats/ |

---

## Tool Details

### tiktok_list_videos

**What it does**: Lists videos for the authenticated user. Returns video list with IDs, titles, and metadata.

**When to use**: Browse your video content or find specific posts.

**Arguments**:
- `username` (optional): Username to list videos for (omit for authenticated user)
- `max_count` (optional): Maximum number of results (max 20, default 10)

**Example LLM prompt**: "List my TikTok videos"

---

### tiktok_get_video

**What it does**: Gets details of a specific video by its ID. Returns video description, statistics, and creation info.

**When to use**: Retrieve information about a specific video post.

**Arguments**:
- `video_id` (required): Video ID

**Example LLM prompt**: "Get details for video 1234567890123456789"

---

### tiktok_upload_video

**What it does**: Uploads a video to TikTok (if your app has permission). Requires Chunked Upload API.

**When to use**: Share video content, automate video posting.

**Arguments**:
- `video_url` (required): URL of the video to upload
- `title` (required): Video title
- `description` (optional): Video description

**Example LLM prompt**: "Upload a video to TikTok titled 'My First TikTok' with video from https://example.com/video.mp4"

**Note**: TikTok restricts video upload to approved partners only.

---

### tiktok_get_user_info

**What it does**: Gets user profile information including username, bio, follower count, and video count.

**When to use**: Retrieve account details or view another user's profile.

**Arguments**:
- `username` (optional): Username (omit for authenticated user)

**Example LLM prompt**: "Get my TikTok profile info"

---

### tiktok_search_users

**What it does**: Searches for TikTok users by keyword. Returns user profiles matching the search query.

**When to use**: Find creators, brands, or specific accounts.

**Arguments**:
- `keyword` (required): Search keyword
- `max_count` (optional): Maximum results (max 20, default 10)

**Example LLM prompt**: "Search for users with keyword 'cooking'"

---

### tiktok_list_user_videos

**What it does**: Lists videos for a specific TikTok user by their username or Open ID.

**When to use**: Browse content from a specific creator.

**Arguments**:
- `username` (required): Username to list videos for
- `open_id` (optional): User's Open ID
- `max_count` (optional): Maximum results (max 20, default 10)

**Example LLM prompt**: "List videos from user cookingwithmike"

---

### tiktok_get_video_comments

**What it does**: Gets comments on a specific video. Returns comment text, author info, and reply counts.

**When to use**: Read discussions and feedback on videos.

**Arguments**:
- `video_id` (required): Video ID
- `max_count` (optional): Maximum results (max 20, default 10)

**Example LLM prompt**: "List comments on video 1234567890123456789"

---

### tiktok_post_comment

**What it does**: Posts a comment on a video. Include the video ID and comment text.

**When to use**: Engage with content or respond to comments.

**Arguments**:
- `video_id` (required): Video ID to comment on
- `comment_text` (required): Comment text

**Example LLM prompt**: "Post a comment on video 1234567890123456789 saying 'Great video!'"

---

### tiktok_get_video_analytics

**What it does**: Gets analytics for a specific video including views, likes, shares, and engagement metrics.

**When to use**: Track performance of your video content.

**Arguments**:
- `video_id` (required): Video ID

**Example LLM prompt**: "Get analytics for video 1234567890123456789"

---

### tiktok_get_user_analytics

**What it does**: Gets analytics for the authenticated user's account including follower growth and profile views.

**When to use**: Track overall account performance and audience insights.

**Arguments**: None (for authenticated user)

**Example LLM prompt**: "Get my TikTok analytics summary"

---

## TikTok API Notes

- **Rate Limits**: Varies by endpoint and user tier
- **Video Upload**: Videos must be accessible via URL for upload
- **Musical Rights**: Videos may include copyrighted music
- **Creator Analytics**: Detailed analytics require creator account
- **Content Policy**: TikTok has strict content guidelines
- **API Access**: Many endpoints require ByteDance approval
