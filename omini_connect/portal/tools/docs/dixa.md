# Dixa

Customer service platform for managing conversations, contacts, agents, and teams.

## Authentication

Dixa uses API token authentication. The token is passed via the `Authorization` header as `Bearer {token}`.

## API Endpoints

Dixa API base URL: `https://api.dixa.io`

## Tools

### Conversations

#### `dixa_list_conversations`
List all conversations with optional filtering.

**Endpoint:** `GET /v1/conversations`

**Scopes:** `read`

**Parameters:**
| Parameter | Type | Description |
|-----------|------|-------------|
| status | string | Filter by status: open, closed, pending |
| from_date | string | Filter from date (ISO 8601) |
| to_date | string | Filter to date (ISO 8601) |
| page_size | integer | Results per page (default: 50) |
| page_token | string | Pagination token |

#### `dixa_get_conversation`
Get details of a specific conversation.

**Endpoint:** `GET /v1/conversations/{id}`

**Scopes:** `read`

**Parameters:**
| Parameter | Type | Description |
|-----------|------|-------------|
| id | string | The conversation ID |

#### `dixa_create_conversation`
Create a new conversation.

**Endpoint:** `POST /v1/conversations`

**Scopes:** `write`

**Parameters:**
| Parameter | Type | Description |
|-----------|------|-------------|
| channel | string | Channel type: email, chat, facebook, twitter |
| contact_id | string | The contact ID |
| message | object | Initial message object |
| priority | string | Priority: normal, high |

### Contacts

#### `dixa_list_contacts`
List all contacts.

**Endpoint:** `GET /v1/contacts`

**Scopes:** `read`

**Parameters:**
| Parameter | Type | Description |
|-----------|------|-------------|
| search | string | Search by name or email |
| page_size | integer | Results per page (default: 50) |
| page_token | string | Pagination token |

#### `dixa_get_contact`
Get details of a specific contact.

**Endpoint:** `GET /v1/contacts/{id}`

**Scopes:** `read`

**Parameters:**
| Parameter | Type | Description |
|-----------|------|-------------|
| id | string | The contact ID |

### Agents

#### `dixa_list_agents`
List all agents in the organization.

**Endpoint:** `GET /v1/agents`

**Scopes:** `read`

**Parameters:**
| Parameter | Type | Description |
|-----------|------|-------------|
| team_id | string | Filter by team ID |
| status | string | Filter by status: online, offline, away |

#### `dixa_get_agent`
Get details of a specific agent.

**Endpoint:** `GET /v1/agents/{id}`

**Scopes:** `read`

**Parameters:**
| Parameter | Type | Description |
|-----------|------|-------------|
| id | string | The agent ID |

### Teams

#### `dixa_list_teams`
List all teams in the organization.

**Endpoint:** `GET /v1/teams`

**Scopes:** `read`

**Parameters:**
| Parameter | Type | Description |
|-----------|------|-------------|
| page_size | integer | Results per page (default: 50) |

#### `dixa_get_team`
Get details of a specific team.

**Endpoint:** `GET /v1/teams/{id}`

**Scopes:** `read`

**Parameters:**
| Parameter | Type | Description |
|-----------|------|-------------|
| id | string | The team ID |

### Routing

#### `dixa_list_routing_rules`
List all routing rules for conversation distribution.

**Endpoint:** `GET /v1/routing-rules`

**Scopes:** `read`

**Parameters:**
| Parameter | Type | Description |
|-----------|------|-------------|
| channel | string | Filter by channel type |

## Rate Limits

Dixa API rate limits vary by endpoint and plan. Standard limits are around 1000 requests per minute.

## Common Use Cases

1. **Support Analytics**: Monitor conversation volumes, resolution times, and agent performance
2. **Contact Management**: Sync customer information with external CRM systems
3. **Team Coordination**: Manage team schedules and routing rules for optimal support distribution
