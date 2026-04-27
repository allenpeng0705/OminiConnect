# TikTok Personal Tools

Provider: `tiktok-personal` | Engine: `nango` | Auth: OAuth2 via Nango

## Overview

TikTok API for personal use to access videos and user info. **Requires oauth2 via nango.**

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with TikTok Personal
- Token stored in Nango, accessed via `connection_ref`
- Scopes: user.info.basic, video.list

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `tiktok_get_user_info` | Get basic user information | GET | /v2/user/info/ |
| `tiktok_list_videos` | List user videos | GET | /v2/video/list/ |
| `tiktok_get_video` | Get video details | GET | /v2/video/ |
| `tiktok_create_video` | Upload a video | POST | /v2/video/upload/ |
| `tiktok_list_comments` | List comments on a video | GET | /v2/comment/list/ |
| `tiktok_post_comment` | Post a comment on a video | POST | /v2/comment/ |
| `tiktok_like_video` | Like a video | POST | /v2/video/like/ |
| `tiktok_list_followers` | List user followers | GET | /v2/user/followers/ |
| `tiktok_list_followings` | List users being followed | GET | /v2/user/followings/ |
| `tiktok_search_content` | Search for content | GET | /v2/search/ |

---

## Tool Details

### tiktok_get_user_info

**What it does**: Get basic user information

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tiktok_list_videos

**What it does**: List user videos

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tiktok_get_video

**What it does**: Get video details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tiktok_create_video

**What it does**: Upload a video

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tiktok_list_comments

**What it does**: List comments on a video

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tiktok_post_comment

**What it does**: Post a comment on a video

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tiktok_like_video

**What it does**: Like a video

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tiktok_list_followers

**What it does**: List user followers

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tiktok_list_followings

**What it does**: List users being followed

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tiktok_search_content

**What it does**: Search for content

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---


## API Notes

- Base URL: `https://open.tiktokapis.com`
- Docs: https://nango.dev/docs/integrations/all/tiktok-personal
