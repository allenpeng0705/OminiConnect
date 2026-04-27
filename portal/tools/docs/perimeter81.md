# Perimeter81 Tools

Provider: `perimeter81` | Engine: `nango` | Auth: Two-Step API Key via Nango

## Overview

These tools wrap the Perimeter81 API. They allow AI agents to manage users, networks, devices, policies, and activity logs. Perimeter81 is a network security platform. **Requires Perimeter81 API Key authentication.**

## Authentication

**Nango Two-Step API**:
- Uses API key for token exchange
- Token stored in Nango, accessed via `connection_ref`
- Base URL: https://api.perimeter81.com/api/rest
- Requires domain in connection config

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `perimeter81_list_users` | List users | GET | /users |
| `perimeter81_get_user` | Get user details | GET | /users/{userId} |
| `perimeter81_list_networks` | List networks | GET | /networks |
| `perimeter81_get_network` | Get network details | GET | /networks/{networkId} |
| `perimeter81_list_devices` | List devices | GET | /devices |
| `perimeter81_get_device` | Get device details | GET | /devices/{deviceId} |
| `perimeter81_list_policies` | List policies | GET | /policies |
| `perimeter81_get_policy` | Get policy details | GET | /policies/{policyId} |
| `perimeter81_list_activity_logs` | List activity logs | GET | /activity-logs |
| `perimeter81_get_organization` | Get organization info | GET | /organization |

---

## Tool Details

### perimeter81_list_users

**What it does**: Lists all users in the organization.

**When to use**: Browse user directory, manage access.

**Arguments**:
- `status` (optional): Filter by status (active, inactive)

**Example LLM prompt**: "List all active users"

---

### perimeter81_get_user

**What it does**: Gets detailed information about a specific user.

**When to use**: Get user details, permissions.

**Arguments**:
- `userId` (required): User ID

**Example LLM prompt**: "Get details for user 12345"

---

### perimeter81_list_networks

**What it does**: Lists all networks in the organization.

**When to use**: Browse network infrastructure.

**Arguments**:
- `type` (optional): Filter by network type

**Example LLM prompt**: "List all networks"

---

### perimeter81_get_network

**What it does**: Gets detailed information about a specific network.

**When to use**: Get network configuration, settings.

**Arguments**:
- `networkId` (required): Network ID

**Example LLM prompt**: "Get details for network 67890"

---

### perimeter81_list_devices

**What it does**: Lists all devices in the organization.

**When to use**: Browse devices, check status.

**Arguments**:
- `status` (optional): Filter by status (online, offline)
- `networkId` (optional): Filter by network ID

**Example LLM prompt**: "List all online devices"

---

### perimeter81_get_device

**What it does**: Gets detailed information about a specific device.

**When to use**: Get device details, configuration.

**Arguments**:
- `deviceId` (required): Device ID

**Example LLM prompt**: "Get details for device DEV-11111"

---

### perimeter81_list_policies

**What it does**: Lists all policies in the organization.

**When to use**: Browse security policies.

**Arguments**:
- `type` (optional): Filter by policy type

**Example LLM prompt**: "List all security policies"

---

### perimeter81_get_policy

**What it does**: Gets detailed information about a specific policy.

**When to use**: Get policy details, rules.

**Arguments**:
- `policyId` (required): Policy ID

**Example LLM prompt**: "Get details for policy POL-22222"

---

### perimeter81_list_activity_logs

**What it does**: Lists activity logs for the organization.

**When to use**: Audit user activity, security review.

**Arguments**:
- `startDate` (optional): Start date (YYYY-MM-DD)
- `endDate` (optional): End date (YYYY-MM-DD)
- `userId` (optional): Filter by user ID

**Example LLM prompt**: "Show activity logs for the past week"

---

### perimeter81_get_organization

**What it does**: Gets organization information.

**When to use**: Get organization settings, configuration.

**Arguments**: None

**Example LLM prompt**: "Get our organization information"

---

## Perimeter81 API Notes

- **Two-Step Auth**: API key exchanged for access token
- **Rate limits**: Apply to API calls
- **Date formats**: Use YYYY-MM-DD format
