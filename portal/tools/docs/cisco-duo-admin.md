# Cisco Duo Admin API Tools

Provider: `cisco-duo-admin` | Engine: `nango` | Auth: BASIC via Nango

## Overview

These tools wrap the Cisco Duo Admin API. Duo is a two-factor authentication and secure access platform. **Requires Cisco Duo admin credentials.**

## Authentication

**Nango BASIC**:
- User provides integration key and secret
- Uses HMAC-signed requests for authentication
- Base URL: `https://${connectionConfig.hostname}`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `cisco_duo_list_users` | List users | GET | /admin/v1/users |
| `cisco_duo_get_user` | Get user details | GET | /admin/v1/users/{id} |
| `cisco_duo_create_user` | Create a user | POST | /admin/v1/users |
| `cisco_duo_delete_user` | Delete a user | DELETE | /admin/v1/users/{id} |
| `cisco_duo_list_phones` | List phones | GET | /admin/v1/phones |
| `cisco_duo_get_phone` | Get phone details | GET | /admin/v1/phones/{id} |
| `cisco_duo_list_auth_logs` | List auth logs | GET | /admin/v1/logs/authentication |
| `cisco_duo_list_administrators` | List admins | GET | /admin/v1/administrators |
| `cisco_duo_get_auth_log_entry` | Get auth log entry | GET | /admin/v1/logs/authentication/{id} |
| `cisco_duo_list_directory_users` | List directory users | GET | /admin/v1/directory/users |

---

## Tool Details

### cisco_duo_list_users

**What it does**: Lists all users in the Duo directory.

**When to use**: Find users, view user count.

**Arguments**:
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List all Duo users"

---

### cisco_duo_get_user

**What it does**: Gets details of a specific user.

**When to use**: View user settings and enrolled devices.

**Arguments**:
- `id` (required): User ID

**Example LLM prompt**: "Get user 123 details"

---

### cisco_duo_create_user

**What it does**: Creates a new user in Duo.

**When to use**: Add a new user for 2FA.

**Arguments**:
- `username` (required): Username
- `realname` (optional): Real name
- `email` (optional): Email address

**Example LLM prompt**: "Create a new user johndoe in Duo"

---

### cisco_duo_delete_user

**What it does**: Deletes a user from Duo.

**When to use**: Remove a user from 2FA.

**Arguments**:
- `id` (required): User ID

**Example LLM prompt**: "Delete user 123 from Duo"

---

### cisco_duo_list_phones

**What it does**: Lists all phones registered in Duo.

**When to use**: View enrolled mobile devices.

**Arguments**:
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List all Duo phones"

---

### cisco_duo_get_phone

**What it does**: Gets details of a specific phone.

**When to use**: View phone enrollment status.

**Arguments**:
- `id` (required): Phone ID

**Example LLM prompt**: "Get phone 456 details"

---

### cisco_duo_list_auth_logs

**What it does**: Lists all authentication logs.

**When to use**: Review authentication attempts.

**Arguments**:
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List Duo auth logs from today"

---

### cisco_duo_list_administrators

**What it does**: Lists all Duo administrators.

**When to use**: View admin accounts.

**Arguments**:
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List all Duo administrators"

---

### cisco_duo_get_auth_log_entry

**What it does**: Gets details of a specific auth log entry.

**When to use**: Investigate a specific authentication.

**Arguments**:
- `id` (required): Log entry ID

**Example LLM prompt**: "Get auth log entry 789 details"

---

### cisco_duo_list_directory_users

**What it does**: Lists all users synced from directory.

**When to use**: View directory-synced users.

**Arguments**:
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List directory users"

---

## Cisco Duo Admin API Notes

- **HMAC Signing**: Requires request signing with integration secret
- **Hostname**: API hostname from Duo admin panel
- **Users**: Users enrolled in Duo 2FA
- **Phones**: Mobile devices enrolled for authentication
- **Auth Logs**: All authentication attempts and results
