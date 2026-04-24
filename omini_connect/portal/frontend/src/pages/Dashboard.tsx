import { useEffect, useState } from 'react';
import { Link, useNavigate } from 'react-router-dom';
import { getConnectors, getConnectorStatus, logout } from '../api/client';

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

export default function Dashboard() {
  const navigate = useNavigate();
  const [connectors, setConnectors] = useState<ConnectorInfo[]>([]);
  const [loading, setLoading] = useState(true);
  const [username, setUsername] = useState('');

  useEffect(() => {
    loadData();
    loadMe();
  }, []);

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
        <section style={{ marginBottom: '1.5rem' }}>
          <Link
            to="/connectors/catalog"
            style={{
              display: 'block',
              textDecoration: 'none',
              borderRadius: '12px',
              padding: '1.25rem 1.5rem',
              background: 'linear-gradient(135deg, #312e81 0%, #4f46e5 45%, #6366f1 100%)',
              color: 'white',
              boxShadow: '0 4px 14px rgba(79, 70, 229, 0.35)',
            }}
          >
            <div style={{ fontSize: '1.05rem', fontWeight: 600, marginBottom: '0.35rem' }}>OminiConnect integration library</div>
            <div style={{ fontSize: '0.875rem', opacity: 0.92, lineHeight: 1.45 }}>
              Browse the full set of APIs and SaaS products OminiConnect can connect to. Search the library, then add a connector with a few clicks.
            </div>
            <div style={{ marginTop: '0.75rem', fontSize: '0.8rem', fontWeight: 600 }}>Open library →</div>
          </Link>
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
