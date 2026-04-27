# LaunchDarkly Tools

LaunchDarkly provides feature flags, segments, and user targeting for controlled feature rollouts and A/B testing.

## Provider Overview

- **Provider**: LaunchDarkly
- **API Base**: `https://app.launchdarkly.com/api/v2`
- **Auth**: Bearer token (`LAUNCHDARKLY_API_KEY`)

## Available Tools

### Flags

| Tool | Description |
|------|-------------|
| `launchdarkly_list_flags` | Retrieve a list of all feature flags in a project |
| `launchdarkly_get_flag` | Retrieve a specific feature flag by key |
| `launchdarkly_create_flag` | Create a new feature flag in a project |
| `launchdarkly_update_flag` | Update an existing feature flag |

### Segments

| Tool | Description |
|------|-------------|
| `launchdarkly_list_segments` | Retrieve a list of all segments in a project |
| `launchdarkly_get_segment` | Retrieve a specific segment by key |
| `launchdarkly_create_segment` | Create a new segment in an environment |

### Environments

| Tool | Description |
|------|-------------|
| `launchdarkly_list_environments` | Retrieve a list of all environments in a project |
| `launchdarkly_get_environment` | Retrieve a specific environment by key |

### Members

| Tool | Description |
|------|-------------|
| `launchdarkly_list_members` | Retrieve a list of all members in the account |

## Tool Details

### launchdarkly_list_flags

Retrieve a list of all feature flags in a project.

- **Endpoint**: `GET /v2/flags`
- **Scopes**: `flags:read`
- **Tags**: `flags`, `list`

**Input Schema**:
```yaml
type: object
properties:
  project_key:
    type: string
    description: "The project key"
  environment_key:
    type: string
    description: "The environment key"
  limit:
    type: integer
    description: "Maximum number of flags to return"
required:
  - project_key
```

### launchdarkly_get_flag

Retrieve a specific feature flag by key.

- **Endpoint**: `GET /v2/flags/{project_key}/{flag_key}`
- **Scopes**: `flags:read`
- **Tags**: `flags`, `read`

**Input Schema**:
```yaml
type: object
properties:
  project_key:
    type: string
    description: "The project key"
  flag_key:
    type: string
    description: "The feature flag key"
  environment_key:
    type: string
    description: "The environment key"
required:
  - project_key
  - flag_key
```

### launchdarkly_create_flag

Create a new feature flag in a project.

- **Endpoint**: `POST /v2/flags/{project_key}`
- **Scopes**: `flags:write`
- **Tags**: `flags`, `create`

**Input Schema**:
```yaml
type: object
properties:
  key:
    type: string
    description: "The unique flag key"
  name:
    type: string
    description: "The flag name"
  description:
    type: string
    description: "Description of the flag"
  flag_type:
    type: string
    description: "Flag type (boolean, multivariate)"
  variations:
    type: array
    description: "List of variations for the flag"
required:
  - key
  - name
  - flag_type
```

### launchdarkly_update_flag

Update an existing feature flag.

- **Endpoint**: `PATCH /v2/flags/{project_key}/{flag_key}`
- **Scopes**: `flags:write`
- **Tags**: `flags`, `update`

**Input Schema**:
```yaml
type: object
properties:
  project_key:
    type: string
    description: "The project key"
  flag_key:
    type: string
    description: "The feature flag key"
  name:
    type: string
    description: "The updated flag name"
  description:
    type: string
    description: "Updated description"
  variations:
    type: array
    description: "Updated variations"
required:
  - project_key
  - flag_key
```

### launchdarkly_list_segments

Retrieve a list of all segments in a project.

- **Endpoint**: `GET /v2/segments`
- **Scopes**: `segments:read`
- **Tags**: `segments`, `list`

**Input Schema**:
```yaml
type: object
properties:
  project_key:
    type: string
    description: "The project key"
  environment_key:
    type: string
    description: "The environment key"
required:
  - project_key
```

### launchdarkly_get_segment

Retrieve a specific segment by key.

- **Endpoint**: `GET /v2/segments/{project_key}/{environment_key}/{segment_key}`
- **Scopes**: `segments:read`
- **Tags**: `segments`, `read`

**Input Schema**:
```yaml
type: object
properties:
  project_key:
    type: string
    description: "The project key"
  environment_key:
    type: string
    description: "The environment key"
  segment_key:
    type: string
    description: "The segment key"
required:
  - project_key
  - environment_key
  - segment_key
```

### launchdarkly_create_segment

Create a new segment in an environment.

- **Endpoint**: `POST /v2/segments/{project_key}/{environment_key}`
- **Scopes**: `segments:write`
- **Tags**: `segments`, `create`

**Input Schema**:
```yaml
type: object
properties:
  key:
    type: string
    description: "The unique segment key"
  name:
    type: string
    description: "The segment name"
  description:
    type: string
    description: "Description of the segment"
  tags:
    type: array
    description: "Tags for the segment"
required:
  - key
  - name
```

### launchdarkly_list_environments

Retrieve a list of all environments in a project.

- **Endpoint**: `GET /v2/projects/{project_key}/environments`
- **Scopes**: `environments:read`
- **Tags**: `environments`, `list`

**Input Schema**:
```yaml
type: object
properties:
  project_key:
    type: string
    description: "The project key"
required:
  - project_key
```

### launchdarkly_get_environment

Retrieve a specific environment by key.

- **Endpoint**: `GET /v2/projects/{project_key}/environments/{environment_key}`
- **Scopes**: `environments:read`
- **Tags**: `environments`, `read`

**Input Schema**:
```yaml
type: object
properties:
  project_key:
    type: string
    description: "The project key"
  environment_key:
    type: string
    description: "The environment key"
required:
  - project_key
  - environment_key
```

### launchdarkly_list_members

Retrieve a list of all members in the account.

- **Endpoint**: `GET /v2/members`
- **Scopes**: `members:read`
- **Tags**: `members`, `list`

**Input Schema**:
```yaml
type: object
properties:
  limit:
    type: integer
    description: "Maximum number of members to return"
  offset:
    type: integer
    description: "Offset for pagination"
required: []
```

## Usage Example

```typescript
// List all feature flags in a project
const flags = await tools.execute('launchdarkly_list_flags', {
  project_key: 'my-project'
});

// Create a new feature flag
const newFlag = await tools.execute('launchdarkly_create_flag', {
  key: 'new-feature',
  name: 'New Feature',
  flag_type: 'boolean'
});

// Update a feature flag
await tools.execute('launchdarkly_update_flag', {
  project_key: 'my-project',
  flag_key: 'new-feature',
  name: 'Updated Feature Name'
});
```