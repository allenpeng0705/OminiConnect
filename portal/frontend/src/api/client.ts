const BASE = '';

interface ApiKeyResponse {
  key: string;
  label: string;
  created_at: string;
}

interface AuthCapabilities {
  google_login_enabled: boolean;
}

interface ConnectorStatus {
  platform: string;
  configured: boolean;
  connected: boolean;
}

export interface ConnectorConfig {
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
  /** Server-computed logical entry id (managed = provider slug before `__`). */
  catalog_entry?: string;
}

export class ConnectorConflictError extends Error {
  readonly existing_platform: string;

  constructor(message: string, existing_platform: string) {
    super(message);
    this.name = 'ConnectorConflictError';
    this.existing_platform = existing_platform;
  }
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

export async function login(email: string, password: string): Promise<void> {
  const res = await apiFetch('/auth/login', {
    method: 'POST',
    body: JSON.stringify({ email, password }),
  });
  if (!res.ok) {
    const text = await res.text();
    throw new Error(text || 'Login failed');
  }
}

export async function signup(email: string, password: string, name?: string): Promise<void> {
  const res = await apiFetch('/auth/signup', {
    method: 'POST',
    body: JSON.stringify({ email, password, name }),
  });
  if (!res.ok) {
    const text = await res.text();
    // If the error response is HTML (e.g., from a proxy/gateway error page), show a generic message
    if (text.trim().startsWith('<') || text.includes('DOCTYPE')) {
      throw new Error('Service error — please try again in a moment.');
    }
    // Try to parse as JSON for structured error messages
    try {
      const json = JSON.parse(text);
      throw new Error(json.error || json.message || text);
    } catch {
      if (text && !text.startsWith('<')) {
        throw new Error(text);
      }
      throw new Error('Signup failed');
    }
  }
}

export async function getAuthCapabilities(): Promise<AuthCapabilities> {
  const res = await apiFetch('/auth/capabilities');
  if (!res.ok) {
    return { google_login_enabled: false };
  }
  return res.json();
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
    const raw = await res.text();
    if (res.status === 409) {
      try {
        const j = JSON.parse(raw) as { error?: string; existing_platform?: string };
        throw new ConnectorConflictError(
          typeof j.error === 'string' && j.error.trim() ? j.error : 'A connector for this integration already exists.',
          typeof j.existing_platform === 'string' ? j.existing_platform : '',
        );
      } catch (e) {
        if (e instanceof ConnectorConflictError) throw e;
      }
    }
    throw new Error(raw || 'Failed to save connector');
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
  const res = await apiFetch(`/api/connectors/${encodeURIComponent(platform)}/test`, { method: 'POST' });
  if (!res.ok) {
    const j = await res.json().catch(() => ({} as { error?: string }));
    const msg = (j as { error?: string }).error || res.statusText;
    throw new Error(msg || 'Failed to test connector');
  }
  return res.json() as Promise<{ status: string; message: string }>;
}

export type PortalHealth = {
  service?: string;
  status: string;
  version?: string;
  /** Same as server `PORTAL_BASE_URL` (trimmed, no trailing slash). */
  portal_base_url?: string;
};

export async function healthCheck(): Promise<PortalHealth> {
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

/** Start Nango Connect for a managed (`engine=nango`) connector — returns URL to open in a popup or new tab. */
export async function createNangoConnectSession(platform: string): Promise<{ connect_url: string }> {
  const connect_api_base = window.location.origin;
  const res = await apiFetch('/api/nango/connect-session', {
    method: 'POST',
    body: JSON.stringify({ platform, connect_api_base }),
  });
  if (!res.ok) {
    let detail = res.statusText || 'Failed to start Nango Connect';
    try {
      const j = (await res.json()) as { error?: unknown };
      if (typeof j?.error === 'string' && j.error.trim()) detail = j.error.trim();
    } catch {
      /* ignore */
    }
    throw new Error(detail);
  }
  return res.json() as Promise<{ connect_url: string }>;
}

/** Create a Nango connection with credentials directly (API_KEY, BASIC, or OAUTH2_CC), no Connect UI needed. */
export async function createNangoConnectionDirect(options: {
  platform: string;
  auth_mode: 'API_KEY' | 'BASIC' | 'OAUTH2_CC' | 'SIGNATURE';
  api_key?: string;
  username?: string;
  password?: string;
}): Promise<{ connection_id: string }> {
  const res = await apiFetch('/api/nango/connections', {
    method: 'POST',
    body: JSON.stringify(options),
  });
  if (!res.ok) {
    let detail = res.statusText || 'Failed to create Nango connection';
    try {
      const j = (await res.json()) as { error?: unknown };
      if (typeof j?.error === 'string' && j.error.trim()) detail = j.error.trim();
    } catch {
      /* ignore */
    }
    throw new Error(detail);
  }
  return res.json() as Promise<{ connection_id: string }>;
}

export interface NangoConnectionSummary {
  connection_id: string;
  provider_config_key: string;
  created?: string;
}

/** List Nango connections for the current user (used after Nango Connect popup closes). */
export async function listNangoConnections(platform: string): Promise<NangoConnectionSummary[]> {
  const res = await apiFetch(`/api/nango/connections?platform=${encodeURIComponent(platform)}`);
  if (!res.ok) {
    let detail = res.statusText || 'Failed to list Nango connections';
    try {
      const j = (await res.json()) as { error?: unknown };
      if (typeof j?.error === 'string' && j.error.trim()) detail = j.error.trim();
    } catch {
      /* ignore */
    }
    throw new Error(detail);
  }
  return res.json() as Promise<NangoConnectionSummary[]>;
}

/** One row in the OminiConnect integration library (server-backed catalog). */
export interface IntegrationCatalogRow {
  name: string;
  display_name: string;
  logo_url: string;
  auth_mode?: string;
  categories?: string[];
  docs?: string;
  available_scopes?: string[];
}

export async function getIntegrationCatalog(search?: string): Promise<IntegrationCatalogRow[]> {
  const q = search?.trim() ? `?search=${encodeURIComponent(search.trim())}` : '';
  const res = await apiFetch(`/api/nango/providers${q}`);
  if (!res.ok) {
    if (res.status === 401) {
      throw new Error('Sign in to the portal again — your session expired.');
    }
    let detail = res.statusText || 'Failed to load integration catalog';
    try {
      const j = (await res.json()) as { error?: unknown };
      if (typeof j?.error === 'string' && j.error.trim()) {
        detail = j.error.trim();
      }
    } catch {
      /* ignore */
    }
    throw new Error(detail);
  }
  return res.json();
}

// ============================================================================
// LLM / Natural Language Testing
// ============================================================================

export interface LlmExecuteResponse {
  ok: boolean;
  tool?: string;
  tool_name?: string;
  arguments?: Record<string, unknown>;
  explanation?: string;
  result?: unknown;
  error?: string;
  message?: string;
  candidates?: { tool: string; tool_name: string; score: number }[];
  available_tools_hint?: string;
}

export interface LlmTool {
  slug: string;
  name: string;
  description: string;
  provider: string;
}

export interface LlmExecuteRequest {
  query: string;
  platform?: string;
  context?: Record<string, unknown>;
  llm_url?: string;
  llm_api_key?: string;
  llm_model?: string;
}

export async function executeLlmQuery(req: LlmExecuteRequest): Promise<LlmExecuteResponse> {
  const res = await apiFetch('/api/llm', {
    method: 'POST',
    body: JSON.stringify(req),
  });
  if (!res.ok) {
    let detail = res.statusText || 'LLM request failed';
    try {
      const j = (await res.json()) as { error?: unknown };
      if (typeof j?.error === 'string' && j.error.trim()) {
        detail = j.error.trim();
      }
    } catch {
      /* ignore */
    }
    throw new Error(detail);
  }
  return res.json();
}

export async function getLlmTools(platform?: string): Promise<LlmTool[]> {
  const q = platform ? `?platform=${encodeURIComponent(platform)}` : '';
  const res = await apiFetch(`/api/llm/tools${q}`);
  if (!res.ok) {
    throw new Error('Failed to load LLM tools');
  }
  return res.json();
}
