# Lob Tools

Provider: `lob` | Engine: `nango` | Auth: BASIC via Nango

## Overview

These tools wrap the Lob API. They allow AI agents to manage addresses, letters, and postcards for direct mail. **Requires Lob API key (Secret API Key).**

## Authentication

**Basic Auth via Nango**:
- User provides Lob Secret API Key (as username)
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://api.lob.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `lob_list_addresses` | List addresses | GET | /v1/addresses |
| `lob_get_address` | Get address details | GET | /v1/addresses/{address_id} |
| `lob_create_address` | Create an address | POST | /v1/addresses |
| `lob_delete_address` | Delete an address | DELETE | /v1/addresses/{address_id} |
| `lob_list_letters` | List letters | GET | /v1/letters |
| `lob_get_letter` | Get letter details | GET | /v1/letters/{letter_id} |
| `lob_create_letter` | Create a letter | POST | /v1/letters |
| `lob_list_postcards` | List postcards | GET | /v1/postcards |
| `lob_get_postcard` | Get postcard details | GET | /v1/postcards/{postcard_id} |
| `lob_create_postcard` | Create a postcard | POST | /v1/postcards |

---

## Tool Details

### lob_list_addresses

**What it does**: Lists all addresses.

**When to use**: Find addresses, view address book.

**Arguments**:
- `limit` (optional): Max results (default: 20)
- `offset` (optional): Offset for pagination (default: 0)

**Example LLM prompt**: "List all addresses in Lob"

---

### lob_get_address

**What it does**: Gets details for a specific address.

**When to use**: Get address information.

**Arguments**:
- `address_id` (required): Address ID

**Example LLM prompt**: "Get details for address abc123"

---

### lob_create_address

**What it does**: Creates a new address.

**When to use**: Add addresses to address book.

**Arguments**:
- `name` (optional): Recipient name
- `address_line1` (required): Street address
- `address_city` (required): City
- `address_state` (required): State
- `address_zip` (required): ZIP code
- `address_country` (optional): Country

**Example LLM prompt**: "Create an address for John Doe at 123 Main St, Chicago, IL 60601"

---

### lob_delete_address

**What it does**: Deletes an address.

**When to use**: Remove addresses.

**Arguments**:
- `address_id` (required): Address ID

**Example LLM prompt**: "Delete address abc123"

---

### lob_list_letters

**What it does**: Lists all letters.

**When to use**: View letter history.

**Arguments**:
- `limit` (optional): Max results (default: 20)

**Example LLM prompt**: "List all letters in Lob"

---

### lob_get_letter

**What it does**: Gets details for a specific letter.

**When to use**: Get letter information.

**Arguments**:
- `letter_id` (required): Letter ID

**Example LLM prompt**: "Get details for letter xyz789"

---

### lob_create_letter

**What it does**: Creates a new letter.

**When to use**: Send letters via direct mail.

**Arguments**:
- `to_address` (required): To address object
- `from_address` (required): From address object
- `file` (required): HTML or PDF file content

**Example LLM prompt**: "Create a letter to send to John"

---

### lob_list_postcards

**What it does**: Lists all postcards.

**When to use**: View postcard history.

**Arguments**:
- `limit` (optional): Max results (default: 20)

**Example LLM prompt**: "List all postcards in Lob"

---

### lob_get_postcard

**What it does**: Gets details for a specific postcard.

**When to use**: Get postcard information.

**Arguments**:
- `postcard_id` (required): Postcard ID

**Example LLM prompt**: "Get details for postcard p1"

---

### lob_create_postcard

**What it does**: Creates a new postcard.

**When to use**: Send postcards via direct mail.

**Arguments**:
- `to_address` (required): To address object
- `from_address` (required): From address object
- `front` (required): Front HTML or image URL
- `back` (required): Back HTML or image URL

**Example LLM prompt**: "Create a postcard to send to John"

---

## Lob API Notes

- **Direct Mail**: Print and mail service API
- **Addresses**: Address book management
- **Letters**: Send letters with custom content
- **Postcards**: Send postcard mailings
- **Live/Test Mode**: Use live API keys for real mail
