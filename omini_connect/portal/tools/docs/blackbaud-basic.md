# Blackbaud (Basic Auth) Tools

Provider: `blackbaud-basic` | Engine: `nango` | Auth: BASIC via Nango

## Overview

These tools wrap the Blackbaud SKY API using Basic Authentication. They allow AI agents to manage constituents (donors), gifts, campaigns, funds, and appeals for nonprofit fundraising and CRM. Blackbaud is a provider of software and services for nonprofits.

## Authentication

**Nango BASIC Auth**:
- User provides username and password
- Token stored in Nango, accessed via `connection_ref`
- Requires hostname configuration

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `blackbaud_basic_list_constituents` | List constituents | GET | /constituents/v1/constituents |
| `blackbaud_basic_get_constituent` | Get constituent details | GET | /constituents/v1/constituents/{id} |
| `blackbaud_basic_list_gifts` | List gifts | GET | /gift/v1/gifts |
| `blackbaud_basic_get_gift` | Get gift details | GET | /gift/v1/gifts/{id} |
| `blackbaud_basic_list_campaigns` | List campaigns | GET | /campaign/v1/campaigns |
| `blackbaud_basic_get_campaign` | Get campaign details | GET | /campaign/v1/campaigns/{id} |
| `blackbaud_basic_list_funds` | List funds | GET | /fund/v1/funds |
| `blackbaud_basic_get_fund` | Get fund details | GET | /fund/v1/funds/{id} |
| `blackbaud_basic_list_appeals` | List appeals | GET | /appeal/v1/appeals |
| `blackbaud_basic_get_appeal` | Get appeal details | GET | /appeal/v1/appeals/{id} |

---

## Tool Details

### blackbaud_basic_list_constituents

**What it does**: Lists all constituents (donors, supporters) in the database.

**When to use**: Browse donor database, find specific supporters.

**Arguments**:
- `search_text` (optional): Search by name or email
- `page` (optional): Page number (default 1)
- `page_size` (optional): Results per page (default 20)

**Example LLM prompt**: "List all constituents"

---

### blackbaud_basic_get_constituent

**What it does**: Gets details for a specific constituent.

**When to use**: View donor profile, giving history.

**Arguments**:
- `id` (required): Constituent ID

**Example LLM prompt**: "Get details for constituent 12345"

---

### blackbaud_basic_list_gifts

**What it does**: Lists all gifts (donations) in the database.

**When to use**: Track donations, generate reports.

**Arguments**:
- `constituent_id` (optional): Filter by constituent ID
- `page` (optional): Page number (default 1)
- `page_size` (optional): Results per page (default 20)

**Example LLM prompt**: "List all gifts from the last month"

---

### blackbaud_basic_get_gift

**What it does**: Gets details for a specific gift.

**When to use**: View donation details, check designation.

**Arguments**:
- `id` (required): Gift ID

**Example LLM prompt**: "Get details for gift G-789"

---

### blackbaud_basic_list_campaigns

**What it does**: Lists all fundraising campaigns.

**When to use**: Browse active campaigns, track progress.

**Arguments**:
- `page` (optional): Page number (default 1)
- `page_size` (optional): Results per page (default 20)

**Example LLM prompt**: "List all active campaigns"

---

### blackbaud_basic_get_campaign

**What it does**: Gets details for a specific campaign.

**When to use**: Check campaign goals, progress.

**Arguments**:
- `id` (required): Campaign ID

**Example LLM prompt**: "Get details for campaign C-100"

---

### blackbaud_basic_list_funds

**What it does**: Lists all funds (designations) for donations.

**When to use**: View fund structure, track allocations.

**Arguments**:
- `page` (optional): Page number (default 1)
- `page_size` (optional): Results per page (default 20)

**Example LLM prompt**: "List all funds"

---

### blackbaud_basic_get_fund

**What it does**: Gets details for a specific fund.

**When to use**: Check fund balance, see donations allocated.

**Arguments**:
- `id` (required): Fund ID

**Example LLM prompt**: "Get details for fund F-200"

---

### blackbaud_basic_list_appeals

**What it does**: Lists all fundraising appeals.

**When to use**: View appeal campaigns, track responses.

**Arguments**:
- `page` (optional): Page number (default 1)
- `page_size` (optional): Results per page (default 20)

**Example LLM prompt**: "List all appeals"

---

### blackbaud_basic_get_appeal

**What it does**: Gets details for a specific appeal.

**When to use**: Check appeal performance, response rates.

**Arguments**:
- `id` (required): Appeal ID

**Example LLM prompt**: "Get details for appeal A-300"

---

## Blackbaud Basic Auth API Notes

- **Hostname**: Your Blackbaud hosting hostname (e.g., yourorg.blackbaudhosting.com)
- **SOAP-based**: Uses SOAP web services under the hood
- **Constituents**: Individuals or organizations in the donor database
- **Gifts**: Donations with amount, date, fund, and appeal
- **Same endpoints as OAuth**: These tools use the same API endpoints as the OAuth variant
