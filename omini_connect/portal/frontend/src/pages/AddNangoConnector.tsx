import { useEffect, useState } from 'react';
import { Link, useNavigate, useSearchParams } from 'react-router-dom';
import { getIntegrationCatalog, getNangoIntegrations, type IntegrationCatalogRow, upsertConnector, ConnectorConflictError } from '../api/client';
import { normalizeCatalogResponse } from '../lib/integrationCatalogNormalize';
import { getPortalPublicBase } from '../lib/portalPublicBase';
import ScopeSelector from '../components/ScopeSelector';

export default function AddNangoConnector() {
  const navigate = useNavigate();
  const [params] = useSearchParams();
  const [slug, setSlug] = useState('');
  const [integrations, setIntegrations] = useState<{ unique_key: string; display_name: string; provider?: string }[]>([]);
  const [catalogRows, setCatalogRows] = useState<IntegrationCatalogRow[]>([]);
  const [providerKey, setProviderKey] = useState('');
  const [scopes, setScopes] = useState('');
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState('');

  useEffect(() => {
    const pk = params.get('provider_key') || params.get('prefill');
    if (pk) setProviderKey(pk);
  }, [params]);

  useEffect(() => {
    getNangoIntegrations()
      .then(setIntegrations)
      .catch(() => {
        setIntegrations([]);
        setError('Could not load configured integrations for this environment. Check that you are signed in and that the managed hub is configured on the server.');
      })
      .finally(() => setLoading(false));
  }, []);

  function autoConnectorIdFromProvider(key: string): string {
    const base = key
      .trim()
      .toLowerCase()
      .replace(/[^a-z0-9_-]+/g, '-')
      .replace(/-+/g, '-')
      .replace(/^-+|-+$/g, '')
      .slice(0, 48) || 'connector';
    const suffix = Math.random().toString(36).slice(2, 8);
    return `${base}-${suffix}`;
  }

  useEffect(() => {
    getIntegrationCatalog()
      .then((rows) => setCatalogRows(normalizeCatalogResponse(rows)))
      .catch(() => setCatalogRows([]));
  }, []);

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    setSaving(true);
    setError('');
    const rawSlug = slug.trim().toLowerCase();
    const p = rawSlug || autoConnectorIdFromProvider(providerKey || 'connector');
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
      const publicBase = await getPortalPublicBase();
      const base = publicBase.replace(/\/+$/, '');
      await upsertConnector({
        platform: p,
        client_id: '',
        client_secret: '',
        // Nango uses one global OAuth callback on the API host (portal proxies /oauth/callback).
        redirect_uri: `${base}/oauth/callback`,
        scopes: scopes.split(/\s+/).filter(Boolean),
        engine: 'nango',
        provider_key: providerKey.trim(),
        connection_ref: '',
        agent_id: '',
        enabled: true,
      });
      navigate(`/connectors/${p}`);
    } catch (err) {
      if (err instanceof ConnectorConflictError && err.existing_platform.trim()) {
        navigate(`/connectors/${encodeURIComponent(err.existing_platform.trim())}`, { replace: true });
        return;
      }
      setError(err instanceof Error ? err.message : 'Save failed');
    } finally {
      setSaving(false);
    }
  }

  const selectedProvider = catalogRows.find((r) => r.name === providerKey.trim());

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
          {selectedProvider && (
            <div style={{ marginBottom: '1rem', padding: '0.75rem', borderRadius: '8px', border: '1px solid #dbeafe', background: '#eff6ff' }}>
              <div style={{ fontSize: '0.82rem', color: '#1d4ed8', fontWeight: 600, marginBottom: '0.35rem' }}>
                Provider guidance: {selectedProvider.display_name}
              </div>
              <div style={{ fontSize: '0.8rem', color: '#1e3a8a', lineHeight: 1.4 }}>
                Auth mode: <strong>{selectedProvider.auth_mode || 'Unknown'}</strong>
                {(selectedProvider.categories || []).length > 0 ? ` · Categories: ${selectedProvider.categories!.slice(0, 4).join(', ')}` : ''}
              </div>
              {(selectedProvider.available_scopes || []).length > 0 && (
                <div style={{ marginTop: '0.5rem' }}>
                  <div style={{ fontSize: '0.75rem', color: '#1e3a8a', fontWeight: 500, marginBottom: '0.25rem' }}>Available scopes:</div>
                  <div style={{ display: 'flex', flexWrap: 'wrap', gap: '0.25rem', maxHeight: '80px', overflowY: 'auto' }}>
                    {selectedProvider.available_scopes!.map((scope) => (
                      <code
                        key={scope}
                        style={{
                          fontSize: '0.68rem',
                          padding: '0.1rem 0.35rem',
                          borderRadius: '4px',
                          background: '#dbeafe',
                          color: '#1e40af',
                          cursor: 'pointer',
                          border: scopes.split(/\s+/).includes(scope) ? '1px solid #2563eb' : '1px solid transparent',
                        }}
                        title="Click to toggle scope"
                        onClick={() => {
                          const current = scopes.split(/\s+/).filter(Boolean);
                          if (current.includes(scope)) {
                            setScopes(current.filter(s => s !== scope).join(' '));
                          } else {
                            setScopes([...current, scope].join(' '));
                          }
                        }}
                      >
                        {scope}
                      </code>
                    ))}
                  </div>
                </div>
              )}
              {selectedProvider.docs && (
                <a href={selectedProvider.docs} target="_blank" rel="noreferrer" style={{ display: 'inline-block', marginTop: '0.5rem', fontSize: '0.8rem', color: '#2563eb' }}>
                  Open provider docs ↗
                </a>
              )}
            </div>
          )}
          {error && <div style={{ color: '#d32f2f', marginBottom: '1rem', fontSize: '0.875rem' }}>{error}</div>}

          <div style={{ marginBottom: '1rem' }}>
            <label style={{ display: 'block', marginBottom: '0.375rem', fontSize: '0.875rem', color: '#666' }}>Connector id</label>
            <input
              value={slug}
              onChange={e => setSlug(e.target.value)}
              placeholder="Leave blank to auto-generate"
              style={{ width: '100%', padding: '0.5rem', border: '1px solid #ccc', borderRadius: '4px', boxSizing: 'border-box', fontFamily: 'monospace' }}
            />
            <p style={{ margin: '0.25rem 0 0', fontSize: '0.75rem', color: '#999' }}>
              Used in URLs and in /api/proxy/… paths. Leave empty to auto-generate a unique id.
            </p>
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
            <ScopeSelector value={scopes} providerKey={providerKey} availableScopes={selectedProvider?.available_scopes} onChange={setScopes} />
            <p style={{ margin: '0.25rem 0 0', fontSize: '0.75rem', color: '#999' }}>
              Use minimum required scopes first. You can expand later if the provider returns insufficient scope errors.
            </p>
          </div>

          <button type="submit" disabled={saving || loading} style={{ padding: '0.5rem 1.25rem', background: '#4f46e5', color: 'white', border: 'none', borderRadius: '4px', cursor: 'pointer' }}>
            {saving ? 'Saving…' : 'Create connector'}
          </button>
        </form>
      </main>
    </div>
  );
}
