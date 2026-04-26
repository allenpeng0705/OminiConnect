# Plain Tools

Provider: `plain` | Engine: `nango` | Auth: API Key via Nango

## Overview

These tools wrap the Plain API. They allow AI agents to manage customers, components, comments, tags, and teams. Plain is a customer support platform. **Requires Plain API Key authentication.**

## Authentication

**Nango API Key**:
- Uses Bearer token in Authorization header
- Token stored in Nango, accessed via `connection_ref`
- Base URL: https://core-api.uk.plain.com/graphql
- GraphQL API

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `plain_list_customers` | List customers | POST | /graphql |
| `plain_get_customer` | Get customer details | POST | /graphql |
| `plain_list_components` | List components | POST | /graphql |
| `plain_get_component` | Get component details | POST | /graphql |
| `plain_list_comments` | List comments | POST | /graphql |
| `plain_create_comment` | Create a comment | POST | /graphql |
| `plain_list_tags` | List tags | POST | /graphql |
| `plain_get_user` | Get user info | POST | /graphql |
| `plain_list_teams` | List teams | POST | /graphql |
| `plain_get_customer_by_email` | Get customer by email | POST | /graphql |

---

## Tool Details

### plain_list_customers

**What it does**: Lists all customers.

**When to use**: Browse customer base.

**Arguments**:
- `limit` (optional): Number of results (default 20)

**Example LLM prompt**: "List all customers"

---

### plain_get_customer

**What it does**: Gets detailed information about a specific customer.

**When to use**: Get customer profile, history.

**Arguments**:
- `customerId` (required): Customer ID

**Example LLM prompt**: "Get details for customer 12345"

---

### plain_list_components

**What it does**: Lists all components in the system.

**When to use**: Browse components, timeline.

**Arguments**:
- `customerId` (optional): Filter by customer

**Example LLM prompt**: "List components for customer 12345"

---

### plain_get_component

**What it does**: Gets detailed information about a specific component.

**When to use**: Get component details.

**Arguments**:
- `componentId` (required): Component ID

**Example LLM prompt**: "Get details for component 67890"

---

### plain_list_comments

**What it does**: Lists all comments.

**When to use**: Browse customer interactions.

**Arguments**:
- `customerId` (optional): Filter by customer

**Example LLM prompt**: "List comments for customer 12345"

---

### plain_create_comment

**What it does**: Creates a new comment.

**When to use**: Add customer note, reply.

**Arguments**:
- `customerId` (required): Customer ID
- `content` (required): Comment content

**Example LLM prompt**: "Add a comment to customer 12345"

---

### plain_list_tags

**What it does**: Lists all tags.

**When to use**: Browse available tags.

**Arguments**: None

**Example LLM prompt**: "What tags are available?"

---

### plain_get_user

**What it does**: Gets current user information.

**When to use**: Get user profile.

**Arguments**: None

**Example LLM prompt**: "Get my user information"

---

### plain_list_teams

**What it does**: Lists all teams.

**When to use**: Browse team structure.

**Arguments**: None

**Example LLM prompt**: "What teams are configured?"

---

### plain_get_customer_by_email

**What it does**: Gets customer by email address.

**When to use**: Look up customer from email.

**Arguments**:
- `email` (required): Email address

**Example LLM prompt**: "Find customer with email john@example.com"

---

## Plain API Notes

- **GraphQL API**: Uses GraphQL queries
- **Components**: Building blocks for customer data
- **Timeline**: Customer interactions tracked as components
