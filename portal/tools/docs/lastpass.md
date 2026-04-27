# LastPass Tools

Provider: `lastpass` | Engine: `nango` | Auth: BASIC via Nango

## Overview

These tools wrap the LastPass API. They allow AI agents to manage shared folders, users, groups, and policies. **Requires LastPass credentials (CID + Provhash).**

## Authentication

**Basic Auth via Nango**:
- User provides LastPass CID and Provhash
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://lastpass.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `lastpass_list_shared_folders` | List shared folders | GET | /api/1.0/shared_folders |
| `lastpass_get_shared_folder` | Get shared folder details | GET | /api/1.0/shared_folders/{folder_id} |
| `lastpass_list_users` | List users | GET | /api/1.0/users |
| `lastpass_get_user` | Get user details | GET | /api/1.0/users/{user_id} |
| `lastpass_list_groups` | List groups | GET | /api/1.0/groups |
| `lastpass_get_group` | Get group details | GET | /api/1.0/groups/{group_id} |
| `lastpass_list_policies` | List policies | GET | /api/1.0/policies |
| `lastpass_list_reports` | List reports | GET | /api/1.0/reports |
| `lastpass_list_company_settings` | List company settings | GET | /api/1.0/company_settings |
| `lastpass_get_company_info` | Get company info | GET | /api/1.0/company_info |

---

## Tool Details

### lastpass_list_shared_folders

**What it does**: Lists all shared folders.

**When to use**: View shared folders, find password collections.

**Arguments**: None

**Example LLM prompt**: "List all shared folders in LastPass"

---

### lastpass_get_shared_folder

**What it does**: Gets details for a specific shared folder.

**When to use**: Get shared folder information.

**Arguments**:
- `folder_id` (required): Shared folder ID

**Example LLM prompt**: "Get details for shared folder abc123"

---

### lastpass_list_users

**What it does**: Lists all users.

**When to use**: View users, manage team members.

**Arguments**: None

**Example LLM prompt**: "List all users in LastPass"

---

### lastpass_get_user

**What it does**: Gets details for a specific user.

**When to use**: Get user information.

**Arguments**:
- `user_id` (required): User ID

**Example LLM prompt**: "Get details for user xyz789"

---

### lastpass_list_groups

**What it does**: Lists all groups.

**When to use**: View groups, manage permissions.

**Arguments**: None

**Example LLM prompt**: "List all groups in LastPass"

---

### lastpass_get_group

**What it does**: Gets details for a specific group.

**When to use**: Get group information.

**Arguments**:
- `group_id` (required): Group ID

**Example LLM prompt**: "Get details for group g1"

---

### lastpass_list_policies

**What it does**: Lists all policies.

**When to use**: View security policies.

**Arguments**: None

**Example LLM prompt**: "List all policies in LastPass"

---

### lastpass_list_reports

**What it does**: Lists all reports.

**When to use**: View audit reports.

**Arguments**: None

**Example LLM prompt**: "List all reports in LastPass"

---

### lastpass_list_company_settings

**What it does**: Lists company settings.

**When to use**: View organization settings.

**Arguments**: None

**Example LLM prompt**: "List company settings in LastPass"

---

### lastpass_get_company_info

**What it does**: Gets company information.

**When to use**: Get organization details.

**Arguments**: None

**Example LLM prompt**: "Get company info for LastPass"

---

## LastPass API Notes

- **Password Management**: Enterprise password management
- **Shared Folders**: Collections of shared credentials
- **Users**: Team members with access
- **Groups**: Organize users and permissions
- **Policies**: Security and compliance policies
