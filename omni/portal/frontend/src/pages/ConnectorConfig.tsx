import { useEffect, useState } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import { getConnectors, upsertConnector, deleteConnector, getConnectorStatus, testConnector } from '../api/client';

const PLATFORMS: Record<string, { name: string; color: string; type: 'oauth2' | 'api_key'; scopes_default?: string }> = {
  feishu: { name: 'Feishu / Lark', color: '#00A1E0', type: 'oauth2', scopes_default: 'contact:user.base:readonly' },
  dingtalk: { name: 'DingTalk', color: '#1677FF', type: 'oauth2' },
  wechatwork: { name: 'WeChat Work', color: '#07C160', type: 'oauth2' },
  linkedin: { name: 'LinkedIn', color: '#0A66C2', type: 'oauth2', scopes_default: 'openid profile email' },
  facebook: { name: 'Facebook', color: '#1877F2', type: 'oauth2', scopes_default: 'email public_profile' },
  maton: { name: 'Maton.ai', color: '#6366F1', type: 'api_key' },
  qqmail: { name: 'QQ Enterprise Mail', color: '#12B7F5', type: 'api_key' },
};

export default function ConnectorConfig() {
  const { platform } = useParams<{ platform: string }>();
  const navigate = useNavigate();
  const [clientId, setClientId] = useState('');
  const [clientSecret, setClientSecret] = useState('');
  const [redirectUri, setRedirectUri] = useState('');
  const [scopes, setScopes] = useState('');
  const [enabled, setEnabled] = useState(true);
  const [saving, setSaving] = useState(false);
  const [testing, setTesting] = useState(false);
  const [testResult, setTestResult] = useState<{ status: string; message: string } | null>(null);
  const [error, setError] = useState('');
  const [connected, setConnected] = useState(false);
  const [showClientSecret, setShowClientSecret] = useState(false);
  const [showClientId, setShowClientId] = useState(false);

  const info = platform ? PLATFORMS[platform] : null;
  const isOAuth2 = info?.type === 'oauth2';
  const isQQMail = platform === 'qqmail';

  useEffect(() => {
    if (!platform) return;
    if (isOAuth2) {
      setRedirectUri(`${window.location.origin}/oauth/${platform}/callback`);
    }
    loadExisting();
    loadStatus();
  }, [platform]);

  async function loadExisting() {
    try {
      const list = await getConnectors();
      const existing = list.find(c => c.platform === platform);
      if (existing) {
        setClientId(existing.client_id);
        // If has_client_secret is true, show masked value; otherwise show empty
        // User must re-enter secret only if they want to change it
        setClientSecret(existing.has_client_secret ? '••••••••' : '');
        setScopes(existing.scopes?.join(' ') || '');
        setEnabled(existing.enabled);
      } else if (platform && PLATFORMS[platform]?.scopes_default) {
        setScopes(PLATFORMS[platform].scopes_default!);
      }
    } catch {}
  }

  async function loadStatus() {
    if (!platform) return;
    try {
      const s = await getConnectorStatus(platform);
      setConnected(s.connected);
    } catch {}
  }

  async function handleSave(e: React.FormEvent) {
    e.preventDefault();
    if (!platform) return;
    setSaving(true);
    setError('');
    try {
      // If clientSecret is masked (••••••••), treat as empty (preserve existing on backend)
      const secretToSave = clientSecret === '••••••••' ? '' : clientSecret;
      await upsertConnector({
        platform,
        client_id: clientId,
        client_secret: secretToSave,
        redirect_uri: redirectUri,
        scopes: isOAuth2 ? scopes.split(' ').filter(Boolean) : [],
        enabled,
      });
      await loadStatus();
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Save failed');
    } finally {
      setSaving(false);
    }
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

  if (!platform || !info) {
    return <div style={{ padding: '2rem' }}>Unknown platform</div>;
  }

  return (
    <div style={{ minHeight: '100vh', background: '#f5f5f5' }}>
      <header style={{ background: 'white', borderBottom: '1px solid #e0e0e0', padding: '0 1.5rem', display: 'flex', alignItems: 'center', gap: '1rem' }}>
        <button onClick={() => navigate('/')} style={{ background: 'none', border: 'none', cursor: 'pointer', fontSize: '1rem', color: '#666', padding: '1rem 0' }}>← Back</button>
        <h1 style={{ margin: 0, fontSize: '1.125rem', color: '#333', borderLeft: `4px solid ${info.color}`, paddingLeft: '0.75rem' }}>{info.name}</h1>
        {connected && <span style={{ marginLeft: 'auto', padding: '0.125rem 0.5rem', borderRadius: '9999px', fontSize: '0.75rem', fontWeight: 500, background: '#dcfce7', color: '#166534' }}>Connected</span>}
      </header>

      <main style={{ padding: '2rem', maxWidth: '600px' }}>
        <form onSubmit={handleSave} style={{ background: 'white', borderRadius: '8px', padding: '1.5rem', boxShadow: '0 1px 4px rgba(0,0,0,0.08)' }}>
          <h2 style={{ margin: '0 0 1.5rem', fontSize: '1rem', color: '#333' }}>
            {isOAuth2 ? 'OAuth Configuration' : isQQMail ? 'Corp ID / Secret' : 'API Key Configuration'}
          </h2>
          {error && <div style={{ color: '#d32f2f', marginBottom: '1rem', fontSize: '0.875rem' }}>{error}</div>}

          {isOAuth2 ? (
            <>
              <div style={{ marginBottom: '1rem' }}>
                <label style={{ display: 'block', marginBottom: '0.375rem', fontSize: '0.875rem', color: '#666' }}>Client ID</label>
                <div style={{ position: 'relative', display: 'flex' }}>
                  <input type={showClientId ? 'text' : 'password'} value={clientId} onChange={e => setClientId(e.target.value)} required style={{ flex: 1, padding: '0.5rem', paddingRight: '2.5rem', border: '1px solid #ccc', borderRadius: '4px', boxSizing: 'border-box' }} />
                  <button type="button" onClick={() => setShowClientId(v => !v)} style={{ position: 'absolute', right: '0.5rem', top: '50%', transform: 'translateY(-50%)', background: 'none', border: 'none', cursor: 'pointer', fontSize: '1rem', padding: '0.25rem' }}>
                    {showClientId ? '👁' : '👁‍🗨'}
                  </button>
                </div>
              </div>

              <div style={{ marginBottom: '1rem' }}>
                <label style={{ display: 'block', marginBottom: '0.375rem', fontSize: '0.875rem', color: '#666' }}>Client Secret</label>
                <div style={{ position: 'relative', display: 'flex' }}>
                  <input type={showClientSecret ? 'text' : 'password'} value={clientSecret} onChange={e => setClientSecret(e.target.value)} required style={{ flex: 1, padding: '0.5rem', paddingRight: '2.5rem', border: '1px solid #ccc', borderRadius: '4px', boxSizing: 'border-box' }} />
                  <button type="button" onClick={() => setShowClientSecret(v => !v)} style={{ position: 'absolute', right: '0.5rem', top: '50%', transform: 'translateY(-50%)', background: 'none', border: 'none', cursor: 'pointer', fontSize: '1rem', padding: '0.25rem' }}>
                    {showClientSecret ? '👁' : '👁‍🗨'}
                  </button>
                </div>
              </div>

              <div style={{ marginBottom: '1rem' }}>
                <label style={{ display: 'block', marginBottom: '0.375rem', fontSize: '0.875rem', color: '#666' }}>Redirect URI</label>
                <input type="text" value={redirectUri} onChange={e => setRedirectUri(e.target.value)} required style={{ width: '100%', padding: '0.5rem', border: '1px solid #ccc', borderRadius: '4px', boxSizing: 'border-box', fontFamily: 'monospace', fontSize: '0.8125rem' }} />
                <p style={{ margin: '0.25rem 0 0', fontSize: '0.75rem', color: '#999' }}>Register this URI in the {info.name} developer console.</p>
              </div>

              <div style={{ marginBottom: '1rem' }}>
                <label style={{ display: 'block', marginBottom: '0.375rem', fontSize: '0.875rem', color: '#666' }}>Scopes (space-separated)</label>
                <input type="text" value={scopes} onChange={e => setScopes(e.target.value)} placeholder="contact:user.base:readonly" style={{ width: '100%', padding: '0.5rem', border: '1px solid #ccc', borderRadius: '4px', boxSizing: 'border-box' }} />
              </div>
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
                <p style={{ margin: '0.25rem 0 0', fontSize: '0.75rem', color: '#999' }}>Get your API key from the Maton.ai dashboard.</p>
              </div>
            </>
          )}

          <div style={{ marginBottom: '1.5rem', display: 'flex', alignItems: 'center', gap: '0.5rem' }}>
            <input type="checkbox" id="enabled" checked={enabled} onChange={e => setEnabled(e.target.checked)} />
            <label htmlFor="enabled" style={{ fontSize: '0.875rem', color: '#666' }}>Enabled</label>
          </div>

          <div style={{ display: 'flex', gap: '0.75rem', flexWrap: 'wrap' }}>
            <button type="submit" disabled={saving} style={{ padding: '0.5rem 1.25rem', background: '#1976d2', color: 'white', border: 'none', borderRadius: '4px', cursor: 'pointer' }}>
              {saving ? 'Saving...' : 'Save'}
            </button>
            <button type="button" onClick={handleTest} disabled={testing || !clientId} style={{ padding: '0.5rem 1.25rem', background: '#f5f5f5', color: '#333', border: '1px solid #ccc', borderRadius: '4px', cursor: 'pointer' }}>
              {testing ? 'Testing...' : 'Test Connection'}
            </button>
            {isOAuth2 && connected && (
              <button type="button" onClick={() => window.location.href = `/oauth/${platform}`} style={{ padding: '0.5rem 1.25rem', background: '#dcfce7', color: '#166534', border: '1px solid #86efac', borderRadius: '4px', cursor: 'pointer' }}>
                Reconnect OAuth
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
