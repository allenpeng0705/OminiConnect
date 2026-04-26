# Intercom Tools

Provider: `intercom` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Intercom REST API. They allow AI agents to manage conversations, users, admins, tags, and teams in your Intercom workspace. Intercom is a customer messaging platform that combines chat, email, and automation for customer engagement.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Intercom
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `conversation_read`, `conversation_write`, `contact_read`, `team_read`, `tags_read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `intercom_list_conversations` | List conversations | GET | /conversations |
| `intercom_get_conversation` | Get a specific conversation | GET | /conversations/{id} |
| `intercom_reply_to_conversation` | Reply to a conversation | POST | /conversations/{id}/reply |
| `intercom_list_users` | List users | GET | /users |
| `intercom_get_user` | Get a specific user | GET | /users/{id} |
| `intercom_list_admins` | List admins | GET | /admins |
| `intercom_get_admin` | Get a specific admin | GET | /admins/{id} |
| `intercom_list_tags` | List tags | GET | /tags |
| `intercom_get_tag` | Get a specific tag | GET | /tags/{id} |
| `intercom_list_teams` | List teams | GET | /teams |

---

## Tool Details

### intercom_list_conversations

**What it does**: Returns a paginated list of conversations with their status, participants, and latest message.

**When to use**: Browse open or closed support conversations to track customer issues.

**Arguments**:
- `state` (optional): `open`, `closed`, `all` — default `open`
- `per_page` (optional): default 25
- `page` (optional): default 1

**Example LLM prompt**: "Show me all open conversations"

---

### intercom_get_conversation

**What it does**: Gets full details of a specific conversation including messages, parts, and metadata.

**When to use**: Read the complete context of a customer issue before responding or taking action.

**Arguments**:
- `id` (required): Conversation ID

**Example LLM prompt**: "Show me conversation #12345"

---

### intercom_reply_to_conversation

**What it does**: Sends a reply to an existing conversation. Use this to respond to a contact or team member.

**When to use**: Reply to a customer message in an active conversation.

**Arguments**:
- `id` (required): Conversation ID
- `message` (required): Reply message content
- `type` (optional): Reply type — default `admin`

**Example LLM prompt**: "Reply to conversation #12345 with 'Thank you for reaching out'"

---

### intercom_list_users

**What it does**: Lists all users in your Intercom workspace. Returns user profiles including email, name, custom attributes, and last seen.

**When to use**: Browse your user base to find specific customers or filter by criteria.

**Arguments**:
- `per_page` (optional): default 25
- `page` (optional): default 1
- `email` (optional): Filter by email address

**Example LLM prompt**: "List all users with email containing @company.com"

---

### intercom_get_user

**What it does**: Gets full user profile including custom attributes, conversation history, and activity.

**When to use**: Review a user's profile before responding to their conversation.

**Arguments**:
- `id` (required): User ID

**Example LLM prompt**: "Show me user profile for #67890"

---

### intercom_list_admins

**What it does**: Lists all admins in your Intercom workspace. Returns their names, emails, avatar, and role information.

**When to use**: See team members before assigning conversations or asking for help.

**Arguments**:
- `per_page` (optional): default 25
- `page` (optional): default 1

**Example LLM prompt**: "List all admins in the workspace"

---

### intercom_get_admin

**What it does**: Gets a specific admin's profile including name, email, avatar, and role.

**When to use**: Get details about a team member before handing off a conversation.

**Arguments**:
- `id` (required): Admin ID

**Example LLM prompt**: "Show me admin #42"

---

### intercom_list_tags

**What it does**: Lists all tags in your Intercom workspace. Returns tag names, colors, and usage counts.

**When to use**: See available tags for categorizing users or conversations.

**Arguments**:
- `per_page` (optional): default 25
- `page` (optional): default 1

**Example LLM prompt**: "What tags are available for categorizing customers"

---

### intercom_get_tag

**What it does**: Gets tag details including name, color, and applied user IDs.

**When to use**: Review tag usage or see which users are tagged.

**Arguments**:
- `id` (required): Tag ID

**Example LLM prompt**: "Show me details for the vip-customers tag"

---

### intercom_list_teams

**What it does**: Lists all teams in your Intercom workspace. Returns team names, sizes, and admin assignments.

**When to use**: See team structure and organization before routing conversations.

**Arguments**:
- `per_page` (optional): default 25
- `page` (optional): default 1

**Example LLM prompt**: "List all teams in the workspace"

---

## Intercom API Notes

- **Conversation States**: open, closed, or all
- **User Types**: Registered users with profile data
- **Tags**: Apply to both users and conversations for organization
- **Teams**: Group admins for routing and permissions
- **Pagination**: Default per_page is 25, adjust based on your needs
