# DigitalOcean Tools

Provider: `digitalocean` | Engine: `nango` | Auth: OAuth (via Nango)

## Overview

These tools wrap the DigitalOcean API v2. They allow AI agents to interact with droplets, volumes, domains, and firewalls on behalf of the authenticated user.

## Authentication

**Nango (recommended for OAuth)**:
- User authenticates via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `droplets:read`, `droplets:create`, `droplets:delete`, `domains:read`, `volumes:read`, `firewalls:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `digitalocean_list_droplets` | List all droplets | GET | /v2/droplets |
| `digitalocean_get_droplet` | Get details of a specific droplet | GET | /v2/droplets/{droplet_id} |
| `digitalocean_create_droplet` | Create a new droplet | POST | /v2/droplets |
| `digitalocean_delete_droplet` | Delete a droplet | DELETE | /v2/droplets/{droplet_id} |
| `digitalocean_list_volumes` | List all volumes | GET | /v2/volumes |
| `digitalocean_get_volume` | Get details of a specific volume | GET | /v2/volumes/{volume_id} |
| `digitalocean_list_domains` | List all domains | GET | /v2/domains |
| `digitalocean_get_domain` | Get details of a specific domain | GET | /v2/domains/{domain_name} |
| `digitalocean_list_firewalls` | List all firewalls | GET | /v2/firewalls |
| `digitalocean_get_firewall` | Get details of a specific firewall | GET | /v2/firewalls/{firewall_id} |

---

## Tool Details

### digitalocean_list_droplets

**What it does**: Returns a list of all droplets for the authenticated user with details including name, IP addresses, region, size, and status.

**When to use**: List all droplets to find a specific one, monitor instances, or check status across your infrastructure.

**Arguments**:
- `page` (optional): Page number for pagination (default 1)
- `per_page` (optional): Results per page (default 20, max 200)
- `tag_name` (optional): Filter by tag name

**Example LLM prompt**: "List all my DigitalOcean droplets"

---

### digitalocean_get_droplet

**What it does**: Gets details of a specific droplet including specs, networking, and backups.

**When to use**: Get droplet details, check IP addresses, verify configuration, or monitor status.

**Arguments**:
- `droplet_id` (required): Droplet ID

**Example LLM prompt**: "Show me details for droplet 123456"

---

### digitalocean_create_droplet

**What it does**: Creates a new droplet with specified size, region, image, and optional SSH keys, backups, or tags.

**When to use**: Create a new server instance, provision a development environment, or scale infrastructure.

**Arguments**:
- `name` (required): Droplet name
- `region` (required): Region slug (e.g., nyc1, sfo2, lon1)
- `size` (required): Size slug (e.g., s-1vcpu-1gb, s-2vcpu-4gb)
- `image` (required): Image slug or ID (e.g., ubuntu-22-04-x64, docker)
- `ssh_keys` (optional): Array of SSH key IDs or fingerprints
- `backups` (optional): Enable automatic backups (default false)
- `ipv6` (optional): Enable IPv6 (default false)
- `tags` (optional): Array of tag names
- `user_data` (optional): Cloud-init user data script

**Example LLM prompt**: "Create a new droplet called my-server in nyc1 with 2 CPUs and 4GB RAM running Ubuntu"

---

### digitalocean_delete_droplet

**What it does**: Deletes a specific droplet and all its data. This action is irreversible.

**When to use**: Remove unused droplets, clean up test environments, or deprovision infrastructure.

**Arguments**:
- `droplet_id` (required): Droplet ID

**Example LLM prompt**: "Delete droplet 123456"

---

### digitalocean_list_volumes

**What it does**: Lists all block storage volumes for the authenticated user with details including size, region, and filesystem type.

**When to use**: List volumes, find storage resources, or check volume status.

**Arguments**:
- `page` (optional): Page number (default 1)
- `per_page` (optional): Results per page (default 20)
- `region` (optional): Filter by region slug

**Example LLM prompt**: "List all my volumes"

---

### digitalocean_get_volume

**What it does**: Gets details of a specific volume including size, region, and attached droplet IDs.

**When to use**: Get volume details, check attachment status, or verify volume configuration.

**Arguments**:
- `volume_id` (required): Volume ID

**Example LLM prompt**: "Show me details for volume vol-abc123"

---

### digitalocean_list_domains

**What it does**: Lists all domains registered or managed for the authenticated user with DNS zone information.

**When to use**: List all domains, find domain records, or check domain configuration.

**Arguments**:
- `page` (optional): Page number (default 1)
- `per_page` (optional): Results per page (default 20)

**Example LLM prompt**: "List all my domains"

---

### digitalocean_get_domain

**What it does**: Gets details of a specific domain including zone file and record count.

**When to use**: Get domain details, view DNS configuration, or check zone status.

**Arguments**:
- `domain_name` (required): Domain name

**Example LLM prompt**: "Show me details for example.com"

---

### digitalocean_list_firewalls

**What it does**: Lists all cloud firewalls for the authenticated user with details including name, status, and rule summaries.

**When to use**: List all firewalls, review firewall rules, or check which resources are protected.

**Arguments**:
- `page` (optional): Page number (default 1)
- `per_page` (optional): Results per page (default 20)

**Example LLM prompt**: "List all my firewalls"

---

### digitalocean_get_firewall

**What it does**: Gets details of a specific firewall including all inbound and outbound rules.

**When to use**: Review firewall rules, check what traffic is allowed or blocked, verify security settings.

**Arguments**:
- `firewall_id` (required): Firewall ID

**Example LLM prompt**: "Show me the rules for firewall fw-abc123"

---

## DigitalOcean API Reference

These tools use the DigitalOcean API v2. See official docs for full details:
- https://docs.digitalocean.com/reference/api/api-reference/
- Rate limits: 2000 requests/minute for most endpoints
- Pagination: Use `page` and `per_page` parameters
- All timestamps: ISO 8601 format
