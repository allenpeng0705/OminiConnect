# Gemini, Vertex AI, and Amazon Bedrock with Panda

**Goal:** Use Panda’s **OpenAI-shaped** client API (`POST /v1/chat/completions`) against **Google Gemini**, **Vertex AI (OpenAI-compatible)**, or **Amazon Bedrock (OpenAI-compatible)** by setting **`default_backend` / `routes[].backend_base`**, **`adapter.provider`**, and **credentials**.

**Official OpenAI compatibility docs (read these for limits and model IDs):**

- **Gemini (Google AI Studio / API key):** [OpenAI compatibility](https://ai.google.dev/gemini-api/docs/openai)
- **Vertex AI (GCP):** [OpenAI compatibility](https://cloud.google.com/vertex-ai/generative-ai/docs/start/openai)
- **Bedrock:** [OpenAI API–compatible inference](https://docs.aws.amazon.com/bedrock/latest/userguide/openai-compatibility.html)

---

## How Panda handles API keys (all providers)

Panda’s **default** proxy path **forwards** the request **headers** you send to the gateway. For the usual LLM flow:

1. **Client → Panda:** send the provider credential on the **same** header the upstream expects (usually **`Authorization: Bearer <secret>`**).
2. **Panda → upstream:** that header is forwarded (after hop-by-hop filtering), so the **model host** receives the key.

So:

```bash
export GEMINI_API_KEY="your-key"   # example
curl -s http://127.0.0.1:8080/v1/chat/completions \
  -H "Authorization: Bearer $GEMINI_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{"model":"gemini-2.0-flash","messages":[{"role":"user","content":"hello"}]}'
```

**You do not** need a separate Panda-only env var for this path unless you add a **different** integration (e.g. semantic routing **embed** uses `semantic_cache.embedding_api_key_env`, or **model_failover** backends use `api_key_env` per backend).

**If clients must not hold the provider key:** terminate TLS at an **edge gateway** (Kong, Envoy, etc.) that injects `Authorization: Bearer …`, or use Panda’s **`trusted_gateway`** + your own header contract; **egress** `default_headers` apply to **corporate HTTP tools**, not to the main LLM `default_backend` path.

---

## Google Gemini (API key, OpenAI-compatible)

Use the **OpenAI-compatible base URL** from Google’s docs:

- **Base:** `https://generativelanguage.googleapis.com/v1beta/openai/`
- Chat path becomes: `…/v1/chat/completions` (Panda joins `default_backend` + request path).

**API key:** Create a key in [Google AI Studio](https://aistudio.google.com/) (or equivalent). Use it as **`Authorization: Bearer <key>`** when calling Panda (forwarded to Gemini).

**Optional env var name (example):** `GEMINI_API_KEY` or `GOOGLE_API_KEY` — export and pass it in the client `Authorization` header as above.

**YAML sketch:**

```yaml
listen: "127.0.0.1:8080"
default_backend: "https://generativelanguage.googleapis.com/v1beta/openai"
adapter:
  provider: gemini
```

`adapter.provider: gemini` is a **label** only (same behavior as `openai`); it helps operators see the intended vendor in config.

---

## Google Cloud Vertex AI (OpenAI-compatible)

Vertex exposes an **OpenAI-compatible** surface with a **base URL** that includes **project**, **region**, and often **endpoint id**. See:

- [Vertex AI — OpenAI compatibility](https://cloud.google.com/vertex-ai/generative-ai/docs/start/openai)

**Authentication:** **Not** a long-lived Google AI Studio key. You typically use:

- **OAuth2 access token** with scope for Vertex: `Authorization: Bearer <access_token>`
- Tokens are **short-lived** (often ~1 hour). Obtain via:
  - **Workload identity** / GKE / Cloud Run (recommended in GCP)
  - **Service account JSON key:** `GOOGLE_APPLICATION_CREDENTIALS` + client libraries that mint tokens; for **curl**, you often run `gcloud auth print-access-token` in dev and pass the token in `Authorization`.

**Panda’s role:** Forward `Authorization: Bearer` to your Vertex OpenAI-compatible host. **Token refresh** is your responsibility (sidecar, script, or edge).

**YAML sketch:** set `default_backend` to the **full** OpenAI-compatible base URL from the Vertex doc for your region and project.

```yaml
adapter:
  provider: vertex
# default_backend: "https://{region}-aiplatform.googleapis.com/v1/projects/{PROJECT}/locations/{REGION}/endpoints/openapi"
# (exact string from Google Cloud console / docs for your setup)
```

---

## Amazon Bedrock (OpenAI-compatible)

AWS documents **OpenAI-compatible** inference on Bedrock. See:

- [Invoke OpenAI-compatible APIs on Amazon Bedrock](https://docs.aws.amazon.com/bedrock/latest/userguide/openai-compatibility.html)

**Authentication:** AWS typically uses **AWS Signature Version 4 (SigV4)** in addition to or instead of a simple `Bearer` token. **Panda does not ship SigV4 signing** for the main upstream client today.

**Practical options:**

1. **Use a component that signs requests** (AWS-provided proxy, Lambda, or **API Gateway** in front of Bedrock) and point Panda’s `default_backend` at that **HTTPS** endpoint if it accepts **`Authorization: Bearer`** or a **fixed** header your team controls.
2. **Run Panda behind** a signing layer that adds SigV4 (not documented here as a single YAML snippet).
3. **Track** native SigV4 / Bedrock in the product roadmap if you need **direct** `bedrock-runtime.*.amazonaws.com` from Panda without a middleman.

**YAML label:** you can still set `adapter.provider: bedrock` so config is explicit; **wire** auth must match the endpoint you use.

```yaml
adapter:
  provider: bedrock
# default_backend: must match your chosen deployment (OpenAI-compatible proxy URL or future native support)
```

---

## Summary

| Provider | Typical credential | Panda config label |
|----------|-------------------|-------------------|
| **Gemini** (API key) | `Authorization: Bearer` + API key | `adapter.provider: gemini` |
| **Vertex** | OAuth2 **Bearer** access token (GCP) | `adapter.provider: vertex` |
| **Bedrock** | Often **SigV4** (AWS); simple Bearer only if you use a compatible proxy | `adapter.provider: bedrock` |

All three labels use the **same passthrough** code path as `openai` in `panda-proxy`; they exist so **`panda.yaml`** and runbooks can name the vendor. **Base URL and auth** must match that vendor’s **OpenAI-compatible** documentation.

---

## See also

- [`provider_adapters.md`](./provider_adapters.md) — general passthrough vs native adapters
- [`panda.example.yaml`](../panda.example.yaml) — adapter comment block
