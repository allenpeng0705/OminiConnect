# Clay Integration

## Overview

Clay is a sales intelligence and data enrichment platform that helps teams build and manage data. It provides tools for creating tables, enriching data from various sources, managing integrations, and automating workflows with webhooks.

## Connection

Clay uses API key authentication. To connect your Clay account:

1. Log in to your Clay workspace
2. Navigate to Settings > API
3. Generate a new API key
4. Configure the API key with appropriate permissions

## Available Scopes

| Scope | Description |
|-------|-------------|
| `tables:read` | Read table data |
| `tables:write` | Create and update table data |
| `enrichments:read` | Read enrichment data |
| `integrations:read` | Read integration information |
| `webhooks:write` | Create and manage webhooks |
| `contacts:read` | Read contact information |

## Tools

### Tables

#### List Tables
Retrieve all tables in the workspace.

**Endpoint:** `GET /tables`

**Parameters:**
- `limit` (integer): Maximum number of tables to return

#### Get Table
Retrieve details of a specific table.

**Endpoint:** `GET /tables/{table_id}`

**Parameters:**
- `table_id` (string, required): The table identifier

#### Add Row
Add a new row to a table.

**Endpoint:** `POST /tables/{table_id}/rows`

**Parameters:**
- `table_id` (string, required): The table identifier
- `data` (object, required): The row data to add

#### Update Row
Update an existing row in a table.

**Endpoint:** `PUT /tables/{table_id}/rows/{row_id}`

**Parameters:**
- `table_id` (string, required): The table identifier
- `row_id` (string, required): The row identifier
- `data` (object, required): The updated row data

### Enrichments

#### List Enrichments
Retrieve all available data enrichments.

**Endpoint:** `GET /enrichments`

**Parameters:**
- `limit` (integer): Maximum number of enrichments to return

#### Get Enrichment
Retrieve details of a specific enrichment.

**Endpoint:** `GET /enrichments/{enrichment_id}`

**Parameters:**
- `enrichment_id` (string, required): The enrichment identifier

### Integrations

#### List Integrations
Retrieve all connected integrations.

**Endpoint:** `GET /integrations`

**Parameters:**
- `limit` (integer): Maximum number of integrations to return

#### Get Integration
Retrieve details of a specific integration.

**Endpoint:** `GET /integrations/{integration_id}`

**Parameters:**
- `integration_id` (string, required): The integration identifier

### Webhooks

#### Create Webhook
Create a new webhook for real-time notifications.

**Endpoint:** `POST /webhooks`

**Parameters:**
- `url` (string, required): The URL to receive notifications
- `events` (array, required): List of events to subscribe to

### Contacts

#### List Contacts
Retrieve all contacts in the workspace.

**Endpoint:** `GET /contacts`

**Parameters:**
- `limit` (integer): Maximum number of contacts to return
- `offset` (integer): Number of contacts to skip for pagination

## Use Cases

- **Data Enrichment**: Automatically enrich leads and companies with additional data
- **CRM Sync**: Keep customer data synchronized across multiple platforms
- **Workflow Automation**: Trigger actions based on data changes via webhooks
- **Sales Intelligence**: Build comprehensive profiles of prospects and customers
