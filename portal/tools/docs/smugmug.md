# Smugmug Tools

Provider: `smugmug` | Engine: `nango` | Auth: OAUTH1

## Overview

These tools wrap the Smugmug API. They allow AI agents to interact with Smugmug functionality. **Requires OAUTH1 authentication.**

## Authentication

**OAuth1 Authentication**:
- Uses OAuth 1.0a for authentication
- Nango manages OAuth1 signature generation
- Scopes depend on the account permissions

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `list_albums` | List all albums | GET | /albums |
| `get_album` | Get album details | GET | /albums/{id} |
| `list_images` | List images in album | GET | /albums/{id}/images |
| `get_image` | Get image details | GET | /images/{id} |
| `upload_image` | Upload an image | POST | /albums/{id}/images |
| `get_user` | Get user profile | GET | /user/{id} |
| `list_folders` | List folders | GET | /folders/{id} |
| `create_album` | Create an album | POST | /albums |
| `update_album` | Update album settings | PUT | /albums/{id} |
| `delete_image` | Delete an image | DELETE | /images/{id} |

---

## Tool Details

### list_albums

**What it does**: List all albums

**When to use**: Use this tool when you need to list all albums.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list albums to..."

---

### get_album

**What it does**: Get album details

**When to use**: Use this tool when you need to get album details.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get album to..."

---

### list_images

**What it does**: List images in album

**When to use**: Use this tool when you need to list images in album.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list images to..."

---

### get_image

**What it does**: Get image details

**When to use**: Use this tool when you need to get image details.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get image to..."

---

### upload_image

**What it does**: Upload an image

**When to use**: Use this tool when you need to upload an image.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use upload image to..."

---

### get_user

**What it does**: Get user profile

**When to use**: Use this tool when you need to get user profile.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get user to..."

---

### list_folders

**What it does**: List folders

**When to use**: Use this tool when you need to list folders.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list folders to..."

---

### create_album

**What it does**: Create an album

**When to use**: Use this tool when you need to create an album.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use create album to..."

---

### update_album

**What it does**: Update album settings

**When to use**: Use this tool when you need to update album settings.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use update album to..."

---

### delete_image

**What it does**: Delete an image

**When to use**: Use this tool when you need to delete an image.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use delete image to..."

---

## Smugmug API Notes

- **Auth mode**: OAUTH1
- **Base URL**: https://www.smugmug.com
- **API prefix**: /
- **Rate limits**: Check provider documentation for specific limits
