package ominiconnect

import (
	"context"
	"encoding/json"
	"errors"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"
)

// ─── TestClientNew ─────────────────────────────────────────────────────────────

func TestClientNew(t *testing.T) {
	// Default base URL and timeout
	c := New("sk-test", nil)
	if c.baseURL != DefaultBaseURL {
		t.Errorf("expected default base URL %q, got %q", DefaultBaseURL, c.baseURL)
	}
	if c.httpClient.Timeout != 30*time.Second {
		t.Errorf("expected default timeout 30s, got %v", c.httpClient.Timeout)
	}

	// Custom base URL
	customBase := "http://custom:8080"
	c = New("sk-test", &Options{BaseURL: customBase})
	if c.baseURL != customBase {
		t.Errorf("expected custom base URL %q, got %q", customBase, c.baseURL)
	}

	// Custom timeout
	customTimeout := 5 * time.Second
	c = New("sk-test", &Options{Timeout: customTimeout})
	if c.httpClient.Timeout != customTimeout {
		t.Errorf("expected custom timeout 5s, got %v", c.httpClient.Timeout)
	}
}

// ─── TestClientCall ───────────────────────────────────────────────────────────

func TestClientCall(t *testing.T) {
	var receivedReq http.Request
	srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		receivedReq = *r
		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(map[string]string{"result": "ok"})
	}))
	defer srv.Close()

	c := New("sk-test", &Options{BaseURL: srv.URL})

	resp, err := c.Call(context.Background(), "github", "GET", "/user/repos", map[string]string{"per_page": "10"}, nil)
	if err != nil {
		t.Fatalf("Call failed: %v", err)
	}

	if receivedReq.Method != http.MethodPost {
		t.Errorf("expected POST, got %s", receivedReq.Method)
	}
	if receivedReq.URL.Path != "/api/call/github" {
		t.Errorf("expected path /api/call/github, got %s", receivedReq.URL.Path)
	}
	if receivedReq.Header.Get("Authorization") != "Bearer sk-test" {
		t.Errorf("expected Authorization header, got %q", receivedReq.Header.Get("Authorization"))
	}

	var body map[string]any
	json.Unmarshal(resp, &body)
	if body["result"] != "ok" {
		t.Errorf("unexpected response body: %s", resp)
	}
}

// ─── TestConnectorsManagerList ────────────────────────────────────────────────

func TestConnectorsManagerList(t *testing.T) {
	srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/api/connectors" {
			t.Errorf("unexpected path %s", r.URL.Path)
		}
		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode([]Connector{
			{Platform: "github", Enabled: true, Scopes: []string{"repo", "user"}, CreatedAt: "2026-01-01T00:00:00Z"},
			{Platform: "slack", Enabled: false, Scopes: []string{"chat:write"}, CreatedAt: "2026-01-02T00:00:00Z"},
		})
	}))
	defer srv.Close()

	c := New("sk-test", &Options{BaseURL: srv.URL})
	connectors, err := c.Connectors.List()
	if err != nil {
		t.Fatalf("List failed: %v", err)
	}
	if len(connectors) != 2 {
		t.Errorf("expected 2 connectors, got %d", len(connectors))
	}
	if connectors[0].Platform != "github" {
		t.Errorf("expected platform github, got %s", connectors[0].Platform)
	}
	if !connectors[0].Enabled {
		t.Error("expected github to be enabled")
	}
	if connectors[1].Enabled {
		t.Error("expected slack to be disabled")
	}
}

// ─── TestConnectorsManagerDelete ─────────────────────────────────────────────

func TestConnectorsManagerDelete(t *testing.T) {
	srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/api/connectors/github" {
			t.Errorf("unexpected path %s", r.URL.Path)
		}
		if r.Method != http.MethodDelete {
			t.Errorf("expected DELETE, got %s", r.Method)
		}
		w.WriteHeader(http.StatusOK)
	}))
	defer srv.Close()

	c := New("sk-test", &Options{BaseURL: srv.URL})
	err := c.Connectors.Delete("github")
	if err != nil {
		t.Errorf("Delete failed: %v", err)
	}
}

// ─── TestToolsManagerList ─────────────────────────────────────────────────────

