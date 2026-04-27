# HeyGen Tools

Provider: `heygen` | Engine: `nango` | Auth: OAuth2 via Nango

## Overview

These tools wrap the HeyGen API. They allow AI agents to manage videos, avatars, voices, and templates. HeyGen is an AI video generation platform for creating personalized videos at scale.

## Authentication

**Nango OAuth2**:
- User authenticates via Nango Connect with HeyGen
- Token stored in Nango, accessed via `connection_ref`
- Authorization URL: https://app.heygen.com/oauth/authorize
- Token URL: https://api2.heygen.com/v1/oauth/token
- Refresh URL: https://api2.heygen.com/v1/oauth/refresh_token

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `heygen_list_videos` | List videos | GET | /v1/videos |
| `heygen_get_video` | Get video details | GET | /v1/videos/{video_id} |
| `heygen_create_video` | Create a video | POST | /v1/video |
| `heygen_list_avatars` | List avatars | GET | /v1/avatars |
| `heygen_get_avatar` | Get avatar details | GET | /v1/avatars/{avatar_id} |
| `heygen_list_voices` | List voices | GET | /v1/voices |
| `heygen_get_voice` | Get voice details | GET | /v1/voices/{voice_id} |
| `heygen_list_templates` | List templates | GET | /v1/templates |
| `heygen_get_template` | Get template details | GET | /v1/templates/{template_id} |
| `heygen_list_teams` | List teams | GET | /v1/teams |

---

## Tool Details

### heygen_list_videos

**What it does**: Lists all videos in your HeyGen account.

**When to use**: Browse generated videos.

**Arguments**:
- `page` (optional): Page number (default 1)
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List all videos"

---

### heygen_get_video

**What it does**: Gets detailed information about a specific video.

**When to use**: Get video URL and status.

**Arguments**:
- `video_id` (required): Video ID

**Example LLM prompt**: "Get video with ID abc123"

---

### heygen_create_video

**What it does**: Creates a new video using HeyGen.

**When to use**: Generate a new AI video.

**Arguments**:
- `template_id` (required): Template ID
- `script` (optional): Video script

**Example LLM prompt**: "Create a video using template abc123"

---

### heygen_list_avatars

**What it does**: Lists all available avatars in HeyGen.

**When to use**: Browse AI avatars for videos.

**Arguments**: None

**Example LLM prompt**: "List all avatars"

---

### heygen_get_avatar

**What it does**: Gets detailed information about a specific avatar.

**When to use**: View avatar preview and settings.

**Arguments**:
- `avatar_id` (required): Avatar ID

**Example LLM prompt**: "Get avatar with ID def456"

---

### heygen_list_voices

**What it does**: Lists all available voices in HeyGen.

**When to use**: Browse AI voices for videos.

**Arguments**: None

**Example LLM prompt**: "List all voices"

---

### heygen_get_voice

**What it does**: Gets detailed information about a specific voice.

**When to use**: View voice preview and settings.

**Arguments**:
- `voice_id` (required): Voice ID

**Example LLM prompt**: "Get voice with ID ghi789"

---

### heygen_list_templates

**What it does**: Lists all video templates in HeyGen.

**When to use**: Browse templates for video creation.

**Arguments**: None

**Example LLM prompt**: "List all templates"

---

### heygen_get_template

**What it does**: Gets detailed information about a specific template.

**When to use**: View template structure.

**Arguments**:
- `template_id` (required): Template ID

**Example LLM prompt**: "Get template with ID jkl012"

---

### heygen_list_teams

**What it does**: Lists all teams in your HeyGen account.

**When to use**: View team structure and members.

**Arguments**: None

**Example LLM prompt**: "List all teams"

---

## HeyGen API Notes

- **API Base URL**: https://api2.heygen.com
- **Auth Mode**: OAuth2
- **Videos**: AI-generated videos with avatars
- **Avatars**: AI virtual presenters
- **Voices**: Text-to-speech voice options
- **Templates**: Video templates for consistent branding
- **Teams**: Workspace organization for teams
