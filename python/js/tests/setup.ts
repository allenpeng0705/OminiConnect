import { vi } from 'vitest';

// Global fetch mock that can be configured per test
const fetchMock = vi.fn();
global.fetch = fetchMock;

export function setupFetchMock() {
  fetchMock.mockReset();
  fetchMock.mockResolvedValue({
    ok: true,
    status: 200,
    headers: new Headers({ 'content-length': '0' }),
    json: vi.fn().mockResolvedValue({}),
    text: vi.fn().mockResolvedValue(''),
  });
}

export function mockResponse(data: unknown, init?: ResponseInit) {
  const { status = 200, statusText = 'OK', headers = {} } = init ?? {};
  fetchMock.mockResolvedValue({
    ok: status >= 200 && status < 300,
    status,
    statusText,
    headers: new Headers(headers),
    json: vi.fn().mockResolvedValue(data),
    text: vi.fn().mockResolvedValue(typeof data === 'string' ? data : JSON.stringify(data)),
  });
}

export { fetchMock };
