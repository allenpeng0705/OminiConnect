# Tailscale (OAuth) Tools

Provider: `tailscale` | Engine: `nango` | Auth: Two-Step OAuth via Nango

## Overview

Tailscale is a zero-config VPN that makes devices accessible from anywhere. **Requires two-step oauth via nango.**

## Authentication

**Nango Two-Step OAuth**:
- User authenticates via two-step OAuth flow
- Token stored in Nango, accessed via `connection_ref`
- Scopes: devices:read, devices:write, users:read, users:write, acl:read, acl:write

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `tailscale_list_devices` | List all devices in your Tailscale network | GET | /v2/devices |
| `tailscale_get_device` | Get details of a specific device | GET | /v2/devices/{device_id} |
| `tailscale_list_users` | List all users in your Tailscale organization | GET | /v2/users |
| `tailscale_get_user` | Get details of a specific user | GET | /v2/users/{user_id} |
| `tailscale_list_acls` | List ACL policies | GET | /v2/acl |
| `tailscale_modify_acl` | Update ACL policy | POST | /v2/acl |
| `tailscale_list_tailnets` | List tailnet configurations | GET | /v2/tailnet |
| `tailscale_get_dns_preferences` | Get DNS preferences for a tailnet | GET | /v2/tailnet/{tailnet}/dns |
| `tailscale_list_keys` | List authentication keys | GET | /v2/keys |
| `tailscale_create_key` | Create an authentication key | POST | /v2/keys |

---

## Tool Details

### tailscale_list_devices

**What it does**: List all devices in your Tailscale network

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tailscale_get_device

**What it does**: Get details of a specific device

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tailscale_list_users

**What it does**: List all users in your Tailscale organization

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tailscale_get_user

**What it does**: Get details of a specific user

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tailscale_list_acls

**What it does**: List ACL policies

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tailscale_modify_acl

**What it does**: Update ACL policy

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tailscale_list_tailnets

**What it does**: List tailnet configurations

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tailscale_get_dns_preferences

**What it does**: Get DNS preferences for a tailnet

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tailscale_list_keys

**What it does**: List authentication keys

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---

### tailscale_create_key

**What it does**: Create an authentication key

**When to use**: 

**Arguments**:
- None

**Example LLM prompt**: ""

---


## API Notes

- Base URL: `https://api.tailscale.com/api`
- Docs: https://nango.dev/docs/integrations/all/tailscale
