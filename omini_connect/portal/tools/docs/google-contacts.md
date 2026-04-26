# Google Contacts Tools

Provider: `google-contacts` | Engine: `nango` | Auth: OAUTH2 via Nango (alias: google)

## Overview

These tools wrap the Google People API. They allow AI agents to manage contacts, contact groups, and search contacts. **Requires Google OAuth2 with Contacts permissions.**

## Authentication

**Nango OAUTH2 (Google Contacts)**:
- User authenticates via OAuth2 with Contacts scope
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://people.googleapis.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `google_contacts_list_connections` | List contacts | GET | /v1/people/me/connections |
| `google_contacts_get_person` | Get contact details | GET | /v1/people/{resourceName} |
| `google_contacts_create_contact` | Create contact | POST | /v1/people |
| `google_contacts_update_contact` | Update contact | PATCH | /v1/people/{resourceName} |
| `google_contacts_delete_contact` | Delete contact | DELETE | /v1/people/{resourceName} |
| `google_contacts_list_contact_groups` | List contact groups | GET | /v1/contactGroups |
| `google_contacts_get_contact_group` | Get contact group | GET | /v1/contactGroups/{resourceName} |
| `google_contacts_search_contacts` | Search contacts | POST | /v1/people:searchContacts |
| `google_contacts_batch_get` | Batch get contacts | GET | /v1/people:batchGet |
| `google_contacts_list_other_contacts` | List other contacts | GET | /v1/people/me/otherContacts |

---

## Tool Details

### google_contacts_list_connections

**What it does**: Lists contacts from Google Contacts.

**When to use**: Browse user's contact list.

**Arguments**:
- `pageSize` (optional): Number of contacts (default 20)
- `personFields` (optional): Fields to return

**Example LLM prompt**: "List my contacts"

---

### google_contacts_get_person

**What it does**: Gets detailed information about a contact.

**When to use**: Get contact details.

**Arguments**:
- `resourceName` (required): Contact resource name

**Example LLM prompt**: "Get details for contact people/c123456"

---

### google_contacts_create_contact

**What it does**: Creates a new contact.

**When to use**: Add new contacts.

**Arguments**:
- `givenName` (required): First name
- `familyName` (optional): Last name
- `emailAddress` (optional): Email address
- `phoneNumber` (optional): Phone number

**Example LLM prompt**: "Create a contact for John Doe with email john@example.com"

---

### google_contacts_update_contact

**What it does**: Updates an existing contact.

**When to use**: Modify contact information.

**Arguments**:
- `resourceName` (required): Contact resource name
- `givenName` (optional): New first name
- `familyName` (optional): New last name

**Example LLM prompt**: "Update contact people/c123456 with new phone number"

---

### google_contacts_delete_contact

**What it does**: Deletes a contact.

**When to use**: Remove unwanted contacts.

**Arguments**:
- `resourceName` (required): Contact resource name

**Example LLM prompt**: "Delete contact people/c123456"

---

### google_contacts_list_contact_groups

**What it does**: Lists contact groups.

**When to use**: Browse contact groups.

**Arguments**: None

**Example LLM prompt**: "List my contact groups"

---

### google_contacts_get_contact_group

**What it does**: Gets a contact group by ID.

**When to use**: Get group details and members.

**Arguments**:
- `resourceName` (required): Contact group resource name

**Example LLM prompt**: "Get contact group abc123"

---

### google_contacts_search_contacts

**What it does**: Searches contacts by query.

**When to use**: Find contacts quickly.

**Arguments**:
- `query` (required): Search query
- `pageSize` (optional): Number of results (default 20)

**Example LLM prompt**: "Search for contacts named John"

---

### google_contacts_batch_get

**What it does**: Gets multiple contacts by resource names.

**When to use**: Retrieve several contacts at once.

**Arguments**:
- `resourceNames` (required): Contact resource names

**Example LLM prompt**: "Get contacts people/c123, people/c456"

---

### google_contacts_list_other_contacts

**What it does**: Lists contacts outside user's direct contacts.

**When to use**: Find people in organization.

**Arguments**:
- `pageSize` (optional): Number of contacts (default 20)

**Example LLM prompt**: "List other contacts"

---

## Google Contacts API Notes

- **Resource names**: Unique IDs like "people/c123456"
- **Contact groups**: Organize contacts into groups
- **Other contacts**: Domain-wide contacts
- **Search**: Searches names, emails, phones
