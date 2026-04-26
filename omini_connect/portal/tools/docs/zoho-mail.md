# Zoho Mail Tools

Provider: `zoho-mail` | Engine: `nango` | Auth: OAuth2 via Nango

## Overview

Zoho Mail is an email hosting service for businesses. **Requires oauth2 via nango.**

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Zoho Mail
- Token stored in Nango, accessed via `connection_ref`
- Scopes: read, write

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `zoho_mail_list_emails` | List emails in inbox | GET | /api/v1/accounts/{account_id}/messages |
| `zoho_mail_get_email` | Get email details | GET | /api/v1/accounts/{account_id}/messages/{message_id} |
| `zoho_mail_send_email` | Send an email | POST | /api/v1/accounts/{account_id}/messages |
| `zoho_mail_delete_email` | Delete an email | DELETE | /api/v1/accounts/{account_id}/messages/{message_id} |
| `zoho_mail_list_folders` | List all folders | GET | /api/v1/accounts/{account_id}/folders |
| `zoho_mail_list_contacts` | List all contacts | GET | /api/v1/accounts/{account_id}/contacts |
| `zoho_mail_get_contact` | Get contact details | GET | /api/v1/accounts/{account_id}/contacts/{contact_id} |
| `zoho_mail_create_contact` | Create a new contact | POST | /api/v1/accounts/{account_id}/contacts |
| `zoho_mail_list_signature` | Get email signature | GET | /api/v1/accounts/{account_id}/signature |
| `zoho_mail_search_emails` | Search for emails | GET | /api/v1/accounts/{account_id}/search |

---

## Tool Details

### zoho_mail_list_emails

**What it does**: List emails in inbox

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_mail_get_email

**What it does**: Get email details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_mail_send_email

**What it does**: Send an email

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_mail_delete_email

**What it does**: Delete an email

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_mail_list_folders

**What it does**: List all folders

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_mail_list_contacts

**What it does**: List all contacts

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_mail_get_contact

**What it does**: Get contact details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_mail_create_contact

**What it does**: Create a new contact

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_mail_list_signature

**What it does**: Get email signature

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### zoho_mail_search_emails

**What it does**: Search for emails

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---


## API Notes

- Base URL: `https://mail.zoho.${connectionConfig.extension}`
- Docs: https://nango.dev/docs/integrations/all/zoho-mail
