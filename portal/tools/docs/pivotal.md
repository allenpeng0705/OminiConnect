# Pivotal Tracker

Pivotal Tracker is an agile project management tool that helps teams plan and track their work through stories, iterations, and memberships.

## Authentication

OminiConnect uses OAuth2 to connect to Pivotal Tracker. During the authorization flow, you'll be asked to grant the following permissions:

| Scope | Description |
|-------|-------------|
| `read` | Read access to projects |
| `write` | Write access to projects |

## Configuration

When adding a Pivotal Tracker connection, you'll need to provide:

- **API Token**: Your Pivotal Tracker API token (found in your profile settings)

## Available Tools

### Project Management

| Tool | Description |
|------|-------------|
| `pivotal_list_projects` | Retrieve a list of all projects the user has access to |
| `pivotal_get_project` | Retrieve details for a specific project by its ID |

### Story Management

| Tool | Description |
|------|-------------|
| `pivotal_list_stories` | Retrieve a list of stories in a project |
| `pivotal_get_story` | Retrieve details for a specific story by its ID |
| `pivotal_create_story` | Create a new story in a project |
| `pivotal_update_story` | Update an existing story in a project |

### Iteration Management

| Tool | Description |
|------|-------------|
| `pivotal_list_iterations` | Retrieve a list of iterations in a project |
| `pivotal_get_iteration` | Retrieve details for a specific iteration by its number |

### Membership Management

| Tool | Description |
|------|-------------|
| `pivotal_list_memberships` | Retrieve a list of memberships in a project |
| `pivotal_get_membership` | Retrieve details for a specific project membership |

## Usage Example

```typescript
// List all projects
const projects = await client.tools.execute('pivotal_list_projects', {});

// Get a specific project
const project = await client.tools.execute('pivotal_get_project', {
  project_id: 12345
});

// List stories in a project
const stories = await client.tools.execute('pivotal_list_stories', {
  project_id: 12345,
  story_type: 'feature'
});

// Create a new story
const newStory = await client.tools.execute('pivotal_create_story', {
  project_id: 12345,
  name: 'Implement user authentication',
  story_type: 'feature',
  estimate: 3,
  description: 'Add OAuth2 authentication flow'
});

// Update a story
const updatedStory = await client.tools.execute('pivotal_update_story', {
  project_id: 12345,
  story_id: 67890,
  state: 'finished'
});
```

## Rate Limits

Pivotal Tracker API allows approximately 500 requests per minute per token. The tool execution will return appropriate errors if rate limits are exceeded.
