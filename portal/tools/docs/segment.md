# Segment Tools

Provider: `segment` | Engine: `nango` | Auth: OAuth via Nango

## Overview

These tools wrap the Segment REST API. They allow AI agents to manage sources, destinations, warehouses, track events, and identify users. Segment is a customer data platform that collects and routes data to various analytics and marketing tools.

## Authentication

**Nango OAuth**:
- User authenticates via Nango Connect with Segment
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `sources:read`, `sources:write`, `destinations:read`, `warehouses:read`, `track:write`, `identify:write`, `trackingplans:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `segment_list_sources` | List all sources in your workspace | GET | /workspace/sources |
| `segment_get_source` | Get details of a specific source | GET | /workspace/sources/{source_id} |
| `segment_create_source` | Create a new source | POST | /workspace/sources |
| `segment_list_destinations` | List all destinations | GET | /workspace/destinations |
| `segment_get_destination` | Get details of a specific destination | GET | /workspace/destinations/{destination_id} |
| `segment_list_warehouses` | List all warehouses | GET | /workspace/warehouses |
| `segment_get_warehouse` | Get details of a specific warehouse | GET | /workspace/warehouses/{warehouse_id} |
| `segment_track_event` | Track a customer event | POST | /track |
| `segment_identify_user` | Identify a user with traits | POST | /identify |
| `segment_list_tracks` | List tracked events | GET | /workspace/trackingplans |

---

## Tool Details

### segment_list_sources

**What it does**: Lists all Segment sources in the workspace. Sources are data producers that send events to Segment (websites, apps, servers).

**When to use**: See all data sources, find source IDs for tracking or configuration.

**Arguments**:
- `limit` (optional): Number of sources to return (max 100, default 20)
- `cursor` (optional): Pagination cursor for fetching next page

**Example LLM prompt**: "List all Segment sources in my workspace"

---

### segment_get_source

**What it does**: Gets details of a specific source including its settings, catalog info, and connection status.

**When to use**: Check source configuration, verify source is active, see which destinations receive data.

**Arguments**:
- `source_id` (required): Source ID

**Example LLM prompt**: "Get details for source abc123"

---

### segment_create_source

**What it does**: Creates a new source in your Segment workspace. Choose from catalog sources like websites, mobile apps, or server-side trackers.

**When to use**: Add a new data source to start collecting events.

**Arguments**:
- `name` (required): Source name
- `catalog_name` (optional): Name in the source catalog (e.g., javascript, node, python)
- `workspace_id` (optional): Workspace ID

**Example LLM prompt**: "Create a new source named 'My Web App' using the javascript catalog"

---

### segment_list_destinations

**What it does**: Lists all destinations in your Segment workspace. Destinations receive data from your sources and forward it to downstream tools.

**When to use**: See where data is being sent, find destination IDs for configuration.

**Arguments**:
- `limit` (optional): Number of destinations to return (max 100, default 20)
- `cursor` (optional): Pagination cursor

**Example LLM prompt**: "List all destinations"

---

### segment_get_destination

**What it does**: Gets details of a specific destination including its enabled status, settings, and source mappings.

**When to use**: Verify destination configuration, check connection status.

**Arguments**:
- `destination_id` (required): Destination ID

**Example LLM prompt**: "Get details for destination xyz789"

---

### segment_list_warehouses

**What it does**: Lists all warehouses connected to your Segment workspace. Warehouses store your event and user data for long-term analysis.

**When to use**: See available data storage options, check warehouse status.

**Arguments**:
- `limit` (optional): Number of warehouses to return (max 100, default 20)
- `cursor` (optional): Pagination cursor

**Example LLM prompt**: "List all warehouses"

---

### segment_get_warehouse

**What it does**: Gets details of a specific warehouse including connection status, schema, and sync schedule.

**When to use**: Check warehouse health and configuration.

**Arguments**:
- `warehouse_id` (required): Warehouse ID

**Example LLM prompt**: "Get details for warehouse wh_123"

---

### segment_track_event

**What it does**: Tracks a customer event. Events are actions users take in your product. Include properties for additional context about the event.

**When to use**: Record user actions like page views, purchases, or custom events for analytics.

**Arguments**:
- `user_id` (required): Unique user identifier
- `anonymous_id` (optional): Anonymous ID before user logs in
- `event` (required): Event name (e.g., 'Product Viewed', 'Signed Up')
- `properties` (optional): Additional event properties
- `timestamp` (optional): ISO 8601 timestamp

**Example LLM prompt**: "Track a 'Purchase Completed' event for user 123 with properties amount: 99.99 and currency: USD"

---

### segment_identify_user

**What it does**: Identifies a user and associates traits with them. This links events to a user profile. Use to store user attributes like name, email, and custom properties.

**When to use**: Record user profile information for segmentation.

**Arguments**:
- `user_id` (required): Unique user identifier
- `anonymous_id` (optional): Anonymous ID before user logs in
- `traits` (optional): User traits (email, name, plan, etc.)
- `timestamp` (optional): ISO 8601 timestamp

**Example LLM prompt**: "Identify user 123 with traits email: john@example.com and plan: premium"

---

### segment_list_tracks

**What it does**: Lists tracked events for a user or across your workspace. Useful for auditing event history and debugging data pipelines.

**When to use**: Discover available events, audit event history.

**Arguments**:
- `user_id` (optional): Filter by user ID
- `limit` (optional): Number of tracks to return (max 100, default 20)
- `cursor` (optional): Pagination cursor

**Example LLM prompt**: "List all tracked events in my workspace"

---

## Segment API Notes

- **Sources**: Data producers (websites, apps, servers) that send events to Segment
- **Destinations**: Tools that receive data from Segment (e.g., Google Analytics, Mixpanel)
- **Warehouses**: Data storage solutions (e.g., Redshift, Snowflake) for long-term analysis
- **User ID**: The unique identifier for each user
- **Anonymous ID**: Used for unidentified users before login
- **Event properties**: Key-value pairs describing the event
- **User traits**: Key-value pairs describing the user
- **Tracking plans**: Define expected events and properties for data quality
