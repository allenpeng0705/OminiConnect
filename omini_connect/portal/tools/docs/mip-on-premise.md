# MIP On-Premise Tools

Provider: `mip-on-premise` | Engine: `nango` | Auth: TWO_STEP via Nango

## Overview

These tools wrap the MIP On-Premise API. They allow AI agents to manage companies, users, accounts, journals, and reports. **Requires MIP On-Premise credentials.**

## Authentication

**Nango TWO_STEP**:
- Uses login/password via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://api.mip.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `mip_on_premise_list_companies` | List companies | GET | /api/v1/companies |
| `mip_on_premise_get_company` | Get company details | GET | /api/v1/companies/{companyId} |
| `mip_on_premise_list_users` | List users | GET | /api/v1/users |
| `mip_on_premise_get_user` | Get user details | GET | /api/v1/users/{userId} |
| `mip_on_premise_list_accounts` | List accounts | GET | /api/v1/accounts |
| `mip_on_premise_get_account` | Get account details | GET | /api/v1/accounts/{accountId} |
| `mip_on_premise_list_journals` | List journals | GET | /api/v1/journals |
| `mip_on_premise_get_journal` | Get journal details | GET | /api/v1/journals/{journalId} |
| `mip_on_premise_list_reports` | List reports | GET | /api/v1/reports |
| `mip_on_premise_get_report` | Get report details | GET | /api/v1/reports/{reportId} |

---

## Tool Details

### mip_on_premise_list_companies

**What it does**: Lists all companies in MIP On-Premise.

**When to use**: Browse companies, find company IDs.

**Arguments**: None

**Example LLM prompt**: "List all companies in MIP On-Premise"

---

### mip_on_premise_get_company

**What it does**: Gets details of a specific company.

**When to use**: Check company information.

**Arguments**:
- `companyId` (required): Company ID

**Example LLM prompt**: "Get details for company 12345"

---

### mip_on_premise_list_users

**What it does**: Lists all users in MIP On-Premise.

**When to use**: User management, find users.

**Arguments**: None

**Example LLM prompt**: "List all users"

---

### mip_on_premise_get_user

**What it does**: Gets details of a specific user.

**When to use**: Check user details, permissions.

**Arguments**:
- `userId` (required): User ID

**Example LLM prompt**: "Get details for user 12345"

---

### mip_on_premise_list_accounts

**What it does**: Lists all accounts in MIP On-Premise.

**When to use**: Chart of accounts, account management.

**Arguments**:
- `company_id` (optional): Filter by company ID

**Example LLM prompt**: "List all accounts"

---

### mip_on_premise_get_account

**What it does**: Gets details of a specific account.

**When to use**: Check account details, balance.

**Arguments**:
- `accountId` (required): Account ID

**Example LLM prompt**: "Get details for account 12345"

---

### mip_on_premise_list_journals

**What it does**: Lists all journals in MIP On-Premise.

**When to use**: Journal management, find journals.

**Arguments**:
- `company_id` (optional): Filter by company ID

**Example LLM prompt**: "List all journals"

---

### mip_on_premise_get_journal

**What it does**: Gets details of a specific journal.

**When to use**: Check journal entries.

**Arguments**:
- `journalId` (required): Journal ID

**Example LLM prompt**: "Get details for journal 12345"

---

### mip_on_premise_list_reports

**What it does**: Lists all reports in MIP On-Premise.

**When to use**: Generate financial reports.

**Arguments**:
- `company_id` (optional): Filter by company ID

**Example LLM prompt**: "List all reports"

---

### mip_on_premise_get_report

**What it does**: Gets details of a specific report.

**When to use**: Get report configuration.

**Arguments**:
- `reportId` (required): Report ID

**Example LLM prompt**: "Get details for report 12345"

---

## MIP On-Premise Notes

- **On-premise deployment**: Self-hosted version
- **Companies**: Multi-entity support
- **Accounts**: Chart of accounts
- **Journals**: Accounting journals
- **Reports**: Financial reporting
