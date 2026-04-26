type Props = {
  categories: string[];
  selected: string | null;
  counts: Record<string, number>;
  onSelect: (cat: string | null) => void;
};

export default function CategoryFilterBar({ categories, selected, counts, onSelect }: Props) {
  if (categories.length === 0) return null;

  return (
    <div
      style={{
        display: 'flex',
        gap: '0.4rem',
        overflowX: 'auto',
        paddingBottom: '0.25rem',
        scrollbarWidth: 'thin',
        scrollbarColor: '#cbd5e1 transparent',
      }}
    >
      <button
        type="button"
        onClick={() => onSelect(null)}
        style={{
          flexShrink: 0,
          padding: '0.3rem 0.75rem',
          borderRadius: '9999px',
          border: 'none',
          fontSize: '0.78rem',
          fontWeight: 500,
          cursor: 'pointer',
          background: selected === null ? '#4f46e5' : '#f1f5f9',
          color: selected === null ? 'white' : '#475569',
          transition: 'all 0.15s',
          whiteSpace: 'nowrap',
        }}
      >
        All
      </button>
      {categories.map((cat) => {
        const count = counts[cat] ?? 0;
        const isSelected = selected === cat;
        return (
          <button
            type="button"
            key={cat}
            onClick={() => onSelect(isSelected ? null : cat)}
            style={{
              flexShrink: 0,
              padding: '0.3rem 0.75rem',
              borderRadius: '9999px',
              border: 'none',
              fontSize: '0.78rem',
              fontWeight: 500,
              cursor: 'pointer',
              background: isSelected ? '#4f46e5' : '#f1f5f9',
              color: isSelected ? 'white' : '#475569',
              transition: 'all 0.15s',
              whiteSpace: 'nowrap',
              display: 'flex',
              alignItems: 'center',
              gap: '0.35rem',
            }}
          >
            {cat.charAt(0).toUpperCase() + cat.slice(1)}
            <span
              style={{
                fontSize: '0.68rem',
                padding: '0.05rem 0.35rem',
                borderRadius: '9999px',
                background: isSelected ? 'rgba(255,255,255,0.25)' : '#e2e8f0',
                color: isSelected ? 'white' : '#64748b',
              }}
            >
              {count}
            </span>
          </button>
        );
      })}
    </div>
  );
}
