# Jamf Pro Tools

Provider: `jamf` | Engine: `nango` | Auth: OAuth2 Client Credentials via Nango

## Overview

These tools wrap the Jamf Pro API. They allow AI agents to manage computers, mobile devices, users, and policies. Jamf Pro is a mobile device management (MDM) platform for Apple devices.

## Authentication

**Nango OAuth2 Client Credentials**:
- Uses client credentials flow for server-to-server authentication
- Token stored in Nango, accessed via `connection_ref`
- Token URL: https://{instance}.jamfcloud.com/api/v1/oauth/token
- Instance configured via connection config

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `jamf_list_computers` | List computers | GET | /JSSResource/computers |
| `jamf_get_computer` | Get computer details | GET | /JSSResource/computers/id/{id} |
| `jamf_list_mobile_devices` | List mobile devices | GET | /JSSResource/mobiledevices |
| `jamf_get_mobile_device` | Get mobile device details | GET | /JSSResource/mobiledevices/id/{id} |
| `jamf_list_users` | List users | GET | /JSSResource/users |
| `jamf_get_user` | Get user details | GET | /JSSResource/users/id/{id} |
| `jamf_list_departments` | List departments | GET | /JSSResource/departments |
| `jamf_list_categories` | List categories | GET | /JSSResource/categories |
| `jamf_list_buildings` | List buildings | GET | /JSSResource/buildings |
| `jamf_list_policies` | List policies | GET | /JSSResource/policies |

---

## Tool Details

### jamf_list_computers

**What it does**: Lists all computers in Jamf Pro.

**When to use**: Browse managed computers.

**Arguments**:
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all computers"

---

### jamf_get_computer

**What it does**: Gets detailed information about a specific computer.

**When to use**: View computer details and management status.

**Arguments**:
- `id` (required): Computer ID

**Example LLM prompt**: "Get computer with ID 123"

---

### jamf_list_mobile_devices

**What it does**: Lists all mobile devices in Jamf Pro.

**When to use**: Browse managed mobile devices.

**Arguments**:
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all mobile devices"

---

### jamf_get_mobile_device

**What it does**: Gets detailed information about a specific mobile device.

**When to use**: View mobile device details.

**Arguments**:
- `id` (required): Mobile device ID

**Example LLM prompt**: "Get mobile device with ID 456"

---

### jamf_list_users

**What it does**: Lists all users in Jamf Pro.

**When to use**: Browse user accounts.

**Arguments**: None

**Example LLM prompt**: "List all users"

---

### jamf_get_user

**What it does**: Gets detailed information about a specific user.

**When to use**: View user details and devices.

**Arguments**:
- `id` (required): User ID

**Example LLM prompt**: "Get user with ID 789"

---

### jamf_list_departments

**What it does**: Lists all departments in Jamf Pro.

**When to use**: Browse organizational structure.

**Arguments**: None

**Example LLM prompt**: "List all departments"

---

### jamf_list_categories

**What it does**: Lists all categories in Jamf Pro.

**When to use**: Browse item categories.

**Arguments**: None

**Example LLM prompt**: "List all categories"

---

### jamf_list_buildings

**What it does**: Lists all buildings in Jamf Pro.

**When to use**: Browse location structure.

**Arguments**: None

**Example LLM prompt**: "List all buildings"

---

### jamf_list_policies

**What it does**: Lists all policies in Jamf Pro.

**When to use**: Browse configuration policies.

**Arguments**: None

**Example LLM prompt**: "List all policies"

---

## Jamf Pro API Notes

- **API Base URL**: https://{instance}.jamfcloud.com
- **Auth Mode**: OAuth2 Client Credentials
- **Instance**: Configured via connection config
- **Computers**: Managed Mac computers
- **Mobile Devices**: iPhones, iPads, Apple TVs
- **Users**: Jamf Pro user accounts
- **Departments**: Organizational units
- **Categories**: Item categorization
- **Buildings**: Location management
- **Policies**: Automated configuration policies
