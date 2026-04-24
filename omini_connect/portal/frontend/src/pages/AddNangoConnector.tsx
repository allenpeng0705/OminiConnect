import { useEffect, useState } from 'react';
import { Link, useNavigate, useSearchParams } from 'react-router-dom';
import { getNangoIntegrations, getNangoStatus, upsertConnector } from '../api/client';

export default function AddNangoConnector() {
  const navigate = useNavigate();
  const [params] = useSearchParams();
  const [slug, setSlug] = useState('');
  const [integrations, setIntegrations] = useState<{ unique_key: string; display_name: string; provider?: string }[]>([]);
  const [providerKey, setProviderKey] = useState('');
  const [scopes, setScopes] = useState('');
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState('');
  const [nangoBaseUrl, setNangoBaseUrl] = useState('');
  const [nangoReady, setNangoReady] = useState(false);

  useEffect(() => {
    const pk = params.get('provider_key') || params.get('prefill');
    if (pk) setProviderKey(pk);
  }, [params]);

  useEffect(() => {
    getNangoStatus()
      .then((s) => {
        const base = (s.base_url || '').trim().replace(/\/+$/, '');
        setNangoBaseUrl(base);
        setNangoReady(Boolean(base));
      })
      .catch(() => {
        setNangoBaseUrl('');
        setNangoReady(false);
      });
  }, []);

  useEffect(() => {
    getNangoIntegrations()
      .then(setIntegrations)
      .catch(() => {
        setIntegrations([]);
        setError('Could not load configured integrations for this environment. Check that you are signed in and that the managed hub is configured on the server.');
      })
      .finally(() => setLoading(false));
  }, []);

  function openNangoPortal(path = '') {
    if (!nangoBaseUrl) return;
    const normalizedPath = path.startsWith('/') || path === '' ? path : `/${path}`;
    window.open(`${nangoBaseUrl}${normalizedPath}`, '_blank', 'noopener,noreferrer');
  }

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    setSaving(true);
    setError('');
    const p = slug.trim().toLowerCase();
    if (!/^[a-z0-9][a-z0-9_-]{0,62}$/.test(p)) {
      setError('Connector id: start with a letter or digit, then letters, digits, hyphens, or underscores.');
      setSaving(false);
      return;
    }
    if (!providerKey.trim()) {
      setError('Select or enter an integration unique key.');
      setSaving(false);
      return;
    }
    try {
      await upsertConnector({
        platform: p,
        client_id: '',
        client_secret: '',
        redirect_uri: `${window.location.origin}/oauth/${p}/callback`,
        scopes: scopes.split(/\s+/).filter(Boolean),
        engine: 'nango',
        provider_key: providerKey.trim(),
        connection_ref: '',
        agent_id: '',
        enabled: true,
      });
      navigate(`/connectors/${p}`);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Save failed');
    } finally {
      setSaving(false);
    }
  }

  return (
    <div style={{ minHeight: '100vh', background: '#f5f5f5' }}>
      <header style={{ background: 'white', borderBottom: '1px solid #e0e0e0', padding: '0 1.5rem', display: 'flex', alignItems: 'center', gap: '1rem' }}>
        <Link to="/" style={{ color: '#666', textDecoration: 'none' }}>← Back</Link>
        <h1 style={{ margin: 0, fontSize: '1.125rem', color: '#333' }}>Add connector from library</h1>
      </header>
      <main style={{ padding: '2rem', maxWidth: '560px' }}>
        <form onSubmit={handleSubmit} style={{ background: 'white', borderRadius: '8px', padding: '1.5rem', boxShadow: '0 1px 4px rgba(0,0,0,0.08)' }}>
          <p style={{ margin: '0 0 1rem', fontSize: '0.875rem', color: '#666' }}>
            Registers a connector in OminiConnect that uses the managed integration hub for OAuth and API access. Pick a unique connector id (slug) per connection. Open the{' '}
            <Link to="/connectors/catalog" style={{ color: '#4f46e5' }}>integration library</Link> to choose a provider; the <strong>integration unique key</strong> below must match an integration already enabled in your environment (often the same as the provider id in the library).
          </p>
          {nangoReady && (
            <div style={{ display: 'flex', gap: '0.5rem', flexWrap: 'wrap', marginBottom: '1rem' }}>
              <button
                type="button"
                onClick={() => openNangoPortal()}
                style={{ padding: '0.4rem 0.75rem', border: '1px solid #cbd5e1', borderRadius: '6px', background: '#fff', cursor: 'pointer', fontSize: '0.8rem' }}
              >
                Open Nango portal
              </button>
              <button
                type="button"
                onClick={() => openNangoPortal('/integrations')}
                style={{ padding: '0.4rem 0.75rem', border: '1px solid #cbd5e1', borderRadius: '6px', background: '#fff', cursor: 'pointer', fontSize: '0.8rem' }}
              >
                Open Nango integrations
              </button>
            </div>
          )}
          {error && <div style={{ color: '#d32f2f', marginBottom: '1rem', fontSize: '0.875rem' }}>{error}</div>}

          <div style={{ marginBottom: '1rem' }}>
            <label style={{ display: 'block', marginBottom: '0.375rem', fontSize: '0.875rem', color: '#666' }}>Connector id</label>
            <input
              value={slug}
              onChange={e => setSlug(e.target.value)}
              placeholder="e.g. hubspot-crm"
              required
              style={{ width: '100%', padding: '0.5rem', border: '1px solid #ccc', borderRadius: '4px', boxSizing: 'border-box', fontFamily: 'monospace' }}
            />
            <p style={{ margin: '0.25rem 0 0', fontSize: '0.75rem', color: '#999' }}>Used in URLs and in /api/proxy/… paths (same string as the connector id).</p>
          </div>

          <div style={{ marginBottom: '1rem' }}>
            <label style={{ display: 'block', marginBottom: '0.375rem', fontSize: '0.875rem', color: '#666' }}>Integration unique key</label>
            {loading ? (
              <p style={{ color: '#666', fontSize: '0.875rem' }}>Loading…</p>
            ) : integrations.length > 0 ? (
              <select
                value={providerKey}
                onChange={e => setProviderKey(e.target.value)}
                required
                style={{ width: '100%', padding: '0.5rem', border: '1px solid #ccc', borderRadius: '4px' }}
              >
                <option value="">— choose —</option>
                {integrations.map(i => (
                  <option key={i.unique_key} value={i.unique_key}>
                    {i.display_name} ({i.unique_key})
                  </option>
                ))}
              </select>
            ) : (
              <input
                value={providerKey}
                onChange={e => setProviderKey(e.target.value)}
                placeholder="e.g. hubspot (must match your environment)"
                required
                style={{ width: '100%', padding: '0.5rem', border: '1px solid #ccc', borderRadius: '4px', fontFamily: 'monospace' }}
              />
            )}
          </div>

          <div style={{ marginBottom: '1.5rem' }}>
            <label style={{ display: 'block', marginBottom: '0.375rem', fontSize: '0.875rem', color: '#666' }}>Scopes (optional, space-separated)</label>
            <input value={scopes} onChange={e => setScopes(e.target.value)} placeholder="optional OAuth scopes for the connect session" style={{ width: '100%', padding: '0.5rem', border: '1px solid #ccc', borderRadius: '4px' }} />
          </div>

          <button type="submit" disabled={saving || loading} style={{ padding: '0.5rem 1.25rem', background: '#4f46e5', color: 'white', border: 'none', borderRadius: '4px', cursor: 'pointer' }}>
            {saving ? 'Saving…' : 'Create connector'}
          </button>
        </form>
      </main>
    </div>
  );
}
