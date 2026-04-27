# Hugging Face Tools

Hugging Face provides access to models, datasets, spaces, and inference endpoints for machine learning.

## Provider Overview

- **Provider**: Hugging Face
- **API Base**: `https://huggingface.co/api`
- **Auth**: Bearer token (`HF_TOKEN`)

## Available Tools

### Models

| Tool | Description |
|------|-------------|
| `huggingface_list_models` | Retrieve a list of all available Hugging Face models |
| `huggingface_get_model` | Retrieve details for a specific model by ID |

### Datasets

| Tool | Description |
|------|-------------|
| `huggingface_list_datasets` | Retrieve a list of available datasets on Hugging Face |
| `huggingface_get_dataset` | Retrieve details for a specific dataset by ID |

### Spaces

| Tool | Description |
|------|-------------|
| `huggingface_list_spaces` | Retrieve a list of Hugging Face Spaces (ML apps) |
| `huggingface_get_space` | Retrieve details for a specific Space |

### Inference

| Tool | Description |
|------|-------------|
| `huggingface_create_inference_endpoint` | Create a dedicated inference endpoint for a model |
| `huggingface_list_inference_endpoints` | Retrieve a list of inference endpoints |
| `huggingface_get_inference_endpoint` | Retrieve details for a specific inference endpoint |
| `huggingface_delete_inference_endpoint` | Delete a specific inference endpoint |

## Tool Details

### huggingface_list_models

Retrieve a list of all available Hugging Face models.

- **Endpoint**: `GET /v1/models`
- **Scopes**: `read`
- **Tags**: `models`

**Input Schema**:
```yaml
type: object
properties:
  filter:
    type: string
    description: "Filter models by task type or library"
  sort:
    type: string
    description: "Sort field (e.g., downloads, created_at)"
  direction:
    type: string
    description: "Sort direction (asc or desc)"
  limit:
    type: integer
    description: "Maximum number of models to return"
required: []
```

### huggingface_get_model

Retrieve details for a specific model by ID.

- **Endpoint**: `GET /v1/models/{model_id}`
- **Scopes**: `read`
- **Tags**: `models`

**Input Schema**:
```yaml
type: object
properties:
  model_id:
    type: string
    description: "The model ID (e.g., meta-llama/Llama-2-7b)"
required:
  - model_id
```

### huggingface_list_datasets

Retrieve a list of available datasets on Hugging Face.

- **Endpoint**: `GET /v1/datasets`
- **Scopes**: `read`
- **Tags**: `datasets`

**Input Schema**:
```yaml
type: object
properties:
  filter:
    type: string
    description: "Filter datasets by task type"
  sort:
    type: string
    description: "Sort field"
  direction:
    type: string
    description: "Sort direction"
  limit:
    type: integer
    description: "Maximum number of datasets to return"
required: []
```

### huggingface_get_dataset

Retrieve details for a specific dataset by ID.

- **Endpoint**: `GET /v1/datasets/{dataset_id}`
- **Scopes**: `read`
- **Tags**: `datasets`

**Input Schema**:
```yaml
type: object
properties:
  dataset_id:
    type: string
    description: "The dataset ID"
required:
  - dataset_id
```

### huggingface_list_spaces

Retrieve a list of Hugging Face Spaces (ML apps).

- **Endpoint**: `GET /v1/spaces`
- **Scopes**: `read`
- **Tags**: `spaces`

**Input Schema**:
```yaml
type: object
properties:
  filter:
    type: string
    description: "Filter spaces by SDK or hardware"
  sort:
    type: string
    description: "Sort field"
  direction:
    type: string
    description: "Sort direction"
  limit:
    type: integer
    description: "Maximum number of spaces to return"
required: []
```

### huggingface_get_space

Retrieve details for a specific Space.

- **Endpoint**: `GET /v1/spaces/{space_id}`
- **Scopes**: `read`
- **Tags**: `spaces`

**Input Schema**:
```yaml
type: object
properties:
  space_id:
    type: string
    description: "The space ID (e.g., meta-llama/Llama-2-7b-chat)"
required:
  - space_id
```

### huggingface_create_inference_endpoint

Create a dedicated inference endpoint for a model.

- **Endpoint**: `POST /v1/endpoints`
- **Scopes**: `write`
- **Tags**: `inference`

**Input Schema**:
```yaml
type: object
properties:
  model:
    type: string
    description: "The model to deploy"
  repository:
    type: string
    description: "Repository for the endpoint"
  accelerator:
    type: string
    description: "Hardware accelerator (e.g., gpu-t4-medium)"
  instance_size:
    type: string
    description: "Instance size"
  region:
    type: string
    description: "Deployment region"
required:
  - model
  - repository
  - accelerator
```

### huggingface_list_inference_endpoints

Retrieve a list of inference endpoints.

- **Endpoint**: `GET /v1/endpoints`
- **Scopes**: `read`
- **Tags**: `inference`

**Input Schema**:
```yaml
type: object
properties:
  limit:
    type: integer
    description: "Maximum number of endpoints to return"
required: []
```

### huggingface_get_inference_endpoint

Retrieve details for a specific inference endpoint.

- **Endpoint**: `GET /v1/endpoints/{endpoint_id}`
- **Scopes**: `read`
- **Tags**: `inference`

**Input Schema**:
```yaml
type: object
properties:
  endpoint_id:
    type: string
    description: "The endpoint ID"
required:
  - endpoint_id
```

### huggingface_delete_inference_endpoint

Delete a specific inference endpoint.

- **Endpoint**: `DELETE /v1/endpoints/{endpoint_id}`
- **Scopes**: `write`
- **Tags**: `inference`

**Input Schema**:
```yaml
type: object
properties:
  endpoint_id:
    type: string
    description: "The endpoint ID to delete"
required:
  - endpoint_id
```

## Usage Example

```typescript
// List models sorted by downloads
const models = await tools.execute('huggingface_list_models', {
  sort: 'downloads',
  direction: 'desc',
  limit: 10
});

// Get a specific model
const model = await tools.execute('huggingface_get_model', {
  model_id: 'meta-llama/Llama-2-7b'
});
```
