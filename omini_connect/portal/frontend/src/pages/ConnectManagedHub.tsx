import { useEffect, useState } from 'react';
import { Link, Navigate, useNavigate, useParams } from 'react-router-dom';
import { createNangoConnectSession } from '../api/client';

/**
 * Managed Nango Connect: the hub UI calls `window.open` for OAuth. That is routinely blocked
 * or broken when the hub runs inside our cross-origin iframe, so we default to opening the hub
 * in a **new tab** (top-level window) and keep Finish on this page.
 */
export default function ConnectManagedHub() {
  const { platform } = useParams<{ platform: string }>();
  const navigate = useNavigate();
  const [connectUrl, setConnectUrl] = useState('');
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState('');
  const [showEmbedded, setShowEmbedded] = useState(false);

  useEffect(() => {
    if (!platform) return;
    let cancelled = false;
    setLoading(true);
    setError('');
    setConnectUrl('');
    void (async () => {
      try {
        const { connect_url } = await createNangoConnectSession(platform);
        if (!cancelled) {
          setConnectUrl(connect_url);
        }
      } catch (e) {
        if (!cancelled) {
          setError(e instanceof Error ? e.message : 'Could not start Connect');
        }
      } finally {
        if (!cancelled) setLoading(false);
      }
    })();
    return () => {
      cancelled = true;
    };
  }, [platform]);

  const saveConnection = () => {
    if (!platform) return;
    window.location.assign(`/oauth/${encodeURIComponent(platform)}/nango-finalize`);
  };

  if (!platform) return <Navigate to="/" replace />;

  return (
    <div style={{ minHeight: '100vh', background: '#f8fafc', display: 'flex', flexDirection: 'column' }}>
      <header
        style={{
          flexShrink: 0,
          background: 'white',
          borderBottom: '1px solid #e2e8f0',
          padding: '0.75rem 1.25rem',
          display: 'flex',
          alignItems: 'center',
          gap: '1rem',
          flexWrap: 'wrap',
        }}
      >
        <Link to={`/connectors/${encodeURIComponent(platform)}`} style={{ color: '#64748b', textDecoration: 'none', fontSize: '0.9rem' }}>
          ← Back to connector
        </Link>
        <h1 style={{ margin: 0, fontSize: '1.05rem', color: '#0f172a', fontWeight: 600 }}>Connect account (managed hub)</h1>
      </header>

      <div style={{ flex: 1, padding: '1rem 1.25rem 2rem', maxWidth: '960px', margin: '0 auto', width: '100%', boxSizing: 'border-box' }}>
        <p style={{ margin: '0 0 1rem', fontSize: '0.9rem', color: '#475569', lineHeight: 1.5 }}>
          You are still in the <strong>OminiConnect</strong> portal. The integration hub signs in with a <strong>pop-up</strong> window; browsers usually <strong>block that pop-up when the hub is embedded in a frame</strong> on this page, which looks like endless &quot;Connecting…&quot; inside the hub. Use <strong>Open sign-in in new tab</strong> below so the hub runs in its own tab, complete OAuth there, then return here and click <strong>Finish</strong>.
        </p>

        {loading && <p style={{ color: '#64748b' }}>Starting secure connection…</p>}

        {error && (
          <div style={{ padding: '1rem', borderRadius: '8px', background: '#fef2f2', color: '#b91c1c', marginBottom: '1rem', fontSize: '0.9rem' }}>
            {error}
            <div style={{ marginTop: '0.75rem' }}>
              <button type="button" onClick={() => navigate(0)} style={{ padding: '0.35rem 0.75rem', cursor: 'pointer' }}>
                Retry
              </button>
            </div>
          </div>
        )}

        {connectUrl && !loading && (
          <>
            <div style={{ display: 'flex', flexWrap: 'wrap', gap: '0.75rem', alignItems: 'center', marginBottom: '1rem' }}>
              <a
                href={connectUrl}
                target="_blank"
                rel="noopener noreferrer"
                style={{
                  display: 'inline-block',
                  padding: '0.65rem 1.25rem',
                  borderRadius: '8px',
                  border: 'none',
                  background: '#4f46e5',
                  color: 'white',
                  cursor: 'pointer',
                  fontSize: '0.9rem',
                  fontWeight: 600,
                  textDecoration: 'none',
                }}
              >
                Open sign-in in new tab
              </a>
              <button
                type="button"
                onClick={saveConnection}
                style={{
                  padding: '0.5rem 1.1rem',
                  borderRadius: '8px',
                  border: '1px solid #cbd5e1',
                  background: 'white',
                  cursor: 'pointer',
                  fontSize: '0.875rem',
                  color: '#334155',
                }}
              >
                Finish
              </button>
            </div>
            <p style={{ fontSize: '0.8125rem', color: '#64748b', marginBottom: '1rem' }}>
              Leave this tab open. After the other tab shows that you are connected, come back here and press <strong>Finish</strong> so the portal picks up the new connection.
            </p>
            <label style={{ display: 'flex', alignItems: 'center', gap: '0.5rem', fontSize: '0.8125rem', color: '#64748b', cursor: 'pointer', marginBottom: '0.75rem' }}>
              <input type="checkbox" checked={showEmbedded} onChange={(e) => setShowEmbedded(e.target.checked)} />
              Try embedded hub here (often blocks provider pop-ups — use new tab if sign-in hangs)
            </label>
            {showEmbedded && (
              <div
                style={{
                  borderRadius: '10px',
                  overflow: 'hidden',
                  border: '1px solid #cbd5e1',
                  background: 'white',
                  minHeight: 'min(70vh, 640px)',
                  marginBottom: '1rem',
                }}
              >
                <iframe
                  title="Provider sign-in (embedded)"
                  src={connectUrl}
                  style={{ width: '100%', height: 'min(70vh, 640px)', border: 'none', display: 'block' }}
                  referrerPolicy="no-referrer-when-downgrade"
                  allow="clipboard-read; clipboard-write"
                />
              </div>
            )}
          </>
        )}
      </div>
    </div>
  );
}
