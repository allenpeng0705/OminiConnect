# OpenAI Tools

OpenAI provides access to powerful language models for text generation, embeddings, image creation, and fine-tuning.

## Provider Overview

- **Provider**: OpenAI
- **API Base**: `https://api.openai.com`
- **Auth**: Bearer token (`OPENAI_API_KEY`)

## Available Tools

### Models

| Tool | Description |
|------|-------------|
| `openai_list_models` | Retrieve a list of all available OpenAI models |
| `openai_get_model` | Retrieve details for a specific model by ID |

### Completions

| Tool | Description |
|------|-------------|
| `openai_create_completion` | Create a text completion for a given prompt |
| `openai_create_chat_completion` | Create a chat completion for a conversation |

### Embeddings

| Tool | Description |
|------|-------------|
| `openai_create_embedding` | Create an embedding vector for text input |

### Images

| Tool | Description |
|------|-------------|
| `openai_create_image` | Generate an image from a text description |

### Fine-tuning

| Tool | Description |
|------|-------------|
| `openai_list_fine_tuning_jobs` | Retrieve a list of fine-tuning jobs |
| `openai_get_fine_tuning_job` | Retrieve a fine-tuning job by ID |

### Assistants

| Tool | Description |
|------|-------------|
| `openai_list_assistants` | Retrieve a list of assistants |
| `openai_get_assistant` | Retrieve an assistant by ID |

## Tool Details

### openai_list_models

Retrieve a list of all available OpenAI models.

- **Endpoint**: `GET /v1/models`
- **Scopes**: `model:read`
- **Tags**: `models`

**Input Schema**:
```yaml
type: object
properties: {}
required: []
```

### openai_get_model

Retrieve details for a specific model by ID.

- **Endpoint**: `GET /v1/models/{model}`
- **Scopes**: `model:read`
- **Tags**: `models`

**Input Schema**:
```yaml
type: object
properties:
  model:
    type: string
    description: "The model ID to retrieve"
required:
  - model
```

### openai_create_completion

Create a text completion for a given prompt.

- **Endpoint**: `POST /v1/completions`
- **Scopes**: `completions:write`
- **Tags**: `completions`

**Input Schema**:
```yaml
type: object
properties:
  model:
    type: string
    description: "The model to use for completion"
  prompt:
    type: string
    description: "The prompt to complete"
  max_tokens:
    type: integer
    description: "Maximum number of tokens to generate"
  temperature:
    type: number
    description: "Sampling temperature (0-2)"
required:
  - model
  - prompt
```

### openai_create_chat_completion

Create a chat completion for a conversation.

- **Endpoint**: `POST /v1/chat/completions`
- **Scopes**: `chat.completions:write`
- **Tags**: `chat`

**Input Schema**:
```yaml
type: object
properties:
  model:
    type: string
    description: "The model to use for chat completion"
  messages:
    type: array
    description: "Array of message objects with role and content"
  max_tokens:
    type: integer
    description: "Maximum number of tokens to generate"
  temperature:
    type: number
    description: "Sampling temperature (0-2)"
required:
  - model
  - messages
```

### openai_create_embedding

Create an embedding vector for text input.

- **Endpoint**: `POST /v1/embeddings`
- **Scopes**: `embeddings:write`
- **Tags**: `embeddings`

**Input Schema**:
```yaml
type: object
properties:
  model:
    type: string
    description: "The model to use for embeddings"
  input:
    type: string
    description: "The text to embed"
required:
  - model
  - input
```

### openai_create_image

Generate an image from a text description.

- **Endpoint**: `POST /v1/images/generations`
- **Scopes**: `images:write`
- **Tags**: `images`

**Input Schema**:
```yaml
type: object
properties:
  model:
    type: string
    description: "The model to use for image generation"
  prompt:
    type: string
    description: "A text description of the desired image"
  n:
    type: integer
    description: "Number of images to generate"
  size:
    type: string
    description: "Size of the generated images (e.g., 1024x1024)"
required:
  - prompt
```

### openai_list_fine_tuning_jobs

Retrieve a list of fine-tuning jobs.

- **Endpoint**: `GET /v1/fine_tuning/jobs`
- **Scopes**: `fine_tuning:read`
- **Tags**: `fine-tuning`

**Input Schema**:
```yaml
type: object
properties:
  limit:
    type: integer
    description: "Maximum number of jobs to return"
  after:
    type: string
    description: "Cursor for pagination"
required: []
```

### openai_get_fine_tuning_job

Retrieve a fine-tuning job by ID.

- **Endpoint**: `GET /v1/fine_tuning/jobs/{job_id}`
- **Scopes**: `fine_tuning:read`
- **Tags**: `fine-tuning`

**Input Schema**:
```yaml
type: object
properties:
  job_id:
    type: string
    description: "The fine-tuning job ID"
required:
  - job_id
```

### openai_list_assistants

Retrieve a list of assistants.

- **Endpoint**: `GET /v1/assistants`
- **Scopes**: `assistants:read`
- **Tags**: `assistants`

**Input Schema**:
```yaml
type: object
properties:
  limit:
    type: integer
    description: "Maximum number of assistants to return"
  order:
    type: string
    description: "Sort order (asc or desc)"
required: []
```

### openai_get_assistant

Retrieve an assistant by ID.

- **Endpoint**: `GET /v1/assistants/{assistant_id}`
- **Scopes**: `assistants:read`
- **Tags**: `assistants`

**Input Schema**:
```yaml
type: object
properties:
  assistant_id:
    type: string
    description: "The assistant ID"
required:
  - assistant_id
```

## Usage Example

```typescript
// Create a chat completion
const response = await tools.execute('openai_create_chat_completion', {
  model: 'gpt-4',
  messages: [
    { role: 'user', content: 'Hello, how are you?' }
  ],
  max_tokens: 100
});
```
