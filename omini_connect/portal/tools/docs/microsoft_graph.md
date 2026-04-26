# Microsoft Graph Tools

Provider: `microsoft_graph` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap Microsoft Graph API - covering directory (users, groups), Outlook mail, Calendar, and OneDrive files. They allow AI agents to work with Microsoft's productivity suite on behalf of the authenticated user.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `User.Read.All`, `Group.Read.All`, `Mail.Read`, `Mail.Send`, `Calendars.Read`, `Files.Read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `microsoft_graph_list_users` | List users in directory | GET | /v1.0/users |
| `microsoft_graph_get_user` | Get user details | GET | /v1.0/users/{id} |
| `microsoft_graph_list_groups` | List groups | GET | /v1.0/groups |
| `microsoft_graph_get_group` | Get group details | GET | /v1.0/groups/{id} |
| `microsoft_graph_list_messages` | List emails | GET | /v1.0/me/messages |
| `microsoft_graph_get_message` | Get email by ID | GET | /v1.0/me/messages/{id} |
| `microsoft_graph_send_message` | Send an email | POST | /v1.0/me/sendMail |
| `microsoft_graph_list_calendar_events` | List calendar events | GET | /v1.0/me/events |
| `microsoft_graph_get_calendar_event` | Get event by ID | GET | /v1.0/me/events/{id} |
| `microsoft_graph_list_onedrive_files` | List OneDrive files | GET | /v1.0/me/drive/root/children |

---

## Tool Details

### microsoft_graph_list_users

**What it does**: Lists users in the organization's directory with optional filtering.

**When to use**: Find users, search by name or email, get directory overview.

**Arguments**:
- `$filter` (optional): OData filter expression
- `$top` (optional): Number of users (default 20, max 999)
- `$select` (optional): Fields to select

**Example LLM prompt**: "List users in the organization named John"

---

### microsoft_graph_get_user

**What it does**: Gets details about a specific user including profile information.

**When to use**: Get user details, check user properties, find contact info.

**Arguments**:
- `id` (required): User ID or UPN

**Example LLM prompt**: "Get details for user alice@company.com"

---

### microsoft_graph_list_groups

**What it does**: Lists groups in the organization with optional filtering.

**When to use**: Discover groups, find team groups, get group listings.

**Arguments**:
- `$filter` (optional): OData filter expression
- `$top` (optional): Number of groups (default 20, max 999)
- `$select` (optional): Fields to select

**Example LLM prompt**: "List all groups in the organization"

---

### microsoft_graph_get_group

**What it does**: Gets details of a specific group including members and settings.

**When to use**: Check group membership, get group configuration.

**Arguments**:
- `id` (required): Group ID

**Example LLM prompt**: "Get details for group 12345"

---

### microsoft_graph_list_messages

**What it does**: Lists emails in the authenticated user's mailbox.

**When to use**: Search emails, check inbox, find messages from specific senders.

**Arguments**:
- `$top` (optional): Number of messages (default 10, max 50)
- `$filter` (optional): OData filter (e.g., `isRead eq false`)
- `$select` (optional): Fields to select

**Example LLM prompt**: "Show me unread emails from today"

---

### microsoft_graph_get_message

**What it does**: Gets a specific email message by ID.

**When to use**: Read full email content before replying or taking action.

**Arguments**:
- `id` (required): Message ID

**Example LLM prompt**: "Get the email with ID abc123"

---

### microsoft_graph_send_message

**What it does**: Sends an email to one or more recipients.

**When to use**: Send emails on behalf of the user.

**Arguments**:
- `to` (required): Recipient email address
- `subject` (required): Email subject
- `body` (required): Email body content

**Example LLM prompt**: "Send an email to john@example.com with subject 'Meeting' saying 'Let's meet tomorrow'"

---

### microsoft_graph_list_calendar_events

**What it does**: Lists calendar events for the authenticated user.

**When to use**: Check schedule, find meeting times, see upcoming appointments.

**Arguments**:
- `$top` (optional): Number of events (default 10)
- `$filter` (optional): OData filter
- `$select` (optional): Fields to select
- `$orderby` (optional): Order by field

**Example LLM prompt**: "Show me my calendar for next week"

---

### microsoft_graph_get_calendar_event

**What it does**: Gets details of a specific calendar event.

**When to use**: Get full event details including attendees and description.

**Arguments**:
- `id` (required): Event ID

**Example LLM prompt**: "Get details of event abc123"

---

### microsoft_graph_list_onedrive_files

**What it does**: Lists files in the user's OneDrive, optionally within a specific folder.

**When to use**: Find documents, browse folders, search for files.

**Arguments**:
- `$top` (optional): Number of results (default 20)
- `$filter` (optional): OData filter

**Example LLM prompt**: "List files in my OneDrive"

---

## Microsoft Graph API Notes

- Rate limits apply to all Graph API calls
- ISO 8601 format for all dates/times
- OData filters use standard syntax (e.g., `$filter=isRead eq false`)
- User IDs can be substituted with `me` for the current user
- File paths use OneDrive notation (e.g., `/Documents/report.txt`)