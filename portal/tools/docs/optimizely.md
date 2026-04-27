# Optimizely Tools

Optimizely provides experimentation and feature management for A/B testing, multivariate testing, and feature rollouts.

## Provider Overview

- **Provider**: Optimizely
- **API Base**: `https://api.optimizely.com/api/v2`
- **Auth**: Bearer token (`OPTIMIZELY_API_KEY`)

## Available Tools

### Experiments

| Tool | Description |
|------|-------------|
| `optimizely_list_experiments` | Retrieve a list of all experiments in a project |
| `optimizely_get_experiment` | Retrieve a specific experiment by ID |
| `optimizely_create_experiment` | Create a new experiment in a project |
| `optimizely_update_experiment` | Update an existing experiment |

### Features

| Tool | Description |
|------|-------------|
| `optimizely_list_features` | Retrieve a list of all features in a project |
| `optimizely_get_feature` | Retrieve a specific feature by key |

### Audiences

| Tool | Description |
|------|-------------|
| `optimizely_list_audiences` | Retrieve a list of all audiences in a project |
| `optimizely_get_audience` | Retrieve a specific audience by ID |

### Variations

| Tool | Description |
|------|-------------|
| `optimizely_list_variations` | Retrieve a list of all variations for an experiment |
| `optimizely_get_variation` | Retrieve a specific variation by ID |

## Tool Details

### optimizely_list_experiments

Retrieve a list of all experiments in a project.

- **Endpoint**: `GET /v2/experiments`
- **Scopes**: `experiments:read`
- **Tags**: `experiments`, `list`

**Input Schema**:
```yaml
type: object
properties:
  project_id:
    type: integer
    description: "The project ID"
  limit:
    type: integer
    description: "Maximum number of experiments to return"
  page_token:
    type: string
    description: "Token for pagination"
required:
  - project_id
```

### optimizely_get_experiment

Retrieve a specific experiment by ID.

- **Endpoint**: `GET /v2/experiments/{experiment_id}`
- **Scopes**: `experiments:read`
- **Tags**: `experiments`, `read`

**Input Schema**:
```yaml
type: object
properties:
  experiment_id:
    type: integer
    description: "The experiment ID"
required:
  - experiment_id
```

### optimizely_create_experiment

Create a new experiment in a project.

- **Endpoint**: `POST /v2/experiments`
- **Scopes**: `experiments:write`
- **Tags**: `experiments`, `create`

**Input Schema**:
```yaml
type: object
properties:
  name:
    type: string
    description: "The experiment name"
  description:
    type: string
    description: "Description of the experiment"
  project_id:
    type: integer
    description: "The project ID"
  hypothesis:
    type: string
    description: "The hypothesis for the experiment"
  variants:
    type: array
    description: "List of experiment variants"
required:
  - name
  - project_id
```

### optimizely_update_experiment

Update an existing experiment.

- **Endpoint**: `PATCH /v2/experiments/{experiment_id}`
- **Scopes**: `experiments:write`
- **Tags**: `experiments`, `update`

**Input Schema**:
```yaml
type: object
properties:
  experiment_id:
    type: integer
    description: "The experiment ID"
  name:
    type: string
    description: "Updated experiment name"
  description:
    type: string
    description: "Updated description"
  status:
    type: string
    description: "Experiment status (active, paused, archived)"
required:
  - experiment_id
```

### optimizely_list_features

Retrieve a list of all features in a project.

- **Endpoint**: `GET /v2/features`
- **Scopes**: `features:read`
- **Tags**: `features`, `list`

**Input Schema**:
```yaml
type: object
properties:
  project_id:
    type: integer
    description: "The project ID"
  limit:
    type: integer
    description: "Maximum number of features to return"
required:
  - project_id
```

### optimizely_get_feature

Retrieve a specific feature by key.

- **Endpoint**: `GET /v2/features/{feature_key}`
- **Scopes**: `features:read`
- **Tags**: `features`, `read`

**Input Schema**:
```yaml
type: object
properties:
  feature_key:
    type: string
    description: "The feature key"
  project_id:
    type: integer
    description: "The project ID"
required:
  - feature_key
```

### optimizely_list_audiences

Retrieve a list of all audiences in a project.

- **Endpoint**: `GET /v2/audiences`
- **Scopes**: `audiences:read`
- **Tags**: `audiences`, `list`

**Input Schema**:
```yaml
type: object
properties:
  project_id:
    type: integer
    description: "The project ID"
required:
  - project_id
```

### optimizely_get_audience

Retrieve a specific audience by ID.

- **Endpoint**: `GET /v2/audiences/{audience_id}`
- **Scopes**: `audiences:read`
- **Tags**: `audiences`, `read`

**Input Schema**:
```yaml
type: object
properties:
  audience_id:
    type: string
    description: "The audience ID"
required:
  - audience_id
```

### optimizely_list_variations

Retrieve a list of all variations for an experiment.

- **Endpoint**: `GET /v2/experiments/{experiment_id}/variations`
- **Scopes**: `variations:read`
- **Tags**: `variations`, `list`

**Input Schema**:
```yaml
type: object
properties:
  experiment_id:
    type: integer
    description: "The experiment ID"
required:
  - experiment_id
```

### optimizely_get_variation

Retrieve a specific variation by ID.

- **Endpoint**: `GET /v2/variations/{variation_id}`
- **Scopes**: `variations:read`
- **Tags**: `variations`, `read`

**Input Schema**:
```yaml
type: object
properties:
  variation_id:
    type: string
    description: "The variation ID"
required:
  - variation_id
```

## Usage Example

```typescript
// List all experiments in a project
const experiments = await tools.execute('optimizely_list_experiments', {
  project_id: 12345
});

// Create a new experiment
const experiment = await tools.execute('optimizely_create_experiment', {
  name: 'New Landing Page Test',
  project_id: 12345,
  hypothesis: 'A new landing page will increase conversions'
});

// Get a specific feature
const feature = await tools.execute('optimizely_get_feature', {
  feature_key: 'new-checkout-flow'
});
```