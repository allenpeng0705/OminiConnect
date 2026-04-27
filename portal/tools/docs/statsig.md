# Statsig Tools

Statsig provides feature gates, experimentation, and analytics for modern product development.

## Provider Overview

- **Provider**: Statsig
- **API Base**: `https://api.statsig.com/api/v1`
- **Auth**: Bearer token (`STATSIG_API_KEY`)

## Available Tools

### Feature Gates

| Tool | Description |
|------|-------------|
| `statsig_list_feature_gates` | Retrieve a list of all feature gates in a project |
| `statsig_get_feature_gate` | Retrieve a specific feature gate by name |
| `statsig_create_feature_gate` | Create a new feature gate in a project |
| `statsig_update_feature_gate` | Update an existing feature gate |

### Experiments

| Tool | Description |
|------|-------------|
| `statsig_list_experiments` | Retrieve a list of all experiments in a project |
| `statsig_get_experiment` | Retrieve a specific experiment by ID |

### Users

| Tool | Description |
|------|-------------|
| `statsig_list_users` | Retrieve a list of all users in a project |
| `statsig_get_user` | Retrieve a specific user by ID |

### Metrics

| Tool | Description |
|------|-------------|
| `statsig_list_metrics` | Retrieve a list of all metrics in a project |
| `statsig_get_metric` | Retrieve a specific metric by ID |

## Tool Details

### statsig_list_feature_gates

Retrieve a list of all feature gates in a project.

- **Endpoint**: `GET /v1/feature_gates`
- **Scopes**: `feature_gates:read`
- **Tags**: `feature_gates`, `list`

**Input Schema**:
```yaml
type: object
properties:
  project_id:
    type: string
    description: "The project ID"
  limit:
    type: integer
    description: "Maximum number of gates to return"
required:
  - project_id
```

### statsig_get_feature_gate

Retrieve a specific feature gate by name.

- **Endpoint**: `GET /v1/feature_gates/{gate_name}`
- **Scopes**: `feature_gates:read`
- **Tags**: `feature_gates`, `read`

**Input Schema**:
```yaml
type: object
properties:
  gate_name:
    type: string
    description: "The feature gate name"
required:
  - gate_name
```

### statsig_create_feature_gate

Create a new feature gate in a project.

- **Endpoint**: `POST /v1/feature_gates`
- **Scopes**: `feature_gates:write`
- **Tags**: `feature_gates`, `create`

**Input Schema**:
```yaml
type: object
properties:
  name:
    type: string
    description: "The unique gate name"
  description:
    type: string
    description: "Description of the gate"
  project_id:
    type: string
    description: "The project ID"
  id_type:
    type: string
    description: "The unit type (user, device, custom)"
required:
  - name
  - project_id
```

### statsig_update_feature_gate

Update an existing feature gate.

- **Endpoint**: `PATCH /v1/feature_gates/{gate_name}`
- **Scopes**: `feature_gates:write`
- **Tags**: `feature_gates`, `update`

**Input Schema**:
```yaml
type: object
properties:
  gate_name:
    type: string
    description: "The feature gate name"
  description:
    type: string
    description: "Updated description"
  status:
    type: string
    description: "Gate status (active, inactive)"
required:
  - gate_name
```

### statsig_list_experiments

Retrieve a list of all experiments in a project.

- **Endpoint**: `GET /v1/experiments`
- **Scopes**: `experiments:read`
- **Tags**: `experiments`, `list`

**Input Schema**:
```yaml
type: object
properties:
  project_id:
    type: string
    description: "The project ID"
  limit:
    type: integer
    description: "Maximum number of experiments to return"
required:
  - project_id
```

### statsig_get_experiment

Retrieve a specific experiment by ID.

- **Endpoint**: `GET /v1/experiments/{experiment_id}`
- **Scopes**: `experiments:read`
- **Tags**: `experiments`, `read`

**Input Schema**:
```yaml
type: object
properties:
  experiment_id:
    type: string
    description: "The experiment ID"
required:
  - experiment_id
```

### statsig_list_users

Retrieve a list of all users in a project.

- **Endpoint**: `GET /v1/users`
- **Scopes**: `users:read`
- **Tags**: `users`, `list`

**Input Schema**:
```yaml
type: object
properties:
  project_id:
    type: string
    description: "The project ID"
  limit:
    type: integer
    description: "Maximum number of users to return"
required:
  - project_id
```

### statsig_get_user

Retrieve a specific user by ID.

- **Endpoint**: `GET /v1/users/{user_id}`
- **Scopes**: `users:read`
- **Tags**: `users`, `read`

**Input Schema**:
```yaml
type: object
properties:
  user_id:
    type: string
    description: "The user ID"
required:
  - user_id
```

### statsig_list_metrics

Retrieve a list of all metrics in a project.

- **Endpoint**: `GET /v1/metrics`
- **Scopes**: `metrics:read`
- **Tags**: `metrics`, `list`

**Input Schema**:
```yaml
type: object
properties:
  project_id:
    type: string
    description: "The project ID"
  limit:
    type: integer
    description: "Maximum number of metrics to return"
required:
  - project_id
```

### statsig_get_metric

Retrieve a specific metric by ID.

- **Endpoint**: `GET /v1/metrics/{metric_id}`
- **Scopes**: `metrics:read`
- **Tags**: `metrics`, `read`

**Input Schema**:
```yaml
type: object
properties:
  metric_id:
    type: string
    description: "The metric ID"
required:
  - metric_id
```

## Usage Example

```typescript
// List all feature gates in a project
const gates = await tools.execute('statsig_list_feature_gates', {
  project_id: 'project-123'
});

// Create a new feature gate
const gate = await tools.execute('statsig_create_feature_gate', {
  name: 'new-checkout-flow',
  project_id: 'project-123',
  id_type: 'user'
});

// List all experiments
const experiments = await tools.execute('statsig_list_experiments', {
  project_id: 'project-123'
});
```