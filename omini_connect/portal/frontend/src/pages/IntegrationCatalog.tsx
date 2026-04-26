import { useCallback, useEffect, useMemo, useState } from 'react';
import { Link, useNavigate } from 'react-router-dom';
import CatalogProviderCard from '../components/CatalogProviderCard';
import { getIntegrationCatalog, type IntegrationCatalogRow } from '../api/client';
import { normalizeCatalogResponse, providerSearchBlob } from '../lib/integrationCatalogNormalize';

const PAGE = 72;

export default function IntegrationCatalog() {
  const navigate = useNavigate();
  const [rows, setRows] = useState<IntegrationCatalogRow[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState('');
  const [filter, setFilter] = useState('');
  const [visible, setVisible] = useState(PAGE);

  const load = useCallback(async () => {
    setLoading(true);
    setError('');
    try {
      const data = await getIntegrationCatalog();
      setRows(normalizeCatalogResponse(data));
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Failed to load catalog');
      setRows([]);
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    void load();
  }, [load]);

  const filtered = useMemo(() => {
    const q = filter.trim().toLowerCase();
    if (!q) return rows;
    return rows.filter((p) => providerSearchBlob(p).includes(q));
  }, [rows, filter]);

  useEffect(() => {
    setVisible(PAGE);
  }, [filter, rows.length]);

  const slice = filtered.slice(0, visible);

  return (
    <div style={{ minHeight: '100vh', background: '#f1f5f9' }}>
      <header
        style={{
          position: 'sticky',
          top: 0,
          zIndex: 10,
          background: 'rgba(255,255,255,0.92)',
          backdropFilter: 'blur(8px)',
          borderBottom: '1px solid #e2e8f0',
          padding: '0.75rem 1.25rem',
          display: 'flex',
          flexWrap: 'wrap',
          alignItems: 'center',
          gap: '0.75rem',
        }}
      >
        <Link to="/" style={{ color: '#64748b', textDecoration: 'none', fontSize: '0.9rem' }}>
          ← Dashboard
        </Link>
        <h1 style={{ margin: 0, fontSize: '1.15rem', color: '#0f172a', fontWeight: 600 }}>Integration library</h1>
        <span style={{ fontSize: '0.8rem', color: '#64748b', marginLeft: 'auto' }}>
          {loading ? 'Loading…' : `${filtered.length} match · ${rows.length} total`}
        </span>
      </header>

      <div style={{ padding: '1rem 1.25rem 2rem', maxWidth: '1280px', margin: '0 auto' }}>
        <div
          style={{
            marginBottom: '1.25rem',
            display: 'flex',
            flexWrap: 'wrap',
            gap: '0.75rem',
            alignItems: 'center',
          }}
        >
          <input
            type="search"
            placeholder="Filter by name, display name, category, auth mode…"
            value={filter}
            onChange={(e) => setFilter(e.target.value)}
            style={{
              flex: '1 1 280px',
              maxWidth: '520px',
              padding: '0.65rem 0.9rem',
              borderRadius: '10px',
              border: '1px solid #cbd5e1',
              fontSize: '0.95rem',
              outline: 'none',
              boxShadow: '0 1px 2px rgba(15,23,42,0.06)',
            }}
          />
          <button
            type="button"
            onClick={() => void load()}
            style={{
              padding: '0.55rem 1rem',
              borderRadius: '10px',
              border: '1px solid #cbd5e1',
              background: 'white',
              cursor: 'pointer',
              fontSize: '0.875rem',
              color: '#334155',
            }}
          >
            Refresh
          </button>
        </div>

        {error && (
          <div
            style={{
              padding: '1rem',
              borderRadius: '10px',
              background: '#fef2f2',
              color: '#b91c1c',
              marginBottom: '1rem',
              fontSize: '0.9rem',
            }}
          >
            {error}
            <div style={{ marginTop: '0.5rem', color: '#64748b', fontSize: '0.8rem' }}>
              The catalog is served by your OminiConnect deployment. If this persists, confirm you are signed in and that your administrator has enabled the managed integration hub on the server.
            </div>
          </div>
        )}

        {loading ? (
          <p style={{ color: '#64748b' }}>Loading integration library…</p>
        ) : !error && rows.length === 0 ? (
          <div
            style={{
              padding: '1.25rem',
              borderRadius: '10px',
              background: '#fffbeb',
              border: '1px solid #fcd34d',
              color: '#92400e',
              fontSize: '0.9rem',
            }}
          >
            <strong>No providers loaded.</strong> Set <code style={{ background: '#fef3c7', padding: '0.1rem 0.35rem', borderRadius: '4px' }}>NANGO_BASE_URL</code> in the
            repo-root <code style={{ background: '#fef3c7', padding: '0.1rem 0.35rem', borderRadius: '4px' }}>.env</code> (e.g. <code style={{ background: '#fef3c7', padding: '0.1rem 0.35rem', borderRadius: '4px' }}>http://localhost:3003</code>) and restart the portal. For full Nango API features, add{' '}
            <code style={{ background: '#fef3c7', padding: '0.1rem 0.35rem', borderRadius: '4px' }}>NANGO_SECRET_KEY</code> or run <code style={{ background: '#fef3c7', padding: '0.1rem 0.35rem', borderRadius: '4px' }}>./scripts/sync_nango_secret_to_omini_env.sh</code>. Signing into the <strong>Nango</strong> dashboard is separate from signing into this <strong>OminiConnect</strong> portal.
          </div>
        ) : (
          <>
            <div
              style={{
                display: 'grid',
                gridTemplateColumns: 'repeat(auto-fill, minmax(270px, 1fr))',
                gap: '0.85rem',
              }}
            >
              {slice.map((p) => (
                <CatalogProviderCard
                  key={p.name}
                  row={p}
                  onAddConnector={(providerKey) =>
                    navigate(`/connectors/add-managed?provider_key=${encodeURIComponent(providerKey)}`)
                  }
                  onOpenConnector={(platform) => navigate(`/connectors/${encodeURIComponent(platform)}`)}
                />
              ))}
            </div>
            {visible < filtered.length && (
              <div style={{ marginTop: '1.5rem', textAlign: 'center' }}>
                <button
                  type="button"
                  onClick={() => setVisible((v) => v + PAGE)}
                  style={{
                    padding: '0.55rem 1.25rem',
                    borderRadius: '10px',
                    border: '1px solid #cbd5e1',
                    background: 'white',
                    cursor: 'pointer',
                    color: '#334155',
                    fontSize: '0.875rem',
                  }}
                >
                  Show more ({filtered.length - visible} remaining)
                </button>
              </div>
            )}
          </>
        )}
      </div>
    </div>
  );
}
