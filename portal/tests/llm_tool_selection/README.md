# LLM Tool Selection Test Suite

This suite evaluates whether an LLM can select the correct API tool from a noisy tool set.

It is designed to work with LiteLLM and OpenAI-compatible providers (OpenAI, MiniMax, etc.).

## What This Test Covers

- Tool selection under noise injection (`small`/`medium`/`large`/`full`)
- Query variation robustness (base query + variations)
- Provider-specific datasets from `queries/*.yaml`
- Failure capture and summary reporting

## Recent Reliability Improvements

The harness now includes provider-agnostic response handling:

- Prefer `tool_calls` over `message.content` when both are present.
- Handle `function.arguments` as either JSON string or JSON object.
- Extract JSON objects from noisy content (including fenced blocks or reasoning wrappers).
- Deterministic request settings for selection (`temperature=0`, `top_p=1`).
- Correct `max_queries` control flow and summary accounting.

These fixes are important because some OpenAI-compatible providers return non-empty reasoning text alongside valid `tool_calls`.

## Running Tests

From `portal/tests/llm_tool_selection`:

```bash
# Example: 10-query smoke test for MiniMax through LiteLLM
cargo run -- \
  --scale small \
  --rounds 1 \
  --provider minimax \
  --model minimax:latest \
  --registry ../../tools/registry \
  --queries queries \
  --llm-url http://localhost:4000 \
  --max-queries 10 \
  --delay-ms 200
```

Or use the helper script:

```bash
./scripts/run_tests.sh --scale small --rounds 1 --model minimax:latest
```

## Query Dataset Generation (Realistic Mode)

Query files are generated from registry data with realistic templates and schema-aware hints.

From `portal/`:

```bash
# Regenerate one provider
python3 tests/llm_tool_selection/scripts/generate_queries.py --provider minimax --overwrite

# Regenerate all providers
python3 tests/llm_tool_selection/scripts/generate_queries.py --overwrite
```

Generator behavior:

- Uses `example_queries` when available.
- Infers action from tool slug (including `send -> create`, `assign -> update`).
- Builds natural-language templates from tool/resource names.
- Injects sample values for required parameters.

## Configuration Notes

- `config.yaml` contains default test parameters.
- LiteLLM endpoint and model alias must be available (for example `minimax:latest`).
- Registry path is usually `../../tools/registry` when running from this directory.

## Interpreting Results

- `got: None` usually means response parsing or schema extraction mismatch.
- Persistent failures on one tool often indicate:
  - ambiguous or unrealistic query wording
  - weak tool description overlap
  - missing required parameter context

Use small smoke runs (`--max-queries 10`) before larger sweeps.