func TestToolsManagerList(t *testing.T) {
	srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/api/tools" {
			t.Errorf("unexpected path %s", r.URL.Path)
		}
		// Verify query param
		if r.URL.Query().Get("platform") != "github" {
			t.Errorf("expected platform=github query param, got %s", r.URL.Query().Get("platform"))
		}
		w.Header().Set("Content-Type", "application/json")
		resp := ToolkitsResponse{
			Toolkits: []Toolkit{
				{
					Slug:     "github",
					Name:     "GitHub",
					Provider: "github",
					Tools: []ToolSummary{
						{Slug: "github_list_repos", Name: "List Repositories", Description: "List repos", Method: "GET", Endpoint: "/user/repos", ScopeSatisfied: "yes"},
					},
				},
			},
		}
		json.NewEncoder(w).Encode(resp)
	}))
	defer srv.Close()

	c := New("sk-test", &Options{BaseURL: srv.URL})
	result, err := c.Tools.List(context.Background(), "github")
	if err != nil {
		t.Fatalf("List failed: %v", err)
	}
	if len(result.Toolkits) != 1 {
		t.Errorf("expected 1 toolkit, got %d", len(result.Toolkits))
	}
	if result.Toolkits[0].Slug != "github" {
		t.Errorf("expected toolkit slug github, got %s", result.Toolkits[0].Slug)
	}
	if len(result.Toolkits[0].Tools) != 1 {
		t.Errorf("expected 1 tool, got %d", len(result.Toolkits[0].Tools))
	}
	if result.Toolkits[0].Tools[0].Slug != "github_list_repos" {
		t.Errorf("expected tool slug github_list_repos, got %s", result.Toolkits[0].Tools[0].Slug)
	}
}

// ─── TestToolsManagerSearch ───────────────────────────────────────────────────

func TestToolsManagerSearch(t *testing.T) {
	srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/api/tools/search" {
			t.Errorf("unexpected path %s", r.URL.Path)
		}
		q := r.URL.Query().Get("q")
		if q != "list repos" {
			t.Errorf("expected q=list repos, got %s", q)
		}
		w.Header().Set("Content-Type", "application/json")
		resp := ToolsSearchResponse{
			Query: q,
			Tools: []ToolSummary{
				{Slug: "github_list_repos", Name: "List Repositories"},
			},
		}
		json.NewEncoder(w).Encode(resp)
	}))
	defer srv.Close()

	c := New("sk-test", &Options{BaseURL: srv.URL})
	result, err := c.Tools.Search(context.Background(), "list repos", "")
	if err != nil {
		t.Fatalf("Search failed: %v", err)
	}
	if result.Query != "list repos" {
		t.Errorf("expected query echo 'list repos', got %s", result.Query)
	}
	if len(result.Tools) != 1 {
		t.Errorf("expected 1 tool, got %d", len(result.Tools))
	}
}

// ─── TestToolsManagerExecute ──────────────────────────────────────────────────

func TestToolsManagerExecute(t *testing.T) {
	srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/api/tools/execute" {
			t.Errorf("unexpected path %s", r.URL.Path)
		}
		if r.Method != http.MethodPost {
			t.Errorf("expected POST, got %s", r.Method)
		}
		var body executeRequest
		json.NewDecoder(r.Body).Decode(&body)
		if body.ToolSlug != "github_list_repos" {
			t.Errorf("expected tool_slug github_list_repos, got %s", body.ToolSlug)
		}
		if body.Arguments["per_page"] != float64(10) {
			t.Errorf("expected arguments per_page=10, got %v", body.Arguments["per_page"])
		}
		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(ToolExecuteResult{OK: true, CallID: "call-123"})
	}))
	defer srv.Close()

	c := New("sk-test", &Options{BaseURL: srv.URL})
	result, err := c.Tools.Execute(context.Background(), "github_list_repos", map[string]any{"per_page": 10}, "")
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}
	if !result.OK {
		t.Error("expected ok=true")
	}
	if result.CallID != "call-123" {
		t.Errorf("expected call_id call-123, got %s", result.CallID)
	}
}

// ─── TestApiKeysManagerCreate ─────────────────────────────────────────────────

func TestApiKeysManagerCreate(t *testing.T) {
	srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/auth/apikey" {
			t.Errorf("unexpected path %s", r.URL.Path)
		}
		if r.Method != http.MethodPost {
			t.Errorf("expected POST, got %s", r.Method)
		}
		var body map[string]string
		json.NewDecoder(r.Body).Decode(&body)
		if body["label"] != "pr-reviewer-agent" {
			t.Errorf("expected label pr-reviewer-agent, got %s", body["label"])
		}
		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(ApiKeyCreated{
			Key:       "sk-raw-key-12345",
			Label:     "pr-reviewer-agent",
			CreatedAt: "2026-01-01T00:00:00Z",
		})
	}))
	defer srv.Close()

	c := New("sk-test", &Options{BaseURL: srv.URL})
	result, err := c.ApiKeys.Create("pr-reviewer-agent")
	if err != nil {
		t.Fatalf("Create failed: %v", err)
	}
	if result.Key != "sk-raw-key-12345" {
		t.Errorf("expected raw key sk-raw-key-12345, got %s", result.Key)
	}
	if result.Label != "pr-reviewer-agent" {
		t.Errorf("expected label pr-reviewer-agent, got %s", result.Label)
	}
}

