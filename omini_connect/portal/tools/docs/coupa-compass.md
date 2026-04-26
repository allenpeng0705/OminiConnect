# Coupa Compass Tools

Provider: `coupa-compass` | Engine: `nango` | Auth: OAuth2 Client Credentials via Nango

## Overview

These tools wrap the Coupa Compass API. They allow AI agents to manage invoices, purchase orders, suppliers, and contracts. Coupa Compass is a business spend management platform.

## Authentication

**Nango OAuth2 Client Credentials**:
- Uses client_id and client_secret for machine-to-machine authentication
- Token stored in Nango, accessed via `connection_ref`
- Instance domain configured in connection config

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `coupa_compass_list_invoices` | List invoices | GET | /invoices |
| `coupa_compass_get_invoice` | Get invoice details | GET | /invoices/{id} |
| `coupa_compass_list_purchase_orders` | List purchase orders | GET | /purchase_orders |
| `coupa_compass_get_purchase_order` | Get purchase order details | GET | /purchase_orders/{id} |
| `coupa_compass_list_suppliers` | List suppliers | GET | /suppliers |
| `coupa_compass_get_supplier` | Get supplier details | GET | /suppliers/{id} |
| `coupa_compass_list_contracts` | List contracts | GET | /contracts |
| `coupa_compass_get_contract` | Get contract details | GET | /contracts/{id} |
| `coupa_compass_list_approvals` | List pending approvals | GET | /approvals |
| `coupa_compass_list_spend_data` | List spend data | GET | /spend |

---

## Tool Details

### coupa_compass_list_invoices

**What it does**: Lists all invoices with optional filtering by status, supplier, or date.

**When to use**: Review outstanding invoices, track payment status, find invoices by supplier.

**Arguments**:
- `status` (optional): Filter by submitted, approved, paid, rejected
- `supplier_id` (optional): Filter by supplier ID
- `from_date` (optional): From date (YYYY-MM-DD)
- `to_date` (optional): To date (YYYY-MM-DD)
- `limit` (optional): Max results (default 50, max 200)

**Example LLM prompt**: "List all approved invoices from the last 30 days"

---

### coupa_compass_get_invoice

**What it does**: Gets detailed invoice information including line items and payment terms.

**When to use**: Review invoice details, check line item pricing, verify approval status.

**Arguments**:
- `id` (required): Invoice ID

**Example LLM prompt**: "Get details for invoice INV-12345"

---

### coupa_compass_list_purchase_orders

**What it does**: Lists all purchase orders with filtering options.

**When to use**: Track pending orders, find orders by supplier, monitor requisition status.

**Arguments**:
- `status` (optional): Filter by draft, submitted, approved, ordered
- `supplier_id` (optional): Filter by supplier ID
- `limit` (optional): Max results (default 50, max 200)

**Example LLM prompt**: "List all submitted purchase orders"

---

### coupa_compass_get_purchase_order

**What it does**: Gets detailed purchase order including line items and delivery schedule.

**When to use**: Verify PO details, check quantities and pricing, track delivery status.

**Arguments**:
- `id` (required): Purchase Order ID

**Example LLM prompt**: "Get details for PO PO-67890"

---

### coupa_compass_list_suppliers

**What it does**: Lists all suppliers in the system.

**When to use**: Browse approved suppliers, find supplier details, evaluate vendor options.

**Arguments**:
- `limit` (optional): Max results (default 50, max 200)
- `name` (optional): Filter by supplier name

**Example LLM prompt**: "List all suppliers named Acme"

---

### coupa_compass_get_supplier

**What it does**: Gets detailed supplier information including contact and performance data.

**When to use**: Get supplier contact info, check performance ratings, review contract terms.

**Arguments**:
- `id` (required): Supplier ID

**Example LLM prompt**: "Get details for supplier S-001"

---

### coupa_compass_list_contracts

**What it does**: Lists all contracts with status and type filtering.

**When to use**: Find expiring contracts, review active agreements, search by contract type.

**Arguments**:
- `status` (optional): Filter by draft, active, expired
- `contract_type` (optional): Filter by contract type
- `limit` (optional): Max results (default 50, max 200)

**Example LLM prompt**: "List all active contracts expiring this year"

---

### coupa_compass_get_contract

**What it does**: Gets detailed contract information including terms and pricing tiers.

**When to use**: Review contract details, check pricing, find associated documents.

**Arguments**:
- `id` (required): Contract ID

**Example LLM prompt**: "Get details for contract C-123"

---

### coupa_compass_list_approvals

**What it does**: Lists all pending approvals for the authenticated user.

**When to use**: Find items needing approval, track approval queue, review pending spend.

**Arguments**:
- `status` (optional): Filter by pending, approved, rejected
- `limit` (optional): Max results (default 50, max 200)

**Example LLM prompt**: "List all my pending approvals"

---

### coupa_compass_list_spend_data

**What it does**: Lists spend analytics grouped by supplier, department, or category.

**When to use**: Analyze spending patterns, identify cost reduction opportunities, track budget utilization.

**Arguments**:
- `from_date` (optional): From date (YYYY-MM-DD)
- `to_date` (optional): To date (YYYY-MM-DD)
- `group_by` (optional): Group by supplier, department, or category (default: supplier)
- `limit` (optional): Max results (default 50, max 200)

**Example LLM prompt**: "Show spend by department for Q1 2024"

---

## Coupa Compass API Notes

- **Client Credentials**: Uses OAuth2 machine-to-machine flow, not user OAuth
- **Instance Domain**: Configured per-connection for multi-tenant deployment
- **Invoices**: Spend documents from suppliers requesting payment
- **Purchase Orders**: Internal requisitions approved before spending
- **Contracts**: Legal agreements with suppliers defining terms and pricing
- **Approvals**: Workflow queue for pending authorizations
