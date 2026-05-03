import { useState } from 'react';
import { executeLlmQuery, type LlmExecuteResponse } from '../api/client';

interface LlmConfig {
  provider: string;
  apiKey: string;
  model: string;
  baseUrl: string;
}

const PROVIDERS = [
  { id: 'openai', name: 'OpenAI', defaultModel: 'gpt-4o-mini', defaultBaseUrl: 'https://api.openai.com/v1' },
  { id: 'anthropic', name: 'Anthropic', defaultModel: 'claude-3-5-sonnet-20241022', defaultBaseUrl: 'https://api.anthropic.com' },
  { id: 'minimax', name: 'Minimax', defaultModel: 'MiniMax-M2.7', defaultBaseUrl: 'https://api.minimaxi.com/v1' },
  { id: 'ollama', name: 'Ollama', defaultModel: 'llama3', defaultBaseUrl: 'http://localhost:11434/v1' },
  { id: 'gemini', name: 'Google Gemini', defaultModel: 'gemini-1.5-flash', defaultBaseUrl: 'https://generativelanguage.googleapis.com/v1beta' },
  { id: 'litellm', name: 'LiteLLM Proxy', defaultModel: 'gpt-4o-mini', defaultBaseUrl: 'http://localhost:4000' },
];

interface Props {
  connectedPlatforms: string[];
}

