# Spotify (Client Credentials) Tools

Provider: `spotify-oauth2-cc` | Engine: `nango` | Auth: OAUTH2_CC

## Overview

These tools wrap the Spotify (Client Credentials) API. They allow AI agents to interact with Spotify (Client Credentials) functionality. **Requires OAUTH2_CC authentication.**

## Authentication

**OAuth2 Client Credentials**:
- Uses client_id and client_secret for machine-to-machine auth
- Nango manages token refresh automatically
- Scopes depend on application permissions

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `get_user_profile` | Get current user profile | GET | /me |
| `list_playlists` | List user's playlists | GET | /me/playlists |
| `get_playlist` | Get playlist details | GET | /playlists/{id} |
| `list_tracks` | List tracks in playlist | GET | /playlists/{id}/tracks |
| `get_artist` | Get artist details | GET | /artists/{id} |
| `get_album` | Get album details | GET | /albums/{id} |
| `search` | Search for tracks/artists/playlists | GET | /search |
| `get_recommendations` | Get track recommendations | GET | /recommendations |
| `get_audio_features` | Get audio features for track | GET | /audio-features/{id} |
| `list_followed` | List followed artists | GET | /me/following |

---

## Tool Details

### get_user_profile

**What it does**: Get current user profile

**When to use**: Use this tool when you need to get current user profile.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get user profile to..."

---

### list_playlists

**What it does**: List user's playlists

**When to use**: Use this tool when you need to list user's playlists.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list playlists to..."

---

### get_playlist

**What it does**: Get playlist details

**When to use**: Use this tool when you need to get playlist details.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get playlist to..."

---

### list_tracks

**What it does**: List tracks in playlist

**When to use**: Use this tool when you need to list tracks in playlist.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list tracks to..."

---

### get_artist

**What it does**: Get artist details

**When to use**: Use this tool when you need to get artist details.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get artist to..."

---

### get_album

**What it does**: Get album details

**When to use**: Use this tool when you need to get album details.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get album to..."

---

### search

**What it does**: Search for tracks/artists/playlists

**When to use**: Use this tool when you need to search for tracks/artists/playlists.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use search to..."

---

### get_recommendations

**What it does**: Get track recommendations

**When to use**: Use this tool when you need to get track recommendations.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get recommendations to..."

---

### get_audio_features

**What it does**: Get audio features for track

**When to use**: Use this tool when you need to get audio features for track.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get audio features to..."

---

### list_followed

**What it does**: List followed artists

**When to use**: Use this tool when you need to list followed artists.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list followed artists to..."

---

## Spotify (Client Credentials) API Notes

- **Auth mode**: OAUTH2_CC
- **Base URL**: https://api.spotify.com
- **API prefix**: /v1
- **Rate limits**: Check provider documentation for specific limits