// ─── TestApiKeysManagerList ───────────────────────────────────────────────────

func TestApiKeysManagerList(t *testing.T) {
	srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/auth/apikey" {
			t.Errorf("unexpected path %s", r.URL.Path)
		}
		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode([]ApiKeySummary{
			{KeyHash: "hash1", Label: "key1", CreatedAt: "2026-01-01T00:00:00Z"},
			{KeyHash: "hash2", Label: "key2", CreatedAt: "2026-01-02T00:00:00Z"},
		})
	}))
	defer srv.Close()

	c := New("sk-test", &Options{BaseURL: srv.URL})
	keys, err := c.ApiKeys.List()
	if err != nil {
		t.Fatalf("List failed: %v", err)
	}
	if len(keys) != 2 {
		t.Errorf("expected 2 keys, got %d", len(keys))
	}
	// Verify key_hash is present (raw key is never returned per ApiKeySummary struct)
	for _, k := range keys {
		if k.KeyHash == "" {
			t.Error("key_hash should be present")
		}
	}
}

// ─── TestApiKeysManagerDelete ─────────────────────────────────────────────────

func TestApiKeysManagerDelete(t *testing.T) {
	srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/auth/apikey/hash1" {
			t.Errorf("unexpected path %s", r.URL.Path)
		}
		if r.Method != http.MethodDelete {
			t.Errorf("expected DELETE, got %s", r.Method)
		}
		w.WriteHeader(http.StatusOK)
	}))
	defer srv.Close()

	c := New("sk-test", &Options{BaseURL: srv.URL})
	err := c.ApiKeys.Delete("hash1")
	if err != nil {
		t.Errorf("Delete failed: %v", err)
	}
}

// ─── TestErrors ───────────────────────────────────────────────────────────────

func TestErrors(t *testing.T) {
	// ErrAuth
	if ErrAuth.Error() != "ominiconnect: authentication failed" {
		t.Errorf("unexpected ErrAuth message: %s", ErrAuth.Error())
	}

	// ErrRateLimited
	if ErrRateLimited.Error() != "ominiconnect: rate limited" {
		t.Errorf("unexpected ErrRateLimited message: %s", ErrRateLimited.Error())
	}

	// UpstreamError
	upstreamErr := &UpstreamError{StatusCode: 502, Message: "Bad Gateway"}
	if !errors.Is(upstreamErr, upstreamErr) {
		t.Error("UpstreamError should unwrap to itself")
	}
	if upstreamErr.Error() != "ominiconnect: upstream 502: Bad Gateway" {
		t.Errorf("unexpected UpstreamError message: %s", upstreamErr.Error())
	}

	// NotFoundError
	notFoundErr := &NotFoundError{Path: "/api/tools"}
	if notFoundErr.Error() != "ominiconnect: not found: /api/tools" {
		t.Errorf("unexpected NotFoundError message: %s", notFoundErr.Error())
	}

	// NetError
	netErr := &NetError{Err: errors.New("connection refused")}
	if !errors.Is(netErr, netErr.Err) {
		t.Error("NetError should unwrap to underlying error")
	}
	if netErr.Error() != "ominiconnect: network error: connection refused" {
		t.Errorf("unexpected NetError message: %s", netErr.Error())
	}
}

// ─── TestUpstreamErrorFormat ──────────────────────────────────────────────────

func TestUpstreamErrorFormat(t *testing.T) {
	tests := []struct {
		code    int
		msg     string
		expect  string
	}{
		{400, "Bad Request", "ominiconnect: upstream 400: Bad Request"},
		{401, "Unauthorized", "ominiconnect: upstream 401: Unauthorized"},
		{403, "Forbidden", "ominiconnect: upstream 403: Forbidden"},
		{500, "Internal Server Error", "ominiconnect: upstream 500: Internal Server Error"},
		{503, "Service Unavailable", "ominiconnect: upstream 503: Service Unavailable"},
	}

	for _, tt := range tests {
		err := &UpstreamError{StatusCode: tt.code, Message: tt.msg}
		if err.Error() != tt.expect {
			t.Errorf("UpstreamError{%d, %q}: expected %q, got %q", tt.code, tt.msg, tt.expect, err.Error())
		}
	}
}

