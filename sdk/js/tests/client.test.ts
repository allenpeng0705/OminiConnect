import { describe, it, expect, vi, beforeEach } from 'vitest';
import { OminiConnect } from '../src/index';
import { setupFetchMock, mockResponse, fetchMock } from './setup';

describe('OminiConnect constructor', () => {
  it('uses default baseUrl when not provided', () => {
    const client = new OminiConnect({ apiKey: 'test-key' });
    expect((client as any).baseUrl).toBe('http://localhost:9000');
  });

  it('uses custom baseUrl when provided', () => {
    const client = new OminiConnect({ apiKey: 'test-key', baseUrl: 'https://api.example.com' });
    expect((client as any).baseUrl).toBe('https://api.example.com');
  });

  it('strips trailing slash from baseUrl', () => {
    const client = new OminiConnect({ apiKey: 'test-key', baseUrl: 'http://localhost:9000/' });
    expect((client as any).baseUrl).toBe('http://localhost:9000');
  });
});

describe('client.call()', () => {
  beforeEach(() => {
    setupFetchMock();
  });

  it('makes POST request with method, path, and params in body', async () => {
    mockResponse({ ok: true });
    const client = new OminiConnect({ apiKey: 'test-key' });
    await client.call('github', 'GET', '/user/repos', { params: { sort: 'updated' } });

    expect(fetchMock).toHaveBeenCalledWith(
      'http://localhost:9000/api/call/github',
      expect.objectContaining({
        method: 'POST',
        body: JSON.stringify({
          method: 'GET',
          path: '/user/repos',
          params: { sort: 'updated' },
        }),
      })
    );
  });

  it('makes POST request with body', async () => {
    mockResponse({ ok: true });
    const client = new OminiConnect({ apiKey: 'test-key' });
    await client.call('slack', 'POST', '/api/chat.postMessage', {
      body: { channel: 'C0123', text: 'Hi!' },
    });

    expect(fetchMock).toHaveBeenCalledWith(
      'http://localhost:9000/api/call/slack',
      expect.objectContaining({
        method: 'POST',
        body: JSON.stringify({
          method: 'POST',
          path: '/api/chat.postMessage',
          body: { channel: 'C0123', text: 'Hi!' },
        }),
      })
    );
  });

  it('uppercases the method in body', async () => {
    mockResponse({ ok: true });
    const client = new OminiConnect({ apiKey: 'test-key' });
    await client.call('github', 'get', '/user/repos');

    expect(fetchMock).toHaveBeenCalledWith(
      'http://localhost:9000/api/call/github',
      expect.objectContaining({
        method: 'POST',
        body: JSON.stringify({
          method: 'GET',
          path: '/user/repos',
        }),
      })
    );
  });
});

describe('client.connectors', () => {
  beforeEach(() => {
    setupFetchMock();
  });

  describe('list()', () => {
    it('calls GET /api/connectors', async () => {
      mockResponse([{ platform: 'github', enabled: true, scopes: [], created_at: '2024-01-01' }]);
      const client = new OminiConnect({ apiKey: 'test-key' });
      const connectors = await client.connectors.list();

      expect(connectors).toEqual([{ platform: 'github', enabled: true, scopes: [], created_at: '2024-01-01' }]);
      expect(fetchMock).toHaveBeenCalledWith(
        'http://localhost:9000/api/connectors',
        expect.objectContaining({ method: 'GET' })
      );
    });
  });

  describe('delete()', () => {
    it('calls DELETE /api/connectors/:platform', async () => {
      mockResponse({ ok: true });
      const client = new OminiConnect({ apiKey: 'test-key' });
      const result = await client.connectors.delete('github');

      expect(result).toEqual({ ok: true });
      expect(fetchMock).toHaveBeenCalledWith(
        'http://localhost:9000/api/connectors/github',
        expect.objectContaining({ method: 'DELETE' })
      );
    });
  });
});

