# Perk Tools

Provider: `perk` | Engine: `nango` | Auth: OAuth2 via Nango

## Overview

These tools wrap the Perk (TravelPerk) API. They allow AI agents to manage users, trips, expenses, invoices, and cost centers. Perk is a travel and expense management platform. **Requires Perk OAuth2 authentication.**

## Authentication

**Nango OAuth2**:
- User authenticates via Nango Connect with Perk
- Token stored in Nango, accessed via `connection_ref`
- Base URL: https://api.travelperk.com
- Disable PKCE: true

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `perk_list_users` | List users | GET | /v1/users |
| `perk_get_user` | Get user details | GET | /v1/users/{userId} |
| `perk_list_trips` | List trips | GET | /v1/trips |
| `perk_get_trip` | Get trip details | GET | /v1/trips/{tripId} |
| `perk_list_expenses` | List expenses | GET | /v1/expenses |
| `perk_get_expense` | Get expense details | GET | /v1/expenses/{expenseId} |
| `perk_list_invoices` | List invoices | GET | /v1/invoices |
| `perk_get_invoice` | Get invoice details | GET | /v1/invoices/{invoiceId} |
| `perk_list_cost_centers` | List cost centers | GET | /v1/cost-centers |
| `perk_get_profile` | Get user profile | GET | /v1/profile |

---

## Tool Details

### perk_list_users

**What it does**: Lists all users in the organization.

**When to use**: Browse team directory.

**Arguments**:
- `status` (optional): Filter by status (active, inactive)

**Example LLM prompt**: "List all active users"

---

### perk_get_user

**What it does**: Gets detailed information about a specific user.

**When to use**: Get user details, permissions.

**Arguments**:
- `userId` (required): User ID

**Example LLM prompt**: "Get details for user 12345"

---

### perk_list_trips

**What it does**: Lists all trips in the organization.

**When to use**: Browse trip history, track travel.

**Arguments**:
- `status` (optional): Filter by status
- `startDate` (optional): Start date (YYYY-MM-DD)
- `endDate` (optional): End date (YYYY-MM-DD)

**Example LLM prompt**: "Show trips from this month"

---

### perk_get_trip

**What it does**: Gets detailed information about a specific trip.

**When to use**: Get trip details, itinerary.

**Arguments**:
- `tripId` (required): Trip ID

**Example LLM prompt**: "Get details for trip T-67890"

---

### perk_list_expenses

**What it does**: Lists all expenses in the organization.

**When to use**: Browse expenses, track spending.

**Arguments**:
- `status` (optional): Filter by status
- `userId` (optional): Filter by user ID

**Example LLM prompt**: "List all pending expenses"

---

### perk_get_expense

**What it does**: Gets detailed information about a specific expense.

**When to use**: Get expense details, receipts.

**Arguments**:
- `expenseId` (required): Expense ID

**Example LLM prompt**: "Get details for expense E-11111"

---

### perk_list_invoices

**What it does**: Lists all invoices in the organization.

**When to use**: Browse invoice history.

**Arguments**:
- `status` (optional): Filter by status

**Example LLM prompt**: "List all invoices"

---

### perk_get_invoice

**What it does**: Gets detailed information about a specific invoice.

**When to use**: Get invoice details.

**Arguments**:
- `invoiceId` (required): Invoice ID

**Example LLM prompt**: "Get details for invoice INV-22222"

---

### perk_list_cost_centers

**What it does**: Lists all cost centers in the organization.

**When to use**: Browse cost allocation structure.

**Arguments**: None

**Example LLM prompt**: "What cost centers are configured?"

---

### perk_get_profile

**What it does**: Gets current user profile information.

**When to use**: Get user profile, settings.

**Arguments**: None

**Example LLM prompt**: "Get my profile information"

---

## Perk API Notes

- **OAuth2**: Requires user authentication via OAuth flow
- **Travel Management**: Primarily for travel and expense management
- **Rate limits**: Apply to API calls
- **Date formats**: Use YYYY-MM-DD format
