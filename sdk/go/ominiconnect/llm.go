package ominiconnect

import (
	"context"
	"encoding/json"
	"fmt"
	"net/http"
	"net/url"
)

// ─── LlmManager ───────────────────────────────────────────────────────────────

// LlmManager handles LLM-driven tool discovery and execution.
type LlmManager struct {
	client *Client
}

// Execute sends a natural language query and returns tool candidates or execution plan.
func (m *LlmManager) Execute(ctx context.Context, query string, platform *string) (*LlmExecuteResponse, error) {
	payload := map[string]any{"query": query}
	if platform != nil {
		payload["platform"] = *platform
	}
	resp, err := m.client.do(ctx, http.MethodPost, "/api/llm", nil, payload)
	if err != nil {
		return nil, err
	}
	if resp == nil {
		return nil, nil
	}
	var out LlmExecuteResponse
	if err := json.Unmarshal(*resp, &out); err != nil {
		return nil, fmt.Errorf("decode llm response: %w", err)
	}
	return &out, nil
}

// ListAvailableTools returns available tools per platform, filtered by platform if specified.
func (m *LlmManager) ListAvailableTools(ctx context.Context, platform *string) (*LlmToolsResponse, error) {
	query := url.Values{}
	if platform != nil {
		query.Set("platform", *platform)
	}
	resp, err := m.client.do(ctx, http.MethodGet, "/api/llm/tools", query, nil)
	if err != nil {
		return nil, err
	}
	if resp == nil {
		return nil, nil
	}
	var out LlmToolsResponse
	if err := json.Unmarshal(*resp, &out); err != nil {
		return nil, fmt.Errorf("decode llm tools: %w", err)
	}
	return &out, nil
}

// ─── Response types ──────────────────────────────────────────────────────────

type LlmExecuteResponse struct {
	OK                 bool             `json:"ok"`
	Tool               string           `json:"tool,omitempty"`
	ToolName           string           `json:"tool_name,omitempty"`
	Arguments          any              `json:"arguments,omitempty"`
	Explanation        string           `json:"explanation,omitempty"`
	Result             any              `json:"result,omitempty"`
	Error              string           `json:"error,omitempty"`
	Message            string           `json:"message,omitempty"`
	Candidates         []CandidateTool  `json:"candidates,omitempty"`
	AvailableToolsHint string           `json:"available_tools_hint,omitempty"`
}

type CandidateTool struct {
	Tool        string `json:"tool"`
	Name        string `json:"name"`
	MatchReason string `json:"match_reason"`
}

type LlmToolsResponse struct {
	Platforms map[string]PlatformTools `json:"platforms"`
}

type PlatformTools struct {
	Connected bool           `json:"connected"`
	Tools     []AvailableTool `json:"tools,omitempty"`
}

type AvailableTool struct {
	Slug           string   `json:"slug"`
	Name           string   `json:"name"`
	Description    string   `json:"description"`
	ExampleQueries []string `json:"example_queries"`
	Scopes         []string `json:"scopes"`
	ScopeSatisfied string   `json:"scope_satisfied"`
}