export default function TestPanel({ connectedPlatforms }: Props) {
  const [collapsed, setCollapsed] = useState(true);
  const [config, setConfig] = useState<LlmConfig>({
    provider: 'openai',
    apiKey: '',
    model: 'gpt-4o-mini',
    baseUrl: 'https://api.openai.com/v1',
  });
  const [query, setQuery] = useState('');
  const [platformHint, setPlatformHint] = useState('');
  const [loading, setLoading] = useState(false);
  const [result, setResult] = useState<LlmExecuteResponse | null>(null);
  const [error, setError] = useState('');

  function handleProviderChange(providerId: string) {
    const provider = PROVIDERS.find(p => p.id === providerId);
    if (provider) {
      setConfig({
        ...config,
        provider: providerId,
        model: provider.defaultModel,
        baseUrl: provider.defaultBaseUrl,
      });
    }
  }

  async function handleTest() {
    if (!config.apiKey.trim()) {
      setError('Please enter an API key');
      return;
    }
    if (!query.trim()) {
      setError('Please enter a query');
      return;
    }

    setLoading(true);
    setError('');
    setResult(null);

    try {
      const resp = await executeLlmQuery({
        query: query.trim(),
        platform: platformHint || undefined,
        llm_url: config.baseUrl,
        llm_api_key: config.apiKey,
        llm_model: config.model,
      });
      setResult(resp);
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Request failed');
    } finally {
      setLoading(false);
    }
  }

  function renderResult(r: LlmExecuteResponse) {
    if (!r.ok && r.error) {
      return (
        <div style={{ padding: '0.75rem', background: '#fef2f2', borderRadius: '8px', color: '#b91c1c', fontSize: '0.875rem' }}>
          {r.error}
        </div>
      );
    }

    return (
      <div style={{ display: 'flex', flexDirection: 'column', gap: '0.75rem' }}>
        {r.tool && (
          <div>
            <div style={{ fontSize: '0.75rem', color: '#64748b', marginBottom: '0.25rem' }}>Tool Selected</div>
            <div style={{ fontFamily: 'ui-monospace, monospace', fontSize: '0.875rem', color: '#4f46e5' }}>
              {r.tool_name || r.tool}
            </div>
          </div>
        )}
        {r.explanation && (
          <div>
            <div style={{ fontSize: '0.75rem', color: '#64748b', marginBottom: '0.25rem' }}>Explanation</div>
            <div style={{ fontSize: '0.875rem', color: '#374151' }}>{r.explanation}</div>
          </div>
        )}
        {r.result !== undefined && (
          <div>
            <div style={{ fontSize: '0.75rem', color: '#64748b', marginBottom: '0.25rem' }}>Result</div>
            <pre style={{
              background: '#f8fafc',
              border: '1px solid #e2e8f0',
              borderRadius: '8px',
              padding: '0.75rem',
              fontSize: '0.75rem',
              fontFamily: 'ui-monospace, monospace',
              overflow: 'auto',
              maxHeight: '300px',
              margin: 0,
            }}>
              {JSON.stringify(r.result, null, 2)}
            </pre>
          </div>
        )}
        {r.message && (
          <div style={{ padding: '0.75rem', background: '#fef3c7', borderRadius: '8px', color: '#92400e', fontSize: '0.875rem' }}>
            {r.message}
          </div>
        )}
        {r.candidates && r.candidates.length > 0 && (
          <div>
            <div style={{ fontSize: '0.75rem', color: '#64748b', marginBottom: '0.25rem' }}>Did you mean one of these?</div>
            <div style={{ display: 'flex', flexWrap: 'wrap', gap: '0.5rem' }}>
              {r.candidates.map((c, i) => (
                <span key={i} style={{
                  padding: '0.25rem 0.5rem',
                  borderRadius: '4px',
                  background: '#f1f5f9',
                  fontSize: '0.75rem',
                  fontFamily: 'ui-monospace, monospace',
                }}>
                  {c.tool_name} ({c.score.toFixed(2)})
                </span>
              ))}
            </div>
          </div>
        )}
      </div>
    );
  }

  return (
    <section style={{ marginBottom: '2rem' }}>
      {/* Header */}
      <div
        style={{
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'space-between',
          cursor: 'pointer',
          padding: '0.75rem 1rem',
          background: '#fafafa',
          border: '1px solid #e5e7eb',
          borderRadius: '10px',
          marginBottom: collapsed ? 0 : '1rem',
        }}
        onClick={() => setCollapsed(!collapsed)}
      >
        <div style={{ display: 'flex', alignItems: 'center', gap: '0.5rem' }}>
          <span style={{ fontSize: '1.25rem' }}>{collapsed ? '▶' : '▼'}</span>
          <h2 style={{ fontSize: '1rem', fontWeight: 600, color: '#0f172a', margin: 0 }}>
            Test Panel
          </h2>
          <span style={{ fontSize: '0.75rem', color: '#64748b', background: '#f1f5f9', padding: '0.125rem 0.5rem', borderRadius: '9999px' }}>
            Natural Language
          </span>
        </div>
      </div>

      {!collapsed && (
        <div style={{ background: 'white', border: '1px solid #e2e8f0', borderRadius: '10px', padding: '1.25rem' }}>
          {/* LLM Config */}
          <div style={{ marginBottom: '1.25rem' }}>
            <h3 style={{ fontSize: '0.875rem', fontWeight: 600, color: '#374151', marginBottom: '0.75rem' }}>
              LLM Configuration
            </h3>
            <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fill, minmax(200px, 1fr))', gap: '0.75rem' }}>
              <div>
                <label style={{ display: 'block', fontSize: '0.75rem', color: '#64748b', marginBottom: '0.25rem' }}>Provider</label>
                <select
                  value={config.provider}
                  onChange={(e) => handleProviderChange(e.target.value)}
                  style={{ width: '100%', padding: '0.5rem', border: '1px solid #ccc', borderRadius: '6px', fontSize: '0.875rem', boxSizing: 'border-box' }}
                >
                  {PROVIDERS.map(p => (
                    <option key={p.id} value={p.id}>{p.name}</option>
                  ))}
                </select>
              </div>
              <div>
                <label style={{ display: 'block', fontSize: '0.75rem', color: '#64748b', marginBottom: '0.25rem' }}>API Key</label>
                <input
                  type="password"
                  value={config.apiKey}
                  onChange={(e) => setConfig({ ...config, apiKey: e.target.value })}
                  placeholder="sk-..."
                  style={{ width: '100%', padding: '0.5rem', border: '1px solid #ccc', borderRadius: '6px', fontSize: '0.875rem', boxSizing: 'border-box' }}
                />
              </div>
              <div>
                <label style={{ display: 'block', fontSize: '0.75rem', color: '#64748b', marginBottom: '0.25rem' }}>Model</label>
                <input
                  type="text"
                  value={config.model}
                  onChange={(e) => setConfig({ ...config, model: e.target.value })}
                  placeholder="gpt-4o-mini"
                  style={{ width: '100%', padding: '0.5rem', border: '1px solid #ccc', borderRadius: '6px', fontSize: '0.875rem', boxSizing: 'border-box' }}
                />
              </div>
              <div>
                <label style={{ display: 'block', fontSize: '0.75rem', color: '#64748b', marginBottom: '0.25rem' }}>Base URL</label>
                <input
                  type="text"
                  value={config.baseUrl}
                  onChange={(e) => setConfig({ ...config, baseUrl: e.target.value })}
                  placeholder="https://api.openai.com/v1"
                  style={{ width: '100%', padding: '0.5rem', border: '1px solid #ccc', borderRadius: '6px', fontSize: '0.875rem', boxSizing: 'border-box' }}
                />
              </div>
            </div>
          </div>

          {/* Query */}
          <div style={{ marginBottom: '1rem' }}>
            <h3 style={{ fontSize: '0.875rem', fontWeight: 600, color: '#374151', marginBottom: '0.75rem' }}>
              Natural Language Query
            </h3>
            <div style={{ display: 'flex', gap: '0.75rem', flexWrap: 'wrap' }}>
              <textarea
                value={query}
                onChange={(e) => setQuery(e.target.value)}
                placeholder="e.g., list my GitHub repos, or 查询某公司的工商信息"
                style={{
                  flex: '1 1 300px',
                  padding: '0.5rem',
                  border: '1px solid #ccc',
                  borderRadius: '6px',
                  fontSize: '0.875rem',
                  minHeight: '80px',
                  resize: 'vertical',
                  fontFamily: 'inherit',
                  boxSizing: 'border-box',
                }}
              />
              <div style={{ display: 'flex', flexDirection: 'column', gap: '0.5rem' }}>
                {connectedPlatforms.length > 0 && (
                  <div>
                    <label style={{ display: 'block', fontSize: '0.75rem', color: '#64748b', marginBottom: '0.25rem' }}>Platform (optional)</label>
                    <select
                      value={platformHint}
                      onChange={(e) => setPlatformHint(e.target.value)}
                      style={{ padding: '0.5rem', border: '1px solid #ccc', borderRadius: '6px', fontSize: '0.875rem' }}
                    >
                      <option value="">Auto-detect</option>
                      {connectedPlatforms.map(p => (
                        <option key={p} value={p}>{p}</option>
                      ))}
                    </select>
                  </div>
                )}
                <button
                  onClick={handleTest}
                  disabled={loading || !config.apiKey.trim() || !query.trim()}
                  style={{
                    padding: '0.5rem 1.25rem',
                    borderRadius: '8px',
                    border: 'none',
                    background: loading || !config.apiKey.trim() || !query.trim() ? '#cbd5e1' : '#4f46e5',
                    color: 'white',
                    fontSize: '0.875rem',
                    fontWeight: 600,
                    cursor: loading || !config.apiKey.trim() || !query.trim() ? 'not-allowed' : 'pointer',
                  }}
                >
                  {loading ? 'Processing...' : 'Execute'}
                </button>
              </div>
            </div>
          </div>

          {/* Error */}
          {error && (
            <div style={{ marginBottom: '1rem', padding: '0.75rem', background: '#fef2f2', borderRadius: '8px', color: '#b91c1c', fontSize: '0.875rem' }}>
              {error}
            </div>
          )}

          {/* Result */}
          {result && (
            <div>
              <h3 style={{ fontSize: '0.875rem', fontWeight: 600, color: '#374151', marginBottom: '0.75rem' }}>
                Result
              </h3>
              {renderResult(result)}
            </div>
          )}
        </div>
      )}
    </section>
  );
}