// ─── TestErrorTypesFromResponse ──────────────────────────────────────────────

func TestErrorTypesFromResponse(t *testing.T) {
	// 401 returns ErrAuth
	srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusUnauthorized)
	}))
	defer srv.Close()

	c := New("sk-test", &Options{BaseURL: srv.URL})
	_, err := c.Connectors.List()
	if !errors.Is(err, ErrAuth) {
		t.Errorf("expected ErrAuth, got %v", err)
	}

	// 429 returns ErrRateLimited
	srv2 := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusTooManyRequests)
	}))
	defer srv2.Close()

	c2 := New("sk-test", &Options{BaseURL: srv2.URL})
	_, err = c2.Connectors.List()
	if !errors.Is(err, ErrRateLimited) {
		t.Errorf("expected ErrRateLimited, got %v", err)
	}

	// 404 returns NotFoundError
	srv3 := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusNotFound)
	}))
	defer srv3.Close()

	c3 := New("sk-test", &Options{BaseURL: srv3.URL})
	_, err = c3.Connectors.List()
	var notFoundErr *NotFoundError
	if !errors.As(err, &notFoundErr) {
		t.Errorf("expected NotFoundError, got %v", err)
	}

	// 400 returns UpstreamError with JSON error field
	srv4 := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusBadRequest)
		json.NewEncoder(w).Encode(map[string]string{"error": "invalid request"})
	}))
	defer srv4.Close()

	c4 := New("sk-test", &Options{BaseURL: srv4.URL})
	_, err = c4.Connectors.List()
	var upstreamErr *UpstreamError
	if !errors.As(err, &upstreamErr) {
		t.Errorf("expected UpstreamError, got %v", err)
	}
	if upstreamErr.Message != "invalid request" {
		t.Errorf("expected message 'invalid request', got %q", upstreamErr.Message)
	}
}

// ─── TestClientCallWithBody ───────────────────────────────────────────────────

func TestClientCallWithBody(t *testing.T) {
	srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		var body callRequest
		json.NewDecoder(r.Body).Decode(&body)
		if body.Method != "POST" {
			t.Errorf("expected method POST, got %s", body.Method)
		}
		if body.Path != "/repos" {
			t.Errorf("expected path /repos, got %s", body.Path)
		}
		if body.Body == nil {
			t.Error("expected body to be set")
		}
		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(map[string]any{"created": true})
	}))
	defer srv.Close()

	c := New("sk-test", &Options{BaseURL: srv.URL})
	resp, err := c.Call(context.Background(), "github", "POST", "/repos", nil, map[string]string{"name": "test-repo"})
	if err != nil {
		t.Fatalf("Call failed: %v", err)
	}
	var result map[string]any
	json.Unmarshal(resp, &result)
	if !result["created"].(bool) {
		t.Error("expected created=true in response")
	}
}

// ─── TestEmptyResponse ───────────────────────────────────────────────────────

func TestEmptyResponse(t *testing.T) {
	srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusNoContent)
	}))
	defer srv.Close()

	c := New("sk-test", &Options{BaseURL: srv.URL})
	_, err := c.Connectors.List()
	if err != nil {
		t.Errorf("unexpected error on 204: %v", err)
	}
}

// ─── TestLlmManagerExecute ───────────────────────────────────────────────────

func TestLlmManagerExecute(t *testing.T) {
	srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/api/llm" {
			t.Errorf("unexpected path %s", r.URL.Path)
		}
		if r.Method != http.MethodPost {
			t.Errorf("expected POST, got %s", r.Method)
		}
		var body map[string]any
		json.NewDecoder(r.Body).Decode(&body)
		if body["query"] != "list my github repos" {
			t.Errorf("expected query 'list my github repos', got %v", body["query"])
		}
		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(LlmExecuteResponse{
			OK:          true,
			Tool:        "github_list_repos",
			ToolName:    "List Repositories",
			Explanation: "Routing to github_list_repos",
		})
	}))
	defer srv.Close()

	c := New("sk-test", &Options{BaseURL: srv.URL})
	result, err := c.Llm.Execute(context.Background(), "list my github repos", nil)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}
	if !result.OK {
		t.Error("expected ok=true")
	}
	if result.Tool != "github_list_repos" {
		t.Errorf("expected tool github_list_repos, got %s", result.Tool)
	}
}

