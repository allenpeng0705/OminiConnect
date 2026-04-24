import { useEffect, useMemo, useState } from 'react';
import { Link, useNavigate } from 'react-router-dom';
import CatalogProviderCard from '../components/CatalogProviderCard';
import { getConnectors, getConnectorStatus, getIntegrationCatalog, getNangoStatus, logout, type IntegrationCatalogRow } from '../api/client';
import { normalizeCatalogResponse, providerSearchBlob } from '../lib/integrationCatalogNormalize';

interface ConnectorInfo {
  platform: string;
  client_id: string;
  redirect_uri: string;
  scopes: string[];
  /** `nango` = managed hub (Nango Connect under the hood). */
  engine?: string;
  enabled: boolean;
  connected?: boolean;
  configured?: boolean;
}

const PLATFORMS = [
  { id: 'feishu', name: 'Feishu / Lark', color: '#00A1E0', type: 'oauth2' },
  { id: 'dingtalk', name: 'DingTalk', color: '#1677FF', type: 'oauth2' },
  { id: 'wechatwork', name: 'WeChat Work', color: '#07C160', type: 'oauth2' },
  { id: 'linkedin', name: 'LinkedIn', color: '#0A66C2', type: 'oauth2' },
  { id: 'facebook', name: 'Facebook', color: '#1877F2', type: 'oauth2' },
  { id: 'x', name: 'X (Twitter)', color: '#111111', type: 'oauth2' },
  { id: 'maton', name: 'Maton.ai', color: '#6366F1', type: 'api_key' },
  { id: 'qqmail', name: 'QQ Enterprise Mail', color: '#12B7F5', type: 'api_key' },
];

const CATALOG_HOME_PREVIEW = 36;

