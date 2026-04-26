import { useState } from 'react';
import type { IntegrationCatalogRow } from '../api/client';
import IntegrationProviderLogo from './IntegrationProviderLogo';

type Props = {
  row: IntegrationCatalogRow;
  existingPlatform?: string | null;
  onAddConnector: (providerKey: string) => void;
  onOpenConnector: (platform: string) => void;
};

export default function CatalogProviderListItem({ row: p, existingPlatform, onAddConnector, onOpenConnector }: Props) {
  const [hovered, setHovered] = useState(false);

  return (
    <article
      onMouseEnter={() => setHovered(true)}
      onMouseLeave={() => setHovered(false)}
      style={{
        background: hovered ? '#f8fafc' : 'white',
        borderBottom: '1px solid #e2e8f0',
        padding: '0.75rem 1rem',
        display: 'flex',
        gap: '0.85rem',
        alignItems: 'center',
        transition: 'background 0.15s',
        cursor: 'default',
      }}
    >
      {/* Logo */}
      <IntegrationProviderLogo url={p.logo_url} label={p.display_name || p.name} size={40} />

      {/* Name + key */}
      <div style={{ minWidth: 0, flex: '0 0 200px' }}>
        <div
          style={{
            fontWeight: 600,
            color: '#0f172a',
            fontSize: '0.9rem',
            overflow: 'hidden',
            textOverflow: 'ellipsis',
            whiteSpace: 'nowrap',
          }}
          title={p.display_name || p.name}
        >
          {p.display_name || p.name}
        </div>
        <code
          style={{
            fontSize: '0.68rem',
            color: '#64748b',
            fontFamily: 'ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace',
          }}
        >
          {p.name}
        </code>
      </div>

      {/* Auth mode badge */}
      <div style={{ flex: '0 0 120px' }}>
        {p.auth_mode && (
          <span
            style={{
              fontSize: '0.65rem',
              fontWeight: 600,
              textTransform: 'uppercase',
              letterSpacing: '0.04em',
              padding: '0.18rem 0.45rem',
              borderRadius: '5px',
              background: '#eef2ff',
              color: '#4f46e5',
              border: '1px solid #c7d2fe',
            }}
          >
            {p.auth_mode}
          </span>
        )}
      </div>

      {/* Category chips */}
      <div style={{ flex: 1, display: 'flex', flexWrap: 'wrap', gap: '0.3rem', minWidth: 0 }}>
        {(p.categories || []).slice(0, 4).map((c, i) => (
          <span
            key={`${p.name}-cat-${i}-${c}`}
            style={{
              fontSize: '0.65rem',
              padding: '0.15rem 0.4rem',
              borderRadius: '5px',
              background: '#f1f5f9',
              color: '#475569',
              border: '1px solid #e2e8f0',
              whiteSpace: 'nowrap',
            }}
          >
            {c}
          </span>
        ))}
        {(p.categories || []).length > 4 && (
          <span style={{ fontSize: '0.65rem', color: '#64748b', whiteSpace: 'nowrap' }}>
            +{p.categories!.length - 4}
          </span>
        )}
      </div>

      {/* Actions */}
      <div style={{ flexShrink: 0, display: 'flex', gap: '0.5rem', alignItems: 'center' }}>
        {p.docs && /^https?:\/\//i.test(p.docs) && (
          <a
            href={p.docs}
            target="_blank"
            rel="noreferrer"
            style={{
              fontSize: '0.75rem',
              color: '#64748b',
              textDecoration: 'none',
              padding: '0.3rem 0.5rem',
              borderRadius: '6px',
              transition: 'color 0.15s, background 0.15s',
            }}
            onMouseEnter={(e) => {
              e.currentTarget.style.color = '#2563eb';
              e.currentTarget.style.background = '#f1f5f9';
            }}
            onMouseLeave={(e) => {
              e.currentTarget.style.color = '#64748b';
              e.currentTarget.style.background = 'transparent';
            }}
          >
            Docs ↗
          </a>
        )}
        <button
          type="button"
          onClick={() =>
            existingPlatform
              ? onOpenConnector(existingPlatform)
              : onAddConnector(p.name)
          }
          style={{
            padding: '0.4rem 0.85rem',
            borderRadius: '8px',
            border: 'none',
            background: '#4f46e5',
            color: 'white',
            fontSize: '0.78rem',
            fontWeight: 600,
            cursor: 'pointer',
            transition: 'background 0.15s',
            whiteSpace: 'nowrap',
          }}
          onMouseEnter={(e) => { (e.currentTarget as HTMLButtonElement).style.background = '#4338ca'; }}
          onMouseLeave={(e) => { (e.currentTarget as HTMLButtonElement).style.background = '#4f46e5'; }}
        >
          {existingPlatform ? 'Manage' : 'Connect'}
        </button>
      </div>
    </article>
  );
}
