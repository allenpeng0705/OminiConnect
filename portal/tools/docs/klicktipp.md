# KlickTipp Tools

Provider: `klicktipp` | Engine: `nango` | Auth: API_KEY via Nango

## Overview

These tools wrap the KlickTipp API. They allow AI agents to manage contacts, tags, fields, and campaigns. **Requires KlickTipp API key.**

## Authentication

**API Key via Nango**:
- User provides their KlickTipp API key
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://api.klicktipp.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `klicktipp_list_contacts` | List contacts | GET | /contact |
| `klicktipp_get_contact` | Get contact details | GET | /contact/{contact_id} |
| `klicktipp_create_contact` | Create a contact | POST | /contact |
| `klicktipp_update_contact` | Update a contact | PUT | /contact/{contact_id} |
| `klicktipp_delete_contact` | Delete a contact | DELETE | /contact/{contact_id} |
| `klicktipp_list_tags` | List tags | GET | /tag |
| `klicktipp_add_tag` | Add tag to contact | POST | /contact/{contact_id}/tag |
| `klicktipp_remove_tag` | Remove tag from contact | DELETE | /contact/{contact_id}/tag/{tag_id} |
| `klicktipp_list_fields` | List custom fields | GET | /field |
| `klicktipp_list_campaigns` | List campaigns | GET | /campaign |

---

## Tool Details

### klicktipp_list_contacts

**What it does**: Lists all contacts in KlickTipp.

**When to use**: Find contacts, view subscriber list.

**Arguments**:
- `page` (optional): Page number (default: 1)
- `per_page` (optional): Results per page (default: 20)

**Example LLM prompt**: "List all contacts in KlickTipp"

---

### klicktipp_get_contact

**What it does**: Gets details for a specific contact.

**When to use**: Get contact information, view profile.

**Arguments**:
- `contact_id` (required): Contact ID

**Example LLM prompt**: "Get details for contact 12345"

---

### klicktipp_create_contact

**What it does**: Creates a new contact.

**When to use**: Add subscribers to KlickTipp.

**Arguments**:
- `email` (required): Email address
- `first_name` (optional): First name
- `last_name` (optional): Last name
- `tags` (optional): Tags to apply

**Example LLM prompt**: "Create a contact for john@example.com"

---

### klicktipp_update_contact

**What it does**: Updates an existing contact.

**When to use**: Modify contact data.

**Arguments**:
- `contact_id` (required): Contact ID
- `first_name` (optional): First name
- `last_name` (optional): Last name

**Example LLM prompt**: "Update contact 12345 with new name"

---

### klicktipp_delete_contact

**What it does**: Deletes a contact.

**When to use**: Remove subscribers.

**Arguments**:
- `contact_id` (required): Contact ID

**Example LLM prompt**: "Delete contact 12345"

---

### klicktipp_list_tags

**What it does**: Lists all tags.

**When to use**: View tags, organize contacts.

**Arguments**: None

**Example LLM prompt**: "List all tags in KlickTipp"

---

### klicktipp_add_tag

**What it does**: Adds a tag to a contact.

**When to use**: Categorize contacts.

**Arguments**:
- `contact_id` (required): Contact ID
- `tag` (required): Tag name

**Example LLM prompt**: "Add tag 'VIP' to contact 12345"

---

### klicktipp_remove_tag

**What it does**: Removes a tag from a contact.

**When to use**: Remove categorization.

**Arguments**:
- `contact_id` (required): Contact ID
- `tag_id` (required): Tag ID

**Example LLM prompt**: "Remove tag 111 from contact 12345"

---

### klicktipp_list_fields

**What it does**: Lists all custom fields.

**When to use**: View available fields.

**Arguments**: None

**Example LLM prompt**: "List all custom fields in KlickTipp"

---

### klicktipp_list_campaigns

**What it does**: Lists all campaigns.

**When to use**: View campaigns, manage email marketing.

**Arguments**: None

**Example LLM prompt**: "List all campaigns in KlickTipp"

---

## KlickTipp API Notes

- **Email Marketing**: Email marketing automation platform
- **Contacts**: Subscribers and customers
- **Tags**: Categorize and segment contacts
- **Custom Fields**: Store additional contact data
- **Campaigns**: Email marketing campaigns
