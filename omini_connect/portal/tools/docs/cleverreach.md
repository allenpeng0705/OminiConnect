# CleverReach Tools

Provider: `cleverreach` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the CleverReach API. CleverReach is an email marketing platform for creating and managing email campaigns. **Requires CleverReach OAuth access.**

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with CleverReach
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://rest.cleverreach.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `cleverreach_list_receivers` | List receivers | GET | /receivers |
| `cleverreach_get_receiver` | Get receiver details | GET | /receivers/{id} |
| `cleverreach_create_receiver` | Create a receiver | POST | /receivers |
| `cleverreach_list_groups` | List groups | GET | /groups |
| `cleverreach_get_group` | Get group details | GET | /groups/{id} |
| `cleverreach_create_group` | Create a group | POST | /groups |
| `cleverreach_list_campaigns` | List campaigns | GET | /campaigns |
| `cleverreach_get_campaign` | Get campaign details | GET | /campaigns/{id} |
| `cleverreach_list_emails` | List emails | GET | /emails |
| `cleverreach_create_email` | Create an email | POST | /emails |

---

## Tool Details

### cleverreach_list_receivers

**What it does**: Lists all receivers (subscribers) in your account.

**When to use**: Find subscribers, view subscriber lists.

**Arguments**:
- `group_id` (optional): Filter by group
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List all CleverReach receivers"

---

### cleverreach_get_receiver

**What it does**: Gets details of a specific receiver.

**When to use**: View subscriber profile and history.

**Arguments**:
- `id` (required): Receiver ID

**Example LLM prompt**: "Get receiver 123 details"

---

### cleverreach_create_receiver

**What it does**: Creates a new receiver (subscriber).

**When to use**: Add a new subscriber to your list.

**Arguments**:
- `email` (required): Email address
- `first_name` (optional): First name
- `last_name` (optional): Last name
- `groups` (optional): Array of group IDs

**Example LLM prompt**: "Create a new receiver john@example.com"

---

### cleverreach_list_groups

**What it does**: Lists all groups (lists) in your account.

**When to use**: View subscriber groups.

**Arguments**:
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List all CleverReach groups"

---

### cleverreach_get_group

**What it does**: Gets details of a specific group.

**When to use**: View group settings and member count.

**Arguments**:
- `id` (required): Group ID

**Example LLM prompt**: "Get group 123 details"

---

### cleverreach_create_group

**What it does**: Creates a new group (list).

**When to use**: Create a new subscriber list.

**Arguments**:
- `name` (required): Group name
- `description` (optional): Group description

**Example LLM prompt**: "Create a group called 'Newsletter Subscribers'"

---

### cleverreach_list_campaigns

**What it does**: Lists all email campaigns.

**When to use**: View campaign history.

**Arguments**:
- `status` (optional): Filter by status
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List all campaigns"

---

### cleverreach_get_campaign

**What it does**: Gets details of a specific campaign.

**When to use**: View campaign statistics.

**Arguments**:
- `id` (required): Campaign ID

**Example LLM prompt**: "Get campaign 456 details"

---

### cleverreach_list_emails

**What it does**: Lists all emails.

**When to use**: View created email templates.

**Arguments**:
- `campaign_id` (optional): Filter by campaign

**Example LLM prompt**: "List emails for campaign 456"

---

### cleverreach_create_email

**What it does**: Creates a new email.

**When to use**: Create an email template.

**Arguments**:
- `subject` (required): Email subject
- `body` (required): Email body HTML
- `group_id` (optional): Group ID to send to

**Example LLM prompt**: "Create an email with subject 'Weekly Update'"

---

## CleverReach API Notes

- **Receivers**: Email subscribers in your lists
- **Groups**: Subscriber lists segmented by interest
- **Campaigns**: Email marketing campaigns
- **Emails**: Individual email templates
