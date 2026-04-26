# Metabase Tools

Provider: `metabase` | Engine: `nango` | Auth: API Key (via Nango)

## Overview

These tools wrap the Metabase API. They allow AI agents to explore database structures, list saved questions and dashboards, and create new analytics. Metabase is an open-source business intelligence platform.

## Authentication

**Nango API Key**:
- User provides Metabase API token via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `metabase_full_access`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `metabase_list_databases` | List databases connected to Metabase | GET | /api/database |
| `metabase_get_database` | Get database details | GET | /api/database/{database_id} |
| `metabase_list_tables` | List tables across databases | GET | /api/table |
| `metabase_get_table` | Get table details and columns | GET | /api/table/{table_id} |
| `metabase_list_questions` | List saved questions/cards | GET | /api/card |
| `metabase_get_question` | Get question details | GET | /api/card/{card_id} |
| `metabase_create_question` | Create a new question | POST | /api/card |
| `metabase_list_dashboards` | List dashboards | GET | /api/dashboard |
| `metabase_get_dashboard` | Get dashboard details | GET | /api/dashboard/{dashboard_id} |
| `metabase_get_pulse` | Get pulse/subscription details | GET | /api/pulse/{pulse_id} |

---

## Tool Details

### metabase_list_databases

**What it does**: Returns a list of all databases connected to Metabase with metadata including names, descriptions, and connection status.

**When to use**: Discover available data sources before exploring tables or running queries.

**Arguments**:
- `limit` (optional): Maximum number of databases to return (default 100)

**Example LLM prompt**: "What databases are connected to Metabase?"

---

### metabase_get_database

**What it does**: Returns detailed information about a specific database including tables, features, and connection details.

**When to use**: Understand database structure before listing its tables.

**Arguments**:
- `database_id` (required): Database ID

**Example LLM prompt**: "Get details for database 1"

---

### metabase_list_tables

**What it does**: Lists all tables across databases in Metabase. Returns table names, schemas, and database associations.

**When to use**: Discover available tables for querying across all databases.

**Arguments**:
- `limit` (optional): Maximum number of tables to return (default 100)

**Example LLM prompt**: "Show me all tables available in Metabase"

---

### metabase_get_table

**What it does**: Returns detailed information about a specific table including column definitions, types, and visibility settings.

**When to use**: Understand table structure before writing queries or creating questions.

**Arguments**:
- `table_id` (required): Table ID

**Example LLM prompt**: "What columns are in table 42?"

---

### metabase_list_questions

**What it does**: Lists all saved questions (cards) in Metabase. Returns question names, collections, and query types.

**When to use**: Discover existing analyses before creating new ones.

**Arguments**:
- `limit` (optional): Maximum number of questions to return (default 100)
- `offset` (optional): Offset for pagination (default 0)

**Example LLM prompt**: "List all saved questions in Metabase"

---

### metabase_get_question

**What it does**: Returns detailed information about a specific question including the query definition and results metadata.

**When to use**: Understand how a question is constructed before modifying or replicating it.

**Arguments**:
- `card_id` (required): Card/Question ID

**Example LLM prompt**: "Get details for question 123"

---

### metabase_create_question

**What it does**: Creates a new saved question in Metabase. Specify the query, display type, and optional collection.

**When to use**: Save a new analysis for future use or sharing.

**Arguments**:
- `name` (required): Question name
- `dataset_query` (required): Query definition object
- `display` (optional): Visualization type (e.g., table, bar, line) - default: table
- `collection_id` (optional): Collection to save the question in

**Example LLM prompt**: "Create a question called 'Monthly Sales' that shows total sales by region"

---

### metabase_list_dashboards

**What it does**: Lists all dashboards in Metabase. Returns dashboard names, descriptions, and owner information.

**When to use**: Discover available dashboards for sharing or analysis.

**Arguments**:
- `limit` (optional): Maximum number of dashboards to return (default 100)

**Example LLM prompt**: "Show me all dashboards in Metabase"

---

### metabase_get_dashboard

**What it does**: Returns detailed information about a specific dashboard including cards, tabs, and parameters.

**When to use**: Understand dashboard composition before sharing or modifying.

**Arguments**:
- `dashboard_id` (required): Dashboard ID

**Example LLM prompt**: "Get details for dashboard 5"

---

### metabase_get_pulse

**What it does**: Returns detailed information about a pulse (scheduled report/subscription) including schedule and recipients.

**When to use**: Check scheduled reports and their delivery settings.

**Arguments**:
- `pulse_id` (required): Pulse ID

**Example LLM prompt**: "Get details for pulse 10"

---

## Metabase API Notes

- **Database-first**: Tables and questions are organized under databases
- **Collections**: Questions and dashboards can be organized into collections
- **Questions**: Saved queries with visualization settings
- **Pulses**: Scheduled reports delivered via email or Slack
- **Native queries**: Metabase uses its own query format, not raw SQL
