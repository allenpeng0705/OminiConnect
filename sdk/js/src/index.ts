/**
 * OminiConnect JavaScript/TypeScript SDK.
 */

const DEFAULT_BASE_URL = "http://localhost:9000";

// ─── Error classes ────────────────────────────────────────────────────────────────

export class OminiConnectError extends Error {}

export class AuthError extends OminiConnectError {}

export class ConnectorNotFoundError extends OminiConnectError {}

export class ToolNotFoundError extends OminiConnectError {}

export class ScopeInsufficientError extends OminiConnectError {}

export class UpstreamError extends OminiConnectError {
  constructor(
    public readonly statusCode: number,
    public readonly body: string,
  ) {
    super(`upstream error ${statusCode}: ${body.slice(0, 200)}`);
  }
}

export class NetworkError extends OminiConnectError {}

export class RateLimitedError extends OminiConnectError {}

// ─── Shared types ───────────────────────────────────────────────────────────────

export interface CallOptions {
  params?: Record<string, string | number | boolean>;
  body?: unknown;
}

export interface ToolSummary {
  slug: string;
  name: string;
  description: string;
  method: string;
  endpoint: string;
  scopes: string[];
  scope_satisfied: "yes" | "no" | "unknown";
  tags: string[];
}

export interface Toolkit {
  slug: string;
  name: string;
  logo: string | null;
  provider: string;
  tools: ToolSummary[];
}

export interface ToolkitsResponse {
  toolkits: Toolkit[];
}

export interface ToolsSearchResponse {
  tools: ToolSummary[];
  query: string;
}

export interface ToolExecuteResult {
  ok: boolean;
  body?: unknown;
  error?: string;
  call_id?: string;
  status?: "pending";
  duration_ms?: number;
}

export interface Connector {
  platform: string;
  enabled: boolean;
  scopes: string[];
  created_at: string;
}

export interface ApiKeyCreated {
  key: string;
  label: string;
  created_at: string;
}

export interface ApiKeySummary {
  key_hash: string;
  label: string;
  created_at: string;
}

export interface McpTool {
  name: string;
  description: string;
  input_schema: Record<string, unknown>;
  scope_satisfied?: string;
}

// ─── LLM types ─────────────────────────────────────────────────────────────────

export interface LlmExecuteResponse {
  ok: boolean;
  tool?: string;
  tool_name?: string;
  arguments?: Record<string, unknown>;
  explanation?: string;
  result?: unknown;
  error?: string;
  message?: string;
  candidates?: Array<{tool: string; name: string; match_reason: string}>;
  available_tools_hint?: string;
}

export interface AvailableTool {
  slug: string;
  name: string;
  description: string;
  example_queries: string[];
  scopes: string[];
  scope_satisfied: string;
}

export interface PlatformTools {
  connected: boolean;
  tools?: AvailableTool[];
}

export interface LlmToolsResponse {
  platforms: Record<string, PlatformTools>;
}

// ─── Main client ───────────────────────────────────────────────────────────────

export interface OminiConnectOptions {
  apiKey: string;
  baseUrl?: string;
  fetchFn?: typeof fetch;
}

export class OminiConnect {
  private readonly apiKey: string;
  private readonly baseUrl: string;
  private readonly fetch: typeof fetch;

  /** Connectors manager — list and manage OAuth/API key connections. */
  readonly connectors: ConnectorsManager;

  /** API keys manager — create, list, revoke named API keys. */
  readonly apiKeys: ApiKeysManager;

  /** Tools manager — list, search, execute tools. */
  readonly tools: ToolsManager;

  /** LLM manager — query LLM with tool routing and list available tools. */
  readonly llm: LlmManager;

  constructor(options: OminiConnectOptions) {
    this.apiKey = options.apiKey;
    this.baseUrl = (options.baseUrl ?? DEFAULT_BASE_URL).replace(/\/$/, "");
    this.fetch = options.fetchFn ?? fetch;

    this.connectors = new ConnectorsManager(this);
    this.apiKeys = new ApiKeysManager(this);
    this.tools = new ToolsManager(this);
    this.llm = new LlmManager(this);
  }

  // ─── HTTP layer ────────────────────────────────────────────────────────────

  private async request<T>(
    method: "GET" | "POST" | "DELETE" | "PUT" | "PATCH",
    path: string,
    body?: unknown,
    params?: Record<string, string>,
  ): Promise<T> {
    let url = `${this.baseUrl}${path}`;
    if (params) {
      const qs = new URLSearchParams(
        Object.fromEntries(Object.entries(params).map(([k, v]) => [k, String(v)])),
      ).toString();
      url += `?${qs}`;
    }

    let resp: Response;
    try {
      resp = await this.fetch(url, {
        method,
        headers: {
          Authorization: `Bearer ${this.apiKey}`,
          "Content-Type": "application/json",
        },
        body: body !== undefined ? JSON.stringify(body) : undefined,
      });
    } catch (err) {
      throw new NetworkError();
    }

    if (resp.status === 401) throw new AuthError("invalid or missing API key");
    if (resp.status === 429) throw new RateLimitedError("rate limited — back off and retry");
    if (resp.status === 404) throw new ConnectorNotFoundError(`not found: ${path}`);

    if (!resp.ok) {
      const text = await resp.text().catch(() => "");
      throw new UpstreamError(resp.status, text);
    }

    if (resp.headers.get("content-length") === "0") return undefined as T;
    return resp.json() as Promise<T>;
  }

