# Clerk Tools

Provider: `clerk` | Engine: `nango` | Auth: API Key via Nango

## Overview

These tools wrap the Clerk API. Clerk is an identity and user management platform for authentication. **Requires Clerk Secret Key.**

## Authentication

**Nango API_KEY**:
- User provides their Clerk Secret Key
- Token passed via `Authorization: Bearer` header
- Base URL: `https://api.clerk.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `clerk_list_users` | List users | GET | /v1/users |
| `clerk_get_user` | Get user details | GET | /v1/users/{id} |
| `clerk_create_user` | Create a user | POST | /v1/users |
| `clerk_update_user` | Update a user | PATCH | /v1/users/{id} |
| `clerk_delete_user` | Delete a user | DELETE | /v1/users/{id} |
| `clerk_list_organizations` | List organizations | GET | /v1/organizations |
| `clerk_get_organization` | Get organization details | GET | /v1/organizations/{id} |
| `clerk_list_sessions` | List sessions | GET | /v1/sessions |
| `clerk_get_session` | Get session details | GET | /v1/sessions/{id} |
| `clerk_create_organization` | Create an organization | POST | /v1/organizations |

---

## Tool Details

### clerk_list_users

**What it does**: Lists all users in the Clerk instance.

**When to use**: View all registered users.

**Arguments**:
- `limit` (optional): Max results (default 20)
- `offset` (optional): Pagination offset (default 0)

**Example LLM prompt**: "List all Clerk users"

---

### clerk_get_user

**What it does**: Gets details of a specific user.

**When to use**: View user profile and metadata.

**Arguments**:
- `id` (required): User ID

**Example LLM prompt**: "Get user 123 details"

---

### clerk_create_user

**What it does**: Creates a new user.

**When to use**: Provision a new user account.

**Arguments**:
- `email_addresses` (required): Array of email addresses
- `phone_numbers` (optional): Array of phone numbers
- `password` (optional): Password
- `first_name` (optional): First name
- `last_name` (optional): Last name

**Example LLM prompt**: "Create a new user with email john@example.com"

---

### clerk_update_user

**What it does**: Updates an existing user.

**When to use**: Modify user information.

**Arguments**:
- `id` (required): User ID
- `first_name` (optional): New first name
- `last_name` (optional): New last name

**Example LLM prompt**: "Update user 123 first name to John"

---

### clerk_delete_user

**What it does**: Deletes a user.

**When to use**: Remove a user account.

**Arguments**:
- `id` (required): User ID

**Example LLM prompt**: "Delete user 123"

---

### clerk_list_organizations

**What it does**: Lists all organizations.

**When to use**: View all organizations.

**Arguments**:
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List all organizations"

---

### clerk_get_organization

**What it does**: Gets details of a specific organization.

**When to use**: View organization members and settings.

**Arguments**:
- `id` (required): Organization ID

**Example LLM prompt**: "Get organization 456 details"

---

### clerk_list_sessions

**What it does**: Lists all sessions.

**When to use**: View active user sessions.

**Arguments**:
- `user_id` (optional): Filter by user ID

**Example LLM prompt**: "List sessions for user 123"

---

### clerk_get_session

**What it does**: Gets details of a specific session.

**When to use**: View session status and activity.

**Arguments**:
- `id` (required): Session ID

**Example LLM prompt**: "Get session 789 details"

---

### clerk_create_organization

**What it does**: Creates a new organization.

**When to use**: Create a new organization.

**Arguments**:
- `name` (required): Organization name
- `created_by` (optional): User ID of creator

**Example LLM prompt**: "Create an organization called Acme Corp"

---

## Clerk API Notes

- **Secret Key Format**: Starts with `sk_test_` or `sk_live_`
- **Users**: User accounts with email/phone authentication
- **Organizations**: Multi-tenant organization structure
- **Sessions**: Active authentication sessions
- **JWKS Endpoint**: `/v1/jwks` for token verification
