import { describe, it, expect, vi, beforeEach } from 'vitest';
import { OminiConnect, AuthError, RateLimitedError, ConnectorNotFoundError, UpstreamError, NetworkError } from '../src/index';
import { setupFetchMock, mockResponse, fetchMock } from './setup';

describe('Error handling', () => {
  beforeEach(() => {
    setupFetchMock();
  });

  describe('HTTP 401', () => {
    it('throws AuthError', async () => {
      mockResponse({ error: 'unauthorized' }, { status: 401 });
      const client = new OminiConnect({ apiKey: 'invalid-key' });

      await expect(client.connectors.list()).rejects.toThrow(AuthError);
    });
  });

  describe('HTTP 429', () => {
    it('throws RateLimitedError', async () => {
      mockResponse({ error: 'rate limited' }, { status: 429 });
      const client = new OminiConnect({ apiKey: 'test-key' });

      await expect(client.connectors.list()).rejects.toThrow(RateLimitedError);
    });
  });

  describe('HTTP 404', () => {
    it('throws ConnectorNotFoundError', async () => {
      mockResponse({ error: 'not found' }, { status: 404 });
      const client = new OminiConnect({ apiKey: 'test-key' });

      await expect(client.connectors.delete('unknown')).rejects.toThrow(ConnectorNotFoundError);
    });
  });

  describe('HTTP 500', () => {
    it('throws UpstreamError with statusCode and body', async () => {
      mockResponse({ error: 'internal server error' }, { status: 500 });
      const client = new OminiConnect({ apiKey: 'test-key' });

      try {
        await client.connectors.list();
        expect.fail('should have thrown');
      } catch (err) {
        expect(err).toBeInstanceOf(UpstreamError);
        expect((err as UpstreamError).statusCode).toBe(500);
        expect((err as UpstreamError).body).toBe('{"error":"internal server error"}');
      }
    });
  });

  describe('Network failure', () => {
    it('throws NetworkError', async () => {
      fetchMock.mockRejectedValue(new TypeError('Failed to fetch'));
      const client = new OminiConnect({ apiKey: 'test-key' });

      await expect(client.connectors.list()).rejects.toThrow(NetworkError);
    });
  });
});