export default function Dashboard() {
  const navigate = useNavigate();
  const [connectors, setConnectors] = useState<ConnectorInfo[]>([]);
  const [loading, setLoading] = useState(true);
  const [username, setUsername] = useState('');
  const [catalogRows, setCatalogRows] = useState<IntegrationCatalogRow[]>([]);
  const [catalogLoading, setCatalogLoading] = useState(true);
  const [catalogError, setCatalogError] = useState('');
  const [catalogFilter, setCatalogFilter] = useState('');
  const [nangoBaseUrl, setNangoBaseUrl] = useState('');

  useEffect(() => {
    loadData();
    loadMe();
    void getNangoStatus()
      .then((s) => setNangoBaseUrl((s.base_url || '').trim().replace(/\/+$/, '')))
      .catch(() => setNangoBaseUrl(''));
  }, []);

  useEffect(() => {
    let cancelled = false;
    void (async () => {
      setCatalogLoading(true);
      setCatalogError('');
      try {
        const data = await getIntegrationCatalog();
        if (!cancelled) setCatalogRows(normalizeCatalogResponse(data));
      } catch (e) {
        if (!cancelled) {
          setCatalogError(e instanceof Error ? e.message : 'Failed to load catalog');
          setCatalogRows([]);
        }
      } finally {
        if (!cancelled) setCatalogLoading(false);
      }
    })();
    return () => {
      cancelled = true;
    };
  }, []);

  const catalogFiltered = useMemo(() => {
    const q = catalogFilter.trim().toLowerCase();
    if (!q) return catalogRows;
    return catalogRows.filter((p) => providerSearchBlob(p).includes(q));
  }, [catalogRows, catalogFilter]);

  const catalogPreview = catalogFiltered.slice(0, CATALOG_HOME_PREVIEW);

  async function loadMe() {
    try {
      const me = await fetch('/auth/me').then(r => r.json()).catch(() => null);
      if (me?.username) setUsername(me.username);
    } catch {}
  }

  async function loadData() {
    setLoading(true);
    try {
      const list = await getConnectors();
      const withStatus = await Promise.all(
        list.map(async (c) => {
          try {
            const status = await getConnectorStatus(c.platform);
            return { ...c, connected: status.connected, configured: status.configured };
          } catch {
            return { ...c, connected: false, configured: false };
          }
        })
      );
      setConnectors(withStatus);
    } catch (err) {
      console.error(err);
    } finally {
      setLoading(false);
    }
  }

  async function handleLogout() {
    await logout();
    window.location.href = '/auth/login';
  }

  function handleConnect(platform: string, engine?: string) {
    const p = PLATFORMS.find((x) => x.id === platform);
    if (p?.type === 'api_key') {
      return;
    }
    if (engine === 'nango') {
      navigate(`/connectors/${encodeURIComponent(platform)}/connect`);
      return;
    }
    window.location.href = `/oauth/${platform}`;
  }

  const configuredPlatforms = new Set(connectors.map(c => c.platform));

  return (
    <div style={{ minHeight: '100vh', background: '#f5f5f5' }}>
      {/* Header */}
      <header style={{ background: 'white', borderBottom: '1px solid #e0e0e0', padding: '0 1.5rem', display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
        <h1 style={{ margin: 0, fontSize: '1.25rem', color: '#333' }}>OminiConnect Portal</h1>
        <div style={{ display: 'flex', alignItems: 'center', gap: '1rem' }}>
          <span style={{ color: '#666', fontSize: '0.875rem' }}>{username}</span>
          <Link to="/api-keys" style={{ padding: '0.375rem 0.75rem', background: '#f5f5f5', border: '1px solid #ccc', borderRadius: '4px', cursor: 'pointer', fontSize: '0.875rem', textDecoration: 'none', color: '#333' }}>API Keys</Link>
          <button onClick={handleLogout} style={{ padding: '0.375rem 0.75rem', background: '#f5f5f5', border: '1px solid #ccc', borderRadius: '4px', cursor: 'pointer', fontSize: '0.875rem' }}>Logout</button>
        </div>
      </header>

      <main style={{ padding: '2rem' }}>
        <section style={{ marginBottom: '1.75rem' }}>
          <div
            style={{
              display: 'flex',
              flexWrap: 'wrap',
              alignItems: 'baseline',
              justifyContent: 'space-between',
              gap: '0.75rem',
              marginBottom: '0.75rem',
            }}
          >
            <h2 style={{ fontSize: '1.125rem', color: '#333', margin: 0 }}>Integration library</h2>
            <Link to="/connectors/catalog" style={{ fontSize: '0.875rem', color: '#4f46e5', fontWeight: 500 }}>
              Full library & search →
            </Link>
          </div>
          {nangoBaseUrl && (
            <div style={{ marginBottom: '0.75rem', display: 'flex', gap: '0.5rem', flexWrap: 'wrap' }}>
              <a
                href={`${nangoBaseUrl}/integrations`}
                target="_blank"
                rel="noreferrer"
                style={{
                  display: 'inline-block',
                  padding: '0.35rem 0.7rem',
                  borderRadius: '6px',
                  border: '1px solid #cbd5e1',
                  background: '#fff',
                  color: '#334155',
                  fontSize: '0.8rem',
                  textDecoration: 'none',
                }}
              >
                Manage in Nango portal
              </a>
            </div>
          )}
          <p style={{ margin: '0 0 1rem', fontSize: '0.875rem', color: '#666', lineHeight: 1.45 }}>
            Loaded automatically from your managed hub. Filter here or open the full page for paging and refresh.
          </p>
          <div style={{ marginBottom: '1rem', maxWidth: '420px' }}>
            <input
              type="search"
              placeholder="Filter by name, category, auth…"
              value={catalogFilter}
              onChange={(e) => setCatalogFilter(e.target.value)}
              disabled={catalogLoading || !!catalogError || catalogRows.length === 0}
              style={{
                width: '100%',
                padding: '0.5rem 0.75rem',
                borderRadius: '8px',
                border: '1px solid #ccc',
                fontSize: '0.875rem',
                boxSizing: 'border-box',
              }}
            />
          </div>
          {catalogLoading && <p style={{ color: '#666', fontSize: '0.875rem' }}>Loading integration catalog…</p>}
          {catalogError && (
            <div
              style={{
                padding: '0.85rem 1rem',
                borderRadius: '8px',
                background: '#fef2f2',
                color: '#b91c1c',
                fontSize: '0.875rem',
                marginBottom: '0.75rem',
              }}
            >
              {catalogError}
              <div style={{ marginTop: '0.5rem', color: '#64748b', fontSize: '0.8rem' }}>
                Fix <code style={{ background: '#fee2e2', padding: '0.1rem 0.3rem', borderRadius: '4px' }}>NANGO_BASE_URL</code> in repo-root{' '}
                <code style={{ background: '#fee2e2', padding: '0.1rem 0.3rem', borderRadius: '4px' }}>.env</code> and ensure Nango is running, then refresh this page.
              </div>
            </div>
          )}
          {!catalogLoading && !catalogError && catalogRows.length === 0 && (
            <p style={{ color: '#666', fontSize: '0.875rem' }}>No providers returned. Check hub configuration or open the full library to retry.</p>
          )}
          {!catalogLoading && !catalogError && catalogPreview.length > 0 && (
            <>
              <div style={{ fontSize: '0.8rem', color: '#64748b', marginBottom: '0.65rem' }}>
                Preview: {catalogPreview.length} of {catalogFiltered.length}
                {catalogFilter.trim() ? ' (filtered)' : ''}.
                {catalogRows.length > CATALOG_HOME_PREVIEW ? (
                  <>
                    {' '}
                    <Link to="/connectors/catalog">Full library</Link> pages through all {catalogRows.length}.
                  </>
                ) : null}
              </div>
              <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fill, minmax(260px, 1fr))', gap: '0.75rem' }}>
                {catalogPreview.map((p) => (
                  <CatalogProviderCard
                    key={p.name}
                    row={p}
                    onAddConnector={(providerKey) =>
                      navigate(`/connectors/add-managed?provider_key=${encodeURIComponent(providerKey)}`)
                    }
                  />
                ))}
              </div>
              {catalogFiltered.length > CATALOG_HOME_PREVIEW && (
                <div style={{ marginTop: '1rem' }}>
                  <Link
                    to="/connectors/catalog"
                    style={{
                      display: 'inline-block',
                      padding: '0.45rem 0.9rem',
                      borderRadius: '8px',
                      border: '1px solid #c7d2fe',
                      background: '#eef2ff',
                      color: '#3730a3',
                      fontSize: '0.875rem',
                      fontWeight: 600,
                      textDecoration: 'none',
                    }}
                  >
                    Open full library ({catalogFiltered.length - CATALOG_HOME_PREVIEW} more here)
                  </Link>
                </div>
              )}
            </>
          )}
        </section>

        {/* Configured Connectors */}
        <section style={{ marginBottom: '2rem' }}>
          <h2 style={{ fontSize: '1.125rem', color: '#333', marginBottom: '1rem' }}>Connected Services</h2>
          {loading ? (
            <p style={{ color: '#666' }}>Loading...</p>
          ) : connectors.length === 0 ? (
            <div style={{ background: 'white', padding: '2rem', borderRadius: '8px', boxShadow: '0 1px 4px rgba(0,0,0,0.08)', textAlign: 'center', color: '#666' }}>
              No connectors configured yet. Connect one below.
            </div>
          ) : (
            <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fill, minmax(280px, 1fr))', gap: '1rem' }}>
              {connectors.map(c => {
                const platform = PLATFORMS.find(p => p.id === c.platform);
                return (
                  <div key={c.platform} style={{ background: 'white', borderRadius: '8px', padding: '1.25rem', boxShadow: '0 1px 4px rgba(0,0,0,0.08)', borderLeft: `4px solid ${platform?.color || '#ccc'}` }}>
                    <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', marginBottom: '0.75rem' }}>
                      <h3 style={{ margin: 0, fontSize: '1rem', color: '#333' }}>{platform?.name || c.platform}</h3>
                      <div style={{ display: 'flex', gap: '0.375rem' }}>
                        <span style={{
                          padding: '0.125rem 0.5rem',
                          borderRadius: '9999px',
                          fontSize: '0.75rem',
                          fontWeight: 500,
                          background: c.connected ? '#dcfce7' : c.configured ? '#fef3c7' : '#f3f4f6',
                          color: c.connected ? '#166534' : c.configured ? '#92400e' : '#6b7280',
                        }}>
                          {c.connected ? 'Connected' : c.configured ? 'Configured' : 'Not configured'}
                        </span>
                        <span style={{
                          padding: '0.125rem 0.5rem',
                          borderRadius: '9999px',
                          fontSize: '0.75rem',
                          fontWeight: 500,
                          background: c.engine === 'nango' ? '#e0e7ff' : '#ecfeff',
                          color: c.engine === 'nango' ? '#3730a3' : '#0f766e',
                        }}>
                          {c.engine === 'nango' ? 'Managed' : 'Built-in'}
                        </span>
                      </div>
                    </div>
                    <div style={{ fontSize: '0.8125rem', color: '#666', marginBottom: '1rem' }}>
                      {platform?.type === 'api_key' ? (
                        <span>API Key: <code style={{ background: '#f5f5f5', padding: '0.125rem 0.25rem', borderRadius: '2px' }}>{c.client_id ? '••••••••' : '—'}</code></span>
                      ) : (
                        <span>Client ID: <code style={{ background: '#f5f5f5', padding: '0.125rem 0.25rem', borderRadius: '2px' }}>{c.client_id || '—'}</code></span>
                      )}
                    </div>
                    <div style={{ display: 'flex', gap: '0.5rem' }}>
                      <Link to={`/connectors/${c.platform}`} style={{ padding: '0.375rem 0.75rem', background: '#1976d2', color: 'white', borderRadius: '4px', textDecoration: 'none', fontSize: '0.8125rem' }}>
                        Configure
                      </Link>
                      {(c.engine === 'nango' || platform?.type === 'oauth2') && c.configured && !c.connected && (
                        <button onClick={() => handleConnect(c.platform, c.engine)} style={{ padding: '0.375rem 0.75rem', background: '#dcfce7', color: '#166534', border: 'none', borderRadius: '4px', cursor: 'pointer', fontSize: '0.8125rem' }}>
                          {c.engine === 'nango' ? 'Connect (managed)' : 'Connect OAuth'}
                        </button>
                      )}
                    </div>
                  </div>
                );
              })}
            </div>
          )}
        </section>

        {/* Available Platforms */}
        <section>
          <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', marginBottom: '1rem', flexWrap: 'wrap', gap: '0.5rem' }}>
            <h2 style={{ fontSize: '1.125rem', color: '#333', margin: 0 }}>Connect Service</h2>
            <Link to="/connectors/add-managed" style={{ fontSize: '0.875rem', color: '#4f46e5' }}>+ Add from library</Link>
          </div>
          <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fill, minmax(200px, 1fr))', gap: '0.75rem' }}>
            {PLATFORMS.filter(p => !configuredPlatforms.has(p.id)).map(p => (
              <div key={p.id} style={{ background: 'white', borderRadius: '8px', padding: '1rem', boxShadow: '0 1px 4px rgba(0,0,0,0.08)', borderLeft: `4px solid ${p.color}`, display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
                <span style={{ fontSize: '0.9375rem', color: '#333' }}>{p.name}</span>
                <Link to={`/connectors/${p.id}`} style={{ padding: '0.25rem 0.625rem', background: '#f5f5f5', color: '#333', border: '1px solid #ccc', borderRadius: '4px', textDecoration: 'none', fontSize: '0.8125rem' }}>
                  Connect
                </Link>
              </div>
            ))}
          </div>
        </section>
      </main>
    </div>
  );
}