describe('client.tools', () => {
  beforeEach(() => {
    setupFetchMock();
  });

  describe('list()', () => {
    it('calls GET /api/tools without platform', async () => {
      mockResponse({ toolkits: [] });
      const client = new OminiConnect({ apiKey: 'test-key' });
      await client.tools.list();

      expect(fetchMock).toHaveBeenCalledWith(
        'http://localhost:9000/api/tools',
        expect.objectContaining({ method: 'GET' })
      );
    });

    it('calls GET /api/tools?platform=github', async () => {
      mockResponse({ toolkits: [] });
      const client = new OminiConnect({ apiKey: 'test-key' });
      await client.tools.list('github');

      expect(fetchMock).toHaveBeenCalledWith(
        'http://localhost:9000/api/tools?platform=github',
        expect.objectContaining({ method: 'GET' })
      );
    });
  });

  describe('search()', () => {
    it('calls GET /api/tools/search with query', async () => {
      mockResponse({ tools: [], query: 'repos' });
      const client = new OminiConnect({ apiKey: 'test-key' });
      await client.tools.search('repos');

      expect(fetchMock).toHaveBeenCalledWith(
        'http://localhost:9000/api/tools/search?q=repos',
        expect.objectContaining({ method: 'GET' })
      );
    });

    it('includes platform and filter_scope params', async () => {
      mockResponse({ tools: [], query: 'repos' });
      const client = new OminiConnect({ apiKey: 'test-key' });
      await client.tools.search('repos', 'github', 'yes');

      expect(fetchMock).toHaveBeenCalledWith(
        'http://localhost:9000/api/tools/search?q=repos&platform=github&filter_scope=yes',
        expect.objectContaining({ method: 'GET' })
      );
    });
  });

  describe('execute()', () => {
    it('calls POST /api/tools/execute with tool_slug and arguments', async () => {
      mockResponse({ ok: true });
      const client = new OminiConnect({ apiKey: 'test-key' });
      await client.tools.execute('github_list_repos', { owner: 'test' });

      expect(fetchMock).toHaveBeenCalledWith(
        'http://localhost:9000/api/tools/execute',
        expect.objectContaining({
          method: 'POST',
          body: JSON.stringify({
            tool_slug: 'github_list_repos',
            arguments: { owner: 'test' },
          }),
        })
      );
    });

    it('includes callback_url when provided', async () => {
      mockResponse({ ok: true });
      const client = new OminiConnect({ apiKey: 'test-key' });
      await client.tools.execute('github_list_repos', {}, 'https://callback.example.com');

      expect(fetchMock).toHaveBeenCalledWith(
        'http://localhost:9000/api/tools/execute',
        expect.objectContaining({
          method: 'POST',
          body: JSON.stringify({
            tool_slug: 'github_list_repos',
            arguments: {},
            callback_url: 'https://callback.example.com',
          }),
        })
      );
    });
  });
});

describe('client.apiKeys', () => {
  beforeEach(() => {
    setupFetchMock();
  });

  describe('create()', () => {
    it('calls POST /auth/apikey with label', async () => {
      mockResponse({ key: 'sk_test_abc123', label: 'test-key', created_at: '2024-01-01' });
      const client = new OminiConnect({ apiKey: 'test-key' });
      const result = await client.apiKeys.create('test-key');

      expect(result).toEqual({ key: 'sk_test_abc123', label: 'test-key', created_at: '2024-01-01' });
      expect(fetchMock).toHaveBeenCalledWith(
        'http://localhost:9000/auth/apikey',
        expect.objectContaining({
          method: 'POST',
          body: JSON.stringify({ label: 'test-key' }),
        })
      );
    });
  });

  describe('list()', () => {
    it('calls GET /auth/apikey', async () => {
      mockResponse([{ key_hash: 'abc', label: 'test-key', created_at: '2024-01-01' }]);
      const client = new OminiConnect({ apiKey: 'test-key' });
      const result = await client.apiKeys.list();

      expect(result).toEqual([{ key_hash: 'abc', label: 'test-key', created_at: '2024-01-01' }]);
      expect(fetchMock).toHaveBeenCalledWith(
        'http://localhost:9000/auth/apikey',
        expect.objectContaining({ method: 'GET' })
      );
    });
  });

  describe('delete()', () => {
    it('calls DELETE /auth/apikey/:keyHash', async () => {
      mockResponse({ ok: true });
      const client = new OminiConnect({ apiKey: 'test-key' });
      const result = await client.apiKeys.delete('abc123');

      expect(result).toEqual({ ok: true });
      expect(fetchMock).toHaveBeenCalledWith(
        'http://localhost:9000/auth/apikey/abc123',
        expect.objectContaining({ method: 'DELETE' })
      );
    });
  });
});
