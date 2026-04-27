// Package ominiconnect provides a Go client for the OminiConnect API.
//
// Example:
//
//	package main
//
//	import (
//		"fmt"
//		"log"
//
//		"github.com/ominiconnect/go-sdk/ominiconnect"
//	)
//
//	func main() {
//		client := ominiconnect.New("sk-xxxxx", nil)
//
//		// List connected platforms
//		connectors, err := client.Connectors.List()
//		if err != nil {
//			log.Fatal(err)
//		}
//		fmt.Println(connectors)
//
//		// Call GitHub API directly — Maton style
//		user, err := client.Call("github", "GET", "/user", nil, nil)
//		if err != nil {
//			log.Fatal(err)
//		}
//		fmt.Println(user)
//
//		// Create a named API key
//		key, err := client.ApiKeys.Create("pr-reviewer-agent")
//		if err != nil {
//			log.Fatal(err)
//		}
//		fmt.Println(key.Key) // shown once — store securely
//	}
package ominiconnect

import (
	"bytes"
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"net/http"
	"net/url"
	"time"
)

// Base URL for the OminiConnect portal.
const DefaultBaseURL = "http://localhost:9000"

// Client is the main OminiConnect API client.
type Client struct {
	httpClient *http.Client
	baseURL    string
	apiKey     string

	Connectors *ConnectorsManager
	Tools      *ToolsManager
	ApiKeys    *ApiKeysManager
	Llm        *LlmManager
}

// New creates a new OminiConnect client.
// Pass options nil for defaults (30s timeout, localhost:9000).
func New(apiKey string, opts *Options) *Client {
	baseURL := DefaultBaseURL
	timeout := 30 * time.Second

	if opts != nil {
		if opts.BaseURL != "" {
			baseURL = opts.BaseURL
		}
		if opts.Timeout > 0 {
			timeout = opts.Timeout
		}
	}

	c := &Client{
		httpClient: &http.Client{Timeout: timeout},
		baseURL:    baseURL,
		apiKey:     apiKey,
	}

	c.Connectors = &ConnectorsManager{client: c}
	c.Tools = &ToolsManager{client: c}
	c.ApiKeys = &ApiKeysManager{client: c}
	c.Llm = &LlmManager{client: c}

	return c
}

// Options for New.
type Options struct {
	// BaseURL defaults to http://localhost:9000.
	BaseURL string
	// Timeout for each request (default 30s).
	Timeout time.Duration
}

// ─── HTTP helpers ─────────────────────────────────────────────────────────────

func (c *Client) do(ctx context.Context, method, path string, query url.Values, body any) (*json.RawMessage, error) {
	u, err := url.Parse(c.baseURL + path)
	if err != nil {
		return nil, fmt.Errorf("invalid URL: %w", err)
	}
	if query != nil {
		u.RawQuery = query.Encode()
	}

	var bodyReader io.Reader
	if body != nil {
		b, err := json.Marshal(body)
		if err != nil {
			return nil, fmt.Errorf("encode body: %w", err)
		}
		bodyReader = bytes.NewReader(b)
	}

	req, err := http.NewRequestWithContext(ctx, method, u.String(), bodyReader)
	if err != nil {
		return nil, fmt.Errorf("create request: %w", err)
	}

	req.Header.Set("Authorization", "Bearer "+c.apiKey)
	req.Header.Set("Content-Type", "application/json")

	resp, err := c.httpClient.Do(req)
	if err != nil {
		return nil, &NetError{Err: err}
	}
	defer resp.Body.Close()

	if resp.StatusCode == http.StatusUnauthorized {
		return nil, ErrAuth
	}
	if resp.StatusCode == http.StatusTooManyRequests {
		return nil, ErrRateLimited
	}
	if resp.StatusCode == http.StatusNotFound {
		return nil, &NotFoundError{Path: path}
	}

	if resp.StatusCode >= 400 {
		var e struct{ Error string }
		if dec := json.NewDecoder(resp.Body).Decode(&e); dec == nil && e.Error != "" {
			return nil, &UpstreamError{StatusCode: resp.StatusCode, Message: e.Error}
		}
		return nil, &UpstreamError{StatusCode: resp.StatusCode, Message: resp.Status}
	}

	if resp.ContentLength == 0 {
		return nil, nil
	}

	var out json.RawMessage
	if err := json.NewDecoder(resp.Body).Decode(&out); err != nil {
		return nil, fmt.Errorf("decode response: %w", err)
	}
	return &out, nil
}

