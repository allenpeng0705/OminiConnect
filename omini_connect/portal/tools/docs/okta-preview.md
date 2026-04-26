# Okta (Preview) Tools

Provider: `okta-preview` | Engine: `nango` | Auth: OAuth via Nango (Preview Environment)

## Overview

These tools wrap the Okta Preview API for identity and access management. They allow AI agents to manage users, groups, applications, and view audit logs in the Okta Preview environment. **Requires Okta Preview OAuth authentication.**

## Authentication

**OAuth2**:
- User authenticates via Nango Connect with Okta Preview
- Authorization URL: `https://{subdomain}.oktapreview.com/oauth2/v1/authorize`
- Token URL: `https://{subdomain}.oktapreview.com/oauth2/v1/token`
- Base URL: `https://{subdomain}.oktapreview.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `okta_list_users` | List users | GET | /api/v1/users |
| `okta_get_user` | Get user details | GET | /api/v1/users/{userId} |
| `okta_create_user` | Create user | POST | /api/v1/users |
| `okta_update_user` | Update user | PUT | /api/v1/users/{userId} |
| `okta_list_groups` | List groups | GET | /api/v1/groups |
| `okta_get_group` | Get group details | GET | /api/v1/groups/{groupId} |
| `okta_list_applications` | List applications | GET | /api/v1/apps |
| `okta_get_application` | Get application details | GET | /api/v1/apps/{appName}/{appId} |
| `okta_list_factors` | List factors | GET | /api/v1/users/{userId}/factors |
| `okta_get_logs` | Get system logs | GET | /api/v1/logs |

---

## Tool Details

### okta_list_users

**What it does**: Lists all users in Okta Preview.

**When to use**: Browse user directory, find users by attribute.

**Arguments**:
- `filter` (optional): Filter expression
- `limit` (optional): Number of users (default 100)

**Example LLM prompt**: "List all active users in Okta Preview"

---

### okta_get_user

**What it does**: Gets detailed information for a specific user.

**When to use**: View user profile, status, group memberships.

**Arguments**:
- `userId` (required): User ID or email

**Example LLM prompt**: "Get details for user john@company.com"

---

### okta_create_user

**What it does**: Creates a new user in Okta Preview.

**When to use**: Provision new users, onboard employees.

**Arguments**:
- `profile` (required): User profile object
- `credentials` (optional): User credentials

**Example LLM prompt**: "Create a new user for Jane Doe with email jane@company.com"

---

### okta_update_user

**What it does**: Updates an existing user.

**When to use**: Modify user attributes, change status.

**Arguments**:
- `userId` (required): User ID or email
- `profile` (optional): User profile object

**Example LLM prompt**: "Update user john@company.com department to Engineering"

---

### okta_list_groups

**What it does**: Lists all groups in Okta Preview.

**When to use**: Browse group directory, find groups.

**Arguments**:
- `limit` (optional): Number of groups (default 100)

**Example LLM prompt**: "List all groups in Okta Preview"

---

### okta_get_group

**What it does**: Gets detailed information for a specific group.

**When to use**: View group members, group settings.

**Arguments**:
- `groupId` (required): Group ID

**Example LLM prompt**: "Get details for group Engineers"

---

### okta_list_applications

**What it does**: Lists all applications in Okta Preview.

**When to use**: Browse app catalog, find applications.

**Arguments**:
- `limit` (optional): Number of apps (default 100)

**Example LLM prompt**: "List all SAML applications"

---

### okta_get_application

**What it does**: Gets detailed information for a specific application.

**When to use**: View app settings, assignment status.

**Arguments**:
- `appName` (required): Application name
- `appId` (required): Application ID

**Example LLM prompt**: "Get details for app 0oa123456"

---

### okta_list_factors

**What it does**: Lists all factors for a user.

**When to use**: View enrolled MFA factors, verify second factor.

**Arguments**:
- `userId` (required): User ID

**Example LLM prompt**: "List all factors for user john@company.com"

---

### okta_get_logs

**What it does**: Gets system logs from Okta Preview.

**When to use**: Audit user activity, security investigations.

**Arguments**:
- `since` (optional): Start time (ISO 8601)
- `until` (optional): End time (ISO 8601)
- `filter` (optional): Filter expression

**Example LLM prompt**: "Get logs for user john@company.com from yesterday"

---

## Okta Preview Notes

- **Preview Environment**: Uses oktapreview.com domain
- **Subdomain**: Your Okta Preview organization subdomain
- **Feature parity**: Preview may have newer API features
- **Testing**: Use for testing before production deployment
- **Rate limits**: Respects x-rate-limit-reset header
