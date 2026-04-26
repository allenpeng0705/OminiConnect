# Replicate Tools

Replicate provides access to running machine learning models and managing predictions.

## Provider Overview

- **Provider**: Replicate
- **API Base**: `https://api.replicate.com`
- **Auth**: Bearer token (`REPLICATE_API_TOKEN`)

## Available Tools

### Models

| Tool | Description |
|------|-------------|
| `replicate_list_models` | Retrieve a list of all available Replicate models |
| `replicate_get_model` | Retrieve details for a specific model by owner and name |
| `replicate_get_model_version` | Retrieve a specific version of a model |

### Predictions

| Tool | Description |
|------|-------------|
| `replicate_create_prediction` | Start a new prediction with a model |
| `replicate_get_prediction` | Retrieve a prediction by ID |
| `replicate_list_predictions` | Retrieve a list of predictions |
| `replicate_cancel_prediction` | Cancel a running prediction |

### Collections

| Tool | Description |
|------|-------------|
| `replicate_list_collections` | Retrieve a list of curated model collections |
| `replicate_get_collection` | Retrieve a specific collection by slug |

### Webhooks

| Tool | Description |
|------|-------------|
| `replicate_create_webhook` | Register a webhook for prediction events |

## Tool Details

### replicate_list_models

Retrieve a list of all available Replicate models.

- **Endpoint**: `GET /v1/models`
- **Scopes**: `read`
- **Tags**: `models`

**Input Schema**:
```yaml
type: object
properties:
  filter:
    type: string
    description: "Filter by creator or name pattern"
  limit:
    type: integer
    description: "Maximum number of models to return"
required: []
```

### replicate_get_model

Retrieve details for a specific model by owner and name.

- **Endpoint**: `GET /v1/models/{owner}/{name}`
- **Scopes**: `read`
- **Tags**: `models`

**Input Schema**:
```yaml
type: object
properties:
  owner:
    type: string
    description: "The model owner (username or organization)"
  name:
    type: string
    description: "The model name"
required:
  - owner
  - name
```

### replicate_get_model_version

Retrieve a specific version of a model.

- **Endpoint**: `GET /v1/models/{owner}/{name}/versions/{version_id}`
- **Scopes**: `read`
- **Tags**: `models`

**Input Schema**:
```yaml
type: object
properties:
  owner:
    type: string
    description: "The model owner"
  name:
    type: string
    description: "The model name"
  version_id:
    type: string
    description: "The version ID"
required:
  - owner
  - name
  - version_id
```

### replicate_create_prediction

Start a new prediction with a model.

- **Endpoint**: `POST /v1/predictions`
- **Scopes**: `write`
- **Tags**: `predictions`

**Input Schema**:
```yaml
type: object
properties:
  model:
    type: string
    description: "Model version ID to use"
  input:
    type: object
    description: "Input parameters for the model"
  webhook:
    type: string
    description: "Callback URL for completion notification"
required:
  - model
  - input
```

### replicate_get_prediction

Retrieve a prediction by ID.

- **Endpoint**: `GET /v1/predictions/{prediction_id}`
- **Scopes**: `read`
- **Tags**: `predictions`

**Input Schema**:
```yaml
type: object
properties:
  prediction_id:
    type: string
    description: "The prediction ID"
required:
  - prediction_id
```

### replicate_list_predictions

Retrieve a list of predictions.

- **Endpoint**: `GET /v1/predictions`
- **Scopes**: `read`
- **Tags**: `predictions`

**Input Schema**:
```yaml
type: object
properties:
  limit:
    type: integer
    description: "Maximum number of predictions to return"
required: []
```

### replicate_cancel_prediction

Cancel a running prediction.

- **Endpoint**: `POST /v1/predictions/{prediction_id}/cancel`
- **Scopes**: `write`
- **Tags**: `predictions`

**Input Schema**:
```yaml
type: object
properties:
  prediction_id:
    type: string
    description: "The prediction ID to cancel"
required:
  - prediction_id
```

### replicate_list_collections

Retrieve a list of curated model collections.

- **Endpoint**: `GET /v1/collections`
- **Scopes**: `read`
- **Tags**: `collections`

**Input Schema**:
```yaml
type: object
properties:
  limit:
    type: integer
    description: "Maximum number of collections to return"
required: []
```

### replicate_get_collection

Retrieve a specific collection by slug.

- **Endpoint**: `GET /v1/collections/{slug}`
- **Scopes**: `read`
- **Tags**: `collections`

**Input Schema**:
```yaml
type: object
properties:
  slug:
    type: string
    description: "The collection slug"
required:
  - slug
```

### replicate_create_webhook

Register a webhook for prediction events.

- **Endpoint**: `POST /v1/webhooks`
- **Scopes**: `write`
- **Tags**: `webhooks`

**Input Schema**:
```yaml
type: object
properties:
  url:
    type: string
    description: "The webhook URL"
  events:
    type: array
    description: "Event types to subscribe to"
required:
  - url
  - events
```

## Usage Example

```typescript
// Create a prediction with Llama model
const prediction = await tools.execute('replicate_create_prediction', {
  model: 'meta/llama-2-7b:latest',
  input: {
    prompt: 'Hello, world!'
  },
  webhook: 'https://myapp.com/webhook/replicate'
});

// Get prediction status
const result = await tools.execute('replicate_get_prediction', {
  prediction_id: prediction.id
});
```
