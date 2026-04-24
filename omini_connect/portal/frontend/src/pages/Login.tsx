import { useEffect, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { getAuthConfig, login } from '../api/client';

export default function Login() {
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const [error, setError] = useState('');
  const [loadingMode, setLoadingMode] = useState(true);
  const [authMode, setAuthMode] = useState<'local' | 'oidc'>('local');
  const [oidcReady, setOidcReady] = useState(false);
  const [oidcPath, setOidcPath] = useState('/auth/oidc/login');
  const navigate = useNavigate();

  useEffect(() => {
    void getAuthConfig()
      .then((cfg) => {
        if (cfg.mode === 'oidc') {
          setAuthMode('oidc');
          setOidcReady(Boolean(cfg.oidc_ready));
          setOidcPath(cfg.login_path || '/auth/oidc/login');
        } else {
          setAuthMode('local');
        }
      })
      .catch(() => {
        setAuthMode('local');
      })
      .finally(() => setLoadingMode(false));
  }, []);

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    setError('');
    try {
      await login(username, password);
      navigate('/');
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Login failed');
    }
  }

  return (
    <div style={{ minHeight: '100vh', display: 'flex', alignItems: 'center', justifyContent: 'center', background: '#f5f5f5' }}>
      <form onSubmit={handleSubmit} style={{ background: 'white', padding: '2rem', borderRadius: '8px', boxShadow: '0 2px 8px rgba(0,0,0,0.1)', width: '320px' }}>
        <h2 style={{ margin: '0 0 1.5rem', color: '#333' }}>OminiConnect Portal</h2>
        {error && <div style={{ color: '#d32f2f', marginBottom: '1rem', fontSize: '0.875rem' }}>{error}</div>}
        {loadingMode ? (
          <p style={{ color: '#666', margin: 0 }}>Loading sign-in options...</p>
        ) : authMode === 'oidc' ? (
          <>
            <p style={{ marginTop: 0, marginBottom: '1rem', fontSize: '0.85rem', color: '#666' }}>
              Sign in with your organization identity provider.
            </p>
            <button
              type="button"
              disabled={!oidcReady}
              onClick={() => (window.location.href = oidcPath)}
              style={{ width: '100%', padding: '0.75rem', background: '#1976d2', color: 'white', border: 'none', borderRadius: '4px', cursor: oidcReady ? 'pointer' : 'not-allowed', fontSize: '1rem' }}
            >
              Continue with SSO
            </button>
            {!oidcReady && (
              <p style={{ marginTop: '0.75rem', fontSize: '0.75rem', color: '#d97706' }}>
                OIDC is enabled but not fully configured on the server.
              </p>
            )}
          </>
        ) : (
          <>
            <div style={{ marginBottom: '1rem' }}>
              <label style={{ display: 'block', marginBottom: '0.5rem', fontSize: '0.875rem', color: '#666' }}>Username</label>
              <input
                type="text"
                value={username}
                onChange={e => setUsername(e.target.value)}
                required
                style={{ width: '100%', padding: '0.5rem', border: '1px solid #ccc', borderRadius: '4px', boxSizing: 'border-box' }}
              />
            </div>
            <div style={{ marginBottom: '1.5rem' }}>
              <label style={{ display: 'block', marginBottom: '0.5rem', fontSize: '0.875rem', color: '#666' }}>Password</label>
              <input
                type="password"
                value={password}
                onChange={e => setPassword(e.target.value)}
                required
                style={{ width: '100%', padding: '0.5rem', border: '1px solid #ccc', borderRadius: '4px', boxSizing: 'border-box' }}
              />
            </div>
            <button type="submit" style={{ width: '100%', padding: '0.75rem', background: '#1976d2', color: 'white', border: 'none', borderRadius: '4px', cursor: 'pointer', fontSize: '1rem' }}>
              Login
            </button>
            <p style={{ marginTop: '1rem', fontSize: '0.75rem', color: '#999', textAlign: 'center' }}>
              Default: admin / admin
            </p>
          </>
        )}
      </form>
    </div>
  );
}
