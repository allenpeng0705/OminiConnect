# OpenAI (Admin) Tools

Provider: `openai-admin` | Engine: `nango` | Auth: API_KEY via Nango

## Overview

These tools wrap the OpenAI Admin API for organization management. They allow AI agents to manage users, API tokens, view usage statistics, and billing information. **Requires OpenAI Admin API key authentication.**

## Authentication

**API Key**:
- User provides OpenAI Admin API key
- Key passed via `Authorization: Bearer` header
- Base URL: `https://api.openai.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `openai_list_users` | List organization users | GET | /v1/organization/users |
| `openai_get_user` | Get user details | GET | /v1/organization/users/{user_id} |
| `openai_invite_user` | Invite user | POST | /v1/organization/invites |
| `openai_list_models` | List available models | GET | /v1/models |
| `openai_list_tokens` | List API tokens | GET | /v1/organization/api_tokens |
| `openai_create_token` | Create API token | POST | /v1/organization/api_tokens |
| `openai_revoke_token` | Revoke API token | DELETE | /v1/organization/api_tokens/{token_id} |
| `openai_get_usage` | Get usage statistics | GET | /v1/usage |
| `openai_get_billing` | Get billing information | GET | /v1/billing |
| `openai_get_settings` | Get organization settings | GET | /v1/organization |

---

## Tool Details

### openai_list_users

**What it does**: Lists all users in the OpenAI organization.

**When to use**: Browse organization members, find users.

**Arguments**:
- `limit` (optional): Number of users (default 100)

**Example LLM prompt**: "List all users in our OpenAI organization"

---

### openai_get_user

**What it does**: Gets detailed information for a specific user.

**When to use**: View user role, permissions.

**Arguments**:
- `user_id` (required): User ID

**Example LLM prompt**: "Get details for user user_123"

---

### openai_invite_user

**What it does**: Invites a new user to the OpenAI organization.

**When to use**: Add team members, onboard users.

**Arguments**:
- `email` (required): User email
- `role` (optional): User role (member, admin)

**Example LLM prompt**: "Invite john@company.com to the organization as admin"

---

### openai_list_models

**What it does**: Lists all available models in the organization.

**When to use**: Check available models, usage policies.

**Arguments**: None

**Example LLM prompt**: "List all available OpenAI models"

---

### openai_list_tokens

**What it does**: Lists all API tokens in the organization.

**When to use**: View active tokens, audit access.

**Arguments**: None

**Example LLM prompt**: "List all API tokens"

---

### openai_create_token

**What it does**: Creates a new API token.

**When to use**: Generate access for applications, services.

**Arguments**:
- `name` (required): Token name
- `scopes` (optional): Token scopes

**Example LLM prompt**: "Create a token called 'production-app' with model permissions"

---

### openai_revoke_token

**What it does**: Revokes an existing API token.

**When to use**: Remove access, security incidents.

**Arguments**:
- `token_id` (required): Token ID

**Example LLM prompt**: "Revoke token tk_123456"

---

### openai_get_usage

**What it does**: Gets usage statistics for the organization.

**When to use**: Monitor API usage, track costs.

**Arguments**:
- `start_date` (optional): Start date (YYYY-MM-DD)
- `end_date` (optional): End date (YYYY-MM-DD)

**Example LLM prompt**: "Get usage statistics for this month"

---

### openai_get_billing

**What it does**: Gets billing information for the organization.

**When to use**: View invoice, check balance.

**Arguments**: None

**Example LLM prompt**: "Get current billing information"

---

### openai_get_settings

**What it does**: Gets organization settings.

**When to use**: View organization config, default settings.

**Arguments**: None

**Example LLM prompt**: "Get organization settings"

---

## OpenAI Admin Notes

- **Admin key**: Requires OpenAI admin API key (not regular API key)
- **Organization scope**: All operations are at organization level
- **User roles**: member, admin
- **Token scopes**: Fine-grained permissions for API access
- **Rate limits**: Respects x-ratelimit-reset-requests header
