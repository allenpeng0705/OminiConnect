/**
 * OminiConnect JavaScript/TypeScript SDK.
 */

const DEFAULT_BASE_URL = "http://localhost:8080";

export interface OminiConnectOptions {
  apiKey: string;
  baseUrl?: string;
  fetchFn?: typeof fetch;
}

/**
 * Main SDK client for OminiConnect portal.
 */
export class OminiConnectClient {
  private readonly apiKey: string;
  private readonly baseUrl: string;
  private readonly fetch: typeof fetch;

  constructor(options: OminiConnectOptions) {
    this.apiKey = options.apiKey;
    this.baseUrl = (options.baseUrl ?? DEFAULT_BASE_URL).replace(/\/$/, "");
    this.fetch = options.fetchFn ?? fetch;
  }

  private async request<T>(
    method: "GET" | "POST" | "DELETE",
    path: string,
    body?: unknown,
  ): Promise<T> {
    const resp = await this.fetch(`${this.baseUrl}${path}`, {
      method,
      headers: {
        Authorization: `Bearer ${this.apiKey}`,
        "Content-Type": "application/json",
      },
      body: body !== undefined ? JSON.stringify(body) : undefined,
    });

    if (!resp.ok) {
      const text = await resp.text().catch(() => "");
      throw new Error(`OminiConnect API error ${resp.status}: ${text}`);
    }

    return resp.json() as Promise<T>;
  }

  get<T>(path: string, params?: Record<string, string>): Promise<T> {
    const qs = params ? "?" + new URLSearchParams(params).toString() : "";
    return this.request<T>("GET", path + qs);
  }

  post<T>(path: string, body?: unknown): Promise<T> {
    return this.request<T>("POST", path, body);
  }

  delete<T>(path: string): Promise<T> {
    return this.request<T>("DELETE", path);
  }

  readonly agents = new AgentManager(this);
  readonly tools = new ToolManager(this);
}

// ─── Agents ──────────────────────────────────────────────────────────────────

export interface AgentResponse {
  id: string;
  name: string;
  description: string;
  owner_username: string;
  active: boolean;
  created_at: string;
  api_key?: string; // only on registration
}

export interface AgentSummary {
  id: string;
  name: string;
  description: string;
  active: boolean;
  created_at: string;
}

class AgentManager {
  constructor(private client: OminiConnectClient) {}

  /** Register a new agent. Returns the agent with its raw API key (shown once). */
  register(name: string, description?: string): Promise<AgentResponse> {
    return this.client.post<AgentResponse>("/api/agents", {
      name,
      description,
    });
  }

  /** List all agents for the authenticated user. */
  list(): Promise<{ agents: AgentSummary[] }> {
    return this.client.get<{ agents: AgentSummary[] }>("/api/agents");
  }

  /** Get a specific agent by ID. */
  get(agentId: string): Promise<{ agent: AgentSummary }> {
    return this.client.get<{ agent: AgentSummary }>(`/api/agents/${agentId}`);
  }

  /** Delete an agent and revoke its API key. */
  delete(agentId: string): Promise<{ ok: boolean }> {
    return this.client.delete<{ ok: boolean }>(`/api/agents/${agentId}`);
  }

  /** Deactivate an agent. */
  deactivate(agentId: string): Promise<{ ok: boolean }> {
    return this.client.post<{ ok: boolean }>(`/api/agents/${agentId}/deactivate`);
  }
}

// ─── Tools ───────────────────────────────────────────────────────────────────

export interface Tool {
  slug: string;
  name: string;
  description: string;
  provider: string;
  input_schema: Record<string, unknown>;
  scopes: string[];
  tags?: string[];
  scope_satisfied?: "yes" | "no" | "unknown";
}

export interface ToolExecutionResult {
  ok: boolean;
  body?: unknown;
  error?: string;
  duration_ms?: number;
  call_id?: string; // async callback calls
}

class ToolManager {
  constructor(private client: OminiConnectClient) {}

  /** List available tools, optionally filtered by platform. */
  list(platform?: string): Promise<{ tools: Tool[] }> {
    return this.client.get<{ tools: Tool[] }>(
      "/api/tools",
      platform ? { platform } : undefined,
    );
  }

  /** Search tools by name/description/tags. */
  search(
    q: string,
    platform?: string,
  ): Promise<{ tools: Tool[] }> {
    return this.client.get<{ tools: Tool[] }>(
      "/api/tools/search",
      platform ? { q, platform } : { q },
    );
  }

  /** Execute a tool synchronously. */
  execute(
    toolSlug: string,
    args?: Record<string, unknown>,
    options?: {
      callbackUrl?: string;
      platform?: string;
    },
  ): Promise<ToolExecutionResult> {
    const body: Record<string, unknown> = {
      tool_slug: toolSlug,
      arguments: args ?? {},
    };
    if (options?.callbackUrl) body.callback_url = options.callbackUrl;
    if (options?.platform) body.platform = options.platform;

    return this.client.post<ToolExecutionResult>("/api/tools/execute", body);
  }
}

export default OminiConnectClient;