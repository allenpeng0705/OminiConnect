# Buildium Tools

Provider: `buildium` | Engine: `nango` | Auth: API_KEY via Nango

## Overview

These tools wrap the Buildium API. They allow AI agents to manage residents, rental units, leases, transactions, and maintenance requests for property management. Buildium is a property management platform for residential and commercial real estate.

## Authentication

**Nango API_KEY**:
- User provides Buildium client ID and secret
- Token stored in Nango, accessed via `connection_ref`
- Client ID in header, secret in Authorization

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `buildium_list_residents` | List residents | GET | /v1/residents |
| `buildium_get_resident` | Get resident details | GET | /v1/residents/{residentId} |
| `buildium_list_rentals` | List rental units | GET | /v1/rentals |
| `buildium_get_rental` | Get rental unit details | GET | /v1/rentals/{rentalId} |
| `buildium_list_leases` | List leases | GET | /v1/leases |
| `buildium_get_lease` | Get lease details | GET | /v1/leases/{leaseId} |
| `buildium_list_transactions` | List transactions | GET | /v1/transactions |
| `buildium_get_transaction` | Get transaction details | GET | /v1/transactions/{transactionId} |
| `buildium_list_maintenance` | List maintenance requests | GET | /v1/maintenance |
| `buildium_get_maintenance` | Get maintenance request details | GET | /v1/maintenance/{maintenanceId} |

---

## Tool Details

### buildium_list_residents

**What it does**: Lists all residents in Buildium.

**When to use**: View tenant directory, find residents.

**Arguments**:
- `page` (optional): Page number (default 1)
- `pageSize` (optional): Residents per page (default 20)

**Example LLM prompt**: "List all residents"

---

### buildium_get_resident

**What it does**: Gets details for a specific resident.

**When to use**: View resident info, contact details.

**Arguments**:
- `residentId` (required): Resident ID

**Example LLM prompt**: "Get details for resident 123"

---

### buildium_list_rentals

**What it does**: Lists all rental units.

**When to use**: Browse available units, property inventory.

**Arguments**:
- `page` (optional): Page number (default 1)
- `pageSize` (optional): Rentals per page (default 20)

**Example LLM prompt**: "List all rental units"

---

### buildium_get_rental

**What it does**: Gets details for a specific rental unit.

**When to use**: Check unit details, occupancy status.

**Arguments**:
- `rentalId` (required): Rental ID

**Example LLM prompt**: "Get details for rental unit 456"

---

### buildium_list_leases

**What it does**: Lists all leases.

**When to use**: Track lease agreements, renewals.

**Arguments**:
- `status` (optional): Filter by status
- `page` (optional): Page number (default 1)
- `pageSize` (optional): Leases per page (default 20)

**Example LLM prompt**: "List all active leases"

---

### buildium_get_lease

**What it does**: Gets details for a specific lease.

**When to use**: View lease terms, parties, dates.

**Arguments**:
- `leaseId` (required): Lease ID

**Example LLM prompt**: "Get details for lease 789"

---

### buildium_list_transactions

**What it does**: Lists all financial transactions.

**When to use**: Track rent payments, expenses.

**Arguments**:
- `startDate` (optional): Start date
- `endDate` (optional): End date
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all transactions from this month"

---

### buildium_get_transaction

**What it does**: Gets details for a specific transaction.

**When to use**: View transaction details, payment info.

**Arguments**:
- `transactionId` (required): Transaction ID

**Example LLM prompt**: "Get details for transaction T-100"

---

### buildium_list_maintenance

**What it does**: Lists all maintenance requests.

**When to use**: Track maintenance issues, work orders.

**Arguments**:
- `status` (optional): Filter by status
- `page` (optional): Page number (default 1)

**Example LLM prompt**: "List all open maintenance requests"

---

### buildium_get_maintenance

**What it does**: Gets details for a specific maintenance request.

**When to use**: View maintenance details, status updates.

**Arguments**:
- `maintenanceId` (required): Maintenance ID

**Example LLM prompt**: "Get details for maintenance M-200"

---

## Buildium API Notes

- **Residents**: Tenants and occupants in properties
- **Rentals**: Physical units available for rent
- **Leases**: Legal agreements between owners and residents
- **Transactions**: Financial movements (rent, deposits, fees)
- **Maintenance**: Work orders and repair requests
