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
  client_secret?: string;
  has_client_secret?: boolean;
  redirect_uri: string;
  scopes: string[];
  engine?: string;
  provider_key?: string;
  connection_ref?: string;
  agent_id?: string;
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

export async function getNangoStatus(): Promise<{ configured: boolean; base_url: string; has_secret_key: boolean }> {
  const res = await apiFetch('/api/status/nango');
  if (!res.ok) throw new Error('Failed to get integration hub status');
  return res.json();
}

export interface NangoIntegrationRow {
  unique_key: string;
  display_name: string;
  provider?: string;
}

/** Configured integrations in the linked hub (for picking `provider_key`). */
export async function getNangoIntegrations(): Promise<NangoIntegrationRow[]> {
  const res = await apiFetch('/api/nango/integrations');
  if (!res.ok) throw new Error('Failed to load configured integrations');
  return res.json();
}

/** One row in the OminiConnect integration library (server-backed catalog). */
export interface IntegrationCatalogRow {
  name: string;
  display_name: string;
  logo_url: string;
  auth_mode?: string;
  categories?: string[];
  docs?: string;
}

export async function getIntegrationCatalog(search?: string): Promise<IntegrationCatalogRow[]> {
  const q = search?.trim() ? `?search=${encodeURIComponent(search.trim())}` : '';
  const res = await apiFetch(`/api/nango/providers${q}`);
  if (!res.ok) throw new Error('Failed to load integration catalog');
  return res.json();
}
