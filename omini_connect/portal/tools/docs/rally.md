# Rally

Rally (CA Agile Central) is an enterprise agile planning platform that helps teams manage workspaces, projects, features, defects, and iterations.

## Authentication

OminiConnect uses OAuth2 to connect to Rally. During the authorization flow, you'll be asked to grant the following permissions:

| Scope | Description |
|-------|-------------|
| `rally_api` | Full access to Rally API |

## Configuration

When adding a Rally connection, you'll need to provide:

- **Rally URL**: Your Rally workspace URL (e.g., `https://rally1.rallydev.com`)
- **API Key**: Your Rally API key (found in your profile settings)

## Available Tools

### Project Management

| Tool | Description |
|------|-------------|
| `rally_list_projects` | Retrieve a list of all projects the user has access to |
| `rally_get_project` | Retrieve details for a specific project by its ID |

### Defect Management

| Tool | Description |
|------|-------------|
| `rally_list_defects` | Retrieve a list of defects, optionally filtered by project or iteration |
| `rally_get_defect` | Retrieve details for a specific defect by its ID |
| `rally_create_defect` | Create a new defect in a project |

### Iteration Management

| Tool | Description |
|------|-------------|
| `rally_list_iterations` | Retrieve a list of iterations for a project |
| `rally_get_iteration` | Retrieve details for a specific iteration by its ID |

### User Management

| Tool | Description |
|------|-------------|
| `rally_list_users` | Retrieve a list of users in the workspace |
| `rally_get_user` | Retrieve details for a specific user by their ID |

### Artifact Management

| Tool | Description |
|------|-------------|
| `rally_list_artifact_children` | Retrieve child artifacts (tasks, defects, stories) of a parent artifact |

## Usage Example

```typescript
// List all projects
const projects = await client.tools.execute('rally_list_projects', {});

// Get a specific project
const project = await client.tools.execute('rally_get_project', {
  projectId: '123456789'
});

// List defects in an iteration
const defects = await client.tools.execute('rally_list_defects', {
  iteration: '987654321',
  pageSize: 50
});

// Create a new defect
const newDefect = await client.tools.execute('rally_create_defect', {
  Name: 'Login button not working',
  Description: 'Users report that the login button does not respond on the main page',
  Project: '123456789',
  Priority: 'High'
});
```

## Rate Limits

Rally API rate limits vary by subscription level. The tool execution will return appropriate errors if rate limits are exceeded. Consider implementing exponential backoff for bulk operations.
