# Lumos Tools

Provider: `lumos` | Engine: `nango` | Auth: API_KEY via Nango

## Overview

These tools wrap the Lumos API. They allow AI agents to manage ITAM (IT Asset Management) requests, approvals, and workflows. **Requires Lumos API token.**

## Authentication

**Nango API_KEY**:
- User provides Lumos API token via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Header: `authorization: Bearer ${apiKey}`
- Base URL: `https://api.lumos.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `lumos_list_apps` | List all apps | GET | /apps |
| `lumos_get_app` | Get app details | GET | /apps/{appId} |
| `lumos_list_users` | List users | GET | /users |
| `lumos_get_user` | Get user details | GET | /users/{userId} |
| `lumos_list_requests` | List access requests | GET | /requests |
| `lumos_get_request` | Get request details | GET | /requests/{requestId} |
| `lumos_approve_request` | Approve access request | POST | /requests/{requestId}/approve |
| `lumos_deny_request` | Deny access request | POST | /requests/{requestId}/deny |
| `lumos_list_systems` | List systems | GET | /systems |
| `lumos_get_system` | Get system details | GET | /systems/{systemId} |

---

## Tool Details

### lumos_list_apps

**What it does**: Lists all applications in Lumos.

**When to use**: Browse available applications, find app IDs.

**Arguments**:
- `status` (optional): Filter by status (active, inactive)
- `page` (optional): Page number (default 1)
- `per_page` (optional): Results per page (default 50)

**Example LLM prompt**: "List all active apps in Lumos"

---

### lumos_get_app

**What it does**: Gets detailed information about a specific application.

**When to use**: Get app details, check app configuration.

**Arguments**:
- `appId` (required): App ID

**Example LLM prompt**: "Get details for app APP-12345"

---

### lumos_list_users

**What it does**: Lists all users in Lumos.

**When to use**: Find users, check user app access.

**Arguments**:
- `status` (optional): Filter by status (active, inactive)
- `app_id` (optional): Filter by app ID
- `page` (optional): Page number (default 1)
- `per_page` (optional): Results per page (default 50)

**Example LLM prompt**: "List all users with access to Slack"

---

### lumos_get_user

**What it does**: Gets detailed information about a specific user.

**When to use**: Check user details, verify user status.

**Arguments**:
- `userId` (required): User ID

**Example LLM prompt**: "Get details for user USR-12345"

---

### lumos_list_requests

**What it does**: Lists all access requests in Lumos.

**When to use**: Review pending requests, find requests by status.

**Arguments**:
- `status` (optional): Filter by status (pending, approved, denied)
- `app_id` (optional): Filter by app ID
- `requester_id` (optional): Filter by requester ID
- `page` (optional): Page number (default 1)
- `per_page` (optional): Results per page (default 50)

**Example LLM prompt**: "List all pending access requests"

---

### lumos_get_request

**What it does**: Gets details of a specific access request.

**When to use**: Review request details before approval.

**Arguments**:
- `requestId` (required): Request ID

**Example LLM prompt**: "Get details for request REQ-12345"

---

### lumos_approve_request

**What it does**: Approves an access request.

**When to use**: Grant access to applications.

**Arguments**:
- `requestId` (required): Request ID
- `comment` (optional): Approval comment

**Example LLM prompt**: "Approve request REQ-12345"

---

### lumos_deny_request

**What it does**: Denies an access request.

**When to use**: Reject access requests, request more information.

**Arguments**:
- `requestId` (required): Request ID
- `reason` (optional): Reason for denial

**Example LLM prompt**: "Deny request REQ-12345 because it's missing justification"

---

### lumos_list_systems

**What it does**: Lists all systems in Lumos.

**When to use**: Browse available systems, understand IT infrastructure.

**Arguments**:
- `page` (optional): Page number (default 1)
- `per_page` (optional): Results per page (default 50)

**Example LLM prompt**: "List all systems in Lumos"

---

### lumos_get_system

**What it does**: Gets details of a specific system.

**When to use**: Get system details, understand system configuration.

**Arguments**:
- `systemId` (required): System ID

**Example LLM prompt**: "Get details for system SYS-12345"

---

## Lumos Notes

- **Access requests**: Workflow-based approval process
- **Apps**: Software applications managed by IT
- **Systems**: Backend systems and infrastructure
- **Rate limits**: Implement backoff for bulk operations
