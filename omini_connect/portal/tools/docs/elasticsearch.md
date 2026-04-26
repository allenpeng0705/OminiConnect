# Elasticsearch Tools

Provider: `elasticsearch` | Engine: `nango` | Auth: API Key (via Nango)

## Overview

These tools wrap the Elasticsearch REST API v8. They allow AI agents to manage indices, index and search documents, and monitor cluster health and node statistics.

## Authentication

**Nango (recommended for API Key)**:
- User authenticates via Nango Connect
- API key stored in Nango, accessed via `connection_ref`
- Scopes: `cluster:read`, `cluster:write`, `read`, `write`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `elasticsearch_list_indices` | List all indices | GET | /_cat/indices |
| `elasticsearch_get_index` | Get index details and stats | GET | /{index_name} |
| `elasticsearch_create_index` | Create a new index | PUT | /{index_name} |
| `elasticsearch_delete_index` | Delete an index | DELETE | /{index_name} |
| `elasticsearch_index_document` | Index a single document | PUT | /{index_name}/_doc/{document_id} |
| `elasticsearch_search` | Search documents | POST | /{index_name}/_search |
| `elasticsearch_delete_document` | Delete a document by ID | DELETE | /{index_name}/_doc/{document_id} |
| `elasticsearch_list_clusters` | List connected clusters | GET | /_cluster/state |
| `elasticsearch_get_cluster_health` | Get cluster health status | GET | /_cluster/health |
| `elasticsearch_get_node_stats` | Get node statistics | GET | /_nodes/stats |

---

## Tool Details

### elasticsearch_list_indices

**What it does**: Returns a list of all indices in the Elasticsearch cluster with index names, document counts, and storage size.

**When to use**: See what indices exist, check index sizes, or monitor cluster storage.

**Arguments**:
- `index` (optional): Filter indices by name pattern (e.g., `logs-*`)
- `bytes` (optional): Size format (b, kb, mb, gb, tb, default: mb)
- `detailed` (optional): Return detailed information (default: false)

**Example LLM prompt**: "List all indices in my Elasticsearch cluster"

---

### elasticsearch_get_index

**What it does**: Gets detailed information about a specific index including mappings, settings, and statistics.

**When to use**: Check index configuration, review field mappings, or see index statistics.

**Arguments**:
- `index_name` (required): Index name

**Example LLM prompt**: "Get details about the products index"

---

### elasticsearch_create_index

**What it does**: Creates a new index with optional mappings and settings. Supports custom analyzers, aliases, and number of shards.

**When to use**: Create a new index for a new dataset or application.

**Arguments**:
- `index_name` (required): Index name
- `mappings` (optional): Index field mappings
- `settings` (optional): Index settings (number_of_shards, analyzers, etc.)
- `aliases` (optional): Index aliases

**Example LLM prompt**: "Create a new index called 'articles' with mappings for title, content, author, and date"

---

### elasticsearch_delete_index

**What it does**: Deletes an index and all its documents permanently. Use with caution as this cannot be undone.

**When to use**: Remove outdated indices, delete test data, or clean up unused indices.

**Arguments**:
- `index_name` (required): Index name

**Example LLM prompt**: "Delete the old-logs index"

---

### elasticsearch_index_document

**What it does**: Indexes a single document into an existing index. If document ID is not provided, one will be generated.

**When to use**: Add new records to your search index.

**Arguments**:
- `index_name` (required): Index name
- `document_id` (optional): Document ID (auto-generated if not provided)
- `body` (required): Document body as JSON object
- `refresh` (optional): Refresh index immediately (default: false)

**Example LLM prompt**: "Index a new article in the articles index with title 'Getting Started', author 'john', and date '2024-01-15'"

---

### elasticsearch_search

**What it does**: Searches for documents in an index using query DSL. Supports full-text search, filters, aggregations, and sorting.

**When to use**: Find documents matching a query, filter results, or get aggregated data.

**Arguments**:
- `index_name` (required): Index name
- `query` (optional): Query DSL (match, term, range, bool, etc.)
- `from` (optional): Offset for pagination (default: 0)
- `size` (optional): Number of results (default: 10, max: 10000)
- `sort` (optional): Sort fields (e.g., `['date:desc', 'name:asc']`)
- `_source` (optional): Fields to return in results
- `highlight` (optional): Highlighting configuration
- `aggs` (optional): Aggregations

**Example LLM prompt**: "Search for articles about 'elasticsearch' from author 'john' published in 2024"

---

### elasticsearch_delete_document

**What it does**: Deletes a single document by its ID from an index. Use the bulk API to delete multiple documents.

**When to use**: Remove outdated records, delete deprecated documents, or clean up test data.

**Arguments**:
- `index_name` (required): Index name
- `document_id` (required): Document ID
- `refresh` (optional): Refresh index immediately (default: false)

**Example LLM prompt**: "Delete the document with ID 'abc123' from the articles index"

---

### elasticsearch_list_clusters

**What it does**: Lists connected clusters in a cross-cluster setup. Returns cluster names, statuses, and connectivity info.

**When to use**: Understand cluster topology in multi-cluster deployments.

**Arguments**:
- `flat_settings` (optional): Return settings in flat format (default: false)

**Example LLM prompt**: "Show me the state of all connected clusters"

---

### elasticsearch_get_cluster_health

**What it does**: Gets the health status of the cluster (green, yellow, red). Returns cluster state, number of nodes, and shard allocation.

**When to use**: Check cluster health, monitor status, or diagnose issues.

**Arguments**:
- `index` (optional): Filter health by index name
- `level` (optional): Detail level - cluster, indices, shards (default: cluster)
- `timeout` (optional): Timeout for operation (default: 30s)

**Example LLM prompt**: "Get the health status of my Elasticsearch cluster"

---

### elasticsearch_get_node_stats

**What it does**: Gets detailed statistics for nodes in the cluster including CPU, memory, disk usage, and thread pool information.

**When to use**: Monitor cluster performance, check resource usage, or diagnose node issues.

**Arguments**:
- `node_id` (optional): Specific node ID (returns all if not specified)
- `metrics` (optional): Specific metrics to return (indices, os, process, jvm, etc.)
- `index` (optional): Filter stats by index

**Example LLM prompt**: "Get CPU and memory stats for all nodes in my cluster"

---

## Elasticsearch API Reference

These tools use the Elasticsearch REST API v8. See official docs for full details:
- https://www.elastic.co/guide/en/elasticsearch/reference/current/index.html
- Rate limits: Configured per-deployment (cloud) or based on memory (self-hosted)
- Pagination: Use `from` and `size` parameters (max 10,000 by default)
- All dates: ISO 8601 format (UTC)
