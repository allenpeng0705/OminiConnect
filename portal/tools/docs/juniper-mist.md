# Juniper Mist Tools

Provider: `juniper-mist` | Engine: `nango` | Auth: API_KEY via Nango

## Overview

These tools wrap the Juniper Mist API. They allow AI agents to manage sites, devices, networks, alerts, and clients. **Requires Juniper Mist API token.**

## Authentication

**API Key via Nango**:
- User provides their Juniper Mist API token
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://{subdomain}.mist.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `juniper_mist_get_self` | Get current user | GET | /api/v1/self |
| `juniper_mist_list_sites` | List sites | GET | /api/v1/sites |
| `juniper_mist_get_site` | Get site details | GET | /api/v1/sites/{site_id} |
| `juniper_mist_list_devices` | List devices | GET | /api/v1/devices |
| `juniper_mist_get_device` | Get device details | GET | /api/v1/devices/{device_id} |
| `juniper_mist_list_networks` | List networks | GET | /api/v1/networks |
| `juniper_mist_get_network` | Get network details | GET | /api/v1/networks/{network_id} |
| `juniper_mist_list_alerts` | List alerts | GET | /api/v1/alerts |
| `juniper_mist_list_clients` | List clients | GET | /api/v1/clients |
| `juniper_mist_get_client` | Get client details | GET | /api/v1/clients/{client_id} |

---

## Tool Details

### juniper_mist_get_self

**What it does**: Gets the current authenticated user.

**When to use**: Verify authentication, get user info.

**Arguments**: None

**Example LLM prompt**: "Who is the current Juniper Mist user?"

---

### juniper_mist_list_sites

**What it does**: Lists all sites in the organization.

**When to use**: Find sites, view organization structure.

**Arguments**: None

**Example LLM prompt**: "List all sites in Juniper Mist"

---

### juniper_mist_get_site

**What it does**: Gets details for a specific site.

**When to use**: Get site information, view site settings.

**Arguments**:
- `site_id` (required): Site ID

**Example LLM prompt**: "Get details for site abc123"

---

### juniper_mist_list_devices

**What it does**: Lists all devices in a site or organization.

**When to use**: View devices, manage network hardware.

**Arguments**:
- `site_id` (optional): Site ID filter

**Example LLM prompt**: "List all devices in site abc123"

---

### juniper_mist_get_device

**What it does**: Gets details for a specific device.

**When to use**: Get device information, view device status.

**Arguments**:
- `device_id` (required): Device ID

**Example LLM prompt**: "Get details for device xyz789"

---

### juniper_mist_list_networks

**What it does**: Lists all networks in a site.

**When to use**: View networks, manage network settings.

**Arguments**:
- `site_id` (required): Site ID

**Example LLM prompt**: "List all networks in site abc123"

---

### juniper_mist_get_network

**What it does**: Gets details for a specific network.

**When to use**: Get network information, view network settings.

**Arguments**:
- `network_id` (required): Network ID

**Example LLM prompt**: "Get details for network n1"

---

### juniper_mist_list_alerts

**What it does**: Lists all alerts in a site or organization.

**When to use**: View alerts, monitor network health.

**Arguments**:
- `site_id` (optional): Site ID filter

**Example LLM prompt**: "List all alerts in site abc123"

---

### juniper_mist_list_clients

**What it does**: Lists all clients connected to the network.

**When to use**: View connected devices, monitor client activity.

**Arguments**:
- `site_id` (optional): Site ID
- `network_id` (optional): Network ID

**Example LLM prompt**: "List all clients in network n1"

---

### juniper_mist_get_client

**What it does**: Gets details for a specific client.

**When to use**: Get client information, troubleshoot connectivity.

**Arguments**:
- `client_id` (required): Client ID (MAC address)

**Example LLM prompt**: "Get details for client aa:bb:cc:dd:ee:ff"

---

## Juniper Mist API Notes

- **IDs**: String IDs for sites, devices, networks, clients
- **Client IDs**: MAC addresses for clients
- **Sites**: Organizational units containing devices and networks
- **Networks**: Wireless or wired network configurations
