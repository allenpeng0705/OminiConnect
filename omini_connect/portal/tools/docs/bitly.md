# Bitly Tools

Provider: `bitly` | Engine: `nango` | Auth: OAuth2 via Nango

## Overview

These tools wrap the Bitly API. They allow AI agents to create, manage, and track short links. Bitly is a link management platform that provides URL shortening, analytics, and branded links.

## Authentication

**Nango OAuth2**:
- User authenticates via Bitly OAuth
- Token stored in Nango, accessed via `connection_ref`
- Scopes for link management and analytics

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `bitly_get_user` | Get user info | GET | /v4/user |
| `bitly_list_groups` | List groups | GET | /v4/groups |
| `bitly_get_group` | Get group details | GET | /v4/groups/{groupGuid} |
| `bitly_list_links` | List links | GET | /v4/links |
| `bitly_get_link` | Get link details | GET | /v4/links/{linkGuid} |
| `bitly_create_link` | Create a short link | POST | /v4/links |
| `bitly_update_link` | Update a link | PATCH | /v4/links/{linkGuid} |
| `bitly_delete_link` | Delete a link | DELETE | /v4/links/{linkGuid} |
| `bitly_list_bitlinks` | List bitlinks | GET | /v4/search |
| `bitly_get_metrics` | Get link metrics | GET | /v4/links/{linkGuid}/clicks |

---

## Tool Details

### bitly_get_user

**What it does**: Gets current user information.

**When to use**: Verify account, get user preferences.

**Arguments**: None required

**Example LLM prompt**: "Get my Bitly user info"

---

### bitly_list_groups

**What it does**: Lists all groups in the Bitly account.

**When to use**: Organize links, manage team access.

**Arguments**: None required

**Example LLM prompt**: "List all groups in Bitly"

---

### bitly_get_group

**What it does**: Gets details for a specific group.

**When to use**: Check group settings, view group links.

**Arguments**:
- `groupGuid` (required): Group GUID

**Example LLM prompt**: "Get details for group abc-123"

---

### bitly_list_links

**What it does**: Lists all links for the user or group.

**When to use**: Browse created links, find specific URLs.

**Arguments**:
- `groupGuid` (optional): Filter by group
- `page` (optional): Page number (default 1)
- `size` (optional): Links per page (default 20)

**Example LLM prompt**: "List all links in my account"

---

### bitly_get_link

**What it does**: Gets details for a specific link.

**When to use**: Check link info, view creation date.

**Arguments**:
- `linkGuid` (required): Link GUID or short URL

**Example LLM prompt**: "Get details for bit.ly/abc123"

---

### bitly_create_link

**What it does**: Creates a new short link.

**When to use**: Shorten URLs for sharing, track clicks.

**Arguments**:
- `longUrl` (required): URL to shorten
- `groupGuid` (optional): Group to create link in
- `title` (optional): Link title

**Example LLM prompt**: "Create a short link for https://example.com/very-long-url"

---

### bitly_update_link

**What it does**: Updates a link's properties.

**When to use**: Change link title, archive old links.

**Arguments**:
- `linkGuid` (required): Link GUID
- `title` (optional): New title
- `archived` (optional): Archive status

**Example LLM prompt**: "Update link bit.ly/abc123 with title 'My Link'"

---

### bitly_delete_link

**What it does**: Deletes a link from Bitly.

**When to use**: Remove unwanted links, clean up.

**Arguments**:
- `linkGuid` (required): Link GUID

**Example LLM prompt**: "Delete link bit.ly/abc123"

---

### bitly_list_bitlinks

**What it does**: Searches for bitlinks by query.

**When to use**: Find specific links, search by title or URL.

**Arguments**:
- `query` (required): Search query
- `groupGuid` (optional): Filter by group
- `page` (optional): Page number (default 1)
- `size` (optional): Results per page (default 20)

**Example LLM prompt**: "Search for links containing 'example'"

---

### bitly_get_metrics

**What it does**: Gets click metrics for a link.

**When to use**: Track link performance, analyze traffic.

**Arguments**:
- `linkGuid` (required): Link GUID or short URL
- `unit` (optional): Time unit (day, week, month)
- `units` (optional): Number of units (default 7)

**Example LLM prompt**: "Get click metrics for bit.ly/abc123 for the last 30 days"

---

## Bitly API Notes

- **Link GUID**: Unique identifier for each link
- **Groups**: Organize links by team, project, or campaign
- **Short URL Format**: bit.ly/xxxxx or custom branded domain
- **Metrics**: Include total clicks, referrers, geographies
- **Rate Limits**: API has rate limiting; implement backoff
