import { useEffect, useState } from 'react';
import { Link, Navigate, useNavigate, useParams } from 'react-router-dom';
import { createNangoConnectSession } from '../api/client';

/**
 * In-portal Nango Connect: address bar stays on OminiConnect while the user signs in
 * (Nango Connect is loaded in an iframe or a new tab if providers block framing).
 */
export default function ConnectManagedHub() {
  const { platform } = useParams<{ platform: string }>();
  const navigate = useNavigate();
  const [connectUrl, setConnectUrl] = useState('');
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState('');

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

  const openInNewTab = () => {
    if (connectUrl) window.open(connectUrl, '_blank', 'noopener,noreferrer');
  };

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
          You are still in the <strong>OminiConnect</strong> portal. Sign in with the provider in the area below (or in a new tab if the page does not load). When authorization succeeds, click <strong>Save connection</strong> to store it in OminiConnect.
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
                title="Provider sign-in"
                src={connectUrl}
                style={{ width: '100%', height: 'min(70vh, 640px)', border: 'none', display: 'block' }}
                referrerPolicy="no-referrer-when-downgrade"
              />
            </div>
            <p style={{ fontSize: '0.8125rem', color: '#64748b', marginBottom: '0.75rem' }}>
              If the frame is empty, the provider may block embedding. Use a new tab instead — you remain in OminiConnect after you return and click Save connection.
            </p>
            <div style={{ display: 'flex', flexWrap: 'wrap', gap: '0.5rem', alignItems: 'center' }}>
              <button
                type="button"
                onClick={openInNewTab}
                style={{
                  padding: '0.5rem 1rem',
                  borderRadius: '8px',
                  border: '1px solid #cbd5e1',
                  background: 'white',
                  cursor: 'pointer',
                  fontSize: '0.875rem',
                  color: '#334155',
                }}
              >
                Open sign-in in new tab
              </button>
              <button
                type="button"
                onClick={saveConnection}
                style={{
                  padding: '0.5rem 1.1rem',
                  borderRadius: '8px',
                  border: 'none',
                  background: '#4f46e5',
                  color: 'white',
                  cursor: 'pointer',
                  fontSize: '0.875rem',
                  fontWeight: 600,
                }}
              >
                Save connection
              </button>
            </div>
          </>
        )}
      </div>
    </div>
  );
}
