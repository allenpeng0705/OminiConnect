# Intuit Tools

Provider: `intuit` | Engine: `nango` | Auth: OAuth2 via Nango

## Overview

These tools wrap the QuickBooks API. They allow AI agents to manage companies, accounts, customers, vendors, and invoices. Intuit/QuickBooks is an accounting platform.

## Authentication

**Nango OAuth2**:
- User authenticates via Nango Connect with Intuit
- Token stored in Nango, accessed via `connection_ref`
- Authorization URL: https://appcenter.intuit.com/connect/oauth2
- Token URL: https://oauth.platform.intuit.com/oauth2/v1/tokens/bearer

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `intuit_list_companies` | List companies | GET | /v3/company |
| `intuit_get_company` | Get company details | GET | /v3/company/{companyId} |
| `intuit_list_accounts` | List accounts | GET | /v3/company/{companyId}/query |
| `intuit_get_account` | Get account details | GET | /v3/company/{companyId}/account/{accountId} |
| `intuit_list_customers` | List customers | GET | /v3/company/{companyId}/query |
| `intuit_get_customer` | Get customer details | GET | /v3/company/{companyId}/customer/{customerId} |
| `intuit_list_vendors` | List vendors | GET | /v3/company/{companyId}/query |
| `intuit_get_vendor` | Get vendor details | GET | /v3/company/{companyId}/vendor/{vendorId} |
| `intuit_list_invoices` | List invoices | GET | /v3/company/{companyId}/query |
| `intuit_get_invoice` | Get invoice details | GET | /v3/company/{companyId}/invoice/{invoiceId} |

---

## Tool Details

### intuit_list_companies

**What it does**: Lists all companies in QuickBooks.

**When to use**: View available companies.

**Arguments**: None

**Example LLM prompt**: "List all companies"

---

### intuit_get_company

**What it does**: Gets detailed information about a specific company.

**When to use**: View company settings.

**Arguments**:
- `companyId` (required): Company ID

**Example LLM prompt**: "Get company with ID abc123"

---

### intuit_list_accounts

**What it does**: Lists all accounts in QuickBooks.

**When to use**: Browse chart of accounts.

**Arguments**:
- `companyId` (required): Company ID

**Example LLM prompt**: "List all accounts for company abc123"

---

### intuit_get_account

**What it does**: Gets detailed information about a specific account.

**When to use**: View account balance and transactions.

**Arguments**:
- `companyId` (required): Company ID
- `accountId` (required): Account ID

**Example LLM prompt**: "Get account xyz789"

---

### intuit_list_customers

**What it does**: Lists all customers in QuickBooks.

**When to use**: Browse customer list.

**Arguments**:
- `companyId` (required): Company ID

**Example LLM prompt**: "List all customers for company abc123"

---

### intuit_get_customer

**What it does**: Gets detailed information about a specific customer.

**When to use**: View customer details and balance.

**Arguments**:
- `companyId` (required): Company ID
- `customerId` (required): Customer ID

**Example LLM prompt**: "Get customer xyz456"

---

### intuit_list_vendors

**What it does**: Lists all vendors in QuickBooks.

**When to use**: Browse vendor list.

**Arguments**:
- `companyId` (required): Company ID

**Example LLM prompt**: "List all vendors for company abc123"

---

### intuit_get_vendor

**What it does**: Gets detailed information about a specific vendor.

**When to use**: View vendor details and balance.

**Arguments**:
- `companyId` (required): Company ID
- `vendorId` (required): Vendor ID

**Example LLM prompt**: "Get vendor xyz789"

---

### intuit_list_invoices

**What it does**: Lists all invoices in QuickBooks.

**When to use**: Browse invoice list.

**Arguments**:
- `companyId` (required): Company ID

**Example LLM prompt**: "List all invoices for company abc123"

---

### intuit_get_invoice

**What it does**: Gets detailed information about a specific invoice.

**When to use**: View invoice details and payment status.

**Arguments**:
- `companyId` (required): Company ID
- `invoiceId` (required): Invoice ID

**Example LLM prompt**: "Get invoice inv123"

---

## Intuit API Notes

- **API Base URL**: https://quickbooks.api.intuit.com
- **Auth Mode**: OAuth2
- **Companies**: QuickBooks company files
- **Accounts**: Chart of accounts
- **Customers**: Customer accounts receivable
- **Vendors**: Vendor accounts payable
- **Invoices**: Sales invoices
