# ElevenLabs Tools

Provider: `elevenlabs` | Engine: `nango` | Auth: API_KEY via Nango

## Overview

These tools wrap the ElevenLabs API. They allow AI agents to convert text to speech using AI-generated voices, manage voice libraries, and access synthesis history. ElevenLabs is a leading AI voice platform for content creators, developers, and businesses.

## Authentication

**Nango API_KEY**:
- User provides their ElevenLabs API key via Nango Connect
- Key is stored in Nango, accessed via `connection_ref`
- No OAuth flow - direct API key authentication

## Tool List

| Tool | Description | Method | Endpoint |
|------|-------------|--------|----------|
| `elevenlabs_get_user` | Get current user info | GET | /v1/user |
| `elevenlabs_list_voices` | List available voices | GET | /v1/voices |
| `elevenlabs_get_voice` | Get voice details | GET | /v1/voices/{voice_id} |
| `elevenlabs_list_models` | List available models | GET | /v1/models |
| `elevenlabs_get_model` | Get model details | GET | /v1/models/{model_id} |
| `elevenlabs_text_to_speech` | Convert text to speech | POST | /v1/text-to-speech/{voice_id} |
| `elevenlabs_list_history` | List history entries | GET | /v1/history |
| `elevenlabs_get_history_item` | Get history item details | GET | /v1/history/{history_id} |
| `elevenlabs_delete_history` | Delete history entries | POST | /v1/history/delete |
| `elevenlabs_list_institutions` | List institutions | GET | /v1/institutions |

---

## Tool Details

### elevenlabs_get_user

**What it does**: Gets information about the current authenticated user including subscription status and API usage.

**When to use**: Check account status, verify authentication, get subscription tier.

**Arguments**: None

**Example LLM prompt**: "Get my ElevenLabs account info"

---

### elevenlabs_list_voices

**What it does**: Lists all voices available in the user's library, including default and custom voices.

**When to use**: Find available voices for text-to-speech, browse voice library.

**Arguments**:
- `page_size` (optional): Number of voices to return (default 20)
- `start` (optional): Offset for pagination

**Example LLM prompt**: "List all voices in my ElevenLabs library"

---

### elevenlabs_get_voice

**What it does**: Gets detailed information about a specific voice including settings and preview URL.

**When to use**: Check voice settings before using for TTS.

**Arguments**:
- `voice_id` (required): The voice ID

**Example LLM prompt**: "Get details for voice ID xyz123"

---

### elevenlabs_list_models

**What it does**: Lists all available text-to-speech models with their capabilities.

**When to use**: Select appropriate model for voice cloning or TTS.

**Arguments**: None

**Example LLM prompt**: "What TTS models are available"

---

### elevenlabs_get_model

**What it does**: Gets details of a specific model including supported languages and capabilities.

**When to use**: Understand model limitations before use.

**Arguments**:
- `model_id` (required): The model ID

**Example LLM prompt**: "Get details for the multilingual model"

---

### elevenlabs_text_to_speech

**What it does**: Converts text to speech audio using the specified voice and model.

**When to use**: Generate audio from text, create voiceovers, synthesize speech.

**Arguments**:
- `voice_id` (required): Voice ID to use
- `text` (required): Text to convert
- `model_id` (optional): Model to use (default: eleven_multilingual_v2)
- `voice_settings` (optional): Voice settings (stability, similarity, etc.)

**Example LLM prompt**: "Convert 'Hello world' to speech using voice xyz123"

---

### elevenlabs_list_history

**What it does**: Lists all synthesis history entries for the user.

**When to use**: Find previously generated audio, replay past syntheses.

**Arguments**:
- `page_size` (optional): Number of entries (default 20)
- `start` (optional): Pagination offset

**Example LLM prompt**: "List my recent TTS generations"

---

### elevenlabs_get_history_item

**What it does**: Gets details of a specific history item including audio URL.

**When to use**: Retrieve previously generated audio files.

**Arguments**:
- `history_id` (required): History item ID

**Example LLM prompt**: "Get the audio for history item abc456"

---

### elevenlabs_delete_history

**What it does**: Deletes one or more history entries.

**When to use**: Clean up synthesis history, free up storage.

**Arguments**:
- `history_item_ids` (required): Array of history IDs to delete

**Example LLM prompt**: "Delete history items abc123 and def456"

---

### elevenlabs_list_institutions

**What it does**: Lists all institutions the user has access to.

**When to use**: Manage multi-tenant workspaces.

**Arguments**: None

**Example LLM prompt**: "List my ElevenLabs institutions"

---

## ElevenLabs API Notes

- **Voice IDs**: Unique identifiers for voices in the library
- **Models**: Different models support different languages and features
- **History**: Stores previously generated audio for reuse
- **Institutions**: Support for team workspaces and multi-tenant setups
- **Voice Settings**: Adjust stability, similarity, and style for fine-tuned control
