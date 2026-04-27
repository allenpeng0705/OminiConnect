# Twitch Tools

Provider: `twitch` | Engine: `nango` | Auth: OAuth2 via Nango

## Overview

Twitch is a video streaming platform for live content. **Requires oauth2 via nango.**

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Twitch
- Token stored in Nango, accessed via `connection_ref`
- Scopes: user:read, user:write, channel:read, channel:write

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `twitch_get_users` | Get user information | GET | /helix/users |
| `twitch_get_follows` | Get list of users following a broadcaster | GET | /helix/follows |
| `twitch_list_videos` | List videos for a user | GET | /helix/videos |
| `twitch_get_video` | Get video details | GET | /helix/videos |
| `twitch_get_streams` | Get stream information | GET | /helix/streams |
| `twitch_get_channel_info` | Get channel information | GET | /helix/channels |
| `twitch_update_channel` | Update channel settings | PATCH | /helix/channels |
| `twitch_get_subscriptions` | Get channel subscribers | GET | /helix/subscriptions |
| `twitch_get_moderators` | Get channel moderators | GET | /helix/moderation/moderators |
| `twitch_create_clip` | Create a clip from a broadcast | POST | /helix/clips |

---

## Tool Details

### twitch_get_users

**What it does**: Get user information

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### twitch_get_follows

**What it does**: Get list of users following a broadcaster

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### twitch_list_videos

**What it does**: List videos for a user

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### twitch_get_video

**What it does**: Get video details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### twitch_get_streams

**What it does**: Get stream information

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### twitch_get_channel_info

**What it does**: Get channel information

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### twitch_update_channel

**What it does**: Update channel settings

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### twitch_get_subscriptions

**What it does**: Get channel subscribers

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### twitch_get_moderators

**What it does**: Get channel moderators

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### twitch_create_clip

**What it does**: Create a clip from a broadcast

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---


## API Notes

- Base URL: `https://api.twitch.tv`
- Docs: https://nango.dev/docs/integrations/all/twitch
