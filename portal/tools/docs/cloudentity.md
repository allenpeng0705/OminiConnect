# Cloudentity Tools

Provider: `cloudentity` | Engine: `nango` | Auth: OAuth2 Client Credentials via Nango

## Overview

These tools wrap the Cloudentity API. Cloudentity is an identity and access management platform for securing APIs and applications. **Requires Cloudentity OAuth2 Client Credentials.**

## Authentication

**Nango OAuth2_CC**:
- Uses Client Credentials flow for server-to-server integration
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://${connectionConfig.tenantID}.${connectionConfig.regionID}.authz.cloudentity.io/${connectionConfig.tenantID}/${connectionConfig.workspaceID}`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `cloudentity_list_users` | List users | GET | /admin/users |
| `cloudentity_get_user` | Get user details | GET | /admin/users/{id} |
| `cloudentity_create_user` | Create a user | POST | /admin/users |
| `cloudentity_delete_user` | Delete a user | DELETE | /admin/users/{id} |
| `cloudentity_list_applications` | List applications | GET | /admin/applications |
| `cloudentity_get_application` | Get application details | GET | /admin/applications/{id} |
| `cloudentity_create_application` | Create an application | POST | /admin/applications |
| `cloudentity_list_authorizations` | List authorizations | GET | /admin/authorizations |
| `cloudentity_get_authorization` | Get authorization details | GET | /admin/authorizations/{id} |
| `cloudentity_list_identities` | List identities | GET | /admin/identities |

---

## Tool Details

### cloudentity_list_users

**What it does**: Lists all users in the tenant.

**When to use**: View all identity users.

**Arguments**:
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List all Cloudentity users"

---

### cloudentity_get_user

**What it does**: Gets details of a specific user.

**When to use**: View user profile and attributes.

**Arguments**:
- `id` (required): User ID

**Example LLM prompt**: "Get user 123 details"

---

### cloudentity_create_user

**What it does**: Creates a new user.

**When to use**: Provision a new identity.

**Arguments**:
- `username` (required): Username
- `email` (required): Email address
- `password` (optional): Password

**Example LLM prompt**: "Create a user johndoe with email john@example.com"

---

### cloudentity_delete_user

**What it does**: Deletes a user.

**When to use**: Remove an identity.

**Arguments**:
- `id` (required): User ID

**Example LLM prompt**: "Delete user 123"

---

### cloudentity_list_applications

**What it does**: Lists all applications.

**When to use**: View registered applications.

**Arguments**:
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List all applications"

---

### cloudentity_get_application

**What it does**: Gets details of a specific application.

**When to use**: View application configuration.

**Arguments**:
- `id` (required): Application ID

**Example LLM prompt**: "Get application 123 details"

---

### cloudentity_create_application

**What it does**: Creates a new application.

**When to use**: Register a new application.

**Arguments**:
- `name` (required): Application name
- `type` (optional): Application type

**Example LLM prompt**: "Create an application called 'My App'"

---

### cloudentity_list_authorizations

**What it does**: Lists all authorizations.

**When to use**: View access control policies.

**Arguments**:
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List all authorizations"

---

### cloudentity_get_authorization

**What it does**: Gets details of a specific authorization.

**When to use**: View authorization policy.

**Arguments**:
- `id` (required): Authorization ID

**Example LLM prompt**: "Get authorization 456 details"

---

### cloudentity_list_identities

**What it does**: Lists all identities.

**When to use**: View all identities in the system.

**Arguments**:
- `limit` (optional): Max results (default 20)

**Example LLM prompt**: "List all identities"

---

## Cloudentity API Notes

- **Tenant ID**: Identifies your Cloudentity tenant
- **Region ID**: Specifies your deployment region
- **Workspace ID**: Defines your workspace
- **Users**: Identity users within the tenant
- **Applications**: Registered client applications
- **Authorizations**: Access control policies
