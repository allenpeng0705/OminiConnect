# JumpCloud Tools

Provider: `jumpcloud` | Engine: `nango` | Auth: API_KEY via Nango

## Overview

These tools wrap the JumpCloud API. They allow AI agents to manage users, groups, devices, applications, and policies. **Requires JumpCloud API key.**

## Authentication

**API Key via Nango**:
- User provides their JumpCloud API key
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://console.jumpcloud.com/api`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `jumpcloud_list_users` | List users | GET | /api/users |
| `jumpcloud_get_user` | Get user details | GET | /api/users/{user_id} |
| `jumpcloud_list_groups` | List user groups | GET | /api/groups |
| `jumpcloud_get_group` | Get group details | GET | /api/groups/{group_id} |
| `jumpcloud_list_devices` | List devices | GET | /api/systems |
| `jumpcloud_get_device` | Get device details | GET | /api/systems/{device_id} |
| `jumpcloud_list_applications` | List applications | GET | /api/applications |
| `jumpcloud_list_policies` | List policies | GET | /api/policies |
| `jumpcloud_list_directories` | List directories | GET | /api/directories |
| `jumpcloud_list_sso_applications` | List SSO applications | GET | /api/ssoapplications |

---

## Tool Details

### jumpcloud_list_users

**What it does**: Lists all users in JumpCloud.

**When to use**: Find users, view user list.

**Arguments**:
- `limit` (optional): Max results (default: 20)
- `skip` (optional): Skip for pagination (default: 0)

**Example LLM prompt**: "List all users in JumpCloud"

---

### jumpcloud_get_user

**What it does**: Gets details for a specific user.

**When to use**: Get user information, view user profile.

**Arguments**:
- `user_id` (required): User ID

**Example LLM prompt**: "Get details for user abc123"

---

### jumpcloud_list_groups

**What it does**: Lists all user groups.

**When to use**: View groups, manage permissions.

**Arguments**: None

**Example LLM prompt**: "List all user groups in JumpCloud"

---

### jumpcloud_get_group

**What it does**: Gets details for a specific user group.

**When to use**: Get group information, view members.

**Arguments**:
- `group_id` (required): Group ID

**Example LLM prompt**: "Get details for group xyz789"

---

### jumpcloud_list_devices

**What it does**: Lists all devices.

**When to use**: View devices, manage endpoints.

**Arguments**:
- `limit` (optional): Max results (default: 20)
- `skip` (optional): Skip for pagination (default: 0)

**Example LLM prompt**: "List all devices in JumpCloud"

---

### jumpcloud_get_device

**What it does**: Gets details for a specific device.

**When to use**: Get device information, view device status.

**Arguments**:
- `device_id` (required): Device ID

**Example LLM prompt**: "Get details for device d1"

---

### jumpcloud_list_applications

**What it does**: Lists all applications.

**When to use**: View applications, manage access.

**Arguments**: None

**Example LLM prompt**: "List all applications in JumpCloud"

---

### jumpcloud_list_policies

**What it does**: Lists all policies.

**When to use**: View policies, manage security settings.

**Arguments**: None

**Example LLM prompt**: "List all policies in JumpCloud"

---

### jumpcloud_list_directories

**What it does**: Lists all directories.

**When to use**: View directories, manage directory services.

**Arguments**: None

**Example LLM prompt**: "List all directories in JumpCloud"

---

### jumpcloud_list_sso_applications

**What it does**: Lists all SSO applications.

**When to use**: View SSO apps, manage single sign-on.

**Arguments**: None

**Example LLM prompt**: "List all SSO applications in JumpCloud"

---

## JumpCloud API Notes

- **IDs**: String IDs for users, groups, devices
- **Pagination**: Use limit and skip parameters
- **Directories**: Active Directory or LDAP directories
- **SSO Applications**: SAML/OIDC applications for single sign-on
