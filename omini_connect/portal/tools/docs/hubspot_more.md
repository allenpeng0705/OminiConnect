# HubSpot More Tools

Provider: `hubspot_more` | Engine: `nango` | Auth: OAuth (via Nango)

## Overview

This tool registry covers additional HubSpot CRM services: Deals, Tickets, Line Items, and Products. AI agents can use these tools to manage sales deals, handle support tickets, track products, and manage line items for quotes.

## Authentication

**Nango (OAuth)**:
- User authenticates via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `crm.objects.deals.read`, `crm.objects.deals.write`, `crm.objects.tickets.read`, `crm.objects.tickets.write`, `crm.objects.line_items.read`, `crm.objects.products.read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `hubspot_more_list_deals` | List deals in CRM | GET | /crm/v3/objects/deals |
| `hubspot_more_get_deal` | Get deal details | GET | /crm/v3/objects/deals/{dealId} |
| `hubspot_more_create_deal` | Create a new deal | POST | /crm/v3/objects/deals |
| `hubspot_more_list_tickets` | List support tickets | GET | /crm/v3/objects/tickets |
| `hubspot_more_get_ticket` | Get ticket details | GET | /crm/v3/objects/tickets/{ticketId} |
| `hubspot_more_create_ticket` | Create a support ticket | POST | /crm/v3/objects/tickets |
| `hubspot_more_list_line_items` | List line items | GET | /crm/v3/objects/line_items |
| `hubspot_more_get_line_item` | Get line item details | GET | /crm/v3/objects/line_items/{lineItemId} |
| `hubspot_more_list_products` | List products | GET | /crm/v3/objects/products |
| `hubspot_more_get_product` | Get product details | GET | /crm/v3/objects/products/{productId} |

## Tool Details

### Deals

#### hubspot_more_list_deals

List deals in HubSpot CRM. Filter by stage, amount, close date, or associated company.

**Input Parameters:**
- `limit` (integer, optional): Max results (default 10, max 100)
- `properties` (string, optional): Comma-separated properties to include
- `filter_groups` (string, optional): Filter groups as JSON

#### hubspot_more_get_deal

Get detailed information about a HubSpot deal.

**Input Parameters:**
- `deal_id` (string, required): The deal ID
- `properties` (string, optional): Comma-separated properties to include

#### hubspot_more_create_deal

Create a new deal in HubSpot CRM.

**Input Parameters:**
- `dealname` (string, required): The deal name
- `amount` (string, optional): The deal amount
- `dealstage` (string, optional): The deal stage
- `closedate` (string, optional): The close date (ISO 8601 format)
- `pipeline` (string, optional): The pipeline ID

### Tickets

#### hubspot_more_list_tickets

List support tickets in HubSpot CRM.

**Input Parameters:**
- `limit` (integer, optional): Max results (default 10, max 100)
- `properties` (string, optional): Comma-separated properties to include
- `filter_groups` (string, optional): Filter groups as JSON

#### hubspot_more_get_ticket

Get detailed information about a support ticket.

**Input Parameters:**
- `ticket_id` (string, required): The ticket ID
- `properties` (string, optional): Comma-separated properties to include

#### hubspot_more_create_ticket

Create a new support ticket.

**Input Parameters:**
- `subject` (string, required): The ticket subject
- `description` (string, optional): The ticket description
- `hs_ticket_priority` (string, optional): Priority: HIGH, MEDIUM, or LOW

### Line Items

#### hubspot_more_list_line_items

List line items in HubSpot CRM.

**Input Parameters:**
- `limit` (integer, optional): Max results (default 10, max 100)
- `properties` (string, optional): Comma-separated properties to include
- `filter_groups` (string, optional): Filter groups as JSON

#### hubspot_more_get_line_item

Get detailed information about a line item.

**Input Parameters:**
- `line_item_id` (string, required): The line item ID
- `properties` (string, optional): Comma-separated properties to include

### Products

#### hubspot_more_list_products

List products in HubSpot CRM.

**Input Parameters:**
- `limit` (integer, optional): Max results (default 10, max 100)
- `properties` (string, optional): Comma-separated properties to include
- `filter_groups` (string, optional): Filter groups as JSON

#### hubspot_more_get_product

Get detailed information about a product.

**Input Parameters:**
- `product_id` (string, required): The product ID
- `properties` (string, optional): Comma-separated properties to include
