# Entrata Tools

Provider: `entrata` | Engine: `nango` | Auth: BASIC via Nango

## Overview

These tools wrap the Entrata API. They allow AI agents to manage properties, units, tenants, leases, and work orders. Entrata is a property management platform for residential and commercial real estate.

## Authentication

**Nango BASIC**:
- User provides subdomain and credentials via Nango Connect
- Subdomain is used in the base URL: `https://{subdomain}.entrata.com`
- Credentials are base64 encoded for Basic auth

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `entrata_list_properties` | List properties | GET | /api/v1/properties |
| `entrata_get_property` | Get property details | GET | /api/v1/properties/{id} |
| `entrata_list_units` | List units | GET | /api/v1/properties/{propertyId}/units |
| `entrata_get_unit` | Get unit details | GET | /api/v1/units/{id} |
| `entrata_list_tenants` | List tenants | GET | /api/v1/tenants |
| `entrata_get_tenant` | Get tenant details | GET | /api/v1/tenants/{id} |
| `entrata_list_leases` | List leases | GET | /api/v1/leases |
| `entrata_get_lease` | Get lease details | GET | /api/v1/leases/{id} |
| `entrata_list_work_orders` | List work orders | GET | /api/v1/workorders |
| `entrata_create_work_order` | Create work order | POST | /api/v1/workorders |

---

## Tool Details

### entrata_list_properties

**What it does**: Lists all properties in the account.

**When to use**: Browse property portfolio, find property IDs.

**Arguments**: None

**Example LLM prompt**: "List all properties"

---

### entrata_get_property

**What it does**: Gets detailed information about a specific property.

**When to use**: Get property details, manage property settings.

**Arguments**:
- `id` (required): Property ID

**Example LLM prompt**: "Get details for property 123"

---

### entrata_list_units

**What it does**: Lists all units for a specific property.

**When to use**: View unit availability, check occupancy.

**Arguments**:
- `propertyId` (required): Property ID

**Example LLM prompt**: "List units at property 123"

---

### entrata_get_unit

**What it does**: Gets details of a specific unit.

**When to use**: Check unit status, get unit details.

**Arguments**:
- `id` (required): Unit ID

**Example LLM prompt**: "Get details for unit 456"

---

### entrata_list_tenants

**What it does**: Lists tenants with optional property filtering.

**When to use**: Find tenant information, manage occupancy.

**Arguments**:
- `property_id` (optional): Filter by property

**Example LLM prompt**: "List all tenants at property 123"

---

### entrata_get_tenant

**What it does**: Gets detailed tenant information.

**When to use**: Get tenant contact info, lease details.

**Arguments**:
- `id` (required): Tenant ID

**Example LLM prompt**: "Get details for tenant 789"

---

### entrata_list_leases

**What it does**: Lists leases with filtering options.

**When to use**: Find expiring leases, track occupancy.

**Arguments**:
- `property_id` (optional): Filter by property
- `status` (optional): Filter by status (active, expired)

**Example LLM prompt**: "List all active leases"

---

### entrata_get_lease

**What it does**: Gets details of a specific lease.

**When to use**: Review lease terms, check rent amounts.

**Arguments**:
- `id` (required): Lease ID

**Example LLM prompt**: "Get details for lease 101"

---

### entrata_list_work_orders

**What it does**: Lists maintenance work orders.

**When to use**: Track maintenance requests, assign work.

**Arguments**:
- `property_id` (optional): Filter by property
- `status` (optional): Filter by status

**Example LLM prompt**: "List all open work orders"

---

### entrata_create_work_order

**What it does**: Creates a new maintenance work order.

**When to use**: Report maintenance issues, schedule repairs.

**Arguments**:
- `property_id` (required): Property ID
- `unit_id` (optional): Unit ID
- `description` (required): Work order description
- `priority` (optional): Priority level

**Example LLM prompt**: "Create a work order for a leaky faucet at unit 456"

---

## Entrata API Notes

- **Subdomain**: Each Entrata account has a unique subdomain
- **Property IDs**: Identify each property in the account
- **Unit IDs**: Identify units within properties
- **Work Orders**: Track maintenance requests and repairs
- **Leases**: Link tenants to units with rental terms
