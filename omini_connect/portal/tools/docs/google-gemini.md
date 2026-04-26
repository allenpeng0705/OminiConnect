# Google Gemini Tools

Provider: `google-gemini` | Engine: `nango` | Auth: API_KEY via Nango

## Overview

These tools wrap the Google Gemini API. They allow AI agents to generate content, count tokens, create embeddings, and manage tuned models. **Requires Google Gemini API key.**

## Authentication

**Nango API_KEY**:
- User provides their Gemini API key
- Token stored in Nango, accessed via `connection_ref`
- Base URL: `https://generativelanguage.googleapis.com`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `google_gemini_list_models` | List available models | GET | /v1beta/models |
| `google_gemini_get_model` | Get model details | GET | /v1beta/models/{model_name} |
| `google_gemini_generate_content` | Generate content | POST | /v1beta/models/{model_name}:generateContent |
| `google_gemini_count_tokens` | Count tokens | POST | /v1beta/models/{model_name}:countTokens |
| `google_gemini_embed_content` | Embed content | POST | /v1beta/models/{model_name}:embedContent |
| `google_gemini_list_tuned_models` | List tuned models | GET | /v1beta/tunedModels |
| `google_gemini_create_tuned_model` | Create tuned model | POST | /v1beta/tunedModels |
| `google_gemini_get_tuned_model` | Get tuned model | GET | /v1beta/tunedModels/{tuned_model_name} |
| `google_gemini_delete_tuned_model` | Delete tuned model | DELETE | /v1beta/tunedModels/{tuned_model_name} |
| `google_gemini_batch_embed_contents` | Batch embed | POST | /v1beta/models/{model_name}:batchEmbedContents |

---

## Tool Details

### google_gemini_list_models

**What it does**: Lists available Gemini models.

**When to use**: See available models for generation.

**Arguments**: None

**Example LLM prompt**: "List available Gemini models"

---

### google_gemini_get_model

**What it does**: Gets details about a specific model.

**When to use**: Check model capabilities.

**Arguments**:
- `model_name` (required): Model name (e.g., gemini-pro)

**Example LLM prompt**: "Get details for gemini-pro model"

---

### google_gemini_generate_content

**What it does**: Generates content using a Gemini model.

**When to use**: Generate text with AI.

**Arguments**:
- `model_name` (required): Model name
- `prompt` (required): Text prompt
- `temperature` (optional): Temperature (0-1, default 0.9)
- `max_tokens` (optional): Max output tokens (default 2048)

**Example LLM prompt**: "Generate content using gemini-pro with prompt 'Write a haiku'"

---

### google_gemini_count_tokens

**What it does**: Counts tokens in input content.

**When to use**: Estimate costs before generation.

**Arguments**:
- `model_name` (required): Model name
- `prompt` (required): Text to count

**Example LLM prompt**: "Count tokens in 'Hello world'"

---

### google_gemini_embed_content

**What it does**: Gets embeddings for content.

**When to use**: Create vector representations for search.

**Arguments**:
- `model_name` (required): Model name
- `content` (required): Text to embed

**Example LLM prompt**: "Embed 'The quick brown fox'"

---

### google_gemini_list_tuned_models

**What it does**: Lists tuned models.

**When to use**: See custom fine-tuned models.

**Arguments**: None

**Example LLM prompt**: "List my tuned models"

---

### google_gemini_create_tuned_model

**What it does**: Creates a tuned Gemini model.

**When to use**: Fine-tune a model for specific tasks.

**Arguments**:
- `base_model` (required): Base model name
- `display_name` (required): Display name
- `training_data` (optional): Training data source

**Example LLM prompt**: "Create a tuned model named 'my-tutor' from gemini-pro"

---

### google_gemini_get_tuned_model

**What it does**: Gets details of a tuned model.

**When to use**: Check tuned model status.

**Arguments**:
- `tuned_model_name` (required): Tuned model name

**Example LLM prompt**: "Get details for tuned model abc123"

---

### google_gemini_delete_tuned_model

**What it does**: Deletes a tuned model.

**When to use**: Remove custom models.

**Arguments**:
- `tuned_model_name` (required): Tuned model name

**Example LLM prompt**: "Delete tuned model abc123"

---

### google_gemini_batch_embed_contents

**What it does**: Batch embeds multiple contents.

**When to use**: Embed many texts at once.

**Arguments**:
- `model_name` (required): Model name
- `contents` (required): Array of content strings

**Example LLM prompt**: "Batch embed ['text1', 'text2', 'text3']"

---

## Google Gemini API Notes

- **Models**: gemini-pro, gemini-pro-vision, etc.
- **Temperature**: Controls randomness (0 = deterministic)
- **Tuned models**: Custom fine-tuned versions
- **Embeddings**: Vector representations for semantic search
