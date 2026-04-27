# Ironclad Tools

Provider: `ironclad` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Ironclad Contract Management API. They allow AI agents to manage contracts, counterparties, and templates for legal workflow automation. Ironclad is a leading platform for contract lifecycle management (CLM).

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Ironclad
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `contract:read`, `contract:write`, `counterparty:read`, `template:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `ironclad_list_contracts` | List contracts with filtering | GET | /api/v2/contracts |
| `ironclad_get_contract` | Get contract details | GET | /api/v2/contracts/{id} |
| `ironclad_create_contract` | Create a new contract | POST | /api/v2/contracts |
| `ironclad_update_contract` | Update a contract | PATCH | /api/v2/contracts/{id} |
| `ironclad_list_counterparties` | List counterparties | GET | /api/v2/counterparties |
| `ironclad_get_counterparty` | Get counterparty details | GET | /api/v2/counterparties/{id} |
| `ironclad_list_templates` | List contract templates | GET | /api/v2/templates |
| `ironclad_get_template` | Get template details | GET | /api/v2/templates/{id} |
| `ironclad_search_contracts` | Search contracts | POST | /api/v2/contracts/search |
| `ironclad_get_contract_metadata` | Get contract metadata | GET | /api/v2/contracts/{id}/metadata |

---

## Tool Details

### ironclad_list_contracts

**What it does**: Lists all contracts with optional filtering by status, counterparty, or date range.

**When to use**: Find contracts by status (draft, pending, active, closed), see all contracts with a specific counterparty, or filter by creation date.

**Arguments**:
- `status` (optional): Filter by status (`draft`, `pending`, `active`, `closed`)
- `counterparty_id` (optional): Filter by counterparty ID
- `from_date` (optional): Filter contracts created after this date (ISO 8601)
- `to_date` (optional): Filter contracts created before this date (ISO 8601)
- `limit` (optional): Number of results (default 50)
- `offset` (optional): Pagination offset (default 0)

**Example LLM prompt**: "List all active contracts created in the last 90 days"

---

### ironclad_get_contract

**What it does**: Gets detailed information about a specific contract including parties, terms, and metadata.

**When to use**: Get full details of a contract before updating or analyzing it.

**Arguments**:
- `id` (required): Contract ID

**Example LLM prompt**: "Get details for contract abc-123-def"

---

### ironclad_create_contract

**What it does**: Creates a new contract in Ironclad with specified terms and parties.

**When to use**: Initiate a new contract with a counterparty, optionally using a template.

**Arguments**:
- `title` (required): Contract title
- `counterparty_id` (optional): Counterparty ID
- `template_id` (optional): Template ID to use
- `effective_date` (optional): Contract effective date (ISO 8601)
- `expiration_date` (optional): Contract expiration date (ISO 8601)

**Example LLM prompt**: "Create a new contract titled 'Master Services Agreement' with counterparty Acme Corp"

---

### ironclad_update_contract

**What it does**: Updates an existing contract's fields, status, or terms.

**When to use**: Change contract status, update terms, or modify dates.

**Arguments**:
- `id` (required): Contract ID
- `title` (optional): Updated contract title
- `status` (optional): Updated status
- `effective_date` (optional): Updated effective date (ISO 8601)
- `expiration_date` (optional): Updated expiration date (ISO 8601)

**Example LLM prompt**: "Update contract abc-123 to status 'active' with new expiration date of Dec 31 2026"

---

### ironclad_list_counterparties

**What it does**: Retrieves a list of all counterparties (organizations you contract with).

**When to use**: Find potential counterparties, check existing relationships before creating contracts.

**Arguments**:
- `search` (optional): Search by counterparty name
- `limit` (optional): Number of results (default 50)
- `offset` (optional): Pagination offset (default 0)

**Example LLM prompt**: "List all counterparties with 'Acme' in the name"

---

### ironclad_get_counterparty

**What it does**: Gets detailed information about a specific counterparty including contact info and contract count.

**When to use**: Get full details of a counterparty before creating or updating a contract.

**Arguments**:
- `id` (required): Counterparty ID

**Example LLM prompt**: "Get details for counterparty xyz-789"

---

### ironclad_list_templates

**What it does**: Retrieves a list of contract templates available for creating new contracts.

**When to use**: Find available templates before creating a new contract.

**Arguments**:
- `category` (optional): Filter by template category
- `limit` (optional): Number of results (default 50)

**Example LLM prompt**: "List all contract templates in the 'Sales' category"

---

### ironclad_get_template

**What it does**: Gets detailed information about a contract template including fields and structure.

**When to use**: Understand template fields and requirements before creating a contract from it.

**Arguments**:
- `id` (required): Template ID

**Example LLM prompt**: "Get details for template template-123"

---

### ironclad_search_contracts

**What it does**: Searches for contracts using full-text search across contract titles, content, and metadata.

**When to use**: Find specific contracts when you know keywords or terms but not the exact contract ID.

**Arguments**:
- `query` (required): Search query string
- `status` (optional): Filter by contract statuses (array)
- `date_range` (optional): Date range filter with `from` and `to` fields
- `limit` (optional): Maximum number of results (default 20)

**Example LLM prompt**: "Search for contracts containing 'NDA' and '2026'"

---

### ironclad_get_contract_metadata

**What it does**: Retrieves metadata and custom fields for a specific contract.

**When to use**: Get custom field values and detailed metadata that is not included in the basic contract response.

**Arguments**:
- `id` (required): Contract ID

**Example LLM prompt**: "Get metadata for contract abc-123 to see custom fields"

---

## Ironclad API Notes

- **Contract IDs**: UUID format (e.g., `550e8400-e29b-41d4-a716-446655440000`)
- **Counterparties**: Organizations you have contractual relationships with
- **Templates**: Pre-defined contract structures that can include default terms and fields
- **Search**: Full-text search across titles, content, and metadata fields
- **Metadata**: Custom fields specific to your Ironclad configuration