# Google Tools

Provider: `google` | Engine: `nango` | Auth: OAUTH2 via Nango

## Overview

These tools wrap the Google APIs. They allow AI agents to manage Gmail, Calendar, Contacts, and Drive. **Requires Google OAuth2 authentication.**

## Authentication

**Nango OAUTH2**:
- User authenticates via OAuth2 with Google
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://www.googleapis.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `google_list_emails` | List emails | GET | /gmail/v1/users/me/messages |
| `google_get_email` | Get email details | GET | /gmail/v1/users/me/messages/{id} |
| `google_send_email` | Send an email | POST | /gmail/v1/users/me/messages/send |
| `google_list_calendars` | List calendars | GET | /calendar/v3/users/me/calendarList |
| `google_list_events` | List calendar events | GET | /calendar/v3/calendars/{calendarId}/events |
| `google_create_event` | Create calendar event | POST | /calendar/v3/calendars/{calendarId}/events |
| `google_list_contacts` | List contacts | GET | /v1/people/me/connections |
| `google_get_contact` | Get contact details | GET | /v1/people/{resourceName} |
| `google_list_files` | List Drive files | GET | /drive/v3/files |
| `google_get_file` | Get Drive file details | GET | /drive/v3/files/{fileId} |

---

## Tool Details

### google_list_emails

**What it does**: Lists emails in the user's Gmail mailbox.

**When to use**: Search and browse emails.

**Arguments**:
- `q` (optional): Search query (Gmail syntax)
- `maxResults` (optional): Maximum results (default 20)

**Example LLM prompt**: "List my recent emails from today"

---

### google_get_email

**What it does**: Gets detailed information about a specific email.

**When to use**: Read full email content and headers.

**Arguments**:
- `id` (required): Email ID

**Example LLM prompt**: "Get the email with ID abc123"

---

### google_send_email

**What it does**: Sends an email from the user's Gmail account.

**When to use**: Send emails programmatically.

**Arguments**:
- `to` (required): Recipient email address
- `subject` (required): Email subject
- `body` (required): Email body content

**Example LLM prompt**: "Send an email to john@example.com with subject 'Hello' and body 'Hi John'"

---

### google_list_calendars

**What it does**: Lists all calendars in the user's Google Calendar.

**When to use**: See all available calendars.

**Arguments**: None

**Example LLM prompt**: "List all my calendars"

---

### google_list_events

**What it does**: Lists events from Google Calendar.

**When to use**: View upcoming events.

**Arguments**:
- `calendarId` (optional): Calendar ID (default: primary)
- `timeMin` (optional): Start time (ISO 8601)
- `timeMax` (optional): End time (ISO 8601)
- `maxResults` (optional): Maximum results (default 20)

**Example LLM prompt**: "List my events for today"

---

### google_create_event

**What it does**: Creates a new event in Google Calendar.

**When to use**: Schedule meetings and events.

**Arguments**:
- `calendarId` (optional): Calendar ID (default: primary)
- `summary` (required): Event title
- `start` (required): Start time (ISO 8601)
- `end` (required): End time (ISO 8601)

**Example LLM prompt**: "Create an event titled 'Meeting' from 2pm to 3pm today"

---

### google_list_contacts

**What it does**: Lists contacts from Google Contacts.

**When to use**: Browse contact list.

**Arguments**:
- `pageSize` (optional): Number of results (default 20)

**Example LLM prompt**: "List my contacts"

---

### google_get_contact

**What it does**: Gets detailed information about a contact.

**When to use**: Get contact details.

**Arguments**:
- `resourceName` (required): Contact resource name

**Example LLM prompt**: "Get details for contact people/c123"

---

### google_list_files

**What it does**: Lists files in Google Drive.

**When to use**: Browse Drive files.

**Arguments**:
- `q` (optional): Search query
- `pageSize` (optional): Number of results (default 20)

**Example LLM prompt**: "List files in my Drive"

---

### google_get_file

**What it does**: Gets detailed information about a Drive file.

**When to use**: Get file metadata.

**Arguments**:
- `fileId` (required): File ID

**Example LLM prompt**: "Get details for file abc123"

---

## Google API Notes

- **OAuth scopes**: Each tool requires specific scopes
- **Gmail queries**: Uses Gmail search syntax (e.g., "from:john subject:meeting")
- **Calendar time format**: ISO 8601 format (e.g., "2024-01-15T10:00:00Z")
- **Contacts**: Uses resource names instead of IDs
- **Drive queries**: Supports Google Drive search syntax
