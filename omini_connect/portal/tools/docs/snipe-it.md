# Snipe-IT Tools

Provider: `snipe-it` | Engine: `nango` | Auth: API_KEY

## Overview

These tools wrap the Snipe-IT API. They allow AI agents to interact with Snipe-IT functionality. **Requires API_KEY authentication.**

## Authentication

**API Key Authentication**:
- User provides their API key directly
- Key is passed via header or query parameter
- Scopes depend on the specific API key permissions

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `list_assets` | List all assets | GET | /assets |
| `get_asset` | Get asset details | GET | /assets/{id} |
| `create_asset` | Create a new asset | POST | /assets |
| `update_asset` | Update asset details | PUT | /assets/{id} |
| `list_users` | List all users | GET | /users |
| `get_user` | Get user details | GET | /users/{id} |
| `list_locations` | List all locations | GET | /locations |
| `get_location` | Get location details | GET | /locations/{id} |
| `list_manufacturers` | List manufacturers | GET | /manufacturers |
| `list_categories` | List asset categories | GET | /categories |

---

## Tool Details

### list_assets

**What it does**: List all assets

**When to use**: Use this tool when you need to list all assets.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list assets to..."

---

### get_asset

**What it does**: Get asset details

**When to use**: Use this tool when you need to get asset details.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get asset to..."

---

### create_asset

**What it does**: Create a new asset

**When to use**: Use this tool when you need to create a new asset.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use create asset to..."

---

### update_asset

**What it does**: Update asset details

**When to use**: Use this tool when you need to update asset details.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use update asset to..."

---

### list_users

**What it does**: List all users

**When to use**: Use this tool when you need to list all users.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list users to..."

---

### get_user

**What it does**: Get user details

**When to use**: Use this tool when you need to get user details.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get user to..."

---

### list_locations

**What it does**: List all locations

**When to use**: Use this tool when you need to list all locations.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list locations to..."

---

### get_location

**What it does**: Get location details

**When to use**: Use this tool when you need to get location details.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get location to..."

---

### list_manufacturers

**What it does**: List manufacturers

**When to use**: Use this tool when you need to list manufacturers.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list manufacturers to..."

---

### list_categories

**What it does**: List asset categories

**When to use**: Use this tool when you need to list asset categories.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list categories to..."

---

## Snipe-IT API Notes

- **Auth mode**: API_KEY
- **Base URL**: https://{hostname}/api
- **API prefix**: /
- **Rate limits**: Check provider documentation for specific limits