// ─── Call (Maton-style gateway) ───────────────────────────────────────────────

// Call calls a connected platform's API directly — Maton style.
func (c *Client) Call(ctx context.Context, platform, method, path string, params map[string]string, body any) (json.RawMessage, error) {
	payload := callRequest{
		Method: method,
		Path:   path,
	}
	if params != nil {
		payload.Params = params
	}
	if body != nil {
		payload.Body = body
	}

	resp, err := c.do(ctx, http.MethodPost, "/api/call/"+platform, nil, payload)
	if err != nil {
		return nil, err
	}
	if resp == nil {
		return nil, nil
	}
	return *resp, nil
}

type callRequest struct {
	Method string                 `json:"method"`
	Path   string                 `json:"path"`
	Params map[string]string      `json:"params,omitempty"`
	Body   any                    `json:"body,omitempty"`
}

// ─── Connectors ───────────────────────────────────────────────────────────────

type ConnectorsManager struct {
	client *Client
}

// List returns all connected platforms.
func (m *ConnectorsManager) List() ([]Connector, error) {
	resp, err := m.client.do(context.Background(), http.MethodGet, "/api/connectors", nil, nil)
	if err != nil {
		return nil, err
	}
	if resp == nil {
		return nil, nil
	}
	var out []Connector
	if err := json.Unmarshal(*resp, &out); err != nil {
		return nil, fmt.Errorf("decode connectors: %w", err)
	}
	return out, nil
}

// Delete removes a connected platform.
func (m *ConnectorsManager) Delete(platform string) error {
	_, err := m.client.do(context.Background(), http.MethodDelete, "/api/connectors/"+platform, nil, nil)
	return err
}

// ─── Tools ───────────────────────────────────────────────────────────────────

type ToolsManager struct {
	client *Client
}

// List returns available tools grouped by toolkit.
func (m *ToolsManager) List(ctx context.Context, platform string) (*ToolkitsResponse, error) {
	query := url.Values{}
	if platform != "" {
		query.Set("platform", platform)
	}
	resp, err := m.client.do(ctx, http.MethodGet, "/api/tools", query, nil)
	if err != nil {
		return nil, err
	}
	if resp == nil {
		return nil, nil
	}
	var out ToolkitsResponse
	if err := json.Unmarshal(*resp, &out); err != nil {
		return nil, fmt.Errorf("decode tools: %w", err)
	}
	return &out, nil
}

// Search searches tools by query string.
func (m *ToolsManager) Search(ctx context.Context, q, platform string) (*ToolsSearchResponse, error) {
	query := url.Values{"q": []string{q}}
	if platform != "" {
		query.Set("platform", platform)
	}
	resp, err := m.client.do(ctx, http.MethodGet, "/api/tools/search", query, nil)
	if err != nil {
		return nil, err
	}
	if resp == nil {
		return nil, nil
	}
	var out ToolsSearchResponse
	if err := json.Unmarshal(*resp, &out); err != nil {
		return nil, fmt.Errorf("decode search: %w", err)
	}
	return &out, nil
}

// Execute runs a tool by slug.
func (m *ToolsManager) Execute(ctx context.Context, toolSlug string, args map[string]any, callbackURL string) (*ToolExecuteResult, error) {
	payload := executeRequest{
		ToolSlug:   toolSlug,
		Arguments: args,
	}
	if callbackURL != "" {
		payload.CallbackURL = callbackURL
	}
	resp, err := m.client.do(ctx, http.MethodPost, "/api/tools/execute", nil, payload)
	if err != nil {
		return nil, err
	}
	if resp == nil {
		return nil, nil
	}
	var out ToolExecuteResult
	if err := json.Unmarshal(*resp, &out); err != nil {
		return nil, fmt.Errorf("decode result: %w", err)
	}
	return &out, nil
}

type executeRequest struct {
	ToolSlug    string         `json:"tool_slug"`
	Arguments  map[string]any  `json:"arguments,omitempty"`
	CallbackURL string         `json:"callback_url,omitempty"`
}

