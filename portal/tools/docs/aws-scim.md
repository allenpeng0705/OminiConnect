# AWS SCIM Tools

Provider: `aws-scim` | Engine: `nango` | Auth: API_KEY via Nango (SCIM Access Token)

## Overview

These tools wrap the AWS IAM Identity Center SCIM API. They allow AI agents to manage users and groups via the SCIM 2.0 protocol for automated provisioning and deprovisioning. AWS IAM Identity Center (formerly AWS SSO) provides centralized identity management.

## Authentication

**Nango API_KEY**:
- User provides SCIM Access Token generated from AWS IAM Identity Center
- Token stored in Nango, accessed via `connection_ref`
- Bearer token in Authorization header
- Requires SCIM Endpoint URL configuration

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `aws_scim_list_users` | List users via SCIM | GET | /Users |
| `aws_scim_get_user` | Get user details | GET | /Users/{id} |
| `aws_scim_create_user` | Create a new user | POST | /Users |
| `aws_scim_update_user` | Update user attributes | PUT | /Users/{id} |
| `aws_scim_delete_user` | Delete a user | DELETE | /Users/{id} |
| `aws_scim_list_groups` | List groups via SCIM | GET | /Groups |
| `aws_scim_get_group` | Get group details | GET | /Groups/{id} |
| `aws_scim_create_group` | Create a new group | POST | /Groups |
| `aws_scim_update_group` | Update group members | PUT | /Groups/{id} |
| `aws_scim_list_service_providers` | List SCIM service provider info | GET | /ServiceProviderConfig |

---

## Tool Details

### aws_scim_list_users

**What it does**: Lists all users in AWS IAM Identity Center via SCIM.

**When to use**: Sync user directory, find users for provisioning.

**Arguments**:
- `filter` (optional): SCIM filter expression
- `startIndex` (optional): Pagination start index (default 1)
- `count` (optional): Number of results (default 20)

**Example LLM prompt**: "List all users in AWS IAM Identity Center"

---

### aws_scim_get_user

**What it does**: Gets detailed information about a specific user.

**When to use**: Check user attributes, verify provisioning status.

**Arguments**:
- `id` (required): User ID

**Example LLM prompt**: "Get details for user ID abc-123"

---

### aws_scim_create_user

**What it does**: Creates a new user in AWS IAM Identity Center.

**When to use**: Provision new users automatically, onboard employees.

**Arguments**:
- `userName` (required): Username (must be unique)
- `name` (optional): Name object with givenName and familyName
- `emails` (optional): Email addresses array
- `active` (optional): User active status (default true)

**Example LLM prompt**: "Create a new user john.doe@company.com with name John Doe"

---

### aws_scim_update_user

**What it does**: Updates user attributes in AWS IAM Identity Center.

**When to use**: Modify user information, activate/deactivate users.

**Arguments**:
- `id` (required): User ID
- `userName` (optional): Username
- `name` (optional): Name object
- `emails` (optional): Email addresses
- `active` (optional): User active status

**Example LLM prompt**: "Deactivate user ID abc-123"

---

### aws_scim_delete_user

**What it does**: Deletes a user from AWS IAM Identity Center.

**When to use**: Deprovision users, remove access for departed employees.

**Arguments**:
- `id` (required): User ID

**Example LLM prompt**: "Delete user ID abc-123"

---

### aws_scim_list_groups

**What it does**: Lists all groups in AWS IAM Identity Center via SCIM.

**When to use**: Sync group memberships, find groups for provisioning.

**Arguments**:
- `filter` (optional): SCIM filter expression
- `startIndex` (optional): Pagination start index (default 1)
- `count` (optional): Number of results (default 20)

**Example LLM prompt**: "List all groups"

---

### aws_scim_get_group

**What it does**: Gets details of a specific group including members.

**When to use**: Check group membership, verify permissions.

**Arguments**:
- `id` (required): Group ID

**Example LLM prompt**: "Get details for group ID g-456"

---

### aws_scim_create_group

**What it does**: Creates a new group in AWS IAM Identity Center.

**When to use**: Create new permission groups, organize users.

**Arguments**:
- `displayName` (required): Group display name
- `members` (optional): Array of member IDs

**Example LLM prompt**: "Create a group called 'Developers'"

---

### aws_scim_update_group

**What it does**: Updates group membership in AWS IAM Identity Center.

**When to use**: Add or remove members from groups, manage permissions.

**Arguments**:
- `id` (required): Group ID
- `displayName` (optional): Group display name
- `members` (optional): Array of member IDs

**Example LLM prompt**: "Add user abc-123 to group g-456"

---

### aws_scim_list_service_providers

**What it does**: Gets SCIM service provider configuration.

**When to use**: Verify SCIM capabilities, check supported features.

**Arguments**: None required

**Example LLM prompt**: "Get SCIM service provider configuration"

---

## AWS SCIM API Notes

- **SCIM 2.0**: Uses standard SCIM protocol for user/group management
- **User IDs**: Are opaque identifiers generated by the service
- **Filtering**: Supports SCIM filter expressions for querying
- **Pagination**: Uses startIndex and count parameters
- **Group Members**: Users are referenced by their SCIM IDs
