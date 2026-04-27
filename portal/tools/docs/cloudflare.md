# Cloudflare Tools

Provider: `cloudflare` | Engine: `nango` | Auth: API Key or OAuth (via Nango)

## Overview

These tools wrap the Cloudflare API v4. They allow AI agents to manage zones, DNS records, Pages projects, Workers, and SSL certificates on behalf of the authenticated user.

## Authentication

**Nango (recommended for OAuth)**:
- User authenticates via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `zone:read`, `zone:edit`, `dns:read`, `dns:edit`, `pages:read`, `workers:read`

**Native API Key**:
- Cloudflare API token stored in `client_secret` field
- Direct Bearer token passthrough to Cloudflare API

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `cloudflare_list_zones` | List all zones (domains) | GET | /zones |
| `cloudflare_get_zone` | Get details for a specific zone | GET | /zones/{zone_id} |
| `cloudflare_list_dns_records` | List DNS records for a zone | GET | /zones/{zone_id}/dns_records |
| `cloudflare_get_dns_record` | Get a specific DNS record | GET | /zones/{zone_id}/dns_records/{dns_record_id} |
| `cloudflare_create_dns_record` | Create a new DNS record | POST | /zones/{zone_id}/dns_records |
| `cloudflare_list_pages_projects` | List Cloudflare Pages projects | GET | /accounts/{account_id}/pages/projects |
| `cloudflare_get_pages_project` | Get a Pages project details | GET | /accounts/{account_id}/pages/projects/{project_name} |
| `cloudflare_list_workers` | List Workers scripts | GET | /accounts/{account_id}/workers/scripts |
| `cloudflare_get_worker` | Get Worker script details | GET | /accounts/{account_id}/workers/scripts/{script_name} |
| `cloudflare_list_ssl_certificates` | List SSL certificates | GET | /zones/{zone_id}/sslCertificates |

---

## Tool Details

### cloudflare_list_zones

**What it does**: Returns a list of all zones (domains) in your Cloudflare account with names, status, and configuration.

**When to use**: Browse your domains before managing DNS records or checking zone status.

**Arguments**:
- `page` (optional): Page number for pagination (default: 1)
- `per_page` (optional): Results per page (max 100, default 20)
- `order` (optional): Field to order by (name, status, zone_id)
- `status` (optional): Filter by status (active, pending, initializing, deactivated)

**Example LLM prompt**: "List all my Cloudflare zones"

---

### cloudflare_get_zone

**What it does**: Gets detailed information about a specific zone including DNS settings, security configuration, and traffic statistics.

**When to use**: Check zone configuration or before modifying DNS settings.

**Arguments**:
- `zone_id` (required): Zone ID

**Example LLM prompt**: "Get details about my domain zone"

---

### cloudflare_list_dns_records

**What it does**: Lists all DNS records for a zone with record type, name, content, and TTL. Supports filtering by record type.

**When to use**: View all DNS records before adding, modifying, or troubleshooting DNS settings.

**Arguments**:
- `zone_id` (required): Zone ID
- `type` (optional): Filter by record type (A, AAAA, CNAME, MX, TXT, etc.)
- `name` (optional): Filter by record name
- `page` (optional): Page number (default: 1)
- `per_page` (optional): Results per page (max 100, default 20)

**Example LLM prompt**: "List all A records in my zone"

---

### cloudflare_get_dns_record

**What it does**: Gets detailed information about a specific DNS record including proxied status, TTL, and creation/modification dates.

**When to use**: Check a specific DNS record before editing or deleting it.

**Arguments**:
- `zone_id` (required): Zone ID
- `dns_record_id` (required): DNS record ID

**Example LLM prompt**: "Get details for the www record in my zone"

---

### cloudflare_create_dns_record

**What it does**: Creates a new DNS record for a zone supporting all record types (A, AAAA, CNAME, MX, TXT, etc.).

**When to use**: Add new DNS records for domains, subdomains, or services.

**Arguments**:
- `zone_id` (required): Zone ID
- `type` (required): Record type (A, AAAA, CNAME, MX, TXT, etc.)
- `name` (required): Record name
- `content` (required): Record content/value
- `ttl` (optional): TTL in seconds (1 = auto, default 1)
- `priority` (optional): Priority for MX records
- `proxied` (optional): Enable Cloudflare proxying (default: true)

**Example LLM prompt**: "Create an A record for api.example.com pointing to 192.0.2.1"

---

### cloudflare_list_pages_projects

**What it does**: Lists all Cloudflare Pages projects in your account with project names, subdomains, and deployment status.

**When to use**: Browse Pages projects before checking deployments or build settings.

**Arguments**:
- `account_id` (required): Account ID
- `page` (optional): Page number (default: 1)
- `per_page` (optional): Results per page (default: 20)

**Example LLM prompt**: "List all my Cloudflare Pages projects"

---

### cloudflare_get_pages_project

**What it does**: Gets detailed information about a Cloudflare Pages project including build configuration, deployments, and custom domains.

**When to use**: Check project settings or deployment status before updating.

**Arguments**:
- `account_id` (required): Account ID
- `project_name` (required): Project name

**Example LLM prompt**: "Get details for my pages-project"

---

### cloudflare_list_workers

**What it does**: Lists all Workers scripts in your account with script names, etags, and deployment dates.

**When to use**: Browse existing Workers before creating new ones or checking usage.

**Arguments**:
- `account_id` (required): Account ID
- `page` (optional): Page number (default: 1)
- `per_page` (optional): Results per page (default: 20)

**Example LLM prompt**: "List all my Cloudflare Workers"

---

### cloudflare_get_worker

**What it does**: Gets details about a specific Workers script including code, deployment metadata, and usage statistics.

**When to use**: Review a Worker script before updating or debugging.

**Arguments**:
- `account_id` (required): Account ID
- `script_name` (required): Script name

**Example LLM prompt**: "Show me the code for my-api-worker"

---

### cloudflare_list_ssl_certificates

**What it does**: Lists all SSL certificates in your account including universal, dedicated, and custom certificates.

**When to use**: Audit SSL certificates or check certificate status.

**Arguments**:
- `zone_id` (required): Zone ID

**Example LLM prompt**: "List all SSL certificates for my zone"

---

## Cloudflare API Reference

These tools use the Cloudflare API v4. See official docs for full details:
- https://developers.cloudflare.com/api/
- Rate limits: Varies by endpoint and plan
- Pagination: Use `page` and `per_page` parameters
- All dates: ISO 8601 format
