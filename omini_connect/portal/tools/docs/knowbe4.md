# KnowBe4 Tools

Provider: `knowbe4` | Engine: `nango` | Auth: API_KEY via Nango

## Overview

These tools wrap the KnowBe4 API. They allow AI agents to manage users, groups, campaigns, and security awareness training. **Requires KnowBe4 API key.**

## Authentication

**API Key via Nango**:
- User provides their KnowBe4 API key
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://{subdomain}.knowbe4.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `knowbe4_list_users` | List users | GET | /v1/users |
| `knowbe4_get_user` | Get user details | GET | /v1/users/{user_id} |
| `knowbe4_list_groups` | List groups | GET | /v1/groups |
| `knowbe4_get_group` | Get group details | GET | /v1/groups/{group_id} |
| `knowbe4_list_campaigns` | List campaigns | GET | /v1/campaigns |
| `knowbe4_get_campaign` | Get campaign details | GET | /v1/campaigns/{campaign_id} |
| `knowbe4_list_policies` | List policies | GET | /v1/policies |
| `knowbe4_list_phishing_tests` | List phishing tests | GET | /v1/phishing |
| `knowbe4_list_risky_users` | List risky users | GET | /v1/risky-users |
| `knowbe4_list_remediations` | List remediations | GET | /v1/remediations |

---

## Tool Details

### knowbe4_list_users

**What it does**: Lists all users in KnowBe4.

**When to use**: Find users, view training participants.

**Arguments**:
- `page` (optional): Page number (default: 1)
- `per_page` (optional): Results per page (default: 20)

**Example LLM prompt**: "List all users in KnowBe4"

---

### knowbe4_get_user

**What it does**: Gets details for a specific user.

**When to use**: Get user information, view training status.

**Arguments**:
- `user_id` (required): User ID

**Example LLM prompt**: "Get details for user 12345"

---

### knowbe4_list_groups

**What it does**: Lists all groups.

**When to use**: View groups, manage training groups.

**Arguments**:
- `page` (optional): Page number (default: 1)

**Example LLM prompt**: "List all groups in KnowBe4"

---

### knowbe4_get_group

**What it does**: Gets details for a specific group.

**When to use**: Get group information.

**Arguments**:
- `group_id` (required): Group ID

**Example LLM prompt**: "Get details for group 67890"

---

### knowbe4_list_campaigns

**What it does**: Lists all phishing campaigns.

**When to use**: View campaigns, track security tests.

**Arguments**:
- `page` (optional): Page number (default: 1)

**Example LLM prompt**: "List all campaigns in KnowBe4"

---

### knowbe4_get_campaign

**What it does**: Gets details for a specific campaign.

**When to use**: Get campaign information.

**Arguments**:
- `campaign_id` (required): Campaign ID

**Example LLM prompt**: "Get details for campaign c1"

---

### knowbe4_list_policies

**What it does**: Lists all policies.

**When to use**: View policies, manage security policies.

**Arguments**: None

**Example LLM prompt**: "List all policies in KnowBe4"

---

### knowbe4_list_phishing_tests

**What it does**: Lists all phishing tests.

**When to use**: View phishing tests, track security awareness.

**Arguments**:
- `page` (optional): Page number (default: 1)

**Example LLM prompt**: "List all phishing tests in KnowBe4"

---

### knowbe4_list_risky_users

**What it does**: Lists all risky users.

**When to use**: Find users at risk, prioritize training.

**Arguments**:
- `page` (optional): Page number (default: 1)

**Example LLM prompt**: "List all risky users in KnowBe4"

---

### knowbe4_list_remediations

**What it does**: Lists all remediations.

**When to use**: View remediation actions.

**Arguments**: None

**Example LLM prompt**: "List all remediations in KnowBe4"

---

## KnowBe4 API Notes

- **Security Awareness**: Phishing simulation and training platform
- **Users**: Employees undergoing security training
- **Groups**: Training groups and departments
- **Campaigns**: Phishing campaigns and tests
- **Risky Users**: Users identified as high security risk
