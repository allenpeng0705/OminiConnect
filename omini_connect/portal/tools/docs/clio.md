# Clio Tools

Provider: `clio` | Engine: `nango` | Auth: OAuth (via Nango)

## Overview

These tools wrap the Clio legal practice management API. They allow AI agents to manage clients, matters (cases), documents, calendars, and tasks for a law firm.

## Authentication

**Nango (recommended for OAuth)**:
- User authenticates via Nango Connect
- Token stored in Nango, accessed via `connection_ref`
- Scopes: `clients:read`, `clients:write`, `matters:read`, `matters:write`, `documents:read`, `calendar:read`, `tasks:read`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `clio_list_clients` | Retrieve all clients | GET | /clients |
| `clio_get_client` | Get details of a specific client | GET | /clients/{client_id} |
| `clio_create_client` | Add a new client | POST | /clients |
| `clio_list_matters` | Retrieve all matters (cases) | GET | /matters |
| `clio_get_matter` | Get details of a specific matter | GET | /matters/{matter_id} |
| `clio_create_matter` | Open a new matter for a client | POST | /matters |
| `clio_list_documents` | Retrieve all documents | GET | /documents |
| `clio_get_document` | Get metadata of a specific document | GET | /documents/{document_id} |
| `clio_list_calendar_events` | Retrieve all calendar events | GET | /calendar/events |
| `clio_list_tasks` | Retrieve all tasks | GET | /tasks |

---

## Tool Details

### clio_list_clients

**What it does**: Returns all clients in your legal practice with optional filtering.

**When to use**: Browse client roster before creating matters or pulling documents.

**Arguments**:
- `status` (optional): Filter by status (`active`, `inactive`, `archived`)
- `limit` (optional): Maximum number of clients to return
- `offset` (optional): Offset for pagination

---

### clio_get_client

**What it does**: Gets detailed information about a specific client including contact details.

**When to use**: Review client information before creating matters or sending communications.

**Arguments**:
- `client_id` (required): Unique identifier of the client

---

### clio_create_client

**What it does**: Adds a new client to your legal practice management.

**When to use**: Onboard new clients to the system.

**Arguments**:
- `name` (required): Client or company name
- `type` (optional): Client type (`individual`, `corporation`)
- `email` (optional): Primary email address
- `phone` (optional): Phone number

---

### clio_list_matters

**What it does**: Retrieves all matters (cases) in your practice with optional filtering.

**When to use**: Review active matters before updating status or adding documents.

**Arguments**:
- `client_id` (optional): Filter by client ID
- `status` (optional): Filter by status (`open`, `closed`, `pending`)
- `limit` (optional): Maximum number of matters to return

---

### clio_get_matter

**What it does**: Gets detailed information about a specific matter.

**When to use**: Review matter details before updating status or adding tasks.

**Arguments**:
- `matter_id` (required): Unique identifier of the matter

---

### clio_create_matter

**What it does**: Opens a new matter (case) for a client.

**When to use**: Start a new case or engagement for an existing client.

**Arguments**:
- `client_id` (required): Associated client ID
- `name` (required): Matter title or case name
- `matter_type` (optional): Type of matter (`litigation`, `corporate`, `family`)
- `case_number` (optional): Case or file number

---

### clio_list_documents

**What it does**: Retrieves all documents in your practice with optional filtering.

**When to use**: Browse documents before retrieving or organizing.

**Arguments**:
- `matter_id` (optional): Filter by matter ID
- `type` (optional): Filter by document type
- `limit` (optional): Maximum number of documents to return

---

### clio_get_document

**What it does**: Gets metadata and details of a specific document.

**When to use**: Review document information before downloading or sharing.

**Arguments**:
- `document_id` (required): Unique identifier of the document

---

### clio_list_calendar_events

**What it does**: Retrieves all calendar events and deadlines with optional filtering.

**When to use**: Review upcoming deadlines and appointments.

**Arguments**:
- `matter_id` (optional): Filter by matter ID
- `start_date` (optional): Filter events from this date
- `end_date` (optional): Filter events until this date
- `limit` (optional): Maximum number of events to return

---

### clio_list_tasks

**What it does**: Retrieves all tasks in your practice management with optional filtering.

**When to use**: Track pending work across matters and assignees.

**Arguments**:
- `matter_id` (optional): Filter by matter ID
- `status` (optional): Filter by status (`pending`, `completed`, `overdue`)
- `assignee` (optional): Filter by assignee ID
- `limit` (optional): Maximum number of tasks to return

---

## Clio API Reference

See official docs for full details on rate limits and pagination.
