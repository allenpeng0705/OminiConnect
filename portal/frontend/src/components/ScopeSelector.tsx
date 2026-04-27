import { useMemo, useState } from 'react';

const SCOPE_SUGGESTIONS: Record<string, string[]> = {
  linkedin: ['openid', 'profile', 'email', 'w_member_social', 'rw_organization_admin', 'r_organization_social'],
  facebook: ['email', 'public_profile', 'pages_read_engagement', 'pages_manage_posts', 'business_management'],
  x: ['tweet.read', 'users.read', 'tweet.write', 'offline.access', 'follows.read'],
  slack: ['channels:read', 'channels:history', 'chat:write', 'users:read', 'team:read'],
  hubspot: ['crm.objects.contacts.read', 'crm.objects.contacts.write', 'crm.schemas.contacts.read', 'oauth'],
  'google-mail': ['https://www.googleapis.com/auth/gmail.readonly', 'https://www.googleapis.com/auth/gmail.modify'],
  gmail: ['https://www.googleapis.com/auth/gmail.readonly', 'https://www.googleapis.com/auth/gmail.modify'],
  feishu: ['contact:user.base:readonly'],
};

function normalizeScopes(value: string): string[] {
  return value
    .split(/\s+/)
    .map((s) => s.trim())
    .filter(Boolean);
}

type Props = {
  value: string;
  providerKey?: string;
  /** Available scopes from the catalog for this provider. */
  availableScopes?: string[];
  onChange: (value: string) => void;
};

export default function ScopeSelector({ value, providerKey, availableScopes, onChange }: Props) {
  const [candidate, setCandidate] = useState('');
  const [custom, setCustom] = useState('');
  const [showCustom, setShowCustom] = useState(false);
  const selected = useMemo(() => normalizeScopes(value), [value]);
  const options = useMemo(() => {
    // Prefer catalog-provided scopes; fall back to hardcoded suggestions
    const catalogScopes = availableScopes || [];
    const key = (providerKey || '').trim().toLowerCase();
    const fallbackScopes = key ? (SCOPE_SUGGESTIONS[key] || []) : [];
    const base = catalogScopes.length > 0 ? catalogScopes : fallbackScopes;
    return base.filter((s) => !selected.includes(s));
  }, [providerKey, availableScopes, selected]);

  function write(next: string[]) {
    onChange(next.join(' '));
  }

  function addScope(scope: string) {
    const s = scope.trim();
    if (!s || selected.includes(s)) return;
    write([...selected, s]);
    setCandidate('');
    setCustom('');
  }

  function removeScope(scope: string) {
    write(selected.filter((s) => s !== scope));
  }

  return (
    <div>
      <div style={{ display: 'flex', gap: '0.5rem', marginBottom: '0.5rem', alignItems: 'center' }}>
        <select
          value={candidate}
          onChange={(e) => setCandidate(e.target.value)}
          style={{ flex: 1, padding: '0.5rem', border: '1px solid #ccc', borderRadius: '4px', boxSizing: 'border-box' }}
        >
          <option value="">{options.length > 0 ? 'Select a scope…' : 'No preset scopes for this provider'}</option>
          {options.map((s) => (
            <option key={s} value={s}>{s}</option>
          ))}
        </select>
        <button
          type="button"
          onClick={() => addScope(candidate)}
          disabled={!candidate}
          style={{ padding: '0.45rem 0.8rem', border: '1px solid #cbd5e1', borderRadius: '4px', background: '#fff', cursor: 'pointer' }}
        >
          Add
        </button>
      </div>
      <div style={{ marginBottom: '0.5rem' }}>
        <button
          type="button"
          onClick={() => setShowCustom((v) => !v)}
          style={{ padding: '0.35rem 0.6rem', border: '1px dashed #cbd5e1', borderRadius: '4px', background: '#fff', cursor: 'pointer', fontSize: '0.75rem', color: '#475569' }}
        >
          {showCustom ? 'Hide custom scope input' : 'Need a custom scope?'}
        </button>
      </div>
      {showCustom && (
        <div style={{ display: 'flex', gap: '0.5rem', marginBottom: '0.5rem' }}>
          <input
            type="text"
            value={custom}
            onChange={(e) => setCustom(e.target.value)}
            placeholder="Enter custom scope"
            style={{ flex: 1, padding: '0.5rem', border: '1px solid #ccc', borderRadius: '4px', boxSizing: 'border-box' }}
          />
          <button
            type="button"
            onClick={() => addScope(custom)}
            disabled={!custom.trim()}
            style={{ padding: '0.45rem 0.8rem', border: '1px solid #cbd5e1', borderRadius: '4px', background: '#fff', cursor: 'pointer' }}
          >
            Add custom
          </button>
        </div>
      )}
      {selected.length > 0 ? (
        <div style={{ display: 'flex', flexWrap: 'wrap', gap: '0.4rem' }}>
          {selected.map((scope) => (
            <button
              key={scope}
              type="button"
              onClick={() => removeScope(scope)}
              style={{ padding: '0.2rem 0.5rem', borderRadius: '9999px', border: '1px solid #cbd5e1', background: '#f8fafc', color: '#334155', fontSize: '0.75rem', cursor: 'pointer' }}
              title="Remove scope"
            >
              {scope} ×
            </button>
          ))}
        </div>
      ) : (
        <div style={{ fontSize: '0.75rem', color: '#999' }}>No scopes selected yet.</div>
      )}
    </div>
  );
}
