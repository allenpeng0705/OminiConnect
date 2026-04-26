# Splitwise Tools

Provider: `splitwise` | Engine: `nango` | Auth: OAUTH2

## Overview

These tools wrap the Splitwise API. They allow AI agents to interact with Splitwise functionality. **Requires OAUTH2 authentication.**

## Authentication

**OAuth2 Authentication**:
- User authenticates via OAuth2 authorization code flow
- Nango manages the OAuth handshake and token refresh
- Default scopes depend on the provider configuration

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `get_user` | Get current user | GET | /api/v3/get_user |
| `list_friends` | List all friends | GET | /api/v3/get_friends |
| `list_expenses` | List all expenses | GET | /api/v3/get_expenses |
| `create_expense` | Create an expense | POST | /api/v3/create_expense |
| `list_groups` | List all groups | GET | /api/v3/get_groups |
| `get_group` | Get group details | GET | /api/v3/get_group/{id} |
| `list_comments` | List comments on expense | GET | /api/v3/get_comments |
| `create_comment` | Add comment to expense | POST | /api/v3/create_comment |
| `list_currencies` | List supported currencies | GET | /api/v3/get_currencies |
| `get_balances` | Get user balances | GET | /api/v3/get_balances |

---

## Tool Details

### get_user

**What it does**: Get current user

**When to use**: Use this tool when you need to get current user.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get user to..."

---

### list_friends

**What it does**: List all friends

**When to use**: Use this tool when you need to list all friends.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list friends to..."

---

### list_expenses

**What it does**: List all expenses

**When to use**: Use this tool when you need to list all expenses.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list expenses to..."

---

### create_expense

**What it does**: Create an expense

**When to use**: Use this tool when you need to create an expense.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use create expense to..."

---

### list_groups

**What it does**: List all groups

**When to use**: Use this tool when you need to list all groups.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list groups to..."

---

### get_group

**What it does**: Get group details

**When to use**: Use this tool when you need to get group details.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get group to..."

---

### list_comments

**What it does**: List comments on expense

**When to use**: Use this tool when you need to list comments on expense.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list comments to..."

---

### create_comment

**What it does**: Add comment to expense

**When to use**: Use this tool when you need to add comment to expense.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use create comment to..."

---

### list_currencies

**What it does**: List supported currencies

**When to use**: Use this tool when you need to list supported currencies.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use list currencies to..."

---

### get_balances

**What it does**: Get user balances

**When to use**: Use this tool when you need to get user balances.

**Arguments**:
- None required for this tool

**Example LLM prompt**: "Use get balances to..."

---

## Splitwise API Notes

- **Auth mode**: OAUTH2
- **Base URL**: https://secure.splitwise.com
- **API prefix**: /
- **Rate limits**: Check provider documentation for specific limits
