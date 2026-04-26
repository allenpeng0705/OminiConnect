# Attio

Modern CRM platform for managing contacts, companies, deals, activities, and tasks.

## Provider Details

- **Provider Key**: `attio`
- **Authentication**: API key
- **API Docs**: https://attio.com/developers

## Tool Registry

### Contacts

| Tool | Description |
|------|-------------|
| `attio_list_contacts` | Retrieve a list of all contacts in your workspace |
| `attio_get_contact` | Get details of a specific contact by ID |
| `attio_create_contact` | Create a new contact in your workspace |

### Companies

| Tool | Description |
|------|-------------|
| `attio_list_companies` | Retrieve a list of all companies in your workspace |
| `attio_get_company` | Get details of a specific company by ID |

### Deals

| Tool | Description |
|------|-------------|
| `attio_list_deals` | Retrieve a list of all deals in your workspace |
| `attio_get_deal` | Get details of a specific deal by ID |

### Activities

| Tool | Description |
|------|-------------|
| `attio_list_activities` | Retrieve a list of all activities in your workspace |
| `attio_get_activity` | Get details of a specific activity by ID |

### Tasks

| Tool | Description |
|------|-------------|
| `attio_list_tasks` | Retrieve a list of all tasks in your workspace |

## Scopes

The following scopes are used by Attio tools:

- `contact:read` - Read contacts
- `contact:write` - Create/update contacts
- `company:read` - Read companies
- `deal:read` - Read deals
- `activity:read` - Read activities
- `task:read` - Read tasks

## API Base URL

`https://api.attio.com/api/v2`

## Notes

- Attio uses a simple object-based data model
- All records are identified by UUIDs
- Activities track interactions like emails, calls, and notes
- Tasks support filtering by status (open, completed)