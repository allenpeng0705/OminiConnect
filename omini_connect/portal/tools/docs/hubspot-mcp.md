# HubSpot MCP Tools

Provider: `hubspot-mcp` | Engine: `nango` | Auth: MCP OAuth2 via Nango

## Overview

These tools wrap the HubSpot MCP API. They allow AI agents to manage contacts, companies, deals, tickets, and pipelines. HubSpot MCP uses Model Context Protocol for enhanced AI integration.

## Authentication

**Nango MCP OAuth2**:
- User authenticates via Nango Connect with HubSpot
- Uses MCP (Model Context Protocol) for AI agent integration
- Token stored in Nango, accessed via `connection_ref`
- Authorization URL: https://mcp.hubspot.com/oauth/authorize/user
- Token URL: https://mcp.hubspot.com/oauth/v3/token

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `hubspot_list_contacts` | List contacts | GET | /crm/v3/objects/contacts |
| `hubspot_get_contact` | Get contact details | GET | /crm/v3/objects/contacts/{contactId} |
| `hubspot_list_companies` | List companies | GET | /crm/v3/objects/companies |
| `hubspot_get_company` | Get company details | GET | /crm/v3/objects/companies/{companyId} |
| `hubspot_list_deals` | List deals | GET | /crm/v3/objects/deals |
| `hubspot_get_deal` | Get deal details | GET | /crm/v3/objects/deals/{dealId} |
| `hubspot_list_tickets` | List tickets | GET | /crm/v3/objects/tickets |
| `hubspot_get_ticket` | Get ticket details | GET | /crm/v3/objects/tickets/{ticketId} |
| `hubspot_list_owners` | List owners | GET | /crm/v3/owners |
| `hubspot_list_pipelines` | List pipelines | GET | /crm/v3/pipelines |

---

## Tool Details

### hubspot_list_contacts

**What it does**: Lists all contacts in HubSpot.

**When to use**: Browse contact directory.

**Arguments**:
- `limit` (optional): Max results (default 20)
- `after` (optional): Cursor for pagination

**Example LLM prompt**: "List all contacts"

---

### hubspot_get_contact

**What it does**: Gets detailed information about a specific contact.

**When to use**: View contact profile and company.

**Arguments**:
- `contactId` (required): Contact ID

**Example LLM prompt**: "Get contact with ID abc123"

---

### hubspot_list_companies

**What it does**: Lists all companies in HubSpot.

**When to use**: Browse company directory.

**Arguments**:
- `limit` (optional): Max results (default 20)
- `after` (optional): Cursor for pagination

**Example LLM prompt**: "List all companies"

---

### hubspot_get_company

**What it does**: Gets detailed information about a specific company.

**When to use**: View company details and contacts.

**Arguments**:
- `companyId` (required): Company ID

**Example LLM prompt**: "Get company with ID xyz789"

---

### hubspot_list_deals

**What it does**: Lists all deals in HubSpot.

**When to use**: Browse sales pipeline.

**Arguments**:
- `limit` (optional): Max results (default 20)
- `after` (optional): Cursor for pagination

**Example LLM prompt**: "List all deals"

---

### hubspot_get_deal

**What it does**: Gets detailed information about a specific deal.

**When to use**: View deal details and stage.

**Arguments**:
- `dealId` (required): Deal ID

**Example LLM prompt**: "Get deal with ID dkl456"

---

### hubspot_list_tickets

**What it does**: Lists all tickets in HubSpot.

**When to use**: Browse support tickets.

**Arguments**:
- `limit` (optional): Max results (default 20)
- `after` (optional): Cursor for pagination

**Example LLM prompt**: "List all tickets"

---

### hubspot_get_ticket

**What it does**: Gets detailed information about a specific ticket.

**When to use**: View ticket details and status.

**Arguments**:
- `ticketId` (required): Ticket ID

**Example LLM prompt**: "Get ticket with ID tkt789"

---

### hubspot_list_owners

**What it does**: Lists all owners in HubSpot.

**When to use**: View sales team members.

**Arguments**: None

**Example LLM prompt**: "List all owners"

---

### hubspot_list_pipelines

**What it does**: Lists all pipelines in HubSpot.

**When to use**: View deal pipeline stages.

**Arguments**: None

**Example LLM prompt**: "List all pipelines"

---

## HubSpot MCP API Notes

- **API Base URL**: https://mcp.hubspot.com
- **Protocol**: MCP (Model Context Protocol)
- **Resource**: User authentication for personal data
- **Pagination**: Cursor-based pagination
- **Contacts**: Customer contact records
- **Companies**: Business organizations
- **Deals**: Sales opportunities
- **Tickets**: Customer support tickets
- **Owners**: Sales and support team members
- **Pipelines**: Deal stage pipelines
