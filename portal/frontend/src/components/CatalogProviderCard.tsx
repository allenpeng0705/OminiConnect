import { useState } from 'react';
import type { IntegrationCatalogRow } from '../api/client';
import IntegrationProviderLogo from './IntegrationProviderLogo';

type Props = {
  row: IntegrationCatalogRow;
  /** If set, this catalog entry already has a connector for the current user — primary action opens it. */
  existingPlatform?: string | null;
  onAddConnector: (providerKey: string) => void;
  onOpenConnector: (platform: string) => void;
};

export default function CatalogProviderCard({ row: p, existingPlatform, onAddConnector, onOpenConnector }: Props) {
  const [hovered, setHovered] = useState(false);
  const [pressed, setPressed] = useState(false);

  return (
    <article
      onMouseEnter={() => setHovered(true)}
      onMouseLeave={() => { setHovered(false); setPressed(false); }}
      style={{
        background: hovered ? '#ffffff' : '#ffffff',
        borderRadius: '14px',
        padding: '1.1rem',
        border: hovered ? '1.5px solid #c7d2fe' : '1px solid #e2e8f0',
        boxShadow: hovered
          ? '0 8px 24px rgba(99, 102, 241, 0.12), 0 2px 8px rgba(15, 23, 42, 0.06)'
          : '0 1px 3px rgba(15, 23, 42, 0.06)',
        display: 'flex',
        flexDirection: 'column',
        gap: '0.75rem',
        flex: 1,
        transition: 'all 0.2s ease',
        transform: pressed ? 'scale(0.98)' : hovered ? 'translateY(-2px)' : 'none',
        cursor: 'default',
      }}
    >
      {/* Header: Logo + Name */}
      <div style={{ display: 'flex', gap: '0.85rem', alignItems: 'center' }}>
        <IntegrationProviderLogo url={p.logo_url} label={p.display_name || p.name} size={52} />
        <div style={{ minWidth: 0, flex: 1 }}>
          <div
            style={{
              fontWeight: 600,
              color: '#0f172a',
              fontSize: '1rem',
              lineHeight: 1.3,
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
              fontSize: '0.72rem',
              color: '#64748b',
              wordBreak: 'break-all',
              fontFamily: 'ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace',
            }}
          >
            {p.name}
          </code>
        </div>
      </div>

      {/* Badges: Auth Mode + Categories */}
      <div style={{ display: 'flex', flexWrap: 'wrap', gap: '0.4rem' }}>
        {p.auth_mode && (
          <span
            style={{
              fontSize: '0.67rem',
              fontWeight: 600,
              textTransform: 'uppercase',
              letterSpacing: '0.04em',
              padding: '0.22rem 0.5rem',
              borderRadius: '6px',
              background: '#eef2ff',
              color: '#4f46e5',
              border: '1px solid #c7d2fe',
            }}
          >
            {p.auth_mode}
          </span>
        )}
        {(p.categories || []).slice(0, 3).map((c, i) => (
          <span
            key={`${p.name}-cat-${i}-${c}`}
            style={{
              fontSize: '0.67rem',
              padding: '0.22rem 0.5rem',
              borderRadius: '6px',
              background: '#f1f5f9',
              color: '#475569',
              border: '1px solid #e2e8f0',
            }}
          >
            {c}
          </span>
        ))}
      </div>

      {/* Footer: Actions */}
      <div
        style={{
          marginTop: 'auto',
          display: 'flex',
          flexWrap: 'wrap',
          gap: '0.5rem',
          alignItems: 'center',
        }}
      >
        <button
          type="button"
          onClick={() =>
            existingPlatform
              ? onOpenConnector(existingPlatform)
              : onAddConnector(p.name)
          }
          onMouseDown={() => setPressed(true)}
          onMouseUp={() => setPressed(false)}
          style={{
            padding: '0.45rem 0.9rem',
            borderRadius: '8px',
            border: 'none',
            background: hovered ? '#4338ca' : '#4f46e5',
            color: 'white',
            fontSize: '0.82rem',
            fontWeight: 600,
            cursor: 'pointer',
            transition: 'all 0.15s ease',
            boxShadow: hovered ? '0 2px 8px rgba(79, 70, 229, 0.4)' : '0 1px 2px rgba(79, 70, 229, 0.2)',
          }}
        >
          {existingPlatform ? 'Manage connector' : 'Connect'}
        </button>
        {p.docs && /^https?:\/\//i.test(p.docs) && (
          <a
            href={p.docs}
            target="_blank"
            rel="noreferrer"
            style={{
              fontSize: '0.78rem',
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
      </div>
    </article>
  );
}
