import { useCallback, useEffect, useMemo, useState } from 'react';
import { Link, useNavigate } from 'react-router-dom';
import { getIntegrationCatalog, type IntegrationCatalogRow } from '../api/client';

const PAGE = 72;

function providerSearchBlob(p: IntegrationCatalogRow): string {
  const cats = (p.categories || []).join(' ');
  return `${p.name} ${p.display_name} ${cats} ${p.auth_mode || ''}`.toLowerCase();
}

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
      const cleaned = (Array.isArray(data) ? data : []).filter((r): r is IntegrationCatalogRow => typeof r?.name === 'string' && r.name.length > 0);
      setRows(cleaned);
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
                <article
                  key={p.name}
                  style={{
                    background: 'white',
                    borderRadius: '12px',
                    padding: '1rem',
                    border: '1px solid #e2e8f0',
                    boxShadow: '0 1px 3px rgba(15,23,42,0.06)',
                    display: 'flex',
                    flexDirection: 'column',
                    gap: '0.65rem',
                    transition: 'border-color 0.15s, box-shadow 0.15s',
                  }}
                >
                  <div style={{ display: 'flex', gap: '0.75rem', alignItems: 'flex-start' }}>
                    <Logo url={p.logo_url} label={p.display_name || p.name} />
                    <div style={{ minWidth: 0, flex: 1 }}>
                      <div style={{ fontWeight: 600, color: '#0f172a', fontSize: '0.95rem', lineHeight: 1.3 }}>{p.display_name || p.name}</div>
                      <code style={{ fontSize: '0.75rem', color: '#64748b', wordBreak: 'break-all' }}>{p.name}</code>
                    </div>
                  </div>
                  <div style={{ display: 'flex', flexWrap: 'wrap', gap: '0.35rem' }}>
                    {p.auth_mode && (
                      <span
                        style={{
                          fontSize: '0.68rem',
                          fontWeight: 600,
                          textTransform: 'uppercase',
                          letterSpacing: '0.04em',
                          padding: '0.2rem 0.45rem',
                          borderRadius: '6px',
                          background: '#eef2ff',
                          color: '#4338ca',
                        }}
                      >
                        {p.auth_mode}
                      </span>
                    )}
                    {(p.categories || []).slice(0, 3).map((c) => (
                      <span
                        key={c}
                        style={{
                          fontSize: '0.68rem',
                          padding: '0.2rem 0.45rem',
                          borderRadius: '6px',
                          background: '#f1f5f9',
                          color: '#475569',
                        }}
                      >
                        {c}
                      </span>
                    ))}
                  </div>
                  <div style={{ marginTop: 'auto', display: 'flex', flexWrap: 'wrap', gap: '0.5rem', alignItems: 'center' }}>
                    <button
                      type="button"
                      onClick={() => navigate(`/connectors/add-managed?provider_key=${encodeURIComponent(p.name)}`)}
                      style={{
                        padding: '0.4rem 0.75rem',
                        borderRadius: '8px',
                        border: 'none',
                        background: '#4f46e5',
                        color: 'white',
                        fontSize: '0.8rem',
                        fontWeight: 600,
                        cursor: 'pointer',
                      }}
                    >
                      Add connector
                    </button>
                    {p.docs && /^https?:\/\//i.test(p.docs) && (
                      <a href={p.docs} target="_blank" rel="noreferrer" style={{ fontSize: '0.78rem', color: '#2563eb' }}>
                        Docs ↗
                      </a>
                    )}
                  </div>
                </article>
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

function Logo({ url, label }: { url?: string; label: string }) {
  const [ok, setOk] = useState(true);
  const initial = (label || '?').trim().charAt(0).toUpperCase();
  if (!url || !ok) {
    return (
      <div
        style={{
          width: 44,
          height: 44,
          borderRadius: '10px',
          background: 'linear-gradient(135deg, #6366f1, #8b5cf6)',
          color: 'white',
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'center',
          fontWeight: 700,
          fontSize: '1.1rem',
          flexShrink: 0,
        }}
      >
        {initial}
      </div>
    );
  }
  return (
    <img
      src={url}
      alt=""
      width={44}
      height={44}
      style={{ borderRadius: '10px', objectFit: 'contain', background: '#f8fafc', flexShrink: 0 }}
      onError={() => setOk(false)}
    />
  );
}
