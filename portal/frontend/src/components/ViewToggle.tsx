type Props = {
  value: 'grid' | 'list';
  onChange: (v: 'grid' | 'list') => void;
};

export default function ViewToggle({ value, onChange }: Props) {
  return (
    <div style={{ display: 'flex', gap: '2px', background: '#f1f5f9', borderRadius: '8px', padding: '2px' }}>
      <button
        type="button"
        title="Grid view"
        onClick={() => onChange('grid')}
        style={{
          padding: '0.35rem 0.6rem',
          borderRadius: '6px',
          border: 'none',
          background: value === 'grid' ? '#4f46e5' : 'transparent',
          color: value === 'grid' ? 'white' : '#64748b',
          cursor: 'pointer',
          fontSize: '0.8rem',
          transition: 'all 0.15s',
          display: 'flex',
          alignItems: 'center',
        }}
      >
        <svg width="14" height="14" viewBox="0 0 14 14" fill="none" stroke="currentColor" strokeWidth="1.5">
          <rect x="1" y="1" width="5" height="5" rx="1" />
          <rect x="8" y="1" width="5" height="5" rx="1" />
          <rect x="1" y="8" width="5" height="5" rx="1" />
          <rect x="8" y="8" width="5" height="5" rx="1" />
        </svg>
      </button>
      <button
        type="button"
        title="List view"
        onClick={() => onChange('list')}
        style={{
          padding: '0.35rem 0.6rem',
          borderRadius: '6px',
          border: 'none',
          background: value === 'list' ? '#4f46e5' : 'transparent',
          color: value === 'list' ? 'white' : '#64748b',
          cursor: 'pointer',
          fontSize: '0.8rem',
          transition: 'all 0.15s',
          display: 'flex',
          alignItems: 'center',
        }}
      >
        <svg width="14" height="14" viewBox="0 0 14 14" fill="none" stroke="currentColor" strokeWidth="1.5">
          <line x1="1" y1="3.5" x2="13" y2="3.5" strokeLinecap="round" />
          <line x1="1" y1="7" x2="13" y2="7" strokeLinecap="round" />
          <line x1="1" y1="10.5" x2="13" y2="10.5" strokeLinecap="round" />
        </svg>
      </button>
    </div>
  );
}