  get<T>(path: string, params?: Record<string, string>): Promise<T> {
    return this.request<T>("GET", path, undefined, params);
  }

  post<T>(path: string, body?: unknown): Promise<T> {
    return this.request<T>("POST", path, body);
  }

  delete<T>(path: string): Promise<T> {
    return this.request<T>("DELETE", path);
  }

  // ─── Maton-style direct call ──────────────────────────────────────────────

  /**
   * Call any connected platform's API directly — Maton style.
   *
   * This is the simplest way to use OminiConnect. No tool schemas needed.
   *
   * @example
   * const repos = await client.call("github", "GET", "/user/repos", { params: { sort: "updated" } });
   * await client.call("slack", "POST", "/api/chat.postMessage", { body: { channel: "C0123", text: "Hi!" } });
   */
  async call(
    platform: string,
    method: string,
    path: string,
    options?: CallOptions,
  ): Promise<unknown> {
    const payload: Record<string, unknown> = {
      method: method.toUpperCase(),
      path,
    };
    if (options?.params) payload.params = options.params;
    if (options?.body !== undefined) payload.body = options.body;
    return this.post(`/api/call/${platform}`, payload);
  }
}

// ─── Connectors ────────────────────────────────────────────────────────────────

export class ConnectorsManager {
  constructor(private client: OminiConnect) {}

  /** List all connected platforms. */
  list(): Promise<Connector[]> {
    return this.client.get<Connector[]>("/api/connectors");
  }

  /** Get a specific connector. */
  async get(platform: string): Promise<Connector> {
    try {
      return await this.client.get<Connector>(`/api/connectors/${platform}`);
    } catch {
      throw new ConnectorNotFoundError(`Platform '${platform}' is not connected`);
    }
  }

  /** Remove a connected platform. */
  delete(platform: string): Promise<{ ok: boolean }> {
    return this.client.delete<{ ok: boolean }>(`/api/connectors/${platform}`);
  }
}

// ─── API Keys ─────────────────────────────────────────────────────────────────

export class ApiKeysManager {
  constructor(private client: OminiConnect) {}

  /**
   * Create a new named API key.
   * The raw key is returned ONLY here — store it securely.
   */
  create(label: string): Promise<ApiKeyCreated> {
    return this.client.post<ApiKeyCreated>("/auth/apikey", { label });
  }

  /** List all API keys (raw key is never returned). */
  list(): Promise<ApiKeySummary[]> {
    return this.client.get<ApiKeySummary[]>("/auth/apikey");
  }

  /** Revoke an API key. */
  delete(keyHash: string): Promise<{ ok: boolean }> {
    return this.client.delete<{ ok: boolean }>(`/auth/apikey/${keyHash}`);
  }
}

// ─── Tools ───────────────────────────────────────────────────────────────────

export class ToolsManager {
  constructor(private client: OminiConnect) {}

  /**
   * List available tools, optionally filtered by platform.
   * Returns { toolkits: [...] } grouped by platform.
   */
  list(platform?: string): Promise<ToolkitsResponse> {
    return this.client.get<ToolkitsResponse>(
      "/api/tools",
      platform ? { platform } : undefined,
    );
  }

  /**
   * Search tools by name, description, or tags.
   */
  search(
    q: string,
    platform?: string,
    filterScope?: "yes" | "no" | "any",
  ): Promise<ToolsSearchResponse> {
    const params: Record<string, string> = { q };
    if (platform) params.platform = platform;
    if (filterScope) params.filter_scope = filterScope;
    return this.client.get<ToolsSearchResponse>("/api/tools/search", params);
  }

  /**
   * Execute a tool by slug with structured arguments.
   * Set callbackUrl for async execution (returns immediately).
   */
  execute(
    toolSlug: string,
    args?: Record<string, unknown>,
    callbackUrl?: string,
  ): Promise<ToolExecuteResult> {
    const body: Record<string, unknown> = {
      tool_slug: toolSlug,
      arguments: args ?? {},
    };
    if (callbackUrl) body.callback_url = callbackUrl;
    return this.client.post<ToolExecuteResult>("/api/tools/execute", body);
  }
}

// ─── LLM Manager ───────────────────────────────────────────────────────────────

export class LlmManager {
  constructor(private client: OminiConnect) {}

  /**
   * Execute an LLM query against a specific platform or across all connected platforms.
   * Returns structured execution response with tool candidates and execution results.
   */
  execute(query: string, platform?: string): Promise<LlmExecuteResponse> {
    const body: Record<string, unknown> = {query};
    if (platform) body.platform = platform;
    return this.client.post<LlmExecuteResponse>("/api/llm", body);
  }

  /**
   * List available tools for LLM use, optionally filtered by platform.
   * Returns platforms with their connection status and available tools.
   */
  listAvailableTools(platform?: string): Promise<LlmToolsResponse> {
    return this.client.get<LlmToolsResponse>(
      "/api/llm/tools",
      platform ? {platform} : undefined,
    );
  }
}

// Default export for convenience
export default OminiConnect;
