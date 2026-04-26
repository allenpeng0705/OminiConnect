# Algolia Tools

Provider: `algolia` | Engine: `nango` | Auth: API Key (via Nango)

## Overview

These tools wrap the Algolia Search API v1. They allow AI agents to manage indices, search records, configure settings, and administer API keys for search operations.

## Authentication

**Nango (recommended for API Key)**:
- User authenticates via Nango Connect
- API key stored in Nango, accessed via `connection_ref`
- Scopes: `search`, `addObject`, `updateObject`, `deleteObject`, `manage`, `settings`, `api`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `algolia_list_indices` | List all search indices | GET | /1/indexes |
| `algolia_get_index` | Get details for a specific index | GET | /1/indexes/{index_name} |
| `algolia_search_index` | Search records in an index | POST | /1/indexes/{index_name}/search |
| `algolia_add_objects` | Add multiple objects to an index | POST | /1/indexes/{index_name}/batch |
| `algolia_delete_objects` | Delete objects from an index | POST | /1/indexes/{index_name}/batch |
| `algolia_list_api_keys` | List API keys | GET | /1/keys |
| `algolia_get_settings` | Get index settings | GET | /1/indexes/{index_name}/settings |
| `algolia_update_settings` | Update index settings | PUT | /1/indexes/{index_name}/settings |
| `algolia_list_logs` | List operation logs | GET | /1/logs |
| `algolia_get_log` | Get a specific log entry | GET | /1/logs/{log_id} |

---

## Tool Details

### algolia_list_indices

**What it does**: Returns a list of all search indices in your Algolia application with their sizes and configuration status.

**When to use**: Before searching or modifying index data, to understand what indices are available.

**Arguments**:
- `page` (optional): Page number for pagination (default: 0)
- `hits_per_page` (optional, max 1000): Results per page (default: 20)

**Example LLM prompt**: "What indices do I have in Algolia?"

---

### algolia_get_index

**What it does**: Gets detailed configuration and statistics for a specific index including record count, size, and search statistics.

**When to use**: Check index health, see how many records are indexed, or review index settings.

**Arguments**:
- `index_name` (required): Index name

**Example LLM prompt**: "Get details about my products index"

---

### algolia_search_index

**What it does**: Searches for records in an index using full-text search with support for filtering, faceting, highlighting, and ranking.

**When to use**: Find records matching a query, filter results by attributes, or get paginated results.

**Arguments**:
- `index_name` (required): Index name
- `query` (optional): Search query string (default: "")
- `filters` (optional): Filter expression (e.g., `price > 100 AND category = 'electronics'`)
- `facet_filters` (optional): Array of facet filters
- `facets` (optional): Attributes to facet on
- `attributes_to_retrieve` (optional): Fields to return
- `attributes_to_highlight` (optional): Fields to highlight matches for
- `page` (optional): Page number (default: 0)
- `hits_per_page` (optional, max 1000): Results per page (default: 20)

**Example LLM prompt**: "Search for 'wireless headphones' in my products index with price under 200"

---

### algolia_add_objects

**What it does**: Adds multiple objects to an index in a single batch. Objects become searchable immediately after indexing.

**When to use**: Index multiple records at once, bulk import products, or insert multiple documents.

**Arguments**:
- `index_name` (required): Index name
- `requests` (required): Array of objects to add

**Example LLM prompt**: "Add new products to my products index: [{name: 'Speaker', price: 79.99}, {name: 'Headphones', price: 149.99}]"

---

### algolia_delete_objects

**What it does**: Deletes multiple objects from an index by their objectIDs. Objects will no longer appear in search results.

**When to use**: Remove outdated products, delete deprecated records, or clean up test data in bulk.

**Arguments**:
- `index_name` (required): Index name
- `requests` (required): Array of delete requests with objectIDs

**Example LLM prompt**: "Delete objects with IDs 'prod_123' and 'prod_456' from my products index"

---

### algolia_list_api_keys

**What it does**: Lists all API keys in your Algolia application with their descriptions, permissions, and validity status.

**When to use**: Audit API key usage, check which keys have what permissions, or review security settings.

**Arguments**:
- `page` (optional): Page number (default: 0)
- `hits_per_page` (optional): Results per page (default: 20)

**Example LLM prompt**: "List all my Algolia API keys"

---

### algolia_get_settings

**What it does**: Gets all index settings including searchable attributes, ranking rules, faceting, and synonyms configuration.

**When to use**: Review index configuration before modifying or to understand search behavior.

**Arguments**:
- `index_name` (required): Index name

**Example LLM prompt**: "Get settings for my products index"

---

### algolia_update_settings

**What it does**: Updates index settings including searchable attributes, ranking rules, faceting, and highlighting configuration.

**When to use**: Customize search behavior, change ranking rules, or configure faceting attributes.

**Arguments**:
- `index_name` (required): Index name
- `searchable_attributes` (optional): Attributes to search in
- `attributes_for_faceting` (optional): Attributes to use for faceting
- `ranking` (optional): Ranking rules
- `custom_ranking` (optional): Custom ranking attributes
- `highlights` (optional): Highlighting configuration

**Example LLM prompt**: "Update my products index to search in name, description, and brand; facet on category and price range"

---

### algolia_list_logs

**What it does**: Lists recent API operation logs for your Algolia application with timestamp, operation type, and response status.

**When to use**: Debug search issues, audit API usage, or monitor operations.

**Arguments**:
- `offset` (optional): Log offset for pagination (default: 0)
- `length` (optional): Number of logs to return (max 1000, default: 20)
- `index_name` (optional): Filter logs by index name

**Example LLM prompt**: "Show me recent API logs for my products index"

---

### algolia_get_log

**What it does**: Gets details for a specific log entry including the full request and response for debugging.

**When to use**: Debug a specific API call or investigate an error.

**Arguments**:
- `log_id` (required): Log ID

**Example LLM prompt**: "Get details for log entry log_12345"

---

## Algolia API Reference

These tools use the Algolia REST API v1. See official docs for full details:
- https://www.algolia.com/doc/api-client/getting-started/
- Rate limits: Based on plan (free tier: 10,000 requests/month)
- Pagination: Use `page` and `hits_per_page` parameters
- All dates: ISO 8601 format (UTC)
