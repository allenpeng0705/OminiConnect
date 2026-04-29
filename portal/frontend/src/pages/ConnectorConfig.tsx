import { useEffect, useState, useRef, useCallback, useMemo } from 'react';
import { useParams, useNavigate, useSearchParams } from 'react-router-dom';
import { getConnectors, getIntegrationCatalog, upsertConnector, deleteConnector, getConnectorStatus, testConnector, createNangoConnectSession, createNangoConnectionDirect, listNangoConnections, ConnectorConflictError, type IntegrationCatalogRow } from '../api/client';
import { normalizeCatalogResponse } from '../lib/integrationCatalogNormalize';
import { getPortalPublicBase } from '../lib/portalPublicBase';
import { BUILTIN_OMINI_PLATFORM_RECORD, findBuiltinByConnectorId } from '../lib/builtinPlatforms';
import { resolveBuiltinCatalogLogo } from '../lib/resolveBuiltinCatalogLogo';
import ScopeSelector from '../components/ScopeSelector';
import IntegrationProviderLogo from '../components/IntegrationProviderLogo';

const PLATFORMS: Record<string, { name: string; color: string; type: 'oauth2' | 'api_key' | 'basic'; scopes_default?: string; logo_url?: string }> = {
  ...BUILTIN_OMINI_PLATFORM_RECORD,
  // Note: GitHub PAT uses Nango's “github-pat” provider (API_KEY auth) from the catalog.
  // The builtin 'github' is NOT overridden here to avoid conflict with Nango's github-pat.
  linkedin: { name: 'LinkedIn', color: '#0A66C2', type: 'oauth2', scopes_default: 'openid profile email' },
  facebook: { name: 'Facebook', color: '#1877F2', type: 'oauth2', scopes_default: 'email public_profile' },
  x: { name: 'X (Twitter)', color: '#111111', type: 'oauth2', scopes_default: 'tweet.read users.read tweet.write offline.access' },
};

/** Paths like `/oauth/1password-scim-abc/callback` are legacy managed-hub saves; Nango expects `/oauth/callback`. */
const NATIVE_OAUTH_CALLBACK_SEGMENTS = new Set(['feishu', 'dingtalk', 'wechatwork', 'linkedin', 'facebook', 'x']);

function redirectPathLooksLikeLegacyManagedPerSlugCallback(redirectUri: string): boolean {
  const t = redirectUri.trim();
  if (!t) return false;
  try {
    const u = new URL(t);
    const m = u.pathname.match(/^\/oauth\/([^/]+)\/callback\/?$/);
    if (!m) return false;
    const seg = decodeURIComponent(m[1]);
    if (!seg || seg === 'callback') return false;
    return !NATIVE_OAUTH_CALLBACK_SEGMENTS.has(seg);
  } catch {
    return false;
  }
}

