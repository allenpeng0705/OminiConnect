# fal.ai Tools

Provider: `falai` | Engine: `nango` | Auth: API_KEY via Nango

## Overview

These tools wrap the fal.ai API. They allow AI agents to generate images using AI models, manage webhooks, and check account credits. fal.ai is a serverless AI inference platform.

## Authentication

**Nango API_KEY**:
- User provides their fal.ai API key via Nango Connect
- Key is passed in the Authorization header as `Key {apiKey}`
- Key stored in Nango, accessed via `connection_ref`

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `falai_list_models` | List available models | GET | /v1/models |
| `falai_get_model` | Get model details | GET | /v1/models/{id} |
| `falai_generate_image` | Generate an image | POST | /fal-ai/fast-sdxl |
| `falai_generate_sdxl` | Generate SDXL image | POST | /fal-ai/fast-sdxl |
| `falai_generate_face` | Generate face image | POST | /fal-ai/face |
| `falai_generate_llm` | Generate with LLM | POST | /fal-ai/llm |
| `falai_get_generation` | Get generation status | GET | /v1/generations/{id} |
| `falai_list_webhooks` | List webhooks | GET | /v1/webhooks |
| `falai_register_webhook` | Register a webhook | POST | /v1/webhooks |
| `falai_get_credits` | Get account credits | GET | /v1/credits |

---

## Tool Details

### falai_list_models

**What it does**: Lists all available AI models.

**When to use**: Browse available models, find model IDs.

**Arguments**: None

**Example LLM prompt**: "List all available AI models"

---

### falai_get_model

**What it does**: Gets details of a specific model.

**When to use**: Understand model capabilities.

**Arguments**:
- `id` (required): Model ID

**Example LLM prompt**: "Get details for model fast-sdxl"

---

### falai_generate_image

**What it does**: Generates images using AI.

**When to use**: Create images from text prompts.

**Arguments**:
- `prompt` (required): Text description of desired image
- `image_size` (optional): Output size (default 1024x1024)
- `num_images` (optional): Number of images (default 1)

**Example LLM prompt**: "Generate an image of a sunset over mountains"

---

### falai_generate_sdxl

**What it does**: Generates images using Stable Diffusion XL.

**When to use**: High-quality image generation with SDXL.

**Arguments**:
- `prompt` (required): Text prompt
- `negative_prompt` (optional): Things to avoid
- `image_size` (optional): Output size

**Example LLM prompt**: "Generate a portrait in the style of impressionism"

---

### falai_generate_face

**What it does**: Generates face images.

**When to use**: Create AI faces, avatar generation.

**Arguments**:
- `prompt` (required): Text description

**Example LLM prompt**: "Generate a realistic face of a person with blue eyes"

---

### falai_generate_llm

**What it does**: Generates text using LLM models.

**When to use**: Text generation, completion tasks.

**Arguments**:
- `model` (optional): Model name
- `prompt` (required): Input text
- `max_tokens` (optional): Max output length

**Example LLM prompt**: "Complete this story: Once upon a time..."

---

### falai_get_generation

**What it does**: Gets the status of a generation request.

**When to use**: Check if async generation is complete.

**Arguments**:
- `id` (required): Generation ID

**Example LLM prompt**: "Check status of generation abc123"

---

### falai_list_webhooks

**What it does**: Lists all registered webhooks.

**When to use**: View configured notifications.

**Arguments**: None

**Example LLM prompt**: "List my webhooks"

---

### falai_register_webhook

**What it does**: Registers a new webhook URL.

**When to use**: Set up event notifications.

**Arguments**:
- `url` (required): Webhook endpoint URL
- `events` (optional): Event types to subscribe to

**Example LLM prompt**: "Register a webhook for https://example.com/webhook"

---

### falai_get_credits

**What it does**: Gets account credit balance.

**When to use**: Check usage, monitor credits.

**Arguments**: None

**Example LLM prompt**: "Check my fal.ai credits"

---

## fal.ai API Notes

- **Models**: Various AI models for image and text generation
- **Generations**: Async operations return generation IDs
- **Webhooks**: Event notifications for generation completion
- **Credits**: Usage-based billing with credit balance
- **Image Generation**: Text-to-image using Stable Diffusion
