# Fastly Tools

Provider: `fastly` | Engine: `nango` | Auth: API Key (via Nango)

## Overview

These tools wrap the Fastly API v1. They allow AI agents to manage services, versions, dictionaries, logging endpoints, and cache invalidations.

## Authentication

**Nango (recommended)**:
- User authenticates via Nango Connect
- API key stored in Nango, accessed via `connection_ref`
- Scopes: `global` (Fastly uses global API access)

**Native API Key**:
- Fastly API token passed as Bearer token
- Include in `Fastly-Key` header or via Nango connection

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `fastly_list_services` | List all Fastly services | GET | /services |
| `fastly_get_service` | Get service details | GET | /services/{service_id} |
| `fastly_list_versions` | List service versions | GET | /services/{service_id}/versions |
| `fastly_get_version` | Get version details | GET | /services/{service_id}/versions/{version_number} |
| `fastly_list_dictionaries` | List dictionaries for a service | GET | /services/{service_id}/versions/{version_number}/dictionaries |
| `fastly_get_dictionary` | Get dictionary contents | GET | /services/{service_id}/dictionaries/{dictionary_id}/items |
| `fastly_list_logging_endpoints` | List logging endpoints | GET | /services/{service_id}/versions/{version_number}/logging |
| `fastly_purge_url` | Purge a single URL | POST | /purge/{url} |
| `fastly_create_invalidation` | Create a cache invalidation | POST | /services/{service_id}/purge |
| `fastly_get_invalidation` | Get invalidation status | GET | /services/{service_id}/invocations/{invalidation_id} |

---

## Tool Details

### fastly_list_services

**What it does**: Lists all Fastly services in your account with IDs, names, and current versions.

**When to use**: Browse available services before checking configuration or version history.

**Arguments**:
- `page` (optional): Page number (default: 1)
- `per_page` (optional): Results per page (max 100, default 20)

**Example LLM prompt**: "List all my Fastly services"

---

### fastly_get_service

**What it does**: Gets detailed information about a specific service including versions, domains, backends, and configuration.

**When to use**: Check service configuration before modifying versions or settings.

**Arguments**:
- `service_id` (required): Service ID

**Example LLM prompt**: "Get details for my service abc123"

---

### fastly_list_versions

**What it does**: Lists all versions for a service. Each version represents a snapshot of your service configuration.

**When to use**: View version history before cloning or activating a version.

**Arguments**:
- `service_id` (required): Service ID

**Example LLM prompt**: "List all versions for service abc123"

---

### fastly_get_version

**What it does**: Gets detailed configuration for a specific service version including domains, backends, and VCL code.

**When to use**: Review version details before activating or cloning.

**Arguments**:
- `service_id` (required): Service ID
- `version_number` (required): Version number

**Example LLM prompt**: "Get details for version 5 of service abc123"

---

### fastly_list_dictionaries

**What it does**: Lists all dictionaries for a service version. Dictionaries store key-value pairs for configuration.

**When to use**: View available dictionaries before reading or syncing data.

**Arguments**:
- `service_id` (required): Service ID
- `version_number` (required): Version number

**Example LLM prompt**: "List dictionaries for version 5 of my service"

---

### fastly_get_dictionary

**What it does**: Gets the contents of a dictionary including all key-value pairs.

**When to use**: Read dictionary data or sync configuration values.

**Arguments**:
- `service_id` (required): Service ID
- `dictionary_id` (required): Dictionary ID

**Example LLM prompt**: "Get contents of the feature-flags dictionary"

---

### fastly_list_logging_endpoints

**What it does**: Lists all logging endpoints for a service version. Returns configured log destinations (s3, bigquery, syslog, etc.).

**When to use**: Audit logging configuration or check endpoint status.

**Arguments**:
- `service_id` (required): Service ID
- `version_number` (required): Version number

**Example LLM prompt**: "List all logging endpoints for version 5"

---

### fastly_purge_url

**What it does**: Purges a single URL from Fastly cache immediately. Use this to invalidate specific cached resources.

**When to use**: Immediately invalidate a cached URL after content update.

**Arguments**:
- `url` (required): Full URL to purge

**Example LLM prompt**: "Purge https://example.com/index.html from cache"

---

### fastly_create_invalidation

**What it does**: Creates a cache invalidation request for multiple URLs or a path pattern. Returns an invalidation ID to track status.

**When to use**: Invalidate multiple URLs or patterns after a major content change.

**Arguments**:
- `service_id` (required): Service ID
- `urls` (required): Array of URLs to invalidate
- `soft` (optional): Soft purge (Content-Type change only, default: false)

**Example LLM prompt**: "Invalidate all URLs under /blog/ in my service"

---

### fastly_get_invalidation

**What it does**: Gets the status of a cache invalidation request. Returns progress and completion status.

**When to use**: Check if an invalidation has completed after creating it.

**Arguments**:
- `service_id` (required): Service ID
- `invalidation_id` (required): Invalidation ID

**Example LLM prompt**: "Check the status of invalidation inv_12345"

---

## Fastly API Reference

These tools use the Fastly API v1. See official docs for full details:
- https://developer.fastly.com/reference/api/
- Rate limits: 1000 requests/minute (varies by plan)
- Pagination: Use `page` and `per_page` parameters
- All dates: ISO 8601 format
