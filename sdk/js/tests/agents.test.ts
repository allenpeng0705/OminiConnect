import {describe, expect, it, vi} from 'vitest';
import {OminiConnect, AgentsManager, AuthError} from '../src/index.ts';

const mockFetch = vi.fn();

describe('AgentsManager', () => {
  const client = new OminiConnect({apiKey: 'test-key', fetchFn: mockFetch});

  beforeEach(() => {
    mockFetch.mockReset().mockResolvedValue({
      ok: true,
      status: 200,
      headers: new Headers({'content-type': 'application/json'}),
      json: () => Promise.resolve({}),
    });
  });

  it('register creates agent and returns raw key', async () => {
    mockFetch.mockResolvedValueOnce({
      ok: true,
      status: 201,
      headers: new Headers({'content-type': 'application/json'}),
      json: () =>
        Promise.resolve({
          id: 'agent-123',
          name: 'pr-reviewer',
          description: 'Reviews PRs',
          api_key: 'sk_agent_abc123',
          created_at: '2026-01-01T00:00:00Z',
          enabled: true,
        }),
    });

    const result = await client.agents.register('pr-reviewer', 'Reviews PRs');

    expect(mockFetch).toHaveBeenCalledWith(
      expect.stringContaining('/api/agents'),
      expect.objectContaining({
        method: 'POST',
        body: JSON.stringify({name: 'pr-reviewer', description: 'Reviews PRs'}),
      }),
    );
    expect(result.id).toBe('agent-123');
    expect(result.api_key).toBe('sk_agent_abc123');
  });

  it('list returns all agents', async () => {
    mockFetch.mockResolvedValueOnce({
      ok: true,
      status: 200,
      headers: new Headers({'content-type': 'application/json'}),
      json: () =>
        Promise.resolve([
          {id: 'agent-1', name: 'agent-1', created_at: '2026-01-01', enabled: true},
          {id: 'agent-2', name: 'agent-2', created_at: '2026-01-02', enabled: false},
        ]),
    });

    const result = await client.agents.list();

    expect(result).toHaveLength(2);
    expect(result[0].id).toBe('agent-1');
    expect(result[1].id).toBe('agent-2');
  });

  it('get returns single agent', async () => {
    mockFetch.mockResolvedValueOnce({
      ok: true,
      status: 200,
      headers: new Headers({'content-type': 'application/json'}),
      json: () =>
        Promise.resolve({
          id: 'agent-123',
          name: 'pr-reviewer',
          created_at: '2026-01-01',
          enabled: true,
        }),
    });

    const result = await client.agents.get('agent-123');
    expect(result.id).toBe('agent-123');
    expect(mockFetch).toHaveBeenCalledWith(
      expect.stringContaining('/api/agents/agent-123'),
      expect.any(Object),
    );
  });

  it('delete removes agent', async () => {
    mockFetch.mockResolvedValueOnce({
      ok: true,
      status: 200,
      headers: new Headers({'content-type': 'application/json'}),
      json: () => Promise.resolve({ok: true}),
    });

    await client.agents.delete('agent-123');

    expect(mockFetch).toHaveBeenCalledWith(
      expect.stringContaining('/api/agents/agent-123'),
      expect.objectContaining({method: 'DELETE'}),
    );
  });

  it('deactivate disables agent', async () => {
    mockFetch.mockResolvedValueOnce({
      ok: true,
      status: 200,
      headers: new Headers({'content-type': 'application/json'}),
      json: () =>
        Promise.resolve({
          id: 'agent-123',
          name: 'pr-reviewer',
          enabled: false,
        }),
    });

    const result = await client.agents.deactivate('agent-123');
    expect(result.enabled).toBe(false);
    expect(mockFetch).toHaveBeenCalledWith(
      expect.stringContaining('/api/agents/agent-123/deactivate'),
      expect.objectContaining({method: 'POST'}),
    );
  });

  it('register without description', async () => {
    mockFetch.mockResolvedValueOnce({
      ok: true,
      status: 201,
      headers: new Headers({'content-type': 'application/json'}),
      json: () =>
        Promise.resolve({
          id: 'agent-456',
          name: 'simple-agent',
          api_key: 'sk_agent_xyz',
          created_at: '2026-01-01',
          enabled: true,
        }),
    });

    const result = await client.agents.register('simple-agent');
    expect(result.name).toBe('simple-agent');
  });

  it('throws AuthError on 401', async () => {
    mockFetch.mockResolvedValueOnce({
      ok: false,
      status: 401,
      headers: new Headers({'content-type': 'application/json'}),
    });

    await expect(client.agents.list()).rejects.toThrow(AuthError);
  });
});
