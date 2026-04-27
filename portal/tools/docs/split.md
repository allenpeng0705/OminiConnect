# Split Tools

Split provides feature flags and audience segmentation for controlled rollouts, A/B testing, and experimentation.

## Provider Overview

- **Provider**: Split
- **API Base**: `https://api.split.io/api/v2`
- **Auth**: Bearer token (`SPLIT_API_KEY`)

## Available Tools

### Flags

| Tool | Description |
|------|-------------|
| `split_list_flags` | Retrieve a list of all feature flags in a workspace |
| `split_get_flag` | Retrieve a specific feature flag by name |
| `split_create_flag` | Create a new feature flag in a workspace |
| `split_update_flag` | Update an existing feature flag |

### Segments

| Tool | Description |
|------|-------------|
| `split_list_segments` | Retrieve a list of all segments in a workspace |
| `split_get_segment` | Retrieve a specific segment by name |
| `split_create_segment` | Create a new segment in a workspace |

### Workspaces

| Tool | Description |
|------|-------------|
| `split_list_workspaces` | Retrieve a list of all workspaces in the account |
| `split_get_workspace` | Retrieve a specific workspace by ID |

### Traffic Types

| Tool | Description |
|------|-------------|
| `split_list_traffic_types` | Retrieve a list of all traffic types in a workspace |

## Tool Details

### split_list_flags

Retrieve a list of all feature flags in a workspace.

- **Endpoint**: `GET /v2/flags`
- **Scopes**: `flags:read`
- **Tags**: `flags`, `list`

**Input Schema**:
```yaml
type: object
properties:
  workspace_id:
    type: string
    description: "The workspace ID"
  limit:
    type: integer
    description: "Maximum number of flags to return"
  offset:
    type: integer
    description: "Offset for pagination"
required:
  - workspace_id
```

### split_get_flag

Retrieve a specific feature flag by name.

- **Endpoint**: `GET /v2/flags/{flag_name}`
- **Scopes**: `flags:read`
- **Tags**: `flags`, `read`

**Input Schema**:
```yaml
type: object
properties:
  flag_name:
    type: string
    description: "The feature flag name"
  workspace_id:
    type: string
    description: "The workspace ID"
required:
  - flag_name
```

### split_create_flag

Create a new feature flag in a workspace.

- **Endpoint**: `POST /v2/flags`
- **Scopes**: `flags:write`
- **Tags**: `flags`, `create`

**Input Schema**:
```yaml
type: object
properties:
  name:
    type: string
    description: "The unique flag name"
  description:
    type: string
    description: "Description of the flag"
  traffic_type:
    type: string
    description: "The traffic type (user, account)"
  environment:
    type: string
    description: "Initial environment settings"
required:
  - name
  - traffic_type
```

### split_update_flag

Update an existing feature flag.

- **Endpoint**: `PATCH /v2/flags/{flag_name}`
- **Scopes**: `flags:write`
- **Tags**: `flags`, `update`

**Input Schema**:
```yaml
type: object
properties:
  flag_name:
    type: string
    description: "The feature flag name"
  description:
    type: string
    description: "Updated description"
  name:
    type: string
    description: "Updated flag name"
required:
  - flag_name
```

### split_list_segments

Retrieve a list of all segments in a workspace.

- **Endpoint**: `GET /v2/segments`
- **Scopes**: `segments:read`
- **Tags**: `segments`, `list`

**Input Schema**:
```yaml
type: object
properties:
  workspace_id:
    type: string
    description: "The workspace ID"
  limit:
    type: integer
    description: "Maximum number of segments to return"
required:
  - workspace_id
```

### split_get_segment

Retrieve a specific segment by name.

- **Endpoint**: `GET /v2/segments/{segment_name}`
- **Scopes**: `segments:read`
- **Tags**: `segments`, `read`

**Input Schema**:
```yaml
type: object
properties:
  segment_name:
    type: string
    description: "The segment name"
  workspace_id:
    type: string
    description: "The workspace ID"
required:
  - segment_name
```

### split_create_segment

Create a new segment in a workspace.

- **Endpoint**: `POST /v2/segments`
- **Scopes**: `segments:write`
- **Tags**: `segments`, `create`

**Input Schema**:
```yaml
type: object
properties:
  name:
    type: string
    description: "The unique segment name"
  description:
    type: string
    description: "Description of the segment"
  workspace_id:
    type: string
    description: "The workspace ID"
required:
  - name
  - workspace_id
```

### split_list_workspaces

Retrieve a list of all workspaces in the account.

- **Endpoint**: `GET /v2/workspaces`
- **Scopes**: `workspaces:read`
- **Tags**: `workspaces`, `list`

**Input Schema**:
```yaml
type: object
properties:
  limit:
    type: integer
    description: "Maximum number of workspaces to return"
required: []
```

### split_get_workspace

Retrieve a specific workspace by ID.

- **Endpoint**: `GET /v2/workspaces/{workspace_id}`
- **Scopes**: `workspaces:read`
- **Tags**: `workspaces`, `read`

**Input Schema**:
```yaml
type: object
properties:
  workspace_id:
    type: string
    description: "The workspace ID"
required:
  - workspace_id
```

### split_list_traffic_types

Retrieve a list of all traffic types in a workspace.

- **Endpoint**: `GET /v2/traffic-types`
- **Scopes**: `traffic_types:read`
- **Tags**: `traffic_types`, `list`

**Input Schema**:
```yaml
type: object
properties:
  workspace_id:
    type: string
    description: "The workspace ID"
required:
  - workspace_id
```

## Usage Example

```typescript
// List all feature flags in a workspace
const flags = await tools.execute('split_list_flags', {
  workspace_id: 'my-workspace'
});

// Create a new feature flag
const newFlag = await tools.execute('split_create_flag', {
  name: 'new-feature',
  traffic_type: 'user'
});

// Create a new segment
const segment = await tools.execute('split_create_segment', {
  name: 'beta-users',
  workspace_id: 'my-workspace'
});
```