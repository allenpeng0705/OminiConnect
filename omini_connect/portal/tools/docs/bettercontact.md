# BetterContact Tools

Provider: `bettercontact` | Engine: `nango` | Auth: API_KEY via Nango

## Overview

These tools wrap the BetterContact API. They allow AI agents to manage contacts, companies, and contact lists. BetterContact is a contact management platform that helps businesses organize and enrich their contact data.

## Authentication

**Nango API_KEY**:
- User provides API key from BetterContact
- Token stored in Nango, accessed via `connection_ref`
- API key passed in X-API-Key header

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `bettercontact_get_account` | Get account info | GET | /api/v2/account |
| `bettercontact_list_contacts` | List contacts | GET | /api/v2/contacts |
| `bettercontact_get_contact` | Get contact details | GET | /api/v2/contacts/{id} |
| `bettercontact_create_contact` | Create new contact | POST | /api/v2/contacts |
| `bettercontact_update_contact` | Update contact | PUT | /api/v2/contacts/{id} |
| `bettercontact_delete_contact` | Delete contact | DELETE | /api/v2/contacts/{id} |
| `bettercontact_search_contacts` | Search contacts | GET | /api/v2/contacts/search |
| `bettercontact_list_lists` | List contact lists | GET | /api/v2/lists |
| `bettercontact_get_company` | Get company info | GET | /api/v2/companies/{id} |
| `bettercontact_list_webhooks` | List webhooks | GET | /api/v2/webhooks |

---

## Tool Details

### bettercontact_get_account

**What it does**: Gets account information and settings.

**When to use**: Verify account status, check plan limits.

**Arguments**: None required

**Example LLM prompt**: "Get my BetterContact account info"

---

### bettercontact_list_contacts

**What it does**: Lists all contacts in the account.

**When to use**: Browse contacts, find specific people.

**Arguments**:
- `page` (optional): Page number (default 1)
- `pageSize` (optional): Contacts per page (default 20)

**Example LLM prompt**: "List all contacts in BetterContact"

---

### bettercontact_get_contact

**What it does**: Gets detailed information about a specific contact.

**When to use**: View contact details, check information.

**Arguments**:
- `id` (required): Contact ID

**Example LLM prompt**: "Get details for contact ID 123"

---

### bettercontact_create_contact

**What it does**: Creates a new contact in BetterContact.

**When to use**: Add new leads, customers, or partners.

**Arguments**:
- `email` (required): Contact email
- `firstName` (optional): First name
- `lastName` (optional): Last name
- `phone` (optional): Phone number
- `company` (optional): Company name

**Example LLM prompt**: "Create a new contact for John Doe with email john@company.com"

---

### bettercontact_update_contact

**What it does**: Updates an existing contact.

**When to use**: Modify contact information, update details.

**Arguments**:
- `id` (required): Contact ID
- `email` (optional): Contact email
- `firstName` (optional): First name
- `lastName` (optional): Last name
- `phone` (optional): Phone number
- `company` (optional): Company name

**Example LLM prompt**: "Update contact 123 with new phone number"

---

### bettercontact_delete_contact

**What it does**: Deletes a contact from BetterContact.

**When to use**: Remove outdated or duplicate contacts.

**Arguments**:
- `id` (required): Contact ID

**Example LLM prompt**: "Delete contact 123"

---

### bettercontact_search_contacts

**What it does**: Searches contacts by name, email, or other fields.

**When to use**: Find specific contacts quickly.

**Arguments**:
- `q` (required): Search query
- `page` (optional): Page number (default 1)
- `pageSize` (optional): Results per page (default 20)

**Example LLM prompt**: "Search for contacts named John"

---

### bettercontact_list_lists

**What it does**: Lists all contact lists in the account.

**When to use**: View segmented contact groups.

**Arguments**: None required

**Example LLM prompt**: "List all contact lists"

---

### bettercontact_get_company

**What it does**: Gets company information linked to a contact.

**When to use**: View company details, check company info.

**Arguments**:
- `id` (required): Company ID

**Example LLM prompt**: "Get company details for ID 456"

---

### bettercontact_list_webhooks

**What it does**: Lists all configured webhooks.

**When to use**: View integrations, check webhook status.

**Arguments**: None required

**Example LLM prompt**: "List all webhooks"

---

## BetterContact API Notes

- **Contact ID**: Unique identifier for each contact
- **Email**: Primary identifier for contacts; must be unique
- **Search**: Supports partial matching on name and email
- **Lists**: Contacts can belong to multiple lists for segmentation
- **Company**: Companies can have multiple associated contacts
