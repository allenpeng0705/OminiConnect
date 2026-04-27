# Passportal Tools

Provider: `passportal` | Engine: `nango` | Auth: Two-Step via Nango

## Overview

These tools wrap the Passportal password management API. They allow AI agents to manage clients, sites, passwords, and secure notes. **Requires Passportal Two-Step authentication.**

## Authentication

**Two-Step**:
- Uses Passportal's two-step token generation
- API Key and HMAC Token passed via Nango
- Token URL: `https://{base_url}/api/v2/auth/client_token`
- Base URL: User's Passportal instance

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `passportal_list_clients` | List clients | GET | /api/v2/clients |
| `passportal_get_client` | Get client details | GET | /api/v2/clients/{id} |
| `passportal_list_sites` | List sites | GET | /api/v2/sites |
| `passportal_get_site` | Get site details | GET | /api/v2/sites/{id} |
| `passportal_list_passwords` | List passwords | GET | /api/v2/passwords |
| `passportal_get_password` | Get password details | GET | /api/v2/passwords/{id} |
| `passportal_create_password` | Create password entry | POST | /api/v2/passwords |
| `passportal_list_notes` | List notes | GET | /api/v2/notes |
| `passportal_get_note` | Get note details | GET | /api/v2/notes/{id} |
| `passportal_list_tags` | List tags | GET | /api/v2/tags |

---

## Tool Details

### passportal_list_clients

**What it does**: Lists all clients in Passportal.

**When to use**: Browse clients, find organizations.

**Arguments**: None

**Example LLM prompt**: "List all clients"

---

### passportal_get_client

**What it does**: Gets detailed information for a specific client.

**When to use**: View client details, associated sites.

**Arguments**:
- `id` (required): Client ID

**Example LLM prompt**: "Get details for client 123"

---

### passportal_list_sites

**What it does**: Lists all sites in Passportal.

**When to use**: Browse sites, organize passwords.

**Arguments**:
- `client_id` (optional): Filter by client ID

**Example LLM prompt**: "List all sites for client 123"

---

### passportal_get_site

**What it does**: Gets detailed information for a specific site.

**When to use**: View site settings, associated passwords.

**Arguments**:
- `id` (required): Site ID

**Example LLM prompt**: "Get details for site 456"

---

### passportal_list_passwords

**What it does**: Lists all passwords in Passportal.

**When to use**: Browse credentials, find passwords.

**Arguments**:
- `site_id` (optional): Filter by site ID

**Example LLM prompt**: "List all passwords for site 456"

---

### passportal_get_password

**What it does**: Gets detailed information for a specific password entry.

**When to use**: View credentials, retrieve password.

**Arguments**:
- `id` (required): Password ID

**Example LLM prompt**: "Get details for password 789"

---

### passportal_create_password

**What it does**: Creates a new password entry.

**When to use**: Add new credentials, store passwords.

**Arguments**:
- `site_id` (required): Site ID
- `name` (required): Password name
- `username` (optional): Username
- `password` (optional): Password

**Example LLM prompt**: "Create a password entry for site 456"

---

### passportal_list_notes

**What it does**: Lists all notes in Passportal.

**When to use**: Browse secure notes, find information.

**Arguments**:
- `site_id` (optional): Filter by site ID

**Example LLM prompt**: "List all notes for site 456"

---

### passportal_get_note

**What it does**: Gets detailed information for a specific note.

**When to use**: View note content, secrets.

**Arguments**:
- `id` (required): Note ID

**Example LLM prompt**: "Get details for note 101"

---

### passportal_list_tags

**What it does**: Lists all tags in Passportal.

**When to use**: Organize credentials, find by tag.

**Arguments**: None

**Example LLM prompt**: "List all tags"

---

## Passportal Notes

- **Password management**: Secure credential storage
- **Clients**: Top-level organization units
- **Sites**: Group related passwords
- **Two-step auth**: API Key + HMAC token required
- **Tags**: Organize and filter credentials
