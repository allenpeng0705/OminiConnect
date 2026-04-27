# Gmail Tools

Provider: `google-mail` | Engine: `nango` | Auth: OAUTH2 via Nango (alias: google)

## Overview

These tools wrap the Gmail API. They allow AI agents to manage messages, labels, and threads. **Requires Google OAuth2 with Gmail permissions.**

## Authentication

**Nango OAUTH2 (Gmail)**:
- User authenticates via OAuth2 with Gmail scope
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://gmail.googleapis.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `google_mail_list_messages` | List messages | GET | /gmail/v1/users/me/messages |
| `google_mail_get_message` | Get message details | GET | /gmail/v1/users/me/messages/{id} |
| `google_mail_send_message` | Send message | POST | /gmail/v1/users/me/messages/send |
| `google_mail_list_labels` | List labels | GET | /gmail/v1/users/me/labels |
| `google_mail_create_label` | Create label | POST | /gmail/v1/users/me/labels |
| `google_mail_modify_message` | Modify message | POST | /gmail/v1/users/me/messages/{id}/modify |
| `google_mail_trash_message` | Trash message | POST | /gmail/v1/users/me/messages/{id}/trash |
| `google_mail_untrash_message` | Untrash message | POST | /gmail/v1/users/me/messages/{id}/untrash |
| `google_mail_list_threads` | List threads | GET | /gmail/v1/users/me/threads |
| `google_mail_get_thread` | Get thread details | GET | /gmail/v1/users/me/threads/{id} |

---

## Tool Details

### google_mail_list_messages

**What it does**: Lists messages in Gmail.

**When to use**: Search and browse emails.

**Arguments**:
- `q` (optional): Search query (Gmail syntax)
- `maxResults` (optional): Max results (default 20)

**Example LLM prompt**: "List my emails from today"

---

### google_mail_get_message

**What it does**: Gets detailed information about a message.

**When to use**: Read full email content.

**Arguments**:
- `id` (required): Message ID

**Example LLM prompt**: "Get email with ID abc123"

---

### google_mail_send_message

**What it does**: Sends an email message.

**When to use**: Send emails programmatically.

**Arguments**:
- `to` (required): Recipient email
- `subject` (required): Subject
- `body` (required): Body content

**Example LLM prompt**: "Send an email to john@example.com with subject 'Hello' and body 'Hi John'"

---

### google_mail_list_labels

**What it does**: Lists all labels in Gmail.

**When to use**: See available labels.

**Arguments**: None

**Example LLM prompt**: "List all my labels"

---

### google_mail_create_label

**What it does**: Creates a new label.

**When to use**: Organize emails with custom labels.

**Arguments**:
- `name` (required): Label name
- `labelListVisibility` (optional): Label visibility
- `messageListVisibility` (optional): Message list visibility

**Example LLM prompt**: "Create a label called 'Projects'"

---

### google_mail_modify_message

**What it does**: Adds or removes labels from a message.

**When to use**: Archive or label emails.

**Arguments**:
- `id` (required): Message ID
- `addLabels` (optional): Labels to add
- `removeLabels` (optional): Labels to remove

**Example LLM prompt**: "Add label 'Important' to message abc123"

---

### google_mail_trash_message

**What it does**: Moves a message to trash.

**When to use**: Delete emails.

**Arguments**:
- `id` (required): Message ID

**Example LLM prompt**: "Trash message abc123"

---

### google_mail_untrash_message

**What it does**: Restores a message from trash.

**When to use**: Recover deleted emails.

**Arguments**:
- `id` (required): Message ID

**Example LLM prompt**: "Restore message abc123 from trash"

---

### google_mail_list_threads

**What it does**: Lists email threads.

**When to use**: Browse conversation threads.

**Arguments**:
- `q` (optional): Search query
- `maxResults` (optional): Max results (default 20)

**Example LLM prompt**: "List threads with 'project' in subject"

---

### google_mail_get_thread

**What it does**: Gets a thread with all messages.

**When to use**: Read full conversation.

**Arguments**:
- `id` (required): Thread ID

**Example LLM prompt**: "Get thread abc123"

---

## Gmail API Notes

- **Search queries**: Uses Gmail search syntax (from:, subject:, label:, etc.)
- **Labels**: System labels (INBOX, SENT, TRASH) or custom labels
- **Threads**: Conversations grouped by subject
- **Message ID**: Unique per message
- **Thread ID**: Shared across messages in same conversation
