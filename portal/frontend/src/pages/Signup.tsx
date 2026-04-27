import { useEffect, useState } from 'react';
import { Link, useNavigate } from 'react-router-dom';
import { getAuthCapabilities, signup } from '../api/client';

export default function Signup() {
  const [name, setName] = useState('');
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [error, setError] = useState('');
  const [saving, setSaving] = useState(false);
  const [googleEnabled, setGoogleEnabled] = useState(false);
  const navigate = useNavigate();

  useEffect(() => {
    void getAuthCapabilities()
      .then((caps) => setGoogleEnabled(Boolean(caps.google_login_enabled)))
      .catch(() => setGoogleEnabled(false));
  }, []);

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    setError('');
    setSaving(true);
    try {
      await signup(email, password, name || undefined);
      navigate('/');
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Signup failed');
    } finally {
      setSaving(false);
    }
  }

  return (
    <div style={{ minHeight: '100vh', display: 'grid', placeItems: 'center', background: 'linear-gradient(180deg, #f8fafc 0%, #eef2ff 100%)', padding: '1rem' }}>
      <form
        onSubmit={handleSubmit}
        style={{
          width: '100%',
          maxWidth: '400px',
          background: 'white',
          border: '1px solid #e2e8f0',
          borderRadius: '14px',
          boxShadow: '0 10px 25px rgba(15, 23, 42, 0.08)',
          padding: '1.5rem',
        }}
      >
        <div style={{ marginBottom: '1rem' }}>
          <div style={{ fontSize: '0.78rem', color: '#6366f1', fontWeight: 700, letterSpacing: '0.04em' }}>OMINICONNECT</div>
          <h2 style={{ margin: '0.35rem 0 0.25rem', color: '#0f172a', fontSize: '1.2rem' }}>Create your account</h2>
          <p style={{ margin: 0, color: '#64748b', fontSize: '0.88rem' }}>Start managing connectors from one portal.</p>
        </div>
        {error && <div style={{ color: '#d32f2f', marginBottom: '1rem', fontSize: '0.875rem' }}>{error}</div>}
        <div style={{ marginBottom: '0.9rem' }}>
          <label style={{ display: 'block', marginBottom: '0.45rem', fontSize: '0.82rem', color: '#475569', fontWeight: 600 }}>Name (optional)</label>
          <input
            type="text"
            value={name}
            onChange={e => setName(e.target.value)}
            style={{ width: '100%', padding: '0.62rem 0.7rem', border: '1px solid #cbd5e1', borderRadius: '8px', boxSizing: 'border-box' }}
          />
        </div>
        <div style={{ marginBottom: '0.9rem' }}>
          <label style={{ display: 'block', marginBottom: '0.45rem', fontSize: '0.82rem', color: '#475569', fontWeight: 600 }}>Email</label>
          <input
            type="email"
            value={email}
            onChange={e => setEmail(e.target.value)}
            required
            style={{ width: '100%', padding: '0.62rem 0.7rem', border: '1px solid #cbd5e1', borderRadius: '8px', boxSizing: 'border-box' }}
          />
        </div>
        <div style={{ marginBottom: '1.25rem' }}>
          <label style={{ display: 'block', marginBottom: '0.45rem', fontSize: '0.82rem', color: '#475569', fontWeight: 600 }}>Password</label>
          <input
            type="password"
            value={password}
            onChange={e => setPassword(e.target.value)}
            required
            minLength={8}
            style={{ width: '100%', padding: '0.62rem 0.7rem', border: '1px solid #cbd5e1', borderRadius: '8px', boxSizing: 'border-box' }}
          />
        </div>
        <button type="submit" disabled={saving} style={{ width: '100%', padding: '0.72rem', background: '#4f46e5', color: 'white', border: 'none', borderRadius: '8px', cursor: 'pointer', fontSize: '0.95rem', fontWeight: 600 }}>
          {saving ? 'Creating account...' : 'Sign up'}
        </button>
        {googleEnabled && (
          <button
            type="button"
            onClick={() => (window.location.href = '/auth/google')}
            style={{ width: '100%', marginTop: '0.6rem', padding: '0.72rem', background: 'white', color: '#1f2937', border: '1px solid #d1d5db', borderRadius: '8px', cursor: 'pointer', fontSize: '0.9rem' }}
          >
            Continue with Google
          </button>
        )}
        <p style={{ marginTop: '1rem', fontSize: '0.8rem', color: '#64748b', textAlign: 'center' }}>
          Already have an account? <Link to="/auth/login">Sign in</Link>
        </p>
      </form>
    </div>
  );
}
