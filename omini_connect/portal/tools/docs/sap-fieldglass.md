# Sap-fieldglass Tools

Provider: `sap-fieldglass` | Engine: `nango` | Auth: OAuth via Nango

## Overview

SAP Fieldglass is a vendor management system (VMS) for managing contingent workforce. These tools allow AI agents to manage engagements, workers, invoices, and rate cards.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with SAP Fieldglass
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `engagements:read`, `engagements:write`, `workers:read`, `workers:write`, `invoices:read`, `invoices:write`, `rateCards:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `sap-fieldglass_list_engagements` | List all engagements | GET | /v1/engagements |
| `sap-fieldglass_get_engagement` | Get engagement details | GET | /v1/engagements/{engagementId} |
| `sap-fieldglass_create_engagement` | Create an engagement | POST | /v1/engagements |
| `sap-fieldglass_list_workers` | List all workers | GET | /v1/workers |
| `sap-fieldglass_get_worker` | Get worker details | GET | /v1/workers/{workerId} |
| `sap-fieldglass_list_invoices` | List all invoices | GET | /v1/invoices |
| `sap-fieldglass_get_invoice` | Get invoice details | GET | /v1/invoices/{invoiceId} |
| `sap-fieldglass_approve_invoice` | Approve an invoice | POST | /v1/invoices/{invoiceId}/approve |
| `sap-fieldglass_list_rate_cards` | List rate cards | GET | /v1/rateCards |
| `sap-fieldglass_get_rate_card` | Get rate card details | GET | /v1/rateCards/{rateCardId} |

---

## Tool Details

### sap-fieldglass_list_engagements

**What it does**: Returns a list of all engagements.

**When to use**: View workforce engagements.

**Arguments**:
- `limit` (optional): Number of engagements (default 50)
- `status` (optional): Filter by status

**Example LLM prompt**: "List all engagements"

---

### sap-fieldglass_get_engagement

**What it does**: Gets details of a specific engagement.

**When to use**: Get engagement information.

**Arguments**:
- `engagementId` (required): The engagement ID

**Example LLM prompt**: "Get details for engagement eng_abc123"

---

### sap-fieldglass_create_engagement

**What it does**: Creates a new engagement.

**When to use**: Start a new workforce engagement.

**Arguments**:
- `title` (required): Engagement title
- `startDate` (optional): Start date
- `endDate` (optional): End date
- `workerType` (optional): Worker type

**Example LLM prompt**: "Create an engagement called 'Marketing Project'"

---

### sap-fieldglass_list_workers

**What it does**: Returns a list of all workers.

**When to use**: View worker directory.

**Arguments**:
- `limit` (optional): Number of workers (default 50)
- `type` (optional): Filter by worker type

**Example LLM prompt**: "List all workers"

---

### sap-fieldglass_get_worker

**What it does**: Gets details of a specific worker.

**When to use**: Get worker information.

**Arguments**:
- `workerId` (required): The worker ID

**Example LLM prompt**: "Get details for worker wrk_xyz789"

---

### sap-fieldglass_list_invoices

**What it does**: Returns a list of all invoices.

**When to use**: View invoice history.

**Arguments**:
- `limit` (optional): Number of invoices (default 50)
- `status` (optional): Filter by status

**Example LLM prompt**: "List all pending invoices"

---

### sap-fieldglass_get_invoice

**What it does**: Gets details of a specific invoice.

**When to use**: Get invoice details.

**Arguments**:
- `invoiceId` (required): The invoice ID

**Example LLM prompt**: "Get details for invoice inv_abc123"

---

### sap-fieldglass_approve_invoice

**What it does**: Approves an invoice for payment.

**When to use**: Process invoice approval.

**Arguments**:
- `invoiceId` (required): The invoice ID to approve

**Example LLM prompt**: "Approve invoice inv_123"

---

### sap-fieldglass_list_rate_cards

**What it does**: Returns a list of rate cards.

**When to use**: View pricing templates.

**Arguments**:
- `limit` (optional): Number of rate cards (default 50)

**Example LLM prompt**: "List all rate cards"

---

### sap-fieldglass_get_rate_card

**What it does**: Gets details of a specific rate card.

**When to use**: Get rate card pricing.

**Arguments**:
- `rateCardId` (required): The rate card ID

**Example LLM prompt**: "Get rate card details for rc_abc123"

---

## SAP Fieldglass Notes

- Engagements track worker assignments
- Workers are contingent workforce members
- Invoices are processed through approval workflow
- Rate cards define worker pricing
