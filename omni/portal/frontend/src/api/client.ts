const BASE = '';

interface ApiKeyResponse {
  key: string;
  label: string;
  created_at: string;
}

interface ConnectorStatus {
  platform: string;
  configured: boolean;
  connected: boolean;
}

interface ConnectorConfig {
  platform: string;
  client_id: string;
  client_secret: string;
  redirect_uri: string;
  scopes: string[];
  enabled: boolean;
}

async function apiFetch(path: string, options: RequestInit = {}): Promise<Response> {
  const res = await fetch(`${BASE}${path}`, {
    ...options,
    headers: {
      'Content-Type': 'application/json',
      ...options.headers,
    },
    credentials: 'include',
  });
  return res;
}

export async function login(username: string, password: string): Promise<void> {
  const res = await apiFetch('/auth/login', {
    method: 'POST',
    body: JSON.stringify({ username, password }),
  });
  if (!res.ok) {
    const text = await res.text();
    throw new Error(text || 'Login failed');
  }
}

export async function logout(): Promise<void> {
  await apiFetch('/auth/logout', { method: 'POST' });
}

export async function getMe(): Promise<{ username: string } | null> {
  const res = await apiFetch('/auth/me');
  if (!res.ok) return null;
  return res.json();
}

export async function generateApiKey(label: string): Promise<ApiKeyResponse> {
  const res = await apiFetch('/auth/apikey', {
    method: 'POST',
    body: JSON.stringify({ label }),
  });
  if (!res.ok) throw new Error('Failed to generate API key');
  return res.json();
}

export async function getConnectors(): Promise<ConnectorConfig[]> {
  const res = await apiFetch('/api/connectors');
  if (!res.ok) throw new Error('Failed to fetch connectors');
  return res.json();
}

export async function upsertConnector(config: Partial<ConnectorConfig>): Promise<void> {
  const res = await apiFetch('/api/connectors', {
    method: 'POST',
    body: JSON.stringify(config),
  });
  if (!res.ok) {
    const text = await res.text();
    throw new Error(text || 'Failed to save connector');
  }
}

export async function deleteConnector(platform: string): Promise<void> {
  const res = await apiFetch(`/api/connectors/${platform}`, { method: 'DELETE' });
  if (!res.ok) throw new Error('Failed to delete connector');
}

export async function getConnectorStatus(platform: string): Promise<ConnectorStatus> {
  const res = await apiFetch(`/api/connectors/${platform}/status`);
  if (!res.ok) throw new Error('Failed to get status');
  return res.json();
}

export async function testConnector(platform: string): Promise<{ status: string; message: string }> {
  const res = await apiFetch(`/api/connectors/${platform}/test`, { method: 'POST' });
  if (!res.ok) throw new Error('Failed to test connector');
  return res.json();
}

export async function healthCheck(): Promise<{ status: string }> {
  const res = await apiFetch('/api/status');
  return res.json();
}