func TestLlmManagerExecuteWithPlatform(t *testing.T) {
	srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		var body map[string]any
		json.NewDecoder(r.Body).Decode(&body)
		if body["platform"] != "github" {
			t.Errorf("expected platform github, got %v", body["platform"])
		}
		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(LlmExecuteResponse{OK: true, Tool: "github_list_repos"})
	}))
	defer srv.Close()

	c := New("sk-test", &Options{BaseURL: srv.URL})
	platform := "github"
	result, err := c.Llm.Execute(context.Background(), "list repos", &platform)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}
	if result.Tool != "github_list_repos" {
		t.Errorf("expected tool github_list_repos, got %s", result.Tool)
	}
}

func TestLlmManagerExecuteAmbiguous(t *testing.T) {
	srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(LlmExecuteResponse{
			OK:       false,
			Message:  "Ambiguous query — multiple tools matched",
			Candidates: []CandidateTool{
				{Tool: "github_list_repos", Name: "List Repositories", MatchReason: "keyword match"},
				{Tool: "github_list_issues", Name: "List Issues", MatchReason: "keyword match"},
			},
		})
	}))
	defer srv.Close()

	c := New("sk-test", &Options{BaseURL: srv.URL})
	result, err := c.Llm.Execute(context.Background(), "list my stuff", nil)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}
	if result.OK {
		t.Error("expected ok=false for ambiguous query")
	}
	if len(result.Candidates) != 2 {
		t.Errorf("expected 2 candidates, got %d", len(result.Candidates))
	}
}

func TestLlmManagerListAvailableTools(t *testing.T) {
	srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/api/llm/tools" {
			t.Errorf("unexpected path %s", r.URL.Path)
		}
		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(LlmToolsResponse{
			Platforms: map[string]PlatformTools{
				"github": {
					Connected: true,
					Tools: []AvailableTool{
						{Slug: "github_list_repos", Name: "List Repositories", ExampleQueries: []string{"list my repos"}, ScopeSatisfied: "yes"},
					},
				},
				"slack": {
					Connected: false,
				},
			},
		})
	}))
	defer srv.Close()

	c := New("sk-test", &Options{BaseURL: srv.URL})
	result, err := c.Llm.ListAvailableTools(context.Background(), nil)
	if err != nil {
		t.Fatalf("ListAvailableTools failed: %v", err)
	}
	github, ok := result.Platforms["github"]
	if !ok {
		t.Fatal("expected github platform")
	}
	if !github.Connected {
		t.Error("expected github to be connected")
	}
	if len(github.Tools) != 1 {
		t.Errorf("expected 1 tool, got %d", len(github.Tools))
	}
	if github.Tools[0].Slug != "github_list_repos" {
		t.Errorf("expected slug github_list_repos, got %s", github.Tools[0].Slug)
	}
	slack := result.Platforms["slack"]
	if slack.Connected {
		t.Error("expected slack to not be connected")
	}
}

func TestLlmManagerListAvailableToolsWithPlatform(t *testing.T) {
	srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Query().Get("platform") != "github" {
			t.Errorf("expected platform=github query, got %s", r.URL.Query().Get("platform"))
		}
		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(LlmToolsResponse{
			Platforms: map[string]PlatformTools{
				"github": {Connected: true, Tools: []AvailableTool{}},
			},
		})
	}))
	defer srv.Close()

	c := New("sk-test", &Options{BaseURL: srv.URL})
	platform := "github"
	_, err := c.Llm.ListAvailableTools(context.Background(), &platform)
	if err != nil {
		t.Fatalf("ListAvailableTools failed: %v", err)
	}
}

// ─── TestQueryParamsPreserved ─────────────────────────────────────────────────

func TestQueryParamsPreserved(t *testing.T) {
	srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Type", "application/json")
		var resp ToolsSearchResponse
		resp.Query = r.URL.Query().Get("q")
		resp.Tools = []ToolSummary{}
		json.NewEncoder(w).Encode(resp)
	}))
	defer srv.Close()

	c := New("sk-test", &Options{BaseURL: srv.URL})
	result, err := c.Tools.Search(context.Background(), "repo", "github")
	if err != nil {
		t.Fatalf("Search failed: %v", err)
	}
	if result.Query != "repo" {
		t.Errorf("expected query 'repo', got %s", result.Query)
	}
}
