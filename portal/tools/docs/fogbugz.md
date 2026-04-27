# FogBugz

FogBugz is a bug tracking and project management system that helps teams track cases, manage projects, users, and mailboxes.

## Authentication

OminiConnect uses OAuth2 to connect to FogBugz. During the authorization flow, you'll be asked to grant the following permissions:

| Scope | Description |
|-------|-------------|
| `read` | Read access to bug tracking data |
| `write` | Write access to bug tracking data |

## Configuration

When adding a FogBugz connection, you'll need to provide:

- **FogBugz URL**: Your FogBugz installation URL (e.g., `https://yourcompany.fogbugz.com`)
- **API Token**: Your FogBugz API token (found in your account settings)

## Available Tools

### Case Management

| Tool | Description |
|------|-------------|
| `fogbugz_list_cases` | Retrieve a list of cases, optionally filtered by project or status |
| `fogbugz_get_case` | Retrieve details for a specific case by its ID |
| `fogbugz_create_case` | Create a new case in a project |
| `fogbugz_update_case` | Update an existing case |

### Project Management

| Tool | Description |
|------|-------------|
| `fogbugz_list_projects` | Retrieve a list of all projects |
| `fogbugz_get_project` | Retrieve details for a specific project by its ID |

### User Management

| Tool | Description |
|------|-------------|
| `fogbugz_list_users` | Retrieve a list of all users |
| `fogbugz_get_user` | Retrieve details for a specific user by their ID |

### Mailbox Management

| Tool | Description |
|------|-------------|
| `fogbugz_list_mailboxes` | Retrieve a list of mailboxes in the system |
| `fogbugz_get_mailbox` | Retrieve details for a specific mailbox by its ID |

## Usage Example

```typescript
// List all cases in a project
const cases = await client.tools.execute('fogbugz_list_cases', {
  project: 123,
  status: 'open'
});

// Get a specific case
const caseDetail = await client.tools.execute('fogbugz_get_case', {
  case_id: 456
});

// Create a new case
const newCase = await client.tools.execute('fogbugz_create_case', {
  title: 'Login page not loading',
  project: 123,
  priority: 2,
  description: 'Users are reporting that the login page returns a 500 error'
});

// Update a case
const updatedCase = await client.tools.execute('fogbugz_update_case', {
  case_id: 456,
  status: 'resolved',
  resolved: true
});

// List all users
const users = await client.tools.execute('fogbugz_list_users', {
  active: true
});
```

## Rate Limits

FogBugz API rate limits vary by installation. Consult your FogBugz administrator for specific limits on your self-hosted or cloud instance.
