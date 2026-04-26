# Yahoo Tools

Provider: `yahoo` | Engine: `nango` | Auth: OAuth2 via Nango

## Overview

Yahoo is an email and web services platform. **Requires oauth2 via nango.**

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Yahoo
- Token stored in Nango, accessed via `connection_ref`
- Scopes: read, write

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `yahoo_get_user` | Get current user info | GET | /profile |
| `yahoo_list_emails` | List emails in inbox | GET | /messages |
| `yahoo_get_email` | Get email details | GET | /messages/{id} |
| `yahoo_send_email` | Send an email | POST | /messages |
| `yahoo_delete_email` | Delete an email | DELETE | /messages/{id} |
| `yahoo_list_folders` | List all folders | GET | /folders |
| `yahoo_list_contacts` | List all contacts | GET | /contacts |
| `yahoo_get_contact` | Get contact details | GET | /contacts/{id} |
| `yahoo_create_contact` | Create a new contact | POST | /contacts |
| `yahoo_list_calendars` | List all calendars | GET | /calendars |

---

## Tool Details

### yahoo_get_user

**What it does**: Get current user info

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### yahoo_list_emails

**What it does**: List emails in inbox

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### yahoo_get_email

**What it does**: Get email details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### yahoo_send_email

**What it does**: Send an email

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### yahoo_delete_email

**What it does**: Delete an email

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### yahoo_list_folders

**What it does**: List all folders

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### yahoo_list_contacts

**What it does**: List all contacts

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### yahoo_get_contact

**What it does**: Get contact details

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### yahoo_create_contact

**What it does**: Create a new contact

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### yahoo_list_calendars

**What it does**: List all calendars

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---


## API Notes

- Base URL: `https://${connectionConfig.apiDomain}`
- Docs: https://nango.dev/docs/integrations/all/yahoo
