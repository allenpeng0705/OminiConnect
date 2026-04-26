# Anthropic Tools

Anthropic provides access to Claude models for conversation, reasoning, and evaluation tasks.

## Provider Overview

- **Provider**: Anthropic
- **API Base**: `https://api.anthropic.com`
- **Auth**: Bearer token (`ANTHROPIC_API_KEY`)

## Available Tools

### Messages

| Tool | Description |
|------|-------------|
| `anthropic_create_message` | Create a message in a conversation with Claude |
| `anthropic_create_message_stream` | Create a streaming message for real-time response |
| `anthropic_list_count_messages` | Retrieve a list of message counts for a given conversation |

### Models

| Tool | Description |
|------|-------------|
| `anthropic_list_models` | Retrieve a list of available Anthropic models |
| `anthropic_get_model` | Retrieve details for a specific model |

### Tokens

| Tool | Description |
|------|-------------|
| `anthropic_count_tokens` | Count tokens in a text or messages for a specific model |

### Runs

| Tool | Description |
|------|-------------|
| `anthropic_list_runs` | Retrieve a list of runs for a thread |
| `anthropic_get_run` | Retrieve a specific run by ID |

### Evaluation

| Tool | Description |
|------|-------------|
| `anthropic_list_evaluation_metrics` | Retrieve a list of evaluation metrics for model assessment |
| `anthropic_get_evaluation_metric` | Retrieve a specific evaluation metric by ID |

## Tool Details

### anthropic_create_message

Create a message in a conversation with Claude.

- **Endpoint**: `POST /v1/messages`
- **Scopes**: `messages:write`
- **Tags**: `messages`

**Input Schema**:
```yaml
type: object
properties:
  model:
    type: string
    description: "The model to use (e.g., claude-3-5-haiku-20241007)"
  messages:
    type: array
    description: "Array of message objects with role and content"
  max_tokens:
    type: integer
    description: "Maximum tokens to generate"
  system:
    type: string
    description: "System prompt"
  temperature:
    type: number
    description: "Sampling temperature (0-1)"
required:
  - model
  - messages
  - max_tokens
```

### anthropic_list_models

Retrieve a list of available Anthropic models.

- **Endpoint**: `GET /v1/models`
- **Scopes**: `models:read`
- **Tags**: `models`

**Input Schema**:
```yaml
type: object
properties: {}
required: []
```

### anthropic_get_model

Retrieve details for a specific model.

- **Endpoint**: `GET /v1/models/{model}`
- **Scopes**: `models:read`
- **Tags**: `models`

**Input Schema**:
```yaml
type: object
properties:
  model:
    type: string
    description: "The model identifier"
required:
  - model
```

### anthropic_create_message_stream

Create a streaming message for real-time response.

- **Endpoint**: `POST /v1/messages_stream`
- **Scopes**: `messages:write`
- **Tags**: `messages`

**Input Schema**:
```yaml
type: object
properties:
  model:
    type: string
    description: "The model to use for streaming"
  messages:
    type: array
    description: "Array of message objects"
  max_tokens:
    type: integer
    description: "Maximum tokens to generate"
required:
  - model
  - messages
  - max_tokens
```

### anthropic_list_count_messages

Retrieve a list of message counts for a given conversation.

- **Endpoint**: `GET /v1/count_messages`
- **Scopes**: `count_messages:read`
- **Tags**: `messages`

**Input Schema**:
```yaml
type: object
properties:
  conversation_id:
    type: string
    description: "The conversation ID"
required:
  - conversation_id
```

### anthropic_count_tokens

Count tokens in a text or messages for a specific model.

- **Endpoint**: `POST /v1/messages/count_tokens`
- **Scopes**: `tokens:write`
- **Tags**: `tokens`

**Input Schema**:
```yaml
type: object
properties:
  model:
    type: string
    description: "The model to count tokens for"
  messages:
    type: array
    description: "Array of messages to count"
required:
  - model
  - messages
```

### anthropic_list_runs

Retrieve a list of runs for a thread.

- **Endpoint**: `GET /v1/threads/{thread_id}/runs`
- **Scopes**: `runs:read`
- **Tags**: `runs`

**Input Schema**:
```yaml
type: object
properties:
  thread_id:
    type: string
    description: "The thread ID"
  limit:
    type: integer
    description: "Maximum number of runs to return"
required:
  - thread_id
```

### anthropic_get_run

Retrieve a specific run by ID.

- **Endpoint**: `GET /v1/threads/{thread_id}/runs/{run_id}`
- **Scopes**: `runs:read`
- **Tags**: `runs`

**Input Schema**:
```yaml
type: object
properties:
  thread_id:
    type: string
    description: "The thread ID"
  run_id:
    type: string
    description: "The run ID"
required:
  - thread_id
  - run_id
```

### anthropic_list_evaluation_metrics

Retrieve a list of evaluation metrics for model assessment.

- **Endpoint**: `GET /v1/evaluation_metrics`
- **Scopes**: `evaluation:read`
- **Tags**: `evaluation`

**Input Schema**:
```yaml
type: object
properties: {}
required: []
```

### anthropic_get_evaluation_metric

Retrieve a specific evaluation metric by ID.

- **Endpoint**: `GET /v1/evaluation_metrics/{metric_id}`
- **Scopes**: `evaluation:read`
- **Tags**: `evaluation`

**Input Schema**:
```yaml
type: object
properties:
  metric_id:
    type: string
    description: "The evaluation metric ID"
required:
  - metric_id
```

## Usage Example

```typescript
// Create a message with Claude
const response = await tools.execute('anthropic_create_message', {
  model: 'claude-3-5-haiku-20241007',
  messages: [
    { role: 'user', content: 'Hello, Claude!' }
  ],
  max_tokens: 256
});
```
