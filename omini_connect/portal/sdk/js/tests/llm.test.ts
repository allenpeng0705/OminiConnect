import { describe, it, expect, beforeEach } from 'vitest';
import { OminiConnect } from '../src/index';
import { setupFetchMock, mockResponse } from './setup';

describe('client.llm', () => {
  beforeEach(() => {
    setupFetchMock();
  });

  describe('execute()', () => {
    it('calls POST /api/llm with query', async () => {
      mockResponse({
        ok: true,
        tool: 'github_list_repos',
        tool_name: 'List Repositories',
        arguments: { sort: 'updated', direction: 'desc' },
        explanation: "Routing 'list my github repos' to github_list_repos",
      });
      const client = new OminiConnect({ apiKey: 'test-key' });
      const result = await client.llm.execute('list my github repos');

      expect(result.ok).toBe(true);
      expect(result.tool).toBe('github_list_repos');
      expect(result.arguments).toEqual({ sort: 'updated', direction: 'desc' });
    });

    it('includes platform in request body when specified', async () => {
      mockResponse({ ok: true, tool: 'github_list_repos', tool_name: 'List Repositories', arguments: {} });
      const client = new OminiConnect({ apiKey: 'test-key' });
      await client.llm.execute('list repos', 'github');

      // Verify the POST was made to /api/llm
      const fetchCall = (global.fetch as any).mock.calls[0];
      expect(fetchCall[0]).toContain('/api/llm');
      const body = JSON.parse(fetchCall[1].body);
      expect(body.query).toBe('list repos');
      expect(body.platform).toBe('github');
    });

    it('returns candidates when query is ambiguous', async () => {
      mockResponse({
        ok: false,
        message: 'Ambiguous query — multiple tools matched',
        candidates: [
          { tool: 'github_list_repos', name: 'List Repositories', match_reason: 'keyword match' },
          { tool: 'github_list_issues', name: 'List Issues', match_reason: 'keyword match' },
        ],
        available_tools_hint: 'Try being more specific: which platform?',
      });
      const client = new OminiConnect({ apiKey: 'test-key' });
      const result = await client.llm.execute('list my stuff');

      expect(result.ok).toBe(false);
      expect(result.candidates).toHaveLength(2);
      expect(result.candidates![0].tool).toBe('github_list_repos');
      expect(result.available_tools_hint).toBeDefined();
    });

    it('returns error when tool execution fails', async () => {
      mockResponse({
        ok: false,
        error: 'Insufficient scopes for this operation',
      });
      const client = new OminiConnect({ apiKey: 'test-key' });
      const result = await client.llm.execute('create a repo');

      expect(result.ok).toBe(false);
      expect(result.error).toBe('Insufficient scopes for this operation');
    });
  });

  describe('listAvailableTools()', () => {
    it('calls GET /api/llm/tools', async () => {
      mockResponse({
        platforms: {
          github: {
            connected: true,
            tools: [
              {
                slug: 'github_list_repos',
                name: 'List Repositories',
                description: 'List all GitHub repositories',
                example_queries: ['list my repos', 'show my github repos'],
                scopes: ['repo'],
                scope_satisfied: 'yes',
              },
            ],
          },
          slack: {
            connected: false,
            tools: [],
          },
        },
      });
      const client = new OminiConnect({ apiKey: 'test-key' });
      const result = await client.llm.listAvailableTools();

      expect(result.platforms).toBeDefined();
      expect(result.platforms['github'].connected).toBe(true);
      expect(result.platforms['github'].tools).toHaveLength(1);
      expect(result.platforms['github'].tools![0].slug).toBe('github_list_repos');
    });

    it('includes platform filter in query string', async () => {
      mockResponse({ platforms: {} });
      const client = new OminiConnect({ apiKey: 'test-key' });
      await client.llm.listAvailableTools('github');

      const fetchCall = (global.fetch as any).mock.calls[0];
      expect(fetchCall[0]).toContain('platform=github');
    });
  });
});
