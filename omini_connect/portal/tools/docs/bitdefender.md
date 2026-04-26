# Bitdefender Tools

Provider: `bitdefender` | Engine: `nango` | Auth: BASIC via Nango (API Key)

## Overview

These tools wrap the Bitdefender GravityZone API. They allow AI agents to manage endpoints, security policies, tasks, and alerts. Bitdefender is a cybersecurity platform providing endpoint protection, threat detection, and security management.

## Authentication

**Nango BASIC Auth**:
- User provides Bitdefender API key as username
- Password field is hidden/empty
- Token stored in Nango, accessed via `connection_ref`
- Requires access URL configuration

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `bitdefender_list_endpoints` | List endpoints | GET | /endpoints |
| `bitdefender_get_endpoint` | Get endpoint details | GET | /endpoints/{id} |
| `bitdefender_list_policies` | List security policies | GET | /policies |
| `bitdefender_get_policy` | Get policy details | GET | /policies/{id} |
| `bitdefender_list_tasks` | List tasks | GET | /tasks |
| `bitdefender_get_task` | Get task details | GET | /tasks/{id} |
| `bitdefender_list_alerts` | List security alerts | GET | /alerts |
| `bitdefender_get_alert` | Get alert details | GET | /alerts/{id} |
| `bitdefender_list_groups` | List endpoint groups | GET | /groups |
| `bitdefender_get_quarantine` | Get quarantine items | GET | /quarantine |

---

## Tool Details

### bitdefender_list_endpoints

**What it does**: Lists all endpoints (devices) managed by Bitdefender.

**When to use**: Browse managed devices, check protection status.

**Arguments**:
- `page` (optional): Page number (default 1)
- `pageSize` (optional): Endpoints per page (default 20)

**Example LLM prompt**: "List all endpoints in Bitdefender"

---

### bitdefender_get_endpoint

**What it does**: Gets details for a specific endpoint.

**When to use**: Check device security status, installed protection.

**Arguments**:
- `id` (required): Endpoint ID

**Example LLM prompt**: "Get details for endpoint E-123"

---

### bitdefender_list_policies

**What it does**: Lists all security policies.

**When to use**: View security configuration, compliance status.

**Arguments**:
- `page` (optional): Page number (default 1)
- `pageSize` (optional): Policies per page (default 20)

**Example LLM prompt**: "List all security policies"

---

### bitdefender_get_policy

**What it does**: Gets details for a specific security policy.

**When to use**: Review policy settings, check assigned endpoints.

**Arguments**:
- `id` (required): Policy ID

**Example LLM prompt**: "Get details for policy P-456"

---

### bitdefender_list_tasks

**What it does**: Lists all tasks (scans, updates, etc.).

**When to use**: Track ongoing operations, check task status.

**Arguments**:
- `status` (optional): Filter by status
- `page` (optional): Page number (default 1)
- `pageSize` (optional): Tasks per page (default 20)

**Example LLM prompt**: "List running tasks"

---

### bitdefender_get_task

**What it does**: Gets details for a specific task.

**When to use**: Check scan progress, task results.

**Arguments**:
- `id` (required): Task ID

**Example LLM prompt**: "Get details for task T-789"

---

### bitdefender_list_alerts

**What it does**: Lists all security alerts.

**When to use**: Monitor threats, respond to incidents.

**Arguments**:
- `severity` (optional): Filter by severity (high, medium, low)
- `page` (optional): Page number (default 1)
- `pageSize` (optional): Alerts per page (default 20)

**Example LLM prompt**: "List high severity alerts"

---

### bitdefender_get_alert

**What it does**: Gets details for a specific alert.

**When to use**: Investigate security incidents, take action.

**Arguments**:
- `id` (required): Alert ID

**Example LLM prompt**: "Get details for alert A-100"

---

### bitdefender_list_groups

**What it does**: Lists all endpoint groups.

**When to use**: Organize endpoints, apply group policies.

**Arguments**: None required

**Example LLM prompt**: "List all endpoint groups"

---

### bitdefender_get_quarantine

**What it does**: Gets quarantine items from endpoints.

**When to use**: Review quarantined files, manage threats.

**Arguments**:
- `endpointId` (optional): Filter by endpoint ID

**Example LLM prompt**: "Get quarantine items for endpoint E-123"

---

## Bitdefender API Notes

- **Endpoints**: Devices protected by Bitdefender (computers, servers, mobile)
- **Policies**: Security configurations applied to endpoints
- **Tasks**: Operations like scans, updates, deployments
- **Alerts**: Security events requiring attention
- **Quarantine**: Isolated threats waiting for deletion or restoration
