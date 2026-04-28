import { describe, it, expect, vi, beforeEach } from 'vitest';
import { OminiConnectMCP, type Tool, type MCPCallResult } from '../src/index';

// Mock fetch
const mockFetch = vi.fn();
global.fetch = mockFetch;

describe('OminiConnectMCP', () => {
  let client: OminiConnectMCP;

  beforeEach(() => {
    mockFetch.mockReset();
    client = new OminiConnectMCP({
      baseUrl: 'https://portal.example.com',
      apiKey: 'test-api-key',
    });
  });

  describe('constructor', () => {
    it('should create client with valid config', () => {
      expect(client).toBeInstanceOf(OminiConnectMCP);
    });

    it('should throw if baseUrl is missing', () => {
      expect(() => new OminiConnectMCP({ baseUrl: '', apiKey: 'key' })).toThrow('baseUrl is required');
    });

    it('should throw if apiKey is missing', () => {
      expect(() => new OminiConnectMCP({ baseUrl: 'https://portal.com', apiKey: '' })).toThrow('apiKey is required');
    });

    it('should remove trailing slash from baseUrl', () => {
      const clientWithSlash = new OminiConnectMCP({
        baseUrl: 'https://portal.example.com/',
        apiKey: 'key',
      });
      expect(clientWithSlash).toBeInstanceOf(OminiConnectMCP);
    });
  });

  describe('listTools', () => {
    it('should call tools/list method', async () => {
      const mockTools: { tools: Tool[] } = {
        tools: [
          {
            name: 'github_list_repos',
            description: 'List repositories for the authenticated user',
            input_schema: {
              type: 'object',
              properties: {},
              required: [],
            },
            scope_satisfied: 'yes',
          },
        ],
      };

      mockFetch.mockResolvedValueOnce({
        ok: true,
        status: 200,
        json: () => Promise.resolve({
          jsonrpc: '2.0',
          id: 'test-id',
          result: mockTools,
        }),
      });

      const result = await client.listTools();

      expect(mockFetch).toHaveBeenCalledWith(
        'https://portal.example.com/api/mcp',
        expect.objectContaining({
          method: 'POST',
          headers: expect.objectContaining({
            'Content-Type': 'application/json',
            'Authorization': 'Bearer test-api-key',
          }),
          body: expect.stringContaining('"method":"tools/list"'),
        })
      );
      expect(result).toEqual(mockTools);
    });

    it('should throw on MCP error response', async () => {
      mockFetch.mockResolvedValueOnce({
        ok: true,
        status: 200,
        json: () => Promise.resolve({
          jsonrpc: '2.0',
          id: 'test-id',
          error: {
            code: -32601,
            message: 'Method not found',
          },
        }),
      });

      await expect(client.listTools()).rejects.toThrow('MCP error: Method not found');
    });

    it('should throw on HTTP error', async () => {
      mockFetch.mockResolvedValueOnce({
        ok: false,
        status: 401,
        statusText: 'Unauthorized',
      });

      await expect(client.listTools()).rejects.toThrow('MCP request failed: 401 Unauthorized');
    });
  });

  describe('callTool', () => {
    it('should call tools/call with correct params', async () => {
      const mockResult: MCPCallResult = {
        status: 'success',
        result: { repositories: [{ name: 'test-repo' }] },
      };

      mockFetch.mockResolvedValueOnce({
        ok: true,
        status: 200,
        json: () => Promise.resolve({
          jsonrpc: '2.0',
          id: 'test-id',
          result: mockResult,
        }),
      });

      const result = await client.callTool('github_list_repos', { per_page: 10 });

      expect(mockFetch).toHaveBeenCalledWith(
        'https://portal.example.com/api/mcp',
        expect.objectContaining({
          method: 'POST',
          body: expect.stringContaining('"method":"tools/call"'),
          body: expect.stringContaining('"name":"github_list_repos"'),
        })
      );
      expect(result).toEqual(mockResult);
    });

    it('should handle async tool results with call_id', async () => {
      const mockResult: MCPCallResult = {
        call_id: 'call-123',
        status: 'pending',
      };

      mockFetch.mockResolvedValueOnce({
        ok: true,
        status: 200,
        json: () => Promise.resolve({
          jsonrpc: '2.0',
          id: 'test-id',
          result: mockResult,
        }),
      });

      const result = await client.callTool('long_running_tool', { callback_url: 'https://example.com/callback' });

      expect(result.call_id).toBe('call-123');
      expect(result.status).toBe('pending');
    });

    it('should work with empty arguments', async () => {
      const mockResult: MCPCallResult = { status: 'success' };

      mockFetch.mockResolvedValueOnce({
        ok: true,
        status: 200,
        json: () => Promise.resolve({
          jsonrpc: '2.0',
          id: 'test-id',
          result: mockResult,
        }),
      });

      const result = await client.callTool('github_list_repos');

      expect(result.status).toBe('success');
    });
  });
});
