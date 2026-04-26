# PreciseFP Tools

Provider: `precisefp` | Engine: `nango` | Auth: OAuth2 via Nango

## Overview

These tools wrap the PreciseFP API. They allow AI agents to manage clients, assets, liabilities, goals, and documents. PreciseFP is a financial planning platform. **Requires PreciseFP OAuth2 authentication.**

## Authentication

**Nango OAuth2**:
- User authenticates via Nango Connect with PreciseFP
- Token stored in Nango, accessed via `connection_ref`
- Base URL: https://app.precisefp.com
- Uses wildcard scope '*'

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `precisefp_list_clients` | List clients | GET | /api/clients |
| `precisefp_get_client` | Get client details | GET | /api/clients/{clientId} |
| `precisefp_list_assets` | List assets | GET | /api/assets |
| `precisefp_get_asset` | Get asset details | GET | /api/assets/{assetId} |
| `precisefp_list_liabilities` | List liabilities | GET | /api/liabilities |
| `precisefp_get_liability` | Get liability details | GET | /api/liabilities/{liabilityId} |
| `precisefp_list_goals` | List goals | GET | /api/goals |
| `precisefp_get_goal` | Get goal details | GET | /api/goals/{goalId} |
| `precisefp_list_documents` | List documents | GET | /api/documents |
| `precisefp_get_user` | Get user info | GET | /api/user |

---

## Tool Details

### precisefp_list_clients

**What it does**: Lists all clients in the organization.

**When to use**: Browse client directory.

**Arguments**:
- `status` (optional): Filter by status

**Example LLM prompt**: "List all clients"

---

### precisefp_get_client

**What it does**: Gets detailed information about a specific client.

**When to use**: Get client profile, financial overview.

**Arguments**:
- `clientId` (required): Client ID

**Example LLM prompt**: "Get details for client 12345"

---

### precisefp_list_assets

**What it does**: Lists all assets in the organization.

**When to use**: Browse asset portfolio.

**Arguments**:
- `clientId` (optional): Filter by client

**Example LLM prompt**: "List assets for client 12345"

---

### precisefp_get_asset

**What it does**: Gets detailed information about a specific asset.

**When to use**: Get asset details, value.

**Arguments**:
- `assetId` (required): Asset ID

**Example LLM prompt**: "Get details for asset 67890"

---

### precisefp_list_liabilities

**What it does**: Lists all liabilities in the organization.

**When to use**: Browse liability portfolio.

**Arguments**:
- `clientId` (optional): Filter by client

**Example LLM prompt**: "List liabilities for client 12345"

---

### precisefp_get_liability

**What it does**: Gets detailed information about a specific liability.

**When to use**: Get liability details.

**Arguments**:
- `liabilityId` (required): Liability ID

**Example LLM prompt**: "Get details for liability 11111"

---

### precisefp_list_goals

**What it does**: Lists all financial goals.

**When to use**: Browse client goals.

**Arguments**:
- `clientId` (optional): Filter by client

**Example LLM prompt**: "List goals for client 12345"

---

### precisefp_get_goal

**What it does**: Gets detailed information about a specific goal.

**When to use**: Get goal progress, details.

**Arguments**:
- `goalId` (required): Goal ID

**Example LLM prompt**: "Get details for goal 22222"

---

### precisefp_list_documents

**What it does**: Lists all documents.

**When to use**: Browse client documents.

**Arguments**:
- `clientId` (optional): Filter by client

**Example LLM prompt**: "List documents for client 12345"

---

### precisefp_get_user

**What it does**: Gets current user information.

**When to use**: Get user profile, permissions.

**Arguments**: None

**Example LLM prompt**: "Get my user information"

---

## PreciseFP API Notes

- **OAuth2**: Requires user authentication via OAuth flow
- **Financial Planning**: Assets, liabilities, goals
- **Client Management**: Financial profiles
