import { useState } from 'react';

interface Props {
  url?: string;
  label: string;
  size?: number;
}

export default function IntegrationProviderLogo({ url, label, size = 48 }: Props) {
  const [imgFailed, setImgFailed] = useState(false);

  const initial = (label || '?').trim().charAt(0).toUpperCase();
  const showFallback = !url || imgFailed;

  // Generate a consistent color from the label
  const getGradient = (text: string): string => {
    const gradients = [
      'linear-gradient(135deg, #6366f1, #8b5cf6)',
      'linear-gradient(135deg, #ec4899, #f43f5e)',
      'linear-gradient(135deg, #f59e0b, #ef4444)',
      'linear-gradient(135deg, #10b981, #059669)',
      'linear-gradient(135deg, #3b82f6, #1d4ed8)',
      'linear-gradient(135deg, #8b5cf6, #7c3aed)',
      'linear-gradient(135deg, #14b8a6, #0d9488)',
      'linear-gradient(135deg, #f97316, #ea580c)',
    ];
    let hash = 0;
    for (let i = 0; i < text.length; i++) {
      hash = text.charCodeAt(i) + ((hash << 5) - hash);
    }
    return gradients[Math.abs(hash) % gradients.length];
  };

  if (showFallback) {
    return (
      <div
        style={{
          width: size,
          height: size,
          borderRadius: '12px',
          background: getGradient(label),
          color: 'white',
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'center',
          fontWeight: 700,
          fontSize: size * 0.42,
          flexShrink: 0,
          boxShadow: '0 2px 8px rgba(0,0,0,0.15)',
        }}
      >
        {initial}
      </div>
    );
  }

  return (
    <div
      style={{
        width: size,
        height: size,
        borderRadius: '12px',
        background: '#f8fafc',
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'center',
        flexShrink: 0,
        boxShadow: '0 1px 3px rgba(0,0,0,0.08), inset 0 1px 0 rgba(255,255,255,0.5)',
        border: '1px solid #e2e8f0',
        overflow: 'hidden',
      }}
    >
      <img
        src={url}
        alt=""
        width={size - 4}
        height={size - 4}
        style={{ objectFit: 'contain' }}
        onError={() => setImgFailed(true)}
      />
    </div>
  );
}
