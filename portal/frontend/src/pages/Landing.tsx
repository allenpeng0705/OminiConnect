import { useState } from 'react';
import { Link } from 'react-router-dom';

const EXAMPLES = [
  {
    id: 'direct-proxy',
    title: 'Call GitHub API directly via OminiConnect proxy',
    curl: `# List GitHub repositories
curl -X POST https://api.ominiconnect.com/api/proxy/github \\
  -H "Authorization: Bearer $OMINICONNECT_API_KEY" \\
  -H "Content-Type: application/json" \\
  -d '{"endpoint": "/user/repos", "method": "GET"}'`,
    python: `import requests

resp = requests.post(
    "https://api.ominiconnect.com/api/proxy/github",
    headers={"Authorization": f"Bearer {api_key}"},
    json={"endpoint": "/user/repos", "method": "GET"}
)
repos = resp.json()`,
  },
  {
    id: 'tool-registry',
    title: 'Get available tools for a provider',
    curl: `# List available tools for GitHub
curl https://api.ominiconnect.com/api/tools?provider=github \\
  -H "Authorization: Bearer $OMINICONNECT_API_KEY"`,
    python: `import requests

resp = requests.get(
    "https://api.ominiconnect.com/api/tools",
    params={"provider": "github"},
    headers={"Authorization": f"Bearer {api_key}"}
)
tools = resp.json()  # [{slug, name, description, inputSchema, ...}]`,
  },
  {
    id: 'tool-execution',
    title: 'Execute a tool (list repositories)',
    curl: `# Execute tool: list GitHub repos
curl -X POST https://api.ominiconnect.com/api/tools/execute \\
  -H "Authorization: Bearer $OMINICONNECT_API_KEY" \\
  -H "Content-Type: application/json" \\
  -d '{"tool": "github_list_repos", "params": {"limit": 10}}'`,
    python: `import requests

resp = requests.post(
    "https://api.ominiconnect.com/api/tools/execute",
    headers={"Authorization": f"Bearer {api_key}"},
    json={"tool": "github_list_repos", "params": {"limit": 10}}
)
repos = resp.json()`,
  },
  {
    id: 'ai-delegation',
    title: 'Send natural language request (AI delegation)',
    curl: `# Send natural language request
curl -X POST https://api.ominiconnect.com/api/llm/chat \\
  -H "Authorization: Bearer $OMINICONNECT_API_KEY" \\
  -H "Content-Type: application/json" \\
  -d '{"message": "List my GitHub repositories and show their descriptions"}'`,
    python: `import requests

resp = requests.post(
    "https://api.ominiconnect.com/api/llm/chat",
    headers={"Authorization": f"Bearer {api_key}"},
    json={"message": "List my GitHub repositories and show their descriptions"}
)
result = resp.json()  # {message: "...", tools_used: [...], result: ...}`,
  },
  {
    id: 'connect-service',
    title: 'Connect a service via OAuth (Nango)',
    curl: `# Create Nango connect session
curl -X POST https://api.ominiconnect.com/api/nango/connect \\
  -H "Authorization: Bearer $OMINICONNECT_API_KEY" \\
  -H "Content-Type: application/json" \\
  -d '{"provider": "github"}'`,
    python: `import requests

resp = requests.post(
    "https://api.ominiconnect.com/api/nango/connect",
    headers={"Authorization": f"Bearer {api_key}"},
    json={"provider": "github"}
)
session = resp.json()  # {connect_url: "https://..."}
# Redirect user to session["connect_url"]`,
  },
];

export default function Landing() {
  const [activeTab, setActiveTab] = useState('direct-proxy-curl');

  return (
    <div style={{ minHeight: '100vh', background: '#09090b', color: '#fff' }}>
      {/* Header */}
      <header style={{ position: 'fixed', top: 0, left: 0, right: 0, zIndex: 50, background: 'rgba(9,9,11,0.8)', backdropFilter: 'blur(12px)', borderBottom: '1px solid rgba(255,255,255,0.1)' }}>
        <div style={{ maxWidth: '1200px', margin: '0 auto', padding: '0 1.5rem', display: 'flex', alignItems: 'center', justifyContent: 'space-between', height: '64px' }}>
          <div style={{ display: 'flex', alignItems: 'center', gap: '0.75rem' }}>
            <img src="/images/logos/ominiconnect_logo_with_text.svg" alt="OminiConnect" style={{ height: '32px', objectFit: 'contain' }} />
          </div>
          <div style={{ display: 'flex', alignItems: 'center', gap: '0.75rem' }}>
            <Link to="/auth/login" style={{ color: '#a1a1aa', textDecoration: 'none', fontSize: '0.9rem', fontWeight: 500, padding: '0.5rem 1rem' }}>Sign in</Link>
            <Link to="/auth/signup" style={{ background: '#6366f1', color: '#fff', textDecoration: 'none', fontSize: '0.9rem', fontWeight: 600, padding: '0.5rem 1.25rem', borderRadius: '8px' }}>Get Started</Link>
          </div>
        </div>
      </header>

      {/* Hero Section */}
      <section style={{ paddingTop: '8rem', paddingBottom: '6rem', textAlign: 'center' }}>
        <div style={{ maxWidth: '800px', margin: '0 auto', padding: '0 1.5rem' }}>
          <h1 style={{ fontSize: 'clamp(2.5rem, 6vw, 4rem)', fontWeight: 700, lineHeight: 1.1, marginBottom: '1.5rem', background: 'linear-gradient(135deg, #fff 0%, #a1a1aa 100%)', WebkitBackgroundClip: 'text', WebkitTextFillColor: 'transparent' }}>
            One portal to connect them all
          </h1>
          <p style={{ fontSize: '1.25rem', color: '#a1a1aa', lineHeight: 1.6, maxWidth: '640px', margin: '0 auto 2.5rem' }}>
            OminiConnect gives AI agents and automation workflows unified access to 700+ APIs through a single, consistent interface. No more per-integration complexity.
          </p>
          <div style={{ display: 'flex', gap: '1rem', justifyContent: 'center', flexWrap: 'wrap' }}>
            <Link to="/auth/signup" style={{ background: '#6366f1', color: '#fff', textDecoration: 'none', fontSize: '1rem', fontWeight: 600, padding: '0.875rem 2rem', borderRadius: '10px', display: 'inline-flex', alignItems: 'center', gap: '0.5rem' }}>
              Start for free
              <svg width="16" height="16" viewBox="0 0 16 16" fill="none"><path d="M3 8h10M9 4l4 4-4 4" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round"/></svg>
            </Link>
            <Link to="/auth/login" style={{ background: 'rgba(255,255,255,0.05)', color: '#fff', textDecoration: 'none', fontSize: '1rem', fontWeight: 500, padding: '0.875rem 1.5rem', borderRadius: '10px', border: '1px solid rgba(255,255,255,0.15)' }}>
              Sign in
            </Link>
          </div>
        </div>
      </section>

      {/* Problem / Solution Section */}
      <section style={{ padding: '5rem 0', background: 'rgba(255,255,255,0.02)' }}>
        <div style={{ maxWidth: '1100px', margin: '0 auto', padding: '0 1.5rem' }}>
          <h2 style={{ fontSize: '2rem', fontWeight: 700, textAlign: 'center', marginBottom: '4rem' }}>How OminiConnect works</h2>
          <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(300px, 1fr))', gap: '2rem' }}>
            {/* Approach 1 - Before */}
            <div style={{ background: 'rgba(239,68,68,0.08)', border: '1px solid rgba(239,68,68,0.2)', borderRadius: '16px', padding: '2rem' }}>
              <div style={{ display: 'inline-flex', alignItems: 'center', justifyContent: 'center', width: '40px', height: '40px', background: 'rgba(239,68,68,0.2)', borderRadius: '10px', marginBottom: '1.25rem' }}>
                <span style={{ color: '#fca5a5', fontWeight: 700, fontSize: '0.9rem' }}>1</span>
              </div>
              <h3 style={{ fontSize: '1.1rem', fontWeight: 700, marginBottom: '0.75rem', color: '#fca5a5' }}>Without OminiConnect</h3>
              <p style={{ color: '#a1a1aa', fontSize: '0.9rem', lineHeight: 1.6, marginBottom: '1.5rem' }}>
                Every AI agent needs custom code for each API. Each integration means auth, rate limiting, retries, and different response formats.
              </p>
              <div style={{ display: 'flex', flexDirection: 'column', gap: '0.75rem' }}>
                {['GitHub', 'Slack', 'Notion', 'Salesforce', '+ 696 more'].map((name, i) => (
                  <div key={i} style={{ background: 'rgba(239,68,68,0.1)', border: '1px solid rgba(239,68,68,0.15)', borderRadius: '8px', padding: '0.75rem 1rem', display: 'flex', alignItems: 'center', gap: '0.75rem' }}>
                    <div style={{ width: '28px', height: '28px', background: 'rgba(239,68,68,0.3)', borderRadius: '6px', display: 'flex', alignItems: 'center', justifyContent: 'center', fontSize: '0.75rem', fontWeight: 700, color: '#fca5a5' }}>{name[0]}</div>
                    <span style={{ fontSize: '0.85rem', color: '#d4d4d8' }}>{name}</span>
                    <div style={{ marginLeft: 'auto', fontSize: '0.7rem', color: '#fca5a5', fontWeight: 600, background: 'rgba(239,68,68,0.2)', padding: '0.2rem 0.5rem', borderRadius: '4px' }}>Custom</div>
                  </div>
                ))}
              </div>
              <div style={{ marginTop: '1.25rem', padding: '0.875rem', background: 'rgba(239,68,68,0.05)', borderRadius: '8px', fontSize: '0.8rem', color: '#a1a1aa', textAlign: 'center' }}>
                Each integration = 100s of lines of custom code
              </div>
            </div>

            {/* Approach 2 - After */}
            <div style={{ background: 'rgba(34,197,94,0.08)', border: '1px solid rgba(34,197,94,0.2)', borderRadius: '16px', padding: '2rem' }}>
              <div style={{ display: 'inline-flex', alignItems: 'center', justifyContent: 'center', width: '40px', height: '40px', background: 'rgba(34,197,94,0.2)', borderRadius: '10px', marginBottom: '1.25rem' }}>
                <span style={{ color: '#86efac', fontWeight: 700, fontSize: '0.9rem' }}>2</span>
              </div>
              <h3 style={{ fontSize: '1.1rem', fontWeight: 700, marginBottom: '0.75rem', color: '#86efac' }}>With OminiConnect</h3>
              <p style={{ color: '#a1a1aa', fontSize: '0.9rem', lineHeight: 1.6, marginBottom: '1.5rem' }}>
                AI agents talk to one unified API. OminiConnect handles auth, proxies requests, and returns consistent responses across all providers.
              </p>
              <div style={{ display: 'flex', flexDirection: 'column', gap: '0.75rem' }}>
                <div style={{ background: 'rgba(34,197,94,0.1)', border: '1px solid rgba(34,197,94,0.15)', borderRadius: '8px', padding: '1rem', display: 'flex', alignItems: 'center', justifyContent: 'center', gap: '0.5rem' }}>
                  <div style={{ width: '28px', height: '28px', background: '#6366f1', borderRadius: '6px', display: 'flex', alignItems: 'center', justifyContent: 'center', fontSize: '0.7rem', fontWeight: 700, color: '#fff' }}>AI</div>
                  <span style={{ fontSize: '0.9rem', color: '#d4d4d8', fontWeight: 600 }}>Your AI Agent</span>
                </div>
                <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'center', padding: '0.5rem' }}>
                  <svg width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M12 5v14M5 12h14" stroke="#6366f1" strokeWidth="2" strokeLinecap="round"/></svg>
                </div>
                <div style={{ background: 'rgba(99,102,241,0.15)', border: '1px solid rgba(99,102,241,0.3)', borderRadius: '8px', padding: '1rem', display: 'flex', alignItems: 'center', justifyContent: 'center', gap: '0.5rem' }}>
                  <img src="/images/logos/ominiconnect_logo_with_text.svg" alt="OminiConnect" style={{ height: '22px' }} />
                  <span style={{ fontSize: '0.8rem', color: '#a5b4fc', fontWeight: 600 }}>OminiConnect</span>
                </div>
                <div style={{ display: 'flex', flexWrap: 'wrap', gap: '0.5rem', justifyContent: 'center', paddingTop: '0.5rem' }}>
                  {['GitHub', 'Slack', 'Notion', 'Salesforce', '+696'].map((name, i) => (
                    <div key={i} style={{ background: 'rgba(34,197,94,0.1)', border: '1px solid rgba(34,197,94,0.15)', borderRadius: '6px', padding: '0.3rem 0.6rem', fontSize: '0.75rem', color: '#86efac', fontWeight: 500 }}>
                      {name}
                    </div>
                  ))}
                </div>
              </div>
              <div style={{ marginTop: '1.25rem', padding: '0.875rem', background: 'rgba(34,197,94,0.05)', borderRadius: '8px', fontSize: '0.8rem', color: '#a1a1aa', textAlign: 'center' }}>
                One API, 700+ integrations
              </div>
            </div>
          </div>
        </div>
      </section>

      {/* Features Section */}
      <section style={{ padding: '5rem 0' }}>
        <div style={{ maxWidth: '1100px', margin: '0 auto', padding: '0 1.5rem' }}>
          <h2 style={{ fontSize: '2rem', fontWeight: 700, textAlign: 'center', marginBottom: '1rem' }}>Four ways to use OminiConnect</h2>
          <p style={{ color: '#71717a', textAlign: 'center', maxWidth: '640px', margin: '0 auto 3.5rem', fontSize: '1.05rem' }}>Choose the integration model that fits your architecture — from full AI delegation to direct API access.</p>
          <div style={{ display: 'grid', gridTemplateColumns: 'repeat(2, 1fr)', gap: '1.5rem' }}>
            {[
              {
                title: 'AI Agent Delegation',
                desc: 'Send a natural language request to OminiConnect. Our LLM handles tool selection, execution, and returns the final result. You focus on the conversation, we handle the APIs.',
                tag: 'Full delegation',
                tagColor: 'rgba(99,102,241,0.2)',
                tagText: '#a5b4fc',
              },
              {
                title: 'Tool Registry + Your LLM',
                desc: 'OminiConnect provides provider metadata and tool definitions via API. Your LLM workflow selects the right tools, then calls OminiConnect to execute. You keep control of the LLM.',
                tag: 'Hybrid',
                tagColor: 'rgba(34,197,94,0.15)',
                tagText: '#86efac',
              },
              {
                title: 'Unified API Proxy',
                desc: 'Call any provider API directly through OminiConnect. Same interface for all 700+ providers — no more writing custom integration code for each service.',
                tag: 'API access',
                tagColor: 'rgba(245,158,11,0.15)',
                tagText: '#fcd34d',
              },
              {
                title: 'Integration Hub',
                desc: 'Connect your accounts once via OAuth. OminiConnect manages credentials, handles token refresh, and proxies requests. Same model as Nango — powered by Nango.',
                tag: 'OAuth hub',
                tagColor: 'rgba(236,72,153,0.15)',
                tagText: '#f9a8d4',
              },
            ].map(({ title, desc, tag, tagColor, tagText }) => (
              <div key={title} style={{ background: 'rgba(255,255,255,0.03)', border: '1px solid rgba(255,255,255,0.08)', borderRadius: '12px', padding: '1.5rem' }}>
                <h4 style={{ fontSize: '1rem', fontWeight: 700, margin: '0 0 1rem' }}>{title}</h4>
                <p style={{ color: '#a1a1aa', fontSize: '0.875rem', lineHeight: 1.6, margin: '0 0 1rem' }}>{desc}</p>
                <span style={{ fontSize: '0.7rem', fontWeight: 600, padding: '0.2rem 0.5rem', borderRadius: '4px', background: tagColor, color: tagText }}>{tag}</span>
              </div>
            ))}
          </div>
        </div>
      </section>

      {/* SDKs & REST API Section */}
      <section style={{ padding: '5rem 0', background: 'rgba(255,255,255,0.02)' }}>
        <div style={{ maxWidth: '1100px', margin: '0 auto', padding: '0 1.5rem' }}>
          <h2 style={{ fontSize: '2rem', fontWeight: 700, textAlign: 'center', marginBottom: '1rem' }}>SDKs & REST API</h2>
          <p style={{ color: '#71717a', textAlign: 'center', maxWidth: '640px', margin: '0 auto 3.5rem', fontSize: '1.05rem' }}>Official libraries for popular languages — plus a full REST API for any platform.</p>
          <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(160px, 1fr))', gap: '1rem', marginBottom: '3rem' }}>
            {[
              { name: 'Python', icon: '🐍', install: 'pip install ominiconnect' },
              { name: 'JavaScript', icon: '📜', install: 'npm install @ominiconnect/sdk' },
              { name: 'Go', icon: '🔵', install: 'go get github.com/allenpeng0705/OminiConnect/...' },
              { name: 'Swift', icon: '🍎', install: 'Swift Package Manager' },
              { name: 'Flutter', icon: '🦋', install: 'flutter pub add ominiconnect' },
              { name: 'Rust', icon: '🦀', install: 'cargo add ominiconnect' },
            ].map(({ name, icon, install }) => (
              <div key={name} style={{ background: 'rgba(255,255,255,0.03)', border: '1px solid rgba(255,255,255,0.08)', borderRadius: '12px', padding: '1.25rem', textAlign: 'center' }}>
                <div style={{ fontSize: '2rem', marginBottom: '0.75rem' }}>{icon}</div>
                <h4 style={{ fontSize: '0.95rem', fontWeight: 700, margin: '0 0 0.5rem' }}>{name}</h4>
                <p style={{ color: '#71717a', fontSize: '0.7rem', margin: 0, fontFamily: 'ui-monospace, monospace' }}>{install}</p>
              </div>
            ))}
          </div>
          <div style={{ background: 'rgba(99,102,241,0.08)', border: '1px solid rgba(99,102,241,0.2)', borderRadius: '12px', padding: '1.5rem', textAlign: 'center' }}>
            <p style={{ color: '#a1a1aa', fontSize: '0.9rem', margin: '0 0 0.75rem' }}>Prefer raw HTTP? The REST API works with any language or tool.</p>
            <code style={{ color: '#a5b4fc', fontSize: '0.8rem' }}>https://api.ominiconnect.com/v1</code>
          </div>
        </div>
      </section>

      {/* Code Examples Section */}
      <section style={{ padding: '5rem 0' }}>
        <div style={{ maxWidth: '1100px', margin: '0 auto', padding: '0 1.5rem' }}>
          <h2 style={{ fontSize: '2rem', fontWeight: 700, textAlign: 'center', marginBottom: '1rem' }}>Code examples</h2>
          <p style={{ color: '#71717a', textAlign: 'center', maxWidth: '560px', margin: '0 auto 3rem', fontSize: '1.05rem' }}>Copy and paste-ready examples for each integration pattern.</p>
          <div style={{ display: 'flex', flexDirection: 'column', gap: '1rem' }}>
            {EXAMPLES.map(({ id, title, curl, python }) => (
              <details key={id} style={{ background: 'rgba(0,0,0,0.3)', border: '1px solid rgba(255,255,255,0.1)', borderRadius: '12px', overflow: 'hidden' }}>
                <summary style={{ padding: '1rem 1.25rem', cursor: 'pointer', fontWeight: 600, fontSize: '0.95rem', listStyle: 'none', display: 'flex', alignItems: 'center', gap: '0.75rem' }}>
                  <svg width="16" height="16" viewBox="0 0 16 16" fill="none" style={{ color: '#a1a1aa' }}><path d="M6 4l4 4-4 4" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round"/></svg>
                  {title}
                </summary>
                <div style={{ padding: '0 1.25rem 1.25rem' }}>
                  <div style={{ display: 'flex', gap: '0.5rem', marginBottom: '0.75rem' }}>
                    <button onClick={() => setActiveTab(id + '-curl')} style={{ padding: '0.35rem 0.75rem', fontSize: '0.75rem', borderRadius: '6px', border: 'none', cursor: 'pointer', background: activeTab === id + '-curl' ? '#6366f1' : 'rgba(255,255,255,0.1)', color: '#fff' }}>cURL</button>
                    <button onClick={() => setActiveTab(id + '-python')} style={{ padding: '0.35rem 0.75rem', fontSize: '0.75rem', borderRadius: '6px', border: 'none', cursor: 'pointer', background: activeTab === id + '-python' ? '#6366f1' : 'rgba(255,255,255,0.1)', color: '#fff' }}>Python</button>
                  </div>
                  <pre style={{ margin: 0, padding: '1rem', background: 'rgba(0,0,0,0.5)', borderRadius: '8px', fontSize: '0.8rem', overflow: 'auto', color: '#e4e4e7', fontFamily: 'ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace' }}>
                    {activeTab === id + '-curl' ? curl : python}
                  </pre>
                </div>
              </details>
            ))}
          </div>
        </div>
      </section>

      {/* CTA Section */}
      <section style={{ padding: '5rem 0', background: 'linear-gradient(135deg, rgba(99,102,241,0.15) 0%, rgba(139,92,246,0.15) 100%)' }}>
        <div style={{ maxWidth: '600px', margin: '0 auto', padding: '0 1.5rem', textAlign: 'center' }}>
          <h2 style={{ fontSize: '2rem', fontWeight: 700, marginBottom: '1rem' }}>Ready to simplify your AI integrations?</h2>
          <p style={{ color: '#a1a1aa', marginBottom: '2rem', fontSize: '1.05rem' }}>Get started in minutes. No credit card required.</p>
          <Link to="/auth/signup" style={{ display: 'inline-flex', alignItems: 'center', gap: '0.5rem', background: '#6366f1', color: '#fff', textDecoration: 'none', fontSize: '1.1rem', fontWeight: 600, padding: '1rem 2.5rem', borderRadius: '10px' }}>
            Get started free
            <svg width="16" height="16" viewBox="0 0 16 16" fill="none"><path d="M3 8h10M9 4l4 4-4 4" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round"/></svg>
          </Link>
        </div>
      </section>

      {/* Footer */}
      <footer style={{ padding: '2rem 0', borderTop: '1px solid rgba(255,255,255,0.08)' }}>
        <div style={{ maxWidth: '1100px', margin: '0 auto', padding: '0 1.5rem', display: 'flex', alignItems: 'center', justifyContent: 'space-between', flexWrap: 'wrap', gap: '1rem' }}>
          <div style={{ display: 'flex', alignItems: 'center', gap: '0.5rem' }}>
            <span style={{ color: '#52525b', fontSize: '0.9rem', fontWeight: 600 }}>OminiConnect</span>
          </div>
          <div style={{ display: 'flex', alignItems: 'center', gap: '1rem' }}>
            <a href="https://github.com/allenpeng0705/OminiConnect" target="_blank" rel="noopener noreferrer" style={{ color: '#71717a', fontSize: '0.8rem', display: 'flex', alignItems: 'center', gap: '0.35rem' }}>
              <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor"><path d="M12 0C5.37 0 0 5.37 0 12c0 5.31 3.435 9.795 8.205 11.385.6.105.825-.255.825-.57 0-.285-.015-1.23-.015-2.235-3.015.555-3.795-.735-4.035-1.41-.135-.345-.72-1.41-1.23-1.695-.42-.225-1.02-.78-.015-.795.945-.015 1.62.87 1.845 1.23 1.08 1.815 2.805 1.305 3.495.99.105-.78.42-1.305.765-1.605-2.67-.3-5.46-1.335-5.46-5.925 0-1.305.465-2.385 1.23-3.225-.12-.3-.54-1.53.12-3.18 0 0 1.005-.315 3.3 1.23.96-.27 1.98-.405 3-.405s2.04.135 3 .405c2.295-1.56 3.3-1.23 3.3-1.23.66 1.65.24 2.88.12 3.18.765.84 1.23 1.905 1.23 3.225 0 4.605-2.805 5.625-5.475 5.925.435.375.81 1.095.81 2.22 0 1.605-.015 2.895-.015 3.3 0 .315.225.69.825.57A12.06 12.06 0 0024 12c0-6.63-5.37-12-12-12z"/></svg>
              GitHub
            </a>
            <p style={{ color: '#52525b', fontSize: '0.8rem', margin: 0 }}>© 2026 OminiConnect</p>
          </div>
        </div>
      </footer>
    </div>
  );
}
