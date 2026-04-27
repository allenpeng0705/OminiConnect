# Vimeo (Basic) Tools

Provider: `vimeo-basic` | Engine: `nango` | Auth: Basic Auth via Nango

## Overview

Vimeo is a video hosting and streaming platform. **Requires basic auth via nango.**

## Authentication

**Nango Basic Auth**:
- User provides username/password
- Credentials stored securely in Nango

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `vimeo_get_user` | Get authenticated user info | GET | /users/me |
| `vimeo_list_videos` | List all videos | GET | /users/me/videos |
| `vimeo_get_video` | Get video details | GET | /videos/{video_id} |
| `vimeo_upload_video` | Upload a new video | POST | /me/videos |
| `vimeo_update_video` | Update video metadata | PATCH | /videos/{video_id} |
| `vimeo_delete_video` | Delete a video | DELETE | /videos/{video_id} |
| `vimeo_list_albums` | List all albums | GET | /users/me/albums |
| `vimeo_get_album` | Get album details | GET | /albums/{album_id} |
| `vimeo_create_album` | Create a new album | POST | /me/albums |
| `vimeo_get_likes` | Get liked videos | GET | /users/me/likes |

---

## Tool Details

### vimeo_get_user

**What it does**: Get authenticated user info

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### vimeo_list_videos

**What it does**: List all videos

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### vimeo_get_video

**What it does**: Get video details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### vimeo_upload_video

**What it does**: Upload a new video

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### vimeo_update_video

**What it does**: Update video metadata

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### vimeo_delete_video

**What it does**: Delete a video

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### vimeo_list_albums

**What it does**: List all albums

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### vimeo_get_album

**What it does**: Get album details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### vimeo_create_album

**What it does**: Create a new album

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### vimeo_get_likes

**What it does**: Get liked videos

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---


## API Notes

- Base URL: `https://api.vimeo.com`
- Docs: https://nango.dev/docs/integrations/all/vimeo-basic
