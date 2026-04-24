import type { IntegrationCatalogRow } from '../api/client';
import IntegrationProviderLogo from './IntegrationProviderLogo';

type Props = {
  row: IntegrationCatalogRow;
  onAddConnector: (providerKey: string) => void;
};

export default function CatalogProviderCard({ row: p, onAddConnector }: Props) {
  return (
    <article
      style={{
        background: 'white',
        borderRadius: '12px',
        padding: '1rem',
        border: '1px solid #e2e8f0',
        boxShadow: '0 1px 3px rgba(15,23,42,0.06)',
        display: 'flex',
        flexDirection: 'column',
        gap: '0.65rem',
        transition: 'border-color 0.15s, box-shadow 0.15s',
      }}
    >
      <div style={{ display: 'flex', gap: '0.75rem', alignItems: 'flex-start' }}>
        <IntegrationProviderLogo url={p.logo_url} label={p.display_name || p.name} />
        <div style={{ minWidth: 0, flex: 1 }}>
          <div style={{ fontWeight: 600, color: '#0f172a', fontSize: '0.95rem', lineHeight: 1.3 }}>{p.display_name || p.name}</div>
          <code style={{ fontSize: '0.75rem', color: '#64748b', wordBreak: 'break-all' }}>{p.name}</code>
        </div>
      </div>
      <div style={{ display: 'flex', flexWrap: 'wrap', gap: '0.35rem' }}>
        {p.auth_mode && (
          <span
            style={{
              fontSize: '0.68rem',
              fontWeight: 600,
              textTransform: 'uppercase',
              letterSpacing: '0.04em',
              padding: '0.2rem 0.45rem',
              borderRadius: '6px',
              background: '#eef2ff',
              color: '#4338ca',
            }}
          >
            {p.auth_mode}
          </span>
        )}
        {(p.categories || []).slice(0, 3).map((c, i) => (
          <span
            key={`${p.name}-cat-${i}-${c}`}
            style={{
              fontSize: '0.68rem',
              padding: '0.2rem 0.45rem',
              borderRadius: '6px',
              background: '#f1f5f9',
              color: '#475569',
            }}
          >
            {c}
          </span>
        ))}
      </div>
      <div style={{ marginTop: 'auto', display: 'flex', flexWrap: 'wrap', gap: '0.5rem', alignItems: 'center' }}>
        <button
          type="button"
          onClick={() => onAddConnector(p.name)}
          style={{
            padding: '0.4rem 0.75rem',
            borderRadius: '8px',
            border: 'none',
            background: '#4f46e5',
            color: 'white',
            fontSize: '0.8rem',
            fontWeight: 600,
            cursor: 'pointer',
          }}
        >
          Add connector
        </button>
        {p.docs && /^https?:\/\//i.test(p.docs) && (
          <a href={p.docs} target="_blank" rel="noreferrer" style={{ fontSize: '0.78rem', color: '#2563eb' }}>
            Docs ↗
          </a>
        )}
      </div>
    </article>
  );
}
