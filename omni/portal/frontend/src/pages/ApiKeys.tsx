import { useEffect, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { getMe } from '../api/client';

interface ApiKey {
  key_hash: string;
  username: string;
  label: string;
  created_at: string;
}

interface NewApiKeyResponse {
  key: string;
  label: string;
  created_at: string;
}

export default function ApiKeys() {
  const [apiKeys, setApiKeys] = useState<ApiKey[]>([]);
  const [loading, setLoading] = useState(true);
  const [newKeyLabel, setNewKeyLabel] = useState('');
  const [creating, setCreating] = useState(false);
  const [newKey, setNewKey] = useState<string | null>(null);
  const [error, setError] = useState('');

  const navigate = useNavigate();

  useEffect(() => {
    checkAuth();
    loadApiKeys();
  }, []);

  async function checkAuth() {
    try {
      const me = await getMe();
      if (!me) {
        navigate('/auth/login');
        return;
      }
    } catch {
      navigate('/auth/login');
    }
  }

  async function loadApiKeys() {
    try {
      const res = await fetch('/auth/apikey', {
        headers: { 'Content-Type': 'application/json' },
        credentials: 'include',
      });
      if (res.ok) {
        const keys = await res.json();
        setApiKeys(keys);
      }
    } catch (err) {
      console.error('Failed to load API keys:', err);
    } finally {
      setLoading(false);
    }
  }

  async function handleCreate(e: React.FormEvent) {
    e.preventDefault();
    setCreating(true);
    setError('');
    setNewKey(null);
    try {
      const res = await fetch('/auth/apikey', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        credentials: 'include',
        body: JSON.stringify({ label: newKeyLabel || 'My API Key' }),
      });
      if (!res.ok) {
        const text = await res.text();
        throw new Error(text || 'Failed to create API key');
      }
      const data: NewApiKeyResponse = await res.json();
      setNewKey(data.key);
      setNewKeyLabel('');
      await loadApiKeys();
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to create API key');
    } finally {
      setCreating(false);
    }
  }

  async function handleDelete(keyHash: string) {
    if (!confirm('Delete this API key? This cannot be undone.')) return;
    try {
      const res = await fetch(`/auth/apikey/${keyHash}`, {
        method: 'DELETE',
        credentials: 'include',
      });
      if (!res.ok) throw new Error('Failed to delete');
      await loadApiKeys();
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to delete API key');
    }
  }

  function formatDate(dateStr: string) {
    try {
      return new Date(dateStr).toLocaleString();
    } catch {
      return dateStr;
    }
  }

  return (
    <div style={{ minHeight: '100vh', background: '#f5f5f5' }}>
      <header style={{ background: 'white', borderBottom: '1px solid #e0e0e0', padding: '0 1.5rem', display: 'flex', alignItems: 'center', gap: '1rem' }}>
        <button onClick={() => navigate('/')} style={{ background: 'none', border: 'none', cursor: 'pointer', fontSize: '1rem', color: '#666', padding: '1rem 0' }}>← Back</button>
        <h1 style={{ margin: 0, fontSize: '1.125rem', color: '#333' }}>API Keys</h1>
      </header>

      <main style={{ padding: '2rem', maxWidth: '700px' }}>
        {error && (
          <div style={{ background: '#fef3c7', color: '#92400e', padding: '0.75rem', borderRadius: '4px', marginBottom: '1rem', fontSize: '0.875rem' }}>
            {error}
          </div>
        )}

        {newKey && (
          <div style={{ background: '#dcfce7', border: '1px solid #86efac', borderRadius: '8px', padding: '1.5rem', marginBottom: '1.5rem' }}>
            <p style={{ margin: '0 0 0.75rem', fontWeight: 600, color: '#166534' }}>New API Key — copy it now, you won't see it again:</p>
            <code style={{ display: 'block', background: '#f0fdf4', padding: '0.75rem', borderRadius: '4px', fontFamily: 'monospace', fontSize: '0.875rem', wordBreak: 'break-all', color: '#166534' }}>
              {newKey}
            </code>
            <button
              onClick={() => { navigator.clipboard.writeText(newKey); }}
              style={{ marginTop: '0.75rem', padding: '0.375rem 0.75rem', background: '#166534', color: 'white', border: 'none', borderRadius: '4px', cursor: 'pointer', fontSize: '0.8125rem' }}
            >
              Copy to clipboard
            </button>
          </div>
        )}

        <form onSubmit={handleCreate} style={{ background: 'white', borderRadius: '8px', padding: '1.5rem', boxShadow: '0 1px 4px rgba(0,0,0,0.08)', marginBottom: '1.5rem' }}>
          <h2 style={{ margin: '0 0 1rem', fontSize: '1rem', color: '#333' }}>Create New API Key</h2>
          <div style={{ display: 'flex', gap: '0.75rem', alignItems: 'flex-end' }}>
            <div style={{ flex: 1 }}>
              <label style={{ display: 'block', marginBottom: '0.375rem', fontSize: '0.875rem', color: '#666' }}>Label (optional)</label>
              <input
                type="text"
                value={newKeyLabel}
                onChange={e => setNewKeyLabel(e.target.value)}
                placeholder="My API Key"
                style={{ width: '100%', padding: '0.5rem', border: '1px solid #ccc', borderRadius: '4px', boxSizing: 'border-box' }}
              />
            </div>
            <button type="submit" disabled={creating} style={{ padding: '0.5rem 1.25rem', background: '#1976d2', color: 'white', border: 'none', borderRadius: '4px', cursor: 'pointer' }}>
              {creating ? 'Creating...' : 'Create Key'}
            </button>
          </div>
        </form>

        <div style={{ background: 'white', borderRadius: '8px', boxShadow: '0 1px 4px rgba(0,0,0,0.08)' }}>
          <div style={{ padding: '1rem 1.5rem', borderBottom: '1px solid #e0e0e0' }}>
            <h2 style={{ margin: 0, fontSize: '1rem', color: '#333' }}>Your API Keys</h2>
          </div>
          {loading ? (
            <p style={{ padding: '1.5rem', color: '#666' }}>Loading...</p>
          ) : apiKeys.length === 0 ? (
            <p style={{ padding: '1.5rem', color: '#666' }}>No API keys yet. Create one above.</p>
          ) : (
            <table style={{ width: '100%', borderCollapse: 'collapse' }}>
              <thead>
                <tr style={{ background: '#f9f9f9' }}>
                  <th style={{ textAlign: 'left', padding: '0.75rem 1.5rem', fontSize: '0.8125rem', color: '#666', fontWeight: 500 }}>Label</th>
                  <th style={{ textAlign: 'left', padding: '0.75rem 1rem', fontSize: '0.8125rem', color: '#666', fontWeight: 500 }}>Created</th>
                  <th style={{ textAlign: 'right', padding: '0.75rem 1.5rem', fontSize: '0.8125rem', color: '#666', fontWeight: 500 }}>Action</th>
                </tr>
              </thead>
              <tbody>
                {apiKeys.map(key => (
                  <tr key={key.key_hash} style={{ borderBottom: '1px solid #f0f0f0' }}>
                    <td style={{ padding: '0.75rem 1.5rem', fontSize: '0.875rem', color: '#333' }}>{key.label}</td>
                    <td style={{ padding: '0.75rem 1rem', fontSize: '0.875rem', color: '#666' }}>{formatDate(key.created_at)}</td>
                    <td style={{ padding: '0.75rem 1.5rem', textAlign: 'right' }}>
                      <button
                        onClick={() => handleDelete(key.key_hash)}
                        style={{ padding: '0.25rem 0.625rem', background: 'white', color: '#d32f2f', border: '1px solid #d32f2f', borderRadius: '4px', cursor: 'pointer', fontSize: '0.8125rem' }}
                      >
                        Delete
                      </button>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          )}
        </div>

        <div style={{ marginTop: '1.5rem', background: 'white', borderRadius: '8px', padding: '1.5rem', boxShadow: '0 1px 4px rgba(0,0,0,0.08)' }}>
          <h2 style={{ margin: '0 0 0.75rem', fontSize: '1rem', color: '#333' }}>How to use your API key</h2>
          <p style={{ margin: '0 0 1rem', fontSize: '0.875rem', color: '#666' }}>
            Include your API key in the <code style={{ background: '#f5f5f5', padding: '0.125rem 0.25rem', borderRadius: '2px' }}>Authorization</code> header when making requests:
          </p>
          <pre style={{ background: '#1f2937', color: '#e5e7eb', padding: '1rem', borderRadius: '6px', fontSize: '0.8125rem', overflowX: 'auto', margin: 0 }}>
{`curl -X POST https://mymac.gpt4people.online/api/proxy/linkedin/userinfo \\
  -H "Authorization: Bearer YOUR_API_KEY"`}
          </pre>
        </div>
      </main>
    </div>
  );
}
