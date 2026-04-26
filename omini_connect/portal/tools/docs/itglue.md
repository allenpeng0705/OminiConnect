# IT Glue Tools

Provider: `itglue` | Engine: `nango` | Auth: API Key via Nango

## Overview

These tools wrap the IT Glue API. They allow AI agents to manage organizations, configurations, passwords, and flexible assets. IT Glue is a documentation platform for IT Managed Service Providers.

## Authentication

**Nango API Key**:
- User provides API key via Nango Connect
- Key passed in X-Api-Key header
- Token stored in Nango, accessed via `connection_ref`
- Base URL: https://{subdomain}.itglue.com
- Content-Type: application/vnd.api+json

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `itglue_list_organizations` | List organizations | GET | /organizations |
| `itglue_get_organization` | Get organization details | GET | /organizations/{id} |
| `itglue_list_configs` | List configurations | GET | /configurations |
| `itglue_get_config` | Get configuration details | GET | /configurations/{id} |
| `itglue_list_passwords` | List passwords | GET | /passwords |
| `itglue_get_password` | Get password details | GET | /passwords/{id} |
| `itglue_list_exports` | List exports | GET | /exports |
| `itglue_get_export` | Get export details | GET | /exports/{id} |
| `itglue_list_flexible_assets` | List flexible assets | GET | /flexible_assets |
| `itglue_list_users` | List users | GET | /users |

---

## Tool Details

### itglue_list_organizations

**What it does**: Lists all organizations in IT Glue.

**When to use**: Browse client organizations.

**Arguments**:
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all organizations"

---

### itglue_get_organization

**What it does**: Gets detailed information about a specific organization.

**When to use**: View organization details and configurations.

**Arguments**:
- `id` (required): Organization ID

**Example LLM prompt**: "Get organization with ID 123"

---

### itglue_list_configs

**What it does**: Lists all configurations in IT Glue.

**When to use**: Browse configurations.

**Arguments**:
- `organization_id` (optional): Filter by organization ID
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all configurations for organization 123"

---

### itglue_get_config

**What it does**: Gets detailed information about a specific configuration.

**When to use**: View configuration details and passwords.

**Arguments**:
- `id` (required): Configuration ID

**Example LLM prompt**: "Get configuration with ID 456"

---

### itglue_list_passwords

**What it does**: Lists all passwords in IT Glue.

**When to use**: Browse password database.

**Arguments**:
- `organization_id` (optional): Filter by organization ID

**Example LLM prompt**: "List all passwords"

---

### itglue_get_password

**What it does**: Gets detailed information about a specific password.

**When to use**: View password details.

**Arguments**:
- `id` (required): Password ID

**Example LLM prompt**: "Get password with ID 789"

---

### itglue_list_exports

**What it does**: Lists all exports in IT Glue.

**When to use**: View export history.

**Arguments**: None

**Example LLM prompt**: "List all exports"

---

### itglue_get_export

**What it does**: Gets detailed information about a specific export.

**When to use**: Get export download URL.

**Arguments**:
- `id` (required): Export ID

**Example LLM prompt**: "Get export with ID 101"

---

### itglue_list_flexible_assets

**What it does**: Lists all flexible assets in IT Glue.

**When to use**: Browse custom data structures.

**Arguments**:
- `organization_id` (optional): Filter by organization ID
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all flexible assets"

---

### itglue_list_users

**What it does**: Lists all users in IT Glue.

**When to use**: View team members.

**Arguments**: None

**Example LLM prompt**: "List all users"

---

## IT Glue API Notes

- **API Base URL**: https://{subdomain}.itglue.com
- **Auth Mode**: API Key in X-Api-Key header
- **Content-Type**: application/vnd.api+json
- **Organizations**: Client organizations
- **Configurations**: IT assets and devices
- **Passwords**: Credential storage
- **Flexible Assets**: Custom data structures
- **Exports**: Data export files
- **Users**: Team members
