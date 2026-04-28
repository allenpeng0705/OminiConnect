/**
 * @ominiconnect/sdk-mcp - MCP (Model Context Protocol) Client SDK for OminiConnect
 *
 * This SDK implements the MCP protocol client, allowing AI models like Claude Desktop
 * to discover and execute tools through OminiConnect.
 *
 * Usage:
 * ```typescript
 * import { OminiConnectMCP } from '@ominiconnect/sdk-mcp';
 *
 * const client = new OminiConnectMCP({
 *   baseUrl: 'https://your-portal.com',
 *   apiKey: 'your-api-key'
 * });
 *
 * // List available tools
 * const tools = await client.listTools();
 *
 * // Call a tool
 * const result = await client.callTool('github_list_repos', {});
 * ```
 */

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

export interface MCPClientConfig {
  /** OminiConnect portal base URL */
  baseUrl: string;
  /** API key for authentication */
  apiKey: string;
  /** Optional timeout for requests in ms (default: 60000) */
  timeout?: number;
}

export interface Tool {
  name: string;
  description: string;
  input_schema: {
    type: 'object';
    properties: Record<string, unknown>;
    required?: string[];
  };
  scope_satisfied?: 'yes' | 'no' | 'unknown';
}

export interface ToolListResult {
  tools: Tool[];
}

export interface ToolCallParams {
  name: string;
  arguments?: Record<string, unknown>;
}

export interface ToolCallResult {
  // The result structure varies by tool - returns raw tool response
  [key: string]: unknown;
}

export interface MCPCallResult {
  call_id?: string;
  status?: string;
  result?: ToolCallResult;
  error?: string;
}

// MCP JSON-RPC 2.0 types
interface JSONRPCRequest {
  jsonrpc: '2.0';
  id: unknown;
  method: string;
  params?: unknown;
}

interface JSONRPCSuccessResponse {
  jsonrpc: '2.0';
  id: unknown;
  result: unknown;
}

interface JSONRPCErrorResponse {
  jsonrpc: '2.0';
  id: unknown;
  error: {
    code: number;
    message: string;
  };
}

type JSONRPCResponse = JSONRPCSuccessResponse | JSONRPCErrorResponse;

// ---------------------------------------------------------------------------
// MCP Client
// ---------------------------------------------------------------------------

export class OminiConnectMCP {
  private readonly baseUrl: string;
  private readonly apiKey: string;
  private readonly timeout: number;

  constructor(config: MCPClientConfig) {
    if (!config.baseUrl) {
      throw new Error('baseUrl is required');
    }
    if (!config.apiKey) {
      throw new Error('apiKey is required');
    }
    this.baseUrl = config.baseUrl.replace(/\/$/, ''); // Remove trailing slash
    this.apiKey = config.apiKey;
    this.timeout = config.timeout ?? 60000;
  }

  /**
   * Make an MCP JSON-RPC request to the OminiConnect portal.
   */
  private async request<T>(method: string, params?: unknown): Promise<T> {
    const request: JSONRPCRequest = {
      jsonrpc: '2.0',
      id: this.generateId(),
      method,
      params,
    };

    const response = await fetch(`${this.baseUrl}/api/mcp`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${this.apiKey}`,
      },
      body: JSON.stringify(request),
      signal: AbortSignal.timeout(this.timeout),
    });

    if (!response.ok) {
      throw new Error(`MCP request failed: ${response.status} ${response.statusText}`);
    }

    const data = await response.json() as JSONRPCResponse;

    if ('error' in data) {
      throw new Error(`MCP error: ${data.error.message} (code: ${data.error.code})`);
    }

    return data.result as T;
  }

  /**
   * Generate a unique ID for JSON-RPC requests.
   */
  private generateId(): string {
    return `mcp-${Date.now()}-${Math.random().toString(36).substring(2, 9)}`;
  }

  /**
   * List all available tools for the authenticated user.
   *
   * Returns tools from all connected platforms that the user has access to.
   * The `scope_satisfied` field indicates whether the user's token has
   * the required scopes to use each tool.
   */
  async listTools(): Promise<ToolListResult> {
    return this.request<ToolListResult>('tools/list');
  }

  /**
   * Call a specific tool by name with optional arguments.
   *
   * @param name - The tool's unique slug (e.g., 'github_list_repos')
   * @param arguments - Key-value pairs matching the tool's input schema
   *
   * For tools that support async execution, you can provide a `callback_url`
   * in the arguments to receive results via webhook.
   */
  async callTool(name: string, arguments_: Record<string, unknown> = {}): Promise<MCPCallResult> {
    return this.request<MCPCallResult>('tools/call', {
      name,
      arguments: arguments_,
    });
  }

  /**
   * Create an SSE (Server-Sent Events) connection for receiving async tool results.
   *
   * This is useful for long-running tools that return results via callback.
   * The returned EventSource will receive events with tool execution results.
   *
   * Note: The SSE endpoint requires the same authentication as the main API.
   */
  createSSEConnection(): EventSource {
    const url = `${this.baseUrl}/api/mcp/sse?token=${encodeURIComponent(this.apiKey)}`;
    return new EventSource(url);
  }
}

// ---------------------------------------------------------------------------
// Default export
// ---------------------------------------------------------------------------

export default OminiConnectMCP;
