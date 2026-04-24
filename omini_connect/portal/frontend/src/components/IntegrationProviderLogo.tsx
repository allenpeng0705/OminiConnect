import { useState } from 'react';

export default function IntegrationProviderLogo({ url, label }: { url?: string; label: string }) {
  const [ok, setOk] = useState(true);
  const initial = (label || '?').trim().charAt(0).toUpperCase();
  if (!url || !ok) {
    return (
      <div
        style={{
          width: 44,
          height: 44,
          borderRadius: '10px',
          background: 'linear-gradient(135deg, #6366f1, #8b5cf6)',
          color: 'white',
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'center',
          fontWeight: 700,
          fontSize: '1.1rem',
          flexShrink: 0,
        }}
      >
        {initial}
      </div>
    );
  }
  return (
    <img
      src={url}
      alt=""
      width={44}
      height={44}
      style={{ borderRadius: '10px', objectFit: 'contain', background: '#f8fafc', flexShrink: 0 }}
      onError={() => setOk(false)}
    />
  );
}
