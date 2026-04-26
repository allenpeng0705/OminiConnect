# Microsoft Tools

Provider: `microsoft` | Engine: `nango` | Auth: OAuth2 via Nango

## Overview

These tools wrap the Microsoft Graph API. They allow AI agents to manage users, groups, email, calendars, and OneDrive files. **Requires Microsoft OAuth2.**

## Authentication

**Nango OAUTH2**:
- User authenticates via Nango Connect with Microsoft
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://graph.microsoft.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `microsoft_list_users` | List all users | GET | /v1.0/users |
| `microsoft_get_user` | Get user details | GET | /v1.0/users/{userId} |
| `microsoft_list_groups` | List groups | GET | /v1.0/groups |
| `microsoft_get_group` | Get group details | GET | /v1.0/groups/{groupId} |
| `microsoft_list_messages` | List messages (email) | GET | /v1.0/me/messages |
| `microsoft_get_message` | Get message details | GET | /v1.0/me/messages/{messageId} |
| `microsoft_list_calendars` | List calendars | GET | /v1.0/me/calendars |
| `microsoft_get_calendar_event` | Get calendar event | GET | /v1.0/me/events/{eventId} |
| `microsoft_list_files` | List OneDrive files | GET | /v1.0/me/drive/root/children |
| `microsoft_get_file` | Get file details | GET | /v1.0/me/drive/items/{itemId} |

---

## Tool Details

### microsoft_list_users

**What it does**: Lists all users in the organization via Microsoft Graph.

**When to use**: Find users, browse directory.

**Arguments**:
- `$filter` (optional): OData filter expression
- `$search` (optional): Search query
- `$top` (optional): Max results (default 50)
- `$select` (optional): Fields to select

**Example LLM prompt**: "List all users in the Sales department"

---

### microsoft_get_user

**What it does**: Gets detailed information about a specific user.

**When to use**: Get user profile, check user details.

**Arguments**:
- `userId` (required): User ID or email
- `$select` (optional): Fields to select

**Example LLM prompt**: "Get details for user john@example.com"

---

### microsoft_list_groups

**What it does**: Lists all groups in the organization.

**When to use**: Browse groups, find team groups.

**Arguments**:
- `$filter` (optional): OData filter expression
- `$top` (optional): Max results (default 50)
- `$select` (optional): Fields to select

**Example LLM prompt**: "List all Microsoft 365 groups"

---

### microsoft_get_group

**What it does**: Gets details of a specific group.

**When to use**: Get group members, check group settings.

**Arguments**:
- `groupId` (required): Group ID
- `$select` (optional): Fields to select

**Example LLM prompt**: "Get details for group engineering-team"

---

### microsoft_list_messages

**What it does**: Lists messages in the authenticated user's mailbox.

**When to use**: Browse emails, find messages.

**Arguments**:
- `$filter` (optional): OData filter expression
- `$top` (optional): Max results (default 50)
- `$select` (optional): Fields to select

**Example LLM prompt**: "List unread emails from today"

---

### microsoft_get_message

**What it does**: Gets details of a specific email message.

**When to use**: Read email content, check attachments.

**Arguments**:
- `messageId` (required): Message ID
- `$select` (optional): Fields to select

**Example LLM prompt**: "Get content of email MSG-12345"

---

### microsoft_list_calendars

**What it does**: Lists calendars for the authenticated user.

**When to use**: Browse calendars, find calendar IDs.

**Arguments**:
- `$top` (optional): Max results (default 50)

**Example LLM prompt**: "List all my calendars"

---

### microsoft_get_calendar_event

**What it does**: Gets details of a specific calendar event.

**When to use**: Get meeting details, check attendee list.

**Arguments**:
- `eventId` (required): Event ID

**Example LLM prompt**: "Get details for event EVT-12345"

---

### microsoft_list_files

**What it does**: Lists files in the authenticated user's OneDrive.

**When to use**: Browse files, find documents.

**Arguments**:
- `$top` (optional): Max results (default 50)
- `$select` (optional): Fields to select

**Example LLM prompt**: "List files in my OneDrive"

---

### microsoft_get_file

**What it does**: Gets details of a specific file in OneDrive.

**When to use**: Get file metadata, check file properties.

**Arguments**:
- `itemId` (required): Item ID

**Example LLM prompt**: "Get details for file ITEM-12345"

---

## Microsoft Notes

- **Graph API**: Unified API for Microsoft 365 services
- **Scopes**: User.Read, Mail.Read, Files.Read, etc.
- **OData filters**: Standard Graph query parameters
- **Delegate permissions**: User context authentication
- **Rate limits**: Microsoft Graph has rate limits