export default function ConnectorConfig() {
  const { platform } = useParams<{ platform: string }>();
  const [params] = useSearchParams();
  const navigate = useNavigate();
  const [clientId, setClientId] = useState('');
  const [clientSecret, setClientSecret] = useState('');
  const [redirectUri, setRedirectUri] = useState('');
  const [scopes, setScopes] = useState('');
  const [engine, setEngine] = useState<'omini_connect_native' | 'nango'>('omini_connect_native');
  const [providerKey, setProviderKey] = useState('');
  const [providerSlug, setProviderSlug] = useState('');
  const [connectionRef, setConnectionRef] = useState('');
  const [agentId, setAgentId] = useState('');
  const [enabled, setEnabled] = useState(true);
  const [saving, setSaving] = useState(false);
  const [testing, setTesting] = useState(false);
  const [testResult, setTestResult] = useState<{ status: string; message: string } | null>(null);
  const [error, setError] = useState('');
  const [connected, setConnected] = useState(false);
  const [showClientSecret, setShowClientSecret] = useState(false);
  const [showClientId, setShowClientId] = useState(false);
  const [hasExistingSecret, setHasExistingSecret] = useState(false);
  const [catalogRows, setCatalogRows] = useState<IntegrationCatalogRow[]>([]);
  const [hasExistingConnector, setHasExistingConnector] = useState(false);
  const engineRef = useRef(engine);
  const providerKeyRef = useRef(providerKey);
  engineRef.current = engine;
  providerKeyRef.current = providerKey;

  function autoManagedIntegrationKey(provider: string): string {
    const base = provider
      .trim()
      .toLowerCase()
      .replace(/[^a-z0-9_-]+/g, '-')
      .replace(/-+/g, '-')
      .replace(/^-+|-+$/g, '')
      .slice(0, 42) || 'provider';
    const suffix = Math.random().toString(36).slice(2, 8);
    return `${base}__${base}-${suffix}`;
  }

  const info = platform
    ? (PLATFORMS[platform] ??
        (() => {
          const b = findBuiltinByConnectorId(platform);
          if (b) {
            return {
              name: b.name,
              color: b.color,
              type: b.type,
              scopes_default: b.scopes_default,
              logo_url: b.logo_url,
            };
          }
          return {
            name: `Connector: ${platform}`,
            color: '#6366F1',
            type: 'oauth2' as const,
            scopes_default: '',
          };
        })())
    : null;
  // For Nango-managed connectors from catalog, auth_mode tells us the auth type.
  // Check catalog rows at this point in render (catalogRows loads early via useEffect).
  const catalogAuthMode = catalogRows.find(r => r.name === (providerSlug || platform))?.auth_mode;
  // Normalize auth mode once to avoid repeating toUpperCase/replace.
  const catalogAuthModeUpper = catalogAuthMode?.toUpperCase().replace(/\s+/g, '_') || '';
  const isApiKeyStyle = catalogAuthModeUpper === 'API_KEY';
  const isBasicStyle = catalogAuthModeUpper === 'BASIC';
  const isOAuth2CC = catalogAuthModeUpper === 'OAUTH2_CC';
  // BILL and TBA use Nango Connect popup (like TWO_STEP) — they have complex credential flows.
  const isBILL = catalogAuthModeUpper === 'BILL';
  const isTBA = catalogAuthModeUpper === 'TBA';
  // SIGNATURE (e.g. Emarsys WSSE) uses username + password, same as BASIC.
  const isSignature = catalogAuthModeUpper === 'SIGNATURE';
  // For built-in platforms (not from catalog), check info.type directly for basic.
  const isBasicPlatform = !catalogAuthMode && info?.type === 'basic';

  // Providers missing auth_mode in Nango that we know are OAuth2.
  // These are major platforms (Google, Microsoft, Zoho, Confluence, YouTube, etc.)
  // whose auth_mode field is empty in Nango's providers.yaml.
  const KNOWN_OAUTH2_AS_UNKNOWN = new Set([
    // Google
    'google-ads', 'google-analytics', 'google-bigquery', 'google-calendar', 'google-chat',
    'google-cloud-storage', 'google-contacts', 'google-docs', 'google-drive', 'google-forms',
    'google-gemini', 'google-mail', 'google-maps', 'google-meet', 'google-play',
    'google-search-console', 'google-service-account', 'google-sheet', 'google-slides', 'google-tasks',
    'google-workspace-admin',
    // Microsoft
    'microsoft-ads', 'microsoft-admin', 'microsoft-business-central', 'microsoft-entra-id',
    'microsoft-excel', 'microsoft-oauth2-cc', 'microsoft-planner', 'microsoft-power-bi',
    'microsoft-powerpoint', 'microsoft-teams', 'microsoft-teams-bot', 'microsoft-word',
    // Meta
    'meta-marketing-api',
    // Atlassian
    'confluence', 'confluence-data-center',
    // Zoho
    'zoho-bigin', 'zoho-books', 'zoho-calendar', 'zoho-crm', 'zoho-desk',
    'zoho-inventory', 'zoho-invoice', 'zoho-mail', 'zoho-people', 'zoho-recruit',
    // Other major platforms
    'youtube', 'okta-preview', 'one-drive', 'one-note', 'outlook', 'quickbooks-sandbox',
    'sharepoint-online', 'sharepoint-online-oauth2-cc', 'figjam',
    // ADP (OAuth2 variants)
    'adp-lyric', 'adp-run', 'adp-workforce-now', 'adp-workforce-now-next-gen',
    // Azure
    'azure-blob-storage',
  ]);

  // Use catalogAuthMode when available (Nango managed connectors); otherwise fall back to info.type.
  // For UNKNOWN auth_mode, check if it's a known OAuth2 provider.
  const isOAuth2 = catalogAuthMode
    ? catalogAuthModeUpper === 'OAUTH2' || catalogAuthModeUpper === 'OAUTH1' || catalogAuthModeUpper === 'MCP_OAUTH2' || catalogAuthModeUpper === 'MCP_OAUTH2_GENERIC'
    : (info?.type === 'oauth2' || (platform ? KNOWN_OAUTH2_AS_UNKNOWN.has(platform) : false)) && !isApiKeyStyle && !isBasicPlatform;
  const isQQMail = platform === 'qqmail';
  const isWeChatWork = platform === 'wechatwork';
  /** Native connector `platform: github` — classic or fine-grained PAT; use engine=nango for OAuth Connect instead. */
  const isGitHubPatPlatform = platform === 'github' && engine === 'omini_connect_native';

  /** Test calls POST /api/connectors/:platform/test — uses saved connector row, not unsaved form fields. */
  const showTestButton =
    !isOAuth2 || engine === 'nango' || (isOAuth2 && engine === 'omini_connect_native');
  const testButtonDisabled =
    testing ||
    (engine === 'nango' || (isOAuth2 && engine === 'omini_connect_native')
      ? false
      : isQQMail
        ? !clientId.trim() || !clientSecret.trim()
        : isBasicStyle || isBasicPlatform || isOAuth2CC || isSignature
          ? !clientId.trim() || !clientSecret.trim()
          : !clientId.trim() && !clientSecret.trim());

  const loadExisting = useCallback(async () => {
    if (!platform) return;
    try {
      const list = await getConnectors();
      const existing = list.find((c) => c.platform === platform);
      if (existing) {
        setHasExistingConnector(true);
        setClientId(existing.client_id);
        setHasExistingSecret(!!existing.has_client_secret);
        setClientSecret('');
        setScopes(existing.scopes?.join(' ') || '');
        const ru = (existing.redirect_uri || '').trim();
        let nextEngine = (existing.engine as 'omini_connect_native' | 'nango') || 'omini_connect_native';
        if (redirectPathLooksLikeLegacyManagedPerSlugCallback(ru)) {
          nextEngine = 'nango';
        }
        setEngine(nextEngine);
        setProviderKey(existing.provider_key || existing.platform || '');
        const existingProviderKey = existing.provider_key || '';
        if (existingProviderKey.includes('__')) {
          const [p] = existingProviderKey.split('__');
          setProviderSlug(p || '');
        }
        setConnectionRef(existing.connection_ref || '');
        setAgentId(existing.agent_id || '');
        setEnabled(existing.enabled);
        const base = await getPortalPublicBase();
        const b = base.replace(/\/+$/, '');
        const nangoCallback = `${b}/oauth/callback`;
        const nativeCallback = `${b}/oauth/${encodeURIComponent(platform)}/callback`;
        const catalogManagedSlug = Boolean(platform && !(platform in PLATFORMS));
        const managedHub =
          nextEngine === 'nango' ||
          (existing.provider_key || '').includes('__') ||
          catalogManagedSlug ||
          redirectPathLooksLikeLegacyManagedPerSlugCallback(ru);
        if (managedHub) {
          setRedirectUri(nangoCallback);
        } else if (ru) {
          setRedirectUri(ru);
        } else {
          setRedirectUri(nativeCallback);
        }
      } else {
        setHasExistingConnector(false);
        if (platform && PLATFORMS[platform]?.scopes_default) {
          setScopes(PLATFORMS[platform].scopes_default!);
          if (!providerSlug) {
            setProviderSlug(platform);
          }
          setProviderKey(platform);
        }
        if (isOAuth2 && platform) {
          const base = await getPortalPublicBase();
          const b = base.replace(/\/+$/, '');
          const e = engineRef.current;
          const pk = providerKeyRef.current;
          const catalogManagedSlug = Boolean(platform && !(platform in PLATFORMS));
          const managedHub = e === 'nango' || (pk || '').includes('__') || catalogManagedSlug;
          setRedirectUri(managedHub ? `${b}/oauth/callback` : `${b}/oauth/${encodeURIComponent(platform)}/callback`);
        }
      }
    } catch {
      /* ignore */
    }
  }, [platform, isOAuth2, providerSlug, engine, providerKey]);

  useEffect(() => {
    if (!platform) return;
    void loadExisting();
  }, [platform, loadExisting]);

  useEffect(() => {
    if (!platform) return;
    void loadStatus();
  }, [platform]);

  useEffect(() => {
    if (!platform) return;
    const incomingProviderKey = (params.get('provider_key') || '').trim();
    const incomingEngine = (params.get('engine') || '').trim();
    const isNew = (params.get('new') || '').trim() === '1';
    if (incomingProviderKey) {
      setProviderSlug(incomingProviderKey);
      if (isNew) {
        setProviderKey(autoManagedIntegrationKey(incomingProviderKey));
      } else {
        setProviderKey(incomingProviderKey);
      }
    }
    if (incomingEngine === 'nango' || incomingEngine === 'omini_connect_native') {
      setEngine(incomingEngine);
    } else if (incomingProviderKey) {
      // Provider key in URL comes from managed library flow.
      // Use Nango for all managed integrations (including API_KEY/BASIC auth providers).
      setEngine('nango');
    } else if (platform === 'github' && !incomingProviderKey) {
      // GitHub has native PAT support via engine=omini_connect_native (proxy uses PAT directly).
      // Use native by default; user can switch to nango manually if they prefer Nango Connect.
      setEngine('omini_connect_native');
    } else if (!PLATFORMS[platform]) {
      // Unknown platform ids are managed connectors created from library entries.
      setEngine('nango');
    } else {
      setEngine('omini_connect_native');
    }
  }, [platform, params]);

  useEffect(() => {
    getIntegrationCatalog()
      .then((rows) => {
        const SKIP_AUTH_MODES = new Set(['JWT', 'APP', 'CUSTOM', 'APP_STORE', 'INSTALL_PLUGIN', 'NONE']);
        const filtered = normalizeCatalogResponse(rows).filter(
          (r) => !r.auth_mode || !SKIP_AUTH_MODES.has(r.auth_mode.toUpperCase())
        );
        setCatalogRows(filtered);
      })
      .catch(() => setCatalogRows([]));
  }, []);

  async function loadStatus() {
    if (!platform) return;
    try {
      const s = await getConnectorStatus(platform);
      setConnected(s.connected);
    } catch {}
  }

  async function persistConnectorConfig(newConnectionRef?: string): Promise<boolean> {
    if (!platform) return false;
    setSaving(true);
    setError('');
    try {
      // Only send client_secret if user actually entered a new value
      // If field is empty and hasExistingSecret is true, backend preserves the existing one
      await upsertConnector({
        platform,
        client_id: clientId,
        client_secret: clientSecret,
        redirect_uri: redirectUri,
        scopes: isOAuth2 ? scopes.split(' ').filter(Boolean) : [],
        engine,
        provider_key: providerKey || platform,
        connection_ref: newConnectionRef ?? connectionRef,
        agent_id: agentId,
        enabled,
      });
      await loadStatus();
      setHasExistingConnector(true);
      // Refresh to show empty field again after save
      setHasExistingSecret(true);
      setClientSecret('');
      return true;
    } catch (err) {
      if (err instanceof ConnectorConflictError && err.existing_platform.trim()) {
        navigate(`/connectors/${encodeURIComponent(err.existing_platform.trim())}`, { replace: true });
        return false;
      }
      setError(err instanceof Error ? err.message : 'Connect failed');
      return false;
    } finally {
      setSaving(false);
    }
  }

  /**
   * Open Nango Connect popup and poll until the connection is established.
   * Used for OAUTH2, OAUTH2_CC, TWO_STEP, and API_KEY/BASIC without portal-entered credentials.
   */
  async function openNangoConnectAndWaitForConnection(platform: string): Promise<void> {
    const { connect_url } = await createNangoConnectSession(platform);
    const popup = window.open(connect_url, '_blank', 'noopener,noreferrer');
    if (!popup) {
      throw new Error('Popup was blocked. Allow popups for this site and try again.');
    }

    // Poll until popup closes
    await new Promise<void>((resolve) => {
      const check = setInterval(() => {
        if (popup.closed) {
          clearInterval(check);
          resolve();
        }
      }, 2000);
    });

    // Popup closed — find the new connection
    const connections = await listNangoConnections(platform);
    const connector = await getConnectors().then(list => list.find(c => c.platform === platform));
    const providerKey = connector?.provider_key || platform;
    const latest = connections
      .filter(c => c.provider_config_key === providerKey)
      .sort((a, b) => {
        const ta = a.created || '';
        const tb = b.created || '';
        return tb.localeCompare(ta);
      })[0];

    if (latest) {
      await persistConnectorConfig(latest.connection_id);
    }
  }

  /**
   * Save credentials, then:
   * - Nango + OAuth: open Nango Connect (OAuth).
   * - Nango + API key / Basic with credentials provided: save directly, skip Nango Connect UI.
   * - Nango + API key / Basic without credentials: open Nango Connect to enter token.
   * - Native OAuth: redirect to portal OAuth.
   * - Native API key (e.g. Maton, qqmail): save only — no Nango or OAuth redirect.
   */
  async function handleConnect() {
    if (!platform) return;
    console.log('DEBUG handleConnect ENTRY', { engine, isOAuth2, isApiKeyStyle, isBasicStyle, clientIdTrimmed: clientId.trim(), providerKey, providerSlug });
    if (!isOAuth2) {
      // BILL and TBA always go through Nango Connect popup (complex auth flows).
      if (engine === 'nango' && (isBILL || isTBA)) {
        const ok = await persistConnectorConfig();
        if (!ok) return;
        try {
          await openNangoConnectAndWaitForConnection(platform);
        } catch (err) {
          setError(err instanceof Error ? err.message : 'Failed to open connection wizard');
        }
        navigate('/');
        return;
      }

      // For engine=nango + API_KEY/BASIC/OAUTH2_CC with credentials: create Nango connection directly, no UI needed.
      // For engine=nango + API_KEY/BASIC/OAUTH2_CC without credentials: open Nango Connect UI.
      // For engine=omini_connect_native: save directly, use via proxy.
      if (engine === 'nango' && (isApiKeyStyle || isBasicStyle || isOAuth2CC || isSignature) && (clientId.trim() || clientSecret.trim())) {
        // Build credentials object for API_KEY, BASIC, OAUTH2_CC, or SIGNATURE
        const auth_mode = isApiKeyStyle ? 'API_KEY' : isBasicStyle ? 'BASIC' : isOAuth2CC ? 'OAUTH2_CC' : 'SIGNATURE';
        try {
          console.log('DEBUG: creating Nango connection directly with auth_mode=', auth_mode);
          const result = await createNangoConnectionDirect({
            platform,
            auth_mode,
            api_key: isApiKeyStyle ? clientId.trim() : isOAuth2CC ? clientId.trim() : undefined,
            username: (isBasicStyle || isSignature) ? clientId.trim() : undefined,
            password: (isBasicStyle || isOAuth2CC || isSignature) ? clientSecret.trim() : undefined,
          });
          console.log('DEBUG: Nango connection created, connection_id=', result.connection_id);
          const ok = await persistConnectorConfig(result.connection_id);
          if (!ok) return;
          navigate('/');
        } catch (err) {
          setError(err instanceof Error ? err.message : 'Failed to create connection in Nango');
          return;
        }
      } else {
        const ok = await persistConnectorConfig();
        if (!ok) return;
        if (engine === 'nango' && !clientId.trim()) {
          console.log('DEBUG BRANCH: open Nango Connect UI — engine=nango, no credentials, providerKey=', providerKey);
          try {
            await openNangoConnectAndWaitForConnection(platform);
          } catch (err) {
            setError(err instanceof Error ? err.message : 'Failed to open connection wizard');
          }
          navigate('/');
          return;
        } else {
          console.log('DEBUG BRANCH: skip Nango Connect — engine=nango but credentials provided, saved via persistConnectorConfig, providerKey=', providerKey);
        }
        navigate('/');
      }
      return;
    }
    const ok = await persistConnectorConfig();
    if (!ok) return;
    if (engine === 'nango') {
      try {
        await openNangoConnectAndWaitForConnection(platform);
      } catch (err) {
        setError(err instanceof Error ? err.message : 'Failed to open connection wizard');
      }
      navigate('/');
    } else {
      window.location.href = `/oauth/${encodeURIComponent(platform)}`;
    }
  }

  function handleFormSubmit(e: React.FormEvent) {
    e.preventDefault();
    void handleConnect();
  }

  async function handleTest() {
    if (!platform) return;
    setTesting(true);
    setTestResult(null);
    try {
      const result = await testConnector(platform);
      setTestResult(result);
    } catch (err) {
      setTestResult({ status: 'error', message: err instanceof Error ? err.message : 'Test failed' });
    } finally {
      setTesting(false);
    }
  }

  async function handleDelete() {
    if (!platform) return;
    if (!confirm('Remove this connector?')) return;
    try {
      await deleteConnector(platform);
      navigate('/');
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Delete failed');
    }
  }

  if (!platform) {
    return <div style={{ padding: '2rem' }}>Missing connector id</div>;
  }

  const meta = info!;
  /** Match catalog row for managed slugs (`linkedin-abc`) and built-in ids (`linkedin`). */
  const catalogRowForConnector = useMemo(() => {
    const slug = (providerSlug || '').trim();
    if (slug) {
      const bySlug = catalogRows.find((r) => r.name === slug);
      if (bySlug) return bySlug;
    }
    if (platform) {
      const exact = catalogRows.find((r) => r.name === platform);
      if (exact) return exact;
      return catalogRows.find((r) => platform.startsWith(`${r.name}-`));
    }
    return undefined;
  }, [catalogRows, providerSlug, platform]);

  const providerDocUrl = catalogRowForConnector?.docs;
  const catalogNameToLogo = new Map(
    catalogRows
      .filter((r) => (r.logo_url || '').trim())
      .map((r) => [r.name.toLowerCase().trim(), (r.logo_url || '').trim() as string])
  );
  const configHeaderLogo = (() => {
    const rowUrl = (catalogRowForConnector?.logo_url || '').trim();
    if (rowUrl) return rowUrl;
    const b = findBuiltinByConnectorId(platform);
    if (b) {
      return resolveBuiltinCatalogLogo(b.id, catalogNameToLogo, b.logo_url);
    }
    return platform ? PLATFORMS[platform]?.logo_url : undefined;
  })();
  const catalogManagedSlugUi = Boolean(platform && !(platform in PLATFORMS));
  const showManagedHubOauthCopy =
    engine === 'nango' ||
    (providerKey || '').includes('__') ||
    catalogManagedSlugUi ||
    redirectPathLooksLikeLegacyManagedPerSlugCallback(redirectUri);

  return (
    <div style={{ minHeight: '100vh', background: '#f5f5f5' }}>
      <header style={{ background: 'white', borderBottom: '1px solid #e0e0e0', padding: '0 1.5rem', display: 'flex', alignItems: 'center', gap: '1rem' }}>
        <button onClick={() => navigate(-1)} style={{ background: 'none', border: 'none', cursor: 'pointer', fontSize: '1rem', color: '#666', padding: '1rem 0' }}>← Back</button>
        <IntegrationProviderLogo url={configHeaderLogo} label={meta.name} size={40} />
        <h1 style={{ margin: 0, fontSize: '1.125rem', color: '#333' }}>{meta.name}</h1>
        {connected && <span style={{ marginLeft: 'auto', padding: '0.125rem 0.5rem', borderRadius: '9999px', fontSize: '0.75rem', fontWeight: 500, background: '#dcfce7', color: '#166534' }}>Connected</span>}
      </header>

      {connected && (
        <div style={{ background: '#f0fdf4', borderBottom: '1px solid #bbf7d0', padding: '1rem 2rem' }}>
          <div style={{ maxWidth: '600px', margin: '0 auto' }}>
            <div style={{ fontSize: '0.75rem', fontWeight: 600, color: '#166534', marginBottom: '0.5rem', textTransform: 'uppercase', letterSpacing: '0.05em' }}>
              Connection Info
            </div>
            <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(200px, 1fr))', gap: '0.75rem' }}>
              <div>
                <div style={{ fontSize: '0.7rem', color: '#6b7280', marginBottom: '0.125rem' }}>Auth Mode</div>
                <div style={{ fontSize: '0.8rem', color: '#1f2937' }}>{catalogRowForConnector?.auth_mode || (isGitHubPatPlatform ? 'API_KEY' : 'OAuth2')}</div>
              </div>
              {providerDocUrl && (
                <div>
                  <div style={{ fontSize: '0.7rem', color: '#6b7280', marginBottom: '0.125rem' }}>Integration Guide</div>
                  <a href={providerDocUrl} target="_blank" rel="noreferrer" style={{ fontSize: '0.8rem', color: '#2563eb', display: 'flex', alignItems: 'center', gap: '0.25rem' }}>
                    View docs ↗
                  </a>
                </div>
              )}
            </div>
            {catalogRowForConnector?.available_scopes && catalogRowForConnector!.available_scopes!.length > 0 && (
              <div style={{ marginTop: '0.75rem' }}>
                <div style={{ fontSize: '0.7rem', color: '#6b7280', marginBottom: '0.25rem' }}>Available Scopes</div>
                <div style={{ display: 'flex', flexWrap: 'wrap', gap: '0.25rem' }}>
                  {catalogRowForConnector!.available_scopes!.map(scope => (
                    <span key={scope} style={{ fontSize: '0.7rem', padding: '0.125rem 0.375rem', background: '#dcfce7', color: '#166534', borderRadius: '4px' }}>{scope}</span>
                  ))}
                </div>
              </div>
            )}
            {scopes.trim() && (
              <div style={{ marginTop: '0.75rem' }}>
                <div style={{ fontSize: '0.7rem', color: '#6b7280', marginBottom: '0.25rem' }}>Configured Scopes</div>
                <div style={{ display: 'flex', flexWrap: 'wrap', gap: '0.25rem' }}>
                  {scopes.split(' ').filter(Boolean).map(scope => (
                    <span key={scope} style={{ fontSize: '0.7rem', padding: '0.125rem 0.375rem', background: '#dbeafe', color: '#1e40af', borderRadius: '4px' }}>{scope}</span>
                  ))}
                </div>
              </div>
            )}
          </div>
        </div>
      )}

      <main style={{ padding: '2rem', maxWidth: '600px' }}>
        <form onSubmit={handleFormSubmit} style={{ background: 'white', borderRadius: '8px', padding: '1.5rem', boxShadow: '0 1px 4px rgba(0,0,0,0.08)' }}>
          <h2 style={{ margin: '0 0 1.5rem', fontSize: '1rem', color: '#333' }}>
            {isApiKeyStyle ? 'API Key Configuration' : isBasicStyle ? 'Basic Authentication' : isOAuth2CC ? 'OAuth 2.0 Client Credentials' : isSignature ? 'Signature Authentication (WSSE)' : isOAuth2 ? 'OAuth Configuration' : isQQMail ? 'Corp ID / Secret' : isGitHubPatPlatform ? 'GitHub (personal access token)' : 'API Key Configuration'}
          </h2>
          {error && <div style={{ color: '#d32f2f', marginBottom: '1rem', fontSize: '0.875rem' }}>{error}</div>}

          {(isBasicStyle || isBasicPlatform || isSignature) ? (
            <>
              <div style={{ marginBottom: '1rem' }}>
                <label style={{ display: 'block', marginBottom: '0.375rem', fontSize: '0.875rem', color: '#666' }}>Username</label>
                <input type="text" value={clientId} onChange={e => setClientId(e.target.value)} required placeholder="Username or email" style={{ width: '100%', padding: '0.5rem', border: '1px solid #ccc', borderRadius: '4px', boxSizing: 'border-box' }} />
              </div>
              <div style={{ marginBottom: '1rem' }}>
                <label style={{ display: 'block', marginBottom: '0.375rem', fontSize: '0.875rem', color: '#666' }}>Password</label>
                <div style={{ position: 'relative', display: 'flex' }}>
                  <input type={showClientSecret ? 'text' : 'password'} value={clientSecret} onChange={e => setClientSecret(e.target.value)} required placeholder={hasExistingSecret ? '••••••••' : ''} style={{ flex: 1, padding: '0.5rem', paddingRight: '2.5rem', border: '1px solid #ccc', borderRadius: '4px', boxSizing: 'border-box' }} />
                  <button type="button" onClick={() => setShowClientSecret(v => !v)} style={{ position: 'absolute', right: '0.5rem', top: '50%', transform: 'translateY(-50%)', background: 'none', border: 'none', cursor: 'pointer', fontSize: '1rem', padding: '0.25rem' }}>
                    {showClientSecret ? '👁' : '👁‍🗨'}
                  </button>
                </div>
                {hasExistingSecret && (
                  <p style={{ margin: '0.25rem 0 0', fontSize: '0.75rem', color: '#999' }}>Leave empty to keep existing password, or enter new value to change.</p>
                )}
              </div>
            </>
          ) : isOAuth2CC ? (
            <>
              {providerDocUrl && (
                <div style={{ marginBottom: '1rem', fontSize: '0.875rem' }}>
                  <a href={providerDocUrl} target="_blank" rel="noreferrer" style={{ color: '#2563eb' }}>
                    Provider documentation ↗
                  </a>
                </div>
              )}
              <div style={{ marginBottom: '1rem' }}>
                <label style={{ display: 'block', marginBottom: '0.375rem', fontSize: '0.875rem', color: '#666' }}>Client ID</label>
                <div style={{ position: 'relative', display: 'flex' }}>
                  <input type={showClientId ? 'text' : 'password'} value={clientId} onChange={e => setClientId(e.target.value)} required placeholder="Client ID" style={{ flex: 1, padding: '0.5rem', paddingRight: '2.5rem', border: '1px solid #ccc', borderRadius: '4px', boxSizing: 'border-box' }} />
                  <button type="button" onClick={() => setShowClientId(v => !v)} style={{ position: 'absolute', right: '0.5rem', top: '50%', transform: 'translateY(-50%)', background: 'none', border: 'none', cursor: 'pointer', fontSize: '1rem', padding: '0.25rem' }}>
                    {showClientId ? '👁' : '👁‍🗨'}
                  </button>
                </div>
              </div>
              <div style={{ marginBottom: '1rem' }}>
                <label style={{ display: 'block', marginBottom: '0.375rem', fontSize: '0.875rem', color: '#666' }}>Client Secret</label>
                <div style={{ position: 'relative', display: 'flex' }}>
                  <input type={showClientSecret ? 'text' : 'password'} value={clientSecret} onChange={e => setClientSecret(e.target.value)} required placeholder={hasExistingSecret ? '••••••••' : ''} style={{ flex: 1, padding: '0.5rem', paddingRight: '2.5rem', border: '1px solid #ccc', borderRadius: '4px', boxSizing: 'border-box' }} />
                  <button type="button" onClick={() => setShowClientSecret(v => !v)} style={{ position: 'absolute', right: '0.5rem', top: '50%', transform: 'translateY(-50%)', background: 'none', border: 'none', cursor: 'pointer', fontSize: '1rem', padding: '0.25rem' }}>
                    {showClientSecret ? '👁' : '👁‍🗨'}
                  </button>
                </div>
                {hasExistingSecret && (
                  <p style={{ margin: '0.25rem 0 0', fontSize: '0.75rem', color: '#999' }}>Leave empty to keep existing secret, or enter new value to change.</p>
                )}
              </div>
            </>
          ) : isApiKeyStyle || (!isOAuth2 && !isQQMail) ? (
            <>
              {providerDocUrl && (
                <div style={{ marginBottom: '1rem', fontSize: '0.875rem' }}>
                  <a href={providerDocUrl} target="_blank" rel="noreferrer" style={{ color: '#2563eb' }}>
                    Provider documentation ↗
                  </a>
                </div>
              )}
              <div style={{ marginBottom: '1rem' }}>
                <label style={{ display: 'block', marginBottom: '0.375rem', fontSize: '0.875rem', color: '#666' }}>{isGitHubPatPlatform ? 'Personal access token' : 'API Key'}</label>
                <div style={{ position: 'relative', display: 'flex' }}>
                  <input
                    type={showClientId ? 'text' : 'password'}
                    value={clientId}
                    onChange={e => setClientId(e.target.value)}
                    required
                    placeholder={isGitHubPatPlatform ? 'ghp_… or github_pat_…' : 'sk-...'}
                    style={{ flex: 1, padding: '0.5rem', paddingRight: '2.5rem', border: '1px solid #ccc', borderRadius: '4px', boxSizing: 'border-box', fontFamily: 'monospace' }}
                  />
                  <button type="button" onClick={() => setShowClientId(v => !v)} style={{ position: 'absolute', right: '0.5rem', top: '50%', transform: 'translateY(-50%)', background: 'none', border: 'none', cursor: 'pointer', fontSize: '1rem', padding: '0.25rem' }}>
                    {showClientId ? '👁' : '👁‍🗨'}
                  </button>
                </div>
                <p style={{ margin: '0.25rem 0 0', fontSize: '0.75rem', color: '#999' }}>
                  {isGitHubPatPlatform
                    ? 'Create a classic or fine-grained token in GitHub → Settings → Developer settings. It is sent as Authorization: Bearer to api.github.com.'
                    : 'Enter your API key from the provider.'}
                </p>
              </div>
            </>
          ) : isOAuth2 ? (
            <>
              {providerDocUrl && (
                <div style={{ marginBottom: '1rem', fontSize: '0.875rem' }}>
                  <a href={providerDocUrl} target="_blank" rel="noreferrer" style={{ color: '#2563eb' }}>
                    Provider documentation ↗
                  </a>
                </div>
              )}

              <div style={{ marginBottom: '1rem' }}>
                <label style={{ display: 'block', marginBottom: '0.375rem', fontSize: '0.875rem', color: '#666' }}>Client ID</label>
                <div style={{ position: 'relative', display: 'flex' }}>
                  <input type={showClientId ? 'text' : 'password'} value={clientId} onChange={e => setClientId(e.target.value)} required={!showManagedHubOauthCopy} style={{ flex: 1, padding: '0.5rem', paddingRight: '2.5rem', border: '1px solid #ccc', borderRadius: '4px', boxSizing: 'border-box' }} />
                  <button type="button" onClick={() => setShowClientId(v => !v)} style={{ position: 'absolute', right: '0.5rem', top: '50%', transform: 'translateY(-50%)', background: 'none', border: 'none', cursor: 'pointer', fontSize: '1rem', padding: '0.25rem' }}>
                    {showClientId ? '👁' : '👁‍🗨'}
                  </button>
                </div>
              </div>

              <div style={{ marginBottom: '1rem' }}>
                <label style={{ display: 'block', marginBottom: '0.375rem', fontSize: '0.875rem', color: '#666' }}>Client Secret</label>
                <div style={{ position: 'relative', display: 'flex' }}>
                  <input
                    type="password"
                    value={clientSecret}
                    onChange={e => setClientSecret(e.target.value)}
                    placeholder={hasExistingSecret ? '••••••••' : ''}
                    style={{ flex: 1, padding: '0.5rem', paddingRight: '2.5rem', border: '1px solid #ccc', borderRadius: '4px', boxSizing: 'border-box' }}
                  />
                  <span style={{ position: 'absolute', right: '0.5rem', top: '50%', transform: 'translateY(-50%)', fontSize: '1rem', padding: '0.25rem', color: '#999' }}>
                    {hasExistingSecret ? '🔒' : ''}
                  </span>
                </div>
                {hasExistingSecret && (
                  <p style={{ margin: '0.25rem 0 0', fontSize: '0.75rem', color: '#999' }}>Leave empty to keep existing secret, or enter new value to change.</p>
                )}
              </div>

              <div style={{ marginBottom: '1rem' }}>
                <label style={{ display: 'block', marginBottom: '0.375rem', fontSize: '0.875rem', color: '#666' }}>Redirect URI</label>
                <input type="text" value={redirectUri} onChange={e => setRedirectUri(e.target.value)} required={!showManagedHubOauthCopy} style={{ width: '100%', padding: '0.5rem', border: '1px solid #ccc', borderRadius: '4px', boxSizing: 'border-box', fontFamily: 'monospace', fontSize: '0.8125rem' }} />
                <p style={{ margin: '0.25rem 0 0', fontSize: '0.75rem', color: '#999' }}>Register this URI in the {meta.name} app or developer console.</p>
              </div>

              <div style={{ marginBottom: '1rem' }}>
                <label style={{ display: 'block', marginBottom: '0.375rem', fontSize: '0.875rem', color: '#666' }}>Scopes (space-separated)</label>
                <ScopeSelector
                  value={scopes}
                  providerKey={(providerSlug || platform)}
                  availableScopes={catalogRowForConnector?.available_scopes}
                  onChange={setScopes}
                />
              </div>

              {showManagedHubOauthCopy && (
                <div style={{ marginBottom: '1rem' }}>
                  <label style={{ display: 'block', marginBottom: '0.375rem', fontSize: '0.875rem', color: '#666' }}>Connection</label>
                  {connectionRef.trim() ? (
                    <div style={{ padding: '0.55rem 0.65rem', border: '1px solid #bbf7d0', background: '#f0fdf4', borderRadius: '6px', color: '#166534', fontSize: '0.82rem' }}>
                      Reference: <code>{connectionRef}</code>
                    </div>
                  ) : (
                    <div style={{ padding: '0.55rem 0.65rem', border: '1px solid #e5e7eb', background: '#f9fafb', borderRadius: '6px', color: '#6b7280', fontSize: '0.82rem' }}>
                      Not connected
                    </div>
                  )}
                </div>
              )}

              {isWeChatWork && (
                <div style={{ marginBottom: '1rem' }}>
                  <label style={{ display: 'block', marginBottom: '0.375rem', fontSize: '0.875rem', color: '#666' }}>Agent ID</label>
                  <input type="text" value={agentId} onChange={e => setAgentId(e.target.value)} placeholder="1000001" style={{ width: '100%', padding: '0.5rem', border: '1px solid #ccc', borderRadius: '4px', boxSizing: 'border-box' }} />
                  <p style={{ margin: '0.25rem 0 0', fontSize: '0.75rem', color: '#999' }}>Found in WeChat Work app settings (数字形式的AgentId).</p>
                </div>
              )}
            </>
          ) : isQQMail ? (
            <>
              <div style={{ marginBottom: '1rem' }}>
                <label style={{ display: 'block', marginBottom: '0.375rem', fontSize: '0.875rem', color: '#666' }}>Corp ID</label>
                <div style={{ position: 'relative', display: 'flex' }}>
                  <input type={showClientId ? 'text' : 'password'} value={clientId} onChange={e => setClientId(e.target.value)} required style={{ flex: 1, padding: '0.5rem', paddingRight: '2.5rem', border: '1px solid #ccc', borderRadius: '4px', boxSizing: 'border-box' }} />
                  <button type="button" onClick={() => setShowClientId(v => !v)} style={{ position: 'absolute', right: '0.5rem', top: '50%', transform: 'translateY(-50%)', background: 'none', border: 'none', cursor: 'pointer', fontSize: '1rem', padding: '0.25rem' }}>
                    {showClientId ? '👁' : '👁‍🗨'}
                  </button>
                </div>
              </div>

              <div style={{ marginBottom: '1rem' }}>
                <label style={{ display: 'block', marginBottom: '0.375rem', fontSize: '0.875rem', color: '#666' }}>Corp Secret</label>
                <div style={{ position: 'relative', display: 'flex' }}>
                  <input type={showClientSecret ? 'text' : 'password'} value={clientSecret} onChange={e => setClientSecret(e.target.value)} required style={{ flex: 1, padding: '0.5rem', paddingRight: '2.5rem', border: '1px solid #ccc', borderRadius: '4px', boxSizing: 'border-box' }} />
                  <button type="button" onClick={() => setShowClientSecret(v => !v)} style={{ position: 'absolute', right: '0.5rem', top: '50%', transform: 'translateY(-50%)', background: 'none', border: 'none', cursor: 'pointer', fontSize: '1rem', padding: '0.25rem' }}>
                    {showClientSecret ? '👁' : '👁‍🗨'}
                  </button>
                </div>
                <p style={{ margin: '0.25rem 0 0', fontSize: '0.75rem', color: '#999' }}>Found in QQ Enterprise Mail management console → Application → Secret.</p>
              </div>
            </>
          ) : (
            <>
              <div style={{ marginBottom: '1rem' }}>
                <label style={{ display: 'block', marginBottom: '0.375rem', fontSize: '0.875rem', color: '#666' }}>API Key</label>
                <div style={{ position: 'relative', display: 'flex' }}>
                  <input type={showClientId ? 'text' : 'password'} value={clientId} onChange={e => setClientId(e.target.value)} required placeholder="sk-..." style={{ flex: 1, padding: '0.5rem', paddingRight: '2.5rem', border: '1px solid #ccc', borderRadius: '4px', boxSizing: 'border-box', fontFamily: 'monospace' }} />
                  <button type="button" onClick={() => setShowClientId(v => !v)} style={{ position: 'absolute', right: '0.5rem', top: '50%', transform: 'translateY(-50%)', background: 'none', border: 'none', cursor: 'pointer', fontSize: '1rem', padding: '0.25rem' }}>
                    {showClientId ? '👁' : '👁‍🗨'}
                  </button>
                </div>
                <p style={{ margin: '0.25rem 0 0', fontSize: '0.75rem', color: '#999' }}>Get your API key from the provider.</p>
              </div>
            </>
          )}

          <div style={{ marginBottom: '1.5rem', display: 'flex', alignItems: 'center', gap: '0.5rem' }}>
            {hasExistingConnector && (
              <>
                <input type="checkbox" id="enabled" checked={enabled} onChange={e => setEnabled(e.target.checked)} />
                <label htmlFor="enabled" style={{ fontSize: '0.875rem', color: '#666' }}>Enabled</label>
              </>
            )}
          </div>

          <div style={{ display: 'flex', gap: '0.75rem', flexWrap: 'wrap' }}>
            <button
              type="submit"
              disabled={saving}
              style={{ padding: '0.5rem 1.25rem', background: '#e0e7ff', color: '#3730a3', border: '1px solid #a5b4fc', borderRadius: '4px', cursor: 'pointer' }}
            >
              {saving ? 'Applying…' : 'Connect'}
            </button>
            {showTestButton && (
              <button
                type="button"
                onClick={handleTest}
                disabled={testButtonDisabled}
                style={{ padding: '0.5rem 1.25rem', background: '#f5f5f5', color: '#333', border: '1px solid #ccc', borderRadius: '4px', cursor: 'pointer' }}
              >
                {testing ? 'Testing...' : 'Test Connection'}
              </button>
            )}
            <button type="button" onClick={handleDelete} style={{ padding: '0.5rem 1.25rem', background: 'white', color: '#d32f2f', border: '1px solid #d32f2f', borderRadius: '4px', cursor: 'pointer', marginLeft: 'auto' }}>
              Remove
            </button>
          </div>

          {testResult && (
            <div style={{ marginTop: '1rem', padding: '0.75rem', borderRadius: '4px', background: testResult.status === 'ok' ? '#dcfce7' : '#fef3c7', color: testResult.status === 'ok' ? '#166534' : '#92400e', fontSize: '0.875rem' }}>
              {testResult.message}
            </div>
          )}
        </form>
      </main>
    </div>
  );
}
