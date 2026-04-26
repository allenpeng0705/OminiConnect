# Apollo Integration

Apollo is a sales intelligence platform that provides data about contacts, companies, and campaigns for outbound sales teams.

## Authentication

Apollo uses API key authentication. Configure the following scopes in your Apollo API key:

- `contacts.read` - Read access to contacts
- `contacts.write` - Write access to contacts
- `companies.read` - Read access to companies
- `campaigns.read` - Read access to campaigns
- `sequences.read` - Read access to sequences
- `sequences.write` - Write access to sequences

## Base URL

```
https://api.apollo.io/v1
```

## Rate Limits

- Standard rate limit: 500 requests per minute
- Write operations: 100 requests per minute

## Tools

### Contacts

| Tool | Description |
|------|-------------|
| `apollo_list_contacts` | List contacts in Apollo. Filter by name, email, company, or campaign |
| `apollo_get_contact` | Get detailed information about an Apollo contact |
| `apollo_create_contact` | Create a new contact in Apollo |

### Companies

| Tool | Description |
|------|-------------|
| `apollo_list_companies` | List companies in Apollo. Filter by name, industry, or location |
| `apollo_get_company` | Get detailed information about an Apollo company |

### Campaigns

| Tool | Description |
|------|-------------|
| `apollo_list_campaigns` | List campaigns in Apollo. Filter by status, type, or team |
| `apollo_get_campaign` | Get detailed information about an Apollo campaign |

### Sequences

| Tool | Description |
|------|-------------|
| `apollo_list_sequences` | List email sequences in Apollo. Filter by status or type |
| `apollo_get_sequence` | Get detailed information about an Apollo sequence |
| `apollo_add_contact_to_sequence` | Add a contact to an Apollo sequence |

## Tool Details

### apollo_list_contacts

List contacts in Apollo. Filter by name, email, company, or campaign.

**Endpoint:** `GET /contacts`

**Scopes:** `contacts.read`

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `limit` | integer | No | Max results (default 20, max 100) |
| `filter` | string | No | Filter expression (e.g., company_id='123') |

### apollo_get_contact

Get detailed information about an Apollo contact including contact info and account data.

**Endpoint:** `GET /contacts/{contact_id}`

**Scopes:** `contacts.read`

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `contact_id` | string | Yes | The contact ID |

### apollo_create_contact

Create a new contact in Apollo with name, email, phone, and company information.

**Endpoint:** `POST /contacts`

**Scopes:** `contacts.write`

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `first_name` | string | No | First name |
| `last_name` | string | No | Last name |
| `email` | string | Yes | Email address |
| `phone_number` | string | No | Phone number |
| `company_id` | string | No | Company ID to associate |
| `title` | string | No | Job title |

### apollo_list_companies

List companies in Apollo. Filter by name, industry, or location.

**Endpoint:** `GET /companies`

**Scopes:** `companies.read`

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `limit` | integer | No | Max results (default 20, max 100) |
| `filter` | string | No | Filter expression (e.g., industry='Technology') |

### apollo_get_company

Get detailed information about an Apollo company including funding and metrics.

**Endpoint:** `GET /companies/{company_id}`

**Scopes:** `companies.read`

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `company_id` | string | Yes | The company ID |

### apollo_list_campaigns

List campaigns in Apollo. Filter by status, type, or team.

**Endpoint:** `GET /campaigns`

**Scopes:** `campaigns.read`

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `limit` | integer | No | Max results (default 20, max 100) |
| `filter` | string | No | Filter expression (e.g., status='active') |

### apollo_get_campaign

Get detailed information about an Apollo campaign including contacts and stats.

**Endpoint:** `GET /campaigns/{campaign_id}`

**Scopes:** `campaigns.read`

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `campaign_id` | string | Yes | The campaign ID |

### apollo_list_sequences

List email sequences in Apollo. Filter by status or type.

**Endpoint:** `GET /sequences`

**Scopes:** `sequences.read`

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `limit` | integer | No | Max results (default 20, max 100) |
| `filter` | string | No | Filter expression (e.g., status='active') |

### apollo_get_sequence

Get detailed information about an Apollo sequence including steps and settings.

**Endpoint:** `GET /sequences/{sequence_id}`

**Scopes:** `sequences.read`

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `sequence_id` | string | Yes | The sequence ID |

### apollo_add_contact_to_sequence

Add a contact to an Apollo sequence with optional scheduled date.

**Endpoint:** `POST /sequence_enrollments`

**Scopes:** `sequences.write`

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `contact_id` | string | Yes | The contact ID |
| `sequence_id` | string | Yes | The sequence ID |
| `scheduled_for` | string | No | Schedule date (ISO 8601 format) |