// ─── API Keys ────────────────────────────────────────────────────────────────

type ApiKeysManager struct {
	client *Client
}

// Create makes a new named API key. The raw key is returned ONLY here.
func (m *ApiKeysManager) Create(label string) (*ApiKeyCreated, error) {
	resp, err := m.client.do(context.Background(), http.MethodPost, "/auth/apikey", nil, map[string]string{"label": label})
	if err != nil {
		return nil, err
	}
	if resp == nil {
		return nil, nil
	}
	var out ApiKeyCreated
	if err := json.Unmarshal(*resp, &out); err != nil {
		return nil, fmt.Errorf("decode key: %w", err)
	}
	return &out, nil
}

// List returns all API keys (raw key is never returned).
func (m *ApiKeysManager) List() ([]ApiKeySummary, error) {
	resp, err := m.client.do(context.Background(), http.MethodGet, "/auth/apikey", nil, nil)
	if err != nil {
		return nil, err
	}
	if resp == nil {
		return nil, nil
	}
	var out []ApiKeySummary
	if err := json.Unmarshal(*resp, &out); err != nil {
		return nil, fmt.Errorf("decode keys: %w", err)
	}
	return out, nil
}

// Delete revokes an API key.
func (m *ApiKeysManager) Delete(keyHash string) error {
	_, err := m.client.do(context.Background(), http.MethodDelete, "/auth/apikey/"+keyHash, nil, nil)
	return err
}

// ─── Types ─────────────────────────────────────────────────────────────────────

type Connector struct {
	Platform  string   `json:"platform"`
	Enabled   bool     `json:"enabled"`
	Scopes    []string `json:"scopes"`
	CreatedAt string   `json:"created_at"`
}

type ToolkitsResponse struct {
	Toolkits []Toolkit `json:"toolkits"`
}

type Toolkit struct {
	Slug    string        `json:"slug"`
	Name    string        `json:"name"`
	Logo    *string       `json:"logo"`
	Provider string       `json:"provider"`
	Tools   []ToolSummary `json:"tools"`
}

type ToolSummary struct {
	Slug           string   `json:"slug"`
	Name           string   `json:"name"`
	Description    string   `json:"description"`
	Method         string   `json:"method"`
	Endpoint       string   `json:"endpoint"`
	Scopes         []string `json:"scopes"`
	ScopeSatisfied string   `json:"scope_satisfied"`
	Tags           []string `json:"tags"`
}

type ToolsSearchResponse struct {
	Tools []ToolSummary `json:"tools"`
	Query string        `json:"query"`
}

type ToolExecuteResult struct {
	OK         bool            `json:"ok"`
	Body       json.RawMessage `json:"body,omitempty"`
	Error      string          `json:"error,omitempty"`
	CallID     string          `json:"call_id,omitempty"`
	Status     string          `json:"status,omitempty"`
	DurationMs int            `json:"duration_ms,omitempty"`
}

type ApiKeyCreated struct {
	Key       string `json:"key"`
	Label     string `json:"label"`
	CreatedAt string `json:"created_at"`
}

type ApiKeySummary struct {
	KeyHash   string `json:"key_hash"`
	Label     string `json:"label"`
	CreatedAt string `json:"created_at"`
}

// ─── Errors ─────────────────────────────────────────────────────────────────

var (
	// ErrAuth is returned for invalid or missing API keys.
	ErrAuth = errors.New("ominiconnect: authentication failed")
	// ErrRateLimited is returned when rate limited.
	ErrRateLimited = errors.New("ominiconnect: rate limited")
)

// NetError wraps network failures.
type NetError struct {
	Err error
}

func (e *NetError) Error() string { return "ominiconnect: network error: " + e.Err.Error() }
func (e *NetError) Unwrap() error { return e.Err }

// UpstreamError is returned when the platform API returns a non-2xx.
type UpstreamError struct {
	StatusCode int
	Message    string
}

func (e *UpstreamError) Error() string {
	return fmt.Sprintf("ominiconnect: upstream %d: %s", e.StatusCode, e.Message)
}

// NotFoundError is returned when a resource is not found.
type NotFoundError struct {
	Path string
}

func (e *NotFoundError) Error() string {
	return fmt.Sprintf("ominiconnect: not found: %s", e.Path)
}
