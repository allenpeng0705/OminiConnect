# Google Workspace Tools

Provider: `google_workspace` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap Google Workspace APIs (Gmail, Drive, Calendar, Contacts, Sheets). They allow AI agents to read and send emails, manage calendar events, work with files in Google Drive, access contacts, and read spreadsheet data on behalf of the authenticated user.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `gmail.readonly`, `gmail.send`, `calendar.readonly`, `drive.readonly`, `contacts.readonly`, `spreadsheet.readonly`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `google_workspace_list_gmail_messages` | List Gmail messages | GET | /gmail/v1/users/me/messages |
| `google_workspace_get_gmail_message` | Get Gmail message | GET | /gmail/v1/users/me/messages/{id} |
| `google_workspace_send_gmail_message` | Send Gmail message | POST | /gmail/v1/users/me/messages/send |
| `google_workspace_list_drive_files` | List Drive files | GET | /drive/v3/files |
| `google_workspace_get_drive_file` | Get Drive file metadata | GET | /drive/v3/files/{fileId} |
| `google_workspace_list_calendar_events` | List calendar events | GET | /calendar/v3/calendars/primary/events |
| `google_workspace_get_calendar_event` | Get calendar event | GET | /calendar/v3/calendars/primary/events/{eventId} |
| `google_workspace_list_contacts` | List contacts | GET | /contacts/v3/people/me/connections |
| `google_workspace_get_contact` | Get contact details | GET | /contacts/v3/people/{resourceName} |
| `google_workspace_list_sheets` | List spreadsheets | GET | /sheets/v4/spreadsheets |
| `google_workspace_get_sheets` | Get spreadsheet data | GET | /sheets/v4/spreadsheets/{spreadsheetId}/values/{range} |

---

## Tool Details

### google_workspace_list_gmail_messages

**What it does**: Lists messages in the authenticated user's Gmail mailbox with optional filtering.

**When to use**: Search emails, check for new messages, find messages by sender or subject.

**Arguments**:
- `maxResults` (optional): Max messages to return (default 20, max 100)
- `pageToken` (optional): Page token for pagination
- `q` (optional): Search query (e.g., `from:user@example.com is:unread`)

**Example LLM prompt**: "Show me unread emails from today"

---

### google_workspace_get_gmail_message

**What it does**: Gets a specific email by ID including headers, body, and metadata.

**When to use**: Read the full content of an email before replying or taking action.

**Arguments**:
- `id` (required): Message ID
- `format` (optional): `full`, `metadata`, or `minimal` (default: full)

**Example LLM prompt**: "Show me the email with ID abc123"

---

### google_workspace_send_gmail_message

**What it does**: Sends an email to one or more recipients.

**When to use**: Send emails, responses, or notifications on behalf of the user.

**Arguments**:
- `to` (required): Recipient email address
- `subject` (required): Email subject
- `body` (required): Email body (plain text)
- `cc` (optional): CC recipient
- `bcc` (optional): BCC recipient

**Example LLM prompt**: "Send an email to john@example.com with subject 'Meeting' and body 'Let's meet tomorrow at 3pm'"

---

### google_workspace_list_drive_files

**What it does**: Lists files in the authenticated user's Google Drive, with optional filtering.

**When to use**: Find documents, search for files by name, browse folders.

**Arguments**:
- `pageSize` (optional): Number of files (default 20, max 100)
- `pageToken` (optional): Page token for pagination
- `q` (optional): Query (e.g., `name contains 'report' and mimeType='application/pdf'`)

**Example LLM prompt**: "Find all PDF files in my Drive"

---

### google_workspace_get_drive_file

**What it does**: Gets metadata for a specific file in Google Drive.

**When to use**: Check file details, size, modification date, or sharing settings.

**Arguments**:
- `fileId` (required): File ID
- `fields` (optional): Fields to include

**Example LLM prompt**: "Get details about file abc123"

---

### google_workspace_list_calendar_events

**What it does**: Lists calendar events for the authenticated user, optionally filtered by time range.

**When to use**: Check schedule, find available times, see upcoming appointments.

**Arguments**:
- `calendarId` (optional): Calendar ID (default: primary)
- `timeMin` (optional): Start time (RFC3339 format)
- `timeMax` (optional): End time (RFC3339 format)
- `maxResults` (optional): Max events to return (default 20)
- `q` (optional): Search query
- `singleEvents` (optional): Expand recurring events (default true)
- `orderBy` (optional): `startTime` or `updated`

**Example LLM prompt**: "Show me my calendar events for tomorrow"

---

### google_workspace_get_calendar_event

**What it does**: Gets details of a specific calendar event.

**When to use**: Get full details of an event including attendees, location, and description.

**Arguments**:
- `eventId` (required): Event ID
- `calendarId` (optional): Calendar ID (default: primary)

**Example LLM prompt**: "Get the details of event abc123"

---

### google_workspace_list_contacts

**What it does**: Lists contacts from the authenticated user's Google contacts.

**When to use**: Find contact information, look up colleagues.

**Arguments**:
- `pageSize` (optional): Number of contacts (default 20)
- `pageToken` (optional): Page token for pagination

**Example LLM prompt**: "List my Google contacts"

---

### google_workspace_get_contact

**What it does**: Gets details of a specific contact by resource name.

**When to use**: Get full contact info including phone numbers and addresses.

**Arguments**:
- `resourceName` (required): Contact resource name

**Example LLM prompt**: "Get details for contact people/c1234567890"

---

### google_workspace_list_sheets

**What it does**: Lists spreadsheets in Google Sheets that the user has access to.

**When to use**: Find spreadsheets, discover data sources.

**Arguments**:
- `pageSize` (optional): Maximum spreadsheets (default 20)
- `pageToken` (optional): Page token for pagination

**Example LLM prompt**: "What spreadsheets do I have in Google Sheets?"

---

### google_workspace_get_sheets

**What it does**: Gets data from a Google Sheets spreadsheet by specifying spreadsheet ID and cell range.

**When to use**: Read data from a spreadsheet, extract values for analysis.

**Arguments**:
- `spreadsheetId` (required): The spreadsheet ID
- `range` (required): The cell range (e.g., Sheet1!A1:D10)

**Example LLM prompt**: "Get the data from spreadsheet abc123 in range A1:D10"

---

## Google Workspace API Notes

- Rate limits apply to all Google APIs
- OAuth scopes determine available actions
- RFC3339 format for all timestamps (e.g., `2024-01-01T10:00:00Z`)
- Drive queries use advanced search operators
- Sheets ranges use A1 notation with sheet name prefix