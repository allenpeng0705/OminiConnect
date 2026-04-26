# Outlook Tools

Provider: `outlook` | Engine: `nango` | Auth: OAuth via Nango (Microsoft/Outlook)

## Overview

These tools wrap the Microsoft Graph API for Outlook. They allow AI agents to manage emails, calendars, events, and contacts. **Requires Microsoft OAuth access for Outlook.**

## Authentication

**OAuth via Nango**:
- Alias for Microsoft integration
- User authenticates via Nango Connect with Microsoft
- Token stored in Nango, accessed via `connection_ref`
- Base URL: Uses Microsoft Graph API

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `outlook_list_messages` | List messages | GET | /v1.0/me/mailFolders/inbox/messages |
| `outlook_get_message` | Get message details | GET | /v1.0/me/messages/{messageId} |
| `outlook_send_message` | Send email | POST | /v1.0/me/sendMail |
| `outlook_delete_message` | Delete message | DELETE | /v1.0/me/messages/{messageId} |
| `outlook_list_calendars` | List calendars | GET | /v1.0/me/calendars |
| `outlook_list_events` | List events | GET | /v1.0/me/calendarView |
| `outlook_get_event` | Get event details | GET | /v1.0/me/events/{eventId} |
| `outlook_create_event` | Create event | POST | /v1.0/me/events |
| `outlook_list_contacts` | List contacts | GET | /v1.0/me/contacts |
| `outlook_get_contact` | Get contact details | GET | /v1.0/me/contacts/{contactId} |

---

## Tool Details

### outlook_list_messages

**What it does**: Lists messages in the user's mailbox.

**When to use**: Browse inbox, find emails.

**Arguments**:
- `limit` (optional): Number of messages (default 20)

**Example LLM prompt**: "List my recent emails"

---

### outlook_get_message

**What it does**: Gets detailed information for a specific message.

**When to use**: Read email content, view headers.

**Arguments**:
- `messageId` (required): Message ID

**Example LLM prompt**: "Get details for message ABC123"

---

### outlook_send_message

**What it does**: Sends an email message.

**When to use**: Send emails, compose new messages.

**Arguments**:
- `toRecipients` (required): Recipient email addresses
- `subject` (required): Email subject
- `body` (required): Email body

**Example LLM prompt**: "Send an email to john@company.com with subject 'Hello'"

---

### outlook_delete_message

**What it does**: Deletes a message from the mailbox.

**When to use**: Remove emails, clean up inbox.

**Arguments**:
- `messageId` (required): Message ID

**Example LLM prompt**: "Delete message ABC123"

---

### outlook_list_calendars

**What it does**: Lists all calendars for the user.

**When to use**: View calendars, find available calendars.

**Arguments**: None

**Example LLM prompt**: "List all my calendars"

---

### outlook_list_events

**What it does**: Lists events from the user's calendar.

**When to use**: View schedule, find meetings.

**Arguments**:
- `start_date` (required): Start date/time (ISO 8601)
- `end_date` (required): End date/time (ISO 8601)

**Example LLM prompt**: "List my meetings for today"

---

### outlook_get_event

**What it does**: Gets detailed information for a specific event.

**When to use**: View meeting details, attendee list.

**Arguments**:
- `eventId` (required): Event ID

**Example LLM prompt**: "Get details for event XYZ789"

---

### outlook_create_event

**What it does**: Creates a new calendar event.

**When to use**: Schedule meetings, create appointments.

**Arguments**:
- `subject` (required): Event subject
- `start` (required): Start date/time
- `end` (required): End date/time

**Example LLM prompt**: "Create a meeting for 2pm-3pm today with subject 'Team Sync'"

---

### outlook_list_contacts

**What it does**: Lists contacts from the user's address book.

**When to use**: Browse contacts, find people.

**Arguments**:
- `limit` (optional): Number of contacts (default 20)

**Example LLM prompt**: "List my recent contacts"

---

### outlook_get_contact

**What it does**: Gets detailed information for a specific contact.

**When to use**: View contact details, email, phone.

**Arguments**:
- `contactId` (required): Contact ID

**Example LLM prompt**: "Get details for contact 12345"

---

## Outlook Notes

- **Microsoft Graph**: Uses Microsoft Graph API endpoints
- **Scopes**: Mail.Read, Mail.Send, Calendars.Read, Contacts.Read
- **Message IDs**: Unique identifiers for emails
- **Event timing**: ISO 8601 date-time format
- **Contacts**: User's personal address book
