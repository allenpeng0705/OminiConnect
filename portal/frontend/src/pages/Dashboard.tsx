import { useEffect, useMemo, useRef, useState } from 'react';
import { Link, useNavigate } from 'react-router-dom';
import CatalogProviderCard from '../components/CatalogProviderCard';
import CatalogProviderListItem from '../components/CatalogProviderListItem';
import CategoryFilterBar from '../components/CategoryFilterBar';
import ViewToggle from '../components/ViewToggle';
import IntegrationProviderLogo from '../components/IntegrationProviderLogo';
import { getConnectors, getConnectorStatus, getIntegrationCatalog, getMe, logout, createNangoConnectSession, type IntegrationCatalogRow } from '../api/client';
import { findConnectorPlatformForCatalogProvider } from '../lib/connectorCatalogEntry';
import { normalizeCatalogResponse, providerSearchBlob } from '../lib/integrationCatalogNormalize';
import { BUILTIN_OMINI_PLATFORMS, findBuiltinByConnectorId } from '../lib/builtinPlatforms';
import { filterCatalogExcludingBuiltinDupes } from '../lib/builtinGlobalCatalogExclusions';
import { resolveBuiltinCatalogLogo } from '../lib/resolveBuiltinCatalogLogo';

interface ConnectorInfo {
  platform: string;
  client_id: string;
  redirect_uri: string;
  scopes: string[];
  /** `nango` = managed hub (Nango Connect under the hood). */
  engine?: string;
  provider_key?: string;
  enabled: boolean;
  connected?: boolean;
  configured?: boolean;
}

interface PlatformConfig {
  id: string;
  name: string;
  color: string;
  type: 'oauth2' | 'api_key';
  logo_url?: string;
}

const PLATFORMS: PlatformConfig[] = BUILTIN_OMINI_PLATFORMS;
const PAGE = 20;

/** Most Popular providers — shown in a special category at the top of Global Services. */
const MOST_POPULAR_KEYS = new Set([
  'github', 'linkedin', 'facebook', 'x', 'slack', 'notion', 'jira', 'asana',
  'trello', 'linear', 'salesforce', 'hubspot', 'zendesk', 'intercom',
  'google_ads', 'mailchimp', 'klaviyo', 'dropbox', 'box', 'googledrive',
  'shopify', 'stripe', 'paypal', 'zoom', 'twilio', 'sendgrid',
  'segment', 'mixpanel', 'amplitude', 'vercel', 'datadog', 'newrelic', 'pagerduty',
]);

export default function Dashboard() {
  const navigate = useNavigate();
  const [connectors, setConnectors] = useState<ConnectorInfo[]>([]);
  const [loading, setLoading] = useState(true);
  const [username, setUsername] = useState('');
  const [catalogRows, setCatalogRows] = useState<IntegrationCatalogRow[]>([]);
  const [catalogLoading, setCatalogLoading] = useState(true);
  const [catalogError, setCatalogError] = useState('');
  const [catalogFilter, setCatalogFilter] = useState('');
  const [viewMode, setViewMode] = useState<'grid' | 'list'>('grid');
  const [selectedCategory, setSelectedCategory] = useState<string | null>(null);
  const [homeTab, setHomeTab] = useState<'global' | 'chinese'>('global');
  /** Per-category visible item count, keyed by category name. Defaults to PAGE. */
  const [visiblePerCategory, setVisiblePerCategory] = useState<Record<string, number>>({});
  /** Sentinel div refs, one per category, used by IntersectionObserver for infinite scroll. */
  const sentinelByCategory = useRef<Record<string, HTMLDivElement | null>>({});

  useEffect(() => {
    loadData();
    loadMe();
  }, []);

  useEffect(() => {
    let cancelled = false;
    void (async () => {
      setCatalogLoading(true);
      setCatalogError('');
      try {
        const data = await getIntegrationCatalog();
        if (!cancelled) setCatalogRows(normalizeCatalogResponse(data));
      } catch (e) {
        if (!cancelled) {
          setCatalogError(e instanceof Error ? e.message : 'Failed to load catalog');
          setCatalogRows([]);
        }
      } finally {
        if (!cancelled) setCatalogLoading(false);
      }
    })();
    return () => {
      cancelled = true;
    };
  }, []);

  /** Hide Nango rows that duplicate Omini-only home cards (Maton, QQ Mail). Other providers use Global Services. */
  const connectedProviderKeys = useMemo(
    () => new Set(connectors.filter((c) => c.configured || c.connected).map((c) => c.provider_key?.split('__')[0].trim().toLowerCase()).filter(Boolean)),
    [connectors]
  );
  const globalCatalogRows = useMemo(
    () => filterCatalogExcludingBuiltinDupes(catalogRows).filter((r) => !connectedProviderKeys.has(r.name.toLowerCase())),
    [catalogRows, connectedProviderKeys]
  );

  const catalogFiltered = useMemo(() => {
    const q = catalogFilter.trim().toLowerCase();
    if (!q) return globalCatalogRows;
    return globalCatalogRows.filter((p) => providerSearchBlob(p).includes(q));
  }, [globalCatalogRows, catalogFilter]);

  // All unique categories from the full (unfiltered by category) dataset, sorted
  const allCategories = useMemo(() => {
    const set = new Set<string>();
    for (const row of globalCatalogRows) {
      for (const cat of row.categories || []) set.add(cat);
    }
    return Array.from(set).sort();
  }, [globalCatalogRows]);

  // Category-filtered list (single category selected)
  const categoryFiltered = useMemo(() => {
    if (!selectedCategory) return catalogFiltered;
    return catalogFiltered.filter((p) => (p.categories || []).includes(selectedCategory));
  }, [catalogFiltered, selectedCategory]);

  // Category counts based on the search-filtered dataset
  const categoryCountMap = useMemo(() => {
    const counts: Record<string, number> = {};
    for (const row of catalogFiltered) {
      for (const cat of row.categories || []) {
        counts[cat] = (counts[cat] ?? 0) + 1;
      }
    }
    return counts;
  }, [catalogFiltered]);

  // Grouped structure for "All" (no single category selected)
  const groupedByCategory = useMemo(() => {
    if (selectedCategory) return [];
    const mostPopular: IntegrationCatalogRow[] = [];
    const groups = new Map<string, IntegrationCatalogRow[]>();
    for (const row of categoryFiltered) {
      // Assign Most Popular providers to the special group
      const key = row.name.toLowerCase();
      if (MOST_POPULAR_KEYS.has(key)) {
        mostPopular.push(row);
      } else {
        // Assign to first category only to avoid duplicates
        const cat = row.categories?.[0] ?? 'Other';
        if (!groups.has(cat)) groups.set(cat, []);
        groups.get(cat)!.push(row);
      }
    }
    const result: { category: string; providers: IntegrationCatalogRow[] }[] = [];
    if (mostPopular.length > 0) {
      result.push({ category: 'Most Popular', providers: mostPopular });
    }
    // Sort remaining categories alphabetically
    const sortedGroups = Array.from(groups.entries())
      .map(([category, providers]) => ({ category, providers }))
      .sort((a, b) => a.category.localeCompare(b.category));
    result.push(...sortedGroups);
    return result;
  }, [categoryFiltered, selectedCategory]);

  /** Reset per-category pagination when filter or grouping changes. */
  useEffect(() => {
    setVisiblePerCategory({});
  }, [catalogFiltered, selectedCategory, viewMode]);

  // Build lookup: provider name (lowercase) -> logo_url
  const catalogLogoByProvider = useMemo(() => {
    const map = new Map<string, string>();
    for (const row of catalogRows) {
      if (row.logo_url) {
        map.set(row.name.toLowerCase(), row.logo_url);
      }
    }
    return map;
  }, [catalogRows]);

  function showMoreForCategory(cat: string) {
    setVisiblePerCategory((prev) => ({ ...prev, [cat]: (prev[cat] ?? PAGE) + PAGE }));
  }

  /** IntersectionObserver for infinite scroll — watches all category sentinel divs. */
  useEffect(() => {
    const observer = new IntersectionObserver(
      (entries) => {
        for (const entry of entries) {
          if (entry.isIntersecting) {
            const cat = (entry.target as HTMLDivElement).dataset.category!;
            const providers = groupedByCategory.find((g) => g.category === cat)?.providers ?? [];
            const visible = visiblePerCategory[cat] ?? PAGE;
            if (visible < providers.length) {
              showMoreForCategory(cat);
            }
          }
        }
      },
      { rootMargin: '300px' }
    );
    const sentinels = Object.values(sentinelByCategory.current).filter(Boolean) as HTMLDivElement[];
    sentinels.forEach((s) => observer.observe(s));
    return () => observer.disconnect();
  }, [groupedByCategory, visiblePerCategory]);

  function autoConnectorIdFromProvider(key: string): string {
    // Use the provider key as-is — each provider gets exactly one connector named after the provider.
    const base = key
      .trim()
      .toLowerCase()
      .replace(/[^a-z0-9_-]+/g, '-')
      .replace(/-+/g, '-')
      .replace(/^-+|-+$/g, '')
      .slice(0, 48) || 'connector';
    return base;
  }

  async function loadMe() {
    try {
      const me = await getMe();
      if (me?.username) setUsername(me.username);
    } catch (e) {
      console.error('[Dashboard] loadMe error:', e);
    }
  }

  async function loadData() {
    setLoading(true);
    try {
      const list = await getConnectors();
      const withStatus = await Promise.all(
        list.map(async (c) => {
          try {
            const status = await getConnectorStatus(c.platform);
            return {
              ...c,
              engine: c.engine,
              provider_key: c.provider_key,
              connected: status.connected,
              configured: status.configured,
            };
          } catch {
            return { ...c, connected: false, configured: false };
          }
        })
      );
      setConnectors(withStatus);
    } catch (err) {
      console.error(err);
    } finally {
      setLoading(false);
    }
  }

  async function handleLogout() {
    await logout();
    window.location.href = '/auth/login';
  }

  function handleConnect(connectorPlat: string, engine?: string) {
    const p = findBuiltinByConnectorId(connectorPlat) ?? PLATFORMS.find((x) => x.id === connectorPlat);
    if (p?.type === 'api_key') {
      return;
    }
    if (engine === 'nango') {
      // Open Nango Connect in external browser tab
      createNangoConnectSession(connectorPlat)
        .then(({ connect_url }) => window.open(connect_url, '_blank', 'noopener,noreferrer'))
        .catch(err => console.error('Failed to open connect session:', err));
      return;
    }
    window.location.href = `/oauth/${connectorPlat}`;
  }

  const configuredPlatforms = new Set(connectors.map(c => c.platform));

  return (
    <div style={{ minHeight: '100vh', background: '#f5f5f5' }}>
      {/* Header */}
      <header style={{ background: 'white', borderBottom: '1px solid #e0e0e0', padding: '0 1.5rem', display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
        <img src="/images/logos/ominiconnect_logo_with_text.svg" alt="OminiConnect" style={{ height: '80px', objectFit: 'contain' }} />
        <div style={{ display: 'flex', alignItems: 'center', gap: '1rem' }}>
          <span style={{ color: '#666', fontSize: '0.875rem' }}>{username}</span>
          <Link to="/audit" style={{ padding: '0.375rem 0.75rem', background: '#f5f5f5', border: '1px solid #ccc', borderRadius: '4px', cursor: 'pointer', fontSize: '0.875rem', textDecoration: 'none', color: '#333' }}>Audit Logs</Link>
          <Link to="/api-keys" style={{ padding: '0.375rem 0.75rem', background: '#f5f5f5', border: '1px solid #ccc', borderRadius: '4px', cursor: 'pointer', fontSize: '0.875rem', textDecoration: 'none', color: '#333' }}>API Keys</Link>
          <button onClick={handleLogout} style={{ padding: '0.375rem 0.75rem', background: '#f5f5f5', border: '1px solid #ccc', borderRadius: '4px', cursor: 'pointer', fontSize: '0.875rem' }}>Logout</button>
        </div>
      </header>

      <main style={{ padding: '2rem' }}>
        {/* Connected Services */}
        <section style={{ marginBottom: '2rem' }}>
          <h2 style={{ fontSize: '1.125rem', color: '#333', marginBottom: '1rem' }}>Connected Services</h2>
          {loading ? (
            <p style={{ color: '#666' }}>Loading...</p>
          ) : connectors.length === 0 ? (
            <div style={{ background: 'white', padding: '2rem', borderRadius: '8px', boxShadow: '0 1px 4px rgba(0,0,0,0.08)', textAlign: 'center', color: '#666' }}>
              No connectors configured yet. Connect one below.
            </div>
          ) : (
            <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fill, minmax(260px, 1fr))', gap: '0.75rem' }}>
              {connectors.map(c => {
                const platform =
                  findBuiltinByConnectorId(c.platform) ?? PLATFORMS.find((p) => p.id === c.platform);
                const displayName = platform?.name || c.platform;
                const logoUrl = (() => {
                  if (c.engine === 'nango' && c.provider_key) {
                    const providerKey = c.provider_key.split('__')[0].trim().toLowerCase();
                    const fromCat = catalogLogoByProvider.get(providerKey);
                    if (fromCat) return fromCat;
                  }
                  const b = findBuiltinByConnectorId(c.platform);
                  if (b) {
                    return resolveBuiltinCatalogLogo(b.id, catalogLogoByProvider, b.logo_url);
                  }
                  return platform?.logo_url;
                })();
                return (
                  <div
                    key={c.platform}
                    style={{
                      background: 'white',
                      borderRadius: '14px',
                      padding: '1.1rem',
                      border: '1px solid #e2e8f0',
                      boxShadow: '0 1px 3px rgba(15, 23, 42, 0.06)',
                      display: 'flex',
                      flexDirection: 'column',
                      gap: '0.75rem',
                    }}
                  >
                    {/* Header: Logo + Name */}
                    <div style={{ display: 'flex', gap: '0.85rem', alignItems: 'center' }}>
                      <IntegrationProviderLogo
                        url={logoUrl}
                        label={displayName}
                        size={52}
                      />
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
                          title={displayName}
                        >
                          {displayName}
                        </div>
                        <code
                          style={{
                            fontSize: '0.72rem',
                            color: '#64748b',
                            wordBreak: 'break-all',
                            fontFamily: 'ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace',
                          }}
                        >
                          {c.platform}
                        </code>
                      </div>
                      <div style={{ display: 'flex', gap: '0.375rem', flexShrink: 0 }}>
                        <span style={{
                          padding: '0.125rem 0.5rem',
                          borderRadius: '9999px',
                          fontSize: '0.72rem',
                          fontWeight: 500,
                          background: c.connected ? '#dcfce7' : c.configured ? '#fef3c7' : '#f3f4f6',
                          color: c.connected ? '#166534' : c.configured ? '#92400e' : '#6b7280',
                        }}>
                          {c.connected ? 'Connected' : c.configured ? 'Configured' : 'Not configured'}
                        </span>
                      </div>
                    </div>
                    {/* Scopes */}
                    {c.scopes && c.scopes.length > 0 && (
                      <div style={{ display: 'flex', flexWrap: 'wrap', gap: '0.25rem' }}>
                        {c.scopes.slice(0, 5).map((scope, i) => (
                          <span
                            key={i}
                            style={{
                              fontSize: '0.65rem',
                              padding: '0.1rem 0.4rem',
                              borderRadius: '4px',
                              background: '#f1f5f9',
                              color: '#475569',
                              border: '1px solid #e2e8f0',
                              fontFamily: 'ui-monospace, monospace',
                            }}
                          >
                            {scope}
                          </span>
                        ))}
                        {c.scopes.length > 5 && (
                          <span style={{ fontSize: '0.65rem', color: '#64748b', padding: '0.1rem 0.25rem' }}>
                            +{c.scopes.length - 5} more
                          </span>
                        )}
                      </div>
                    )}
                    {/* Footer: Actions */}
                    <div style={{ marginTop: 'auto', display: 'flex', gap: '0.5rem', alignItems: 'center' }}>
                      <Link
                        to={`/connectors/${c.platform}`}
                        style={{
                          padding: '0.45rem 0.9rem',
                          borderRadius: '8px',
                          border: 'none',
                          background: '#4f46e5',
                          color: 'white',
                          fontSize: '0.82rem',
                          fontWeight: 600,
                          textDecoration: 'none',
                          cursor: 'pointer',
                          boxShadow: '0 1px 2px rgba(79, 70, 229, 0.2)',
                        }}
                      >
                        Configure
                      </Link>
                      {(c.engine === 'nango' || platform?.type === 'oauth2') && c.configured && !c.connected && (
                        <button
                          onClick={() => handleConnect(c.platform, c.engine)}
                          style={{
                            padding: '0.45rem 0.9rem',
                            borderRadius: '8px',
                            border: '1px solid #cbd5e1',
                            background: 'white',
                            color: '#334155',
                            fontSize: '0.82rem',
                            fontWeight: 600,
                            cursor: 'pointer',
                          }}
                        >
                          Connect
                        </button>
                      )}
                    </div>
                  </div>
                );
              })}
            </div>
          )}
        </section>

        {/* Integration Library + Chinese Services */}
        <section style={{ marginBottom: '1.75rem' }}>
          {/* Tab bar */}
          <div
            style={{
              display: 'flex',
              gap: '0',
              marginBottom: '1rem',
              borderBottom: '2px solid #e5e7eb',
            }}
          >
            <button
              type="button"
              onClick={() => setHomeTab('global')}
              style={{
                padding: '0.5rem 1.25rem',
                border: 'none',
                borderBottom: homeTab === 'global' ? '2px solid #4f46e5' : '2px solid transparent',
                background: 'transparent',
                color: homeTab === 'global' ? '#4f46e5' : '#64748b',
                fontSize: '0.9rem',
                fontWeight: 600,
                cursor: 'pointer',
                transition: 'color 0.15s, border-color 0.15s',
                marginBottom: '-2px',
              }}
            >
              Global Services
            </button>
            <button
              type="button"
              onClick={() => setHomeTab('chinese')}
              style={{
                padding: '0.5rem 1.25rem',
                border: 'none',
                borderBottom: homeTab === 'chinese' ? '2px solid #4f46e5' : '2px solid transparent',
                background: 'transparent',
                color: homeTab === 'chinese' ? '#4f46e5' : '#64748b',
                fontSize: '0.9rem',
                fontWeight: 600,
                cursor: 'pointer',
                transition: 'color 0.15s, border-color 0.15s',
                marginBottom: '-2px',
              }}
            >
              Chinese Services
            </button>
          </div>

          {homeTab === 'global' ? (
            /* Global Services content */
            <>
              <p style={{ margin: '0 0 1rem', fontSize: '0.875rem', color: '#666', lineHeight: 1.45 }}>
                Loaded from your managed hub. Integrations that are also on the home page as Omini-only cards (see Chinese Services) are hidden here to avoid duplicating the same service.
              </p>
              <div style={{ marginBottom: '1rem', display: 'flex', flexWrap: 'wrap', gap: '0.5rem', alignItems: 'center' }}>
                <input
                  type="search"
                  placeholder="Filter by name, category, auth…"
                  value={catalogFilter}
                  onChange={(e) => setCatalogFilter(e.target.value)}
                  disabled={catalogLoading || !!catalogError || globalCatalogRows.length === 0}
                  style={{
                    flex: '1 1 200px',
                    maxWidth: '320px',
                    padding: '0.45rem 0.75rem',
                    borderRadius: '8px',
                    border: '1px solid #ccc',
                    fontSize: '0.875rem',
                    boxSizing: 'border-box',
                  }}
                />
                {!catalogLoading && !catalogError && globalCatalogRows.length > 0 && (
                  <>
                    <CategoryFilterBar
                      categories={allCategories}
                      selected={selectedCategory}
                      counts={categoryCountMap}
                      onSelect={setSelectedCategory}
                    />
                    <ViewToggle value={viewMode} onChange={setViewMode} />
                  </>
                )}
              </div>
              {catalogLoading && <p style={{ color: '#666', fontSize: '0.875rem' }}>Loading integration catalog…</p>}
              {catalogError && (
                <div
                  style={{
                    padding: '0.85rem 1rem',
                    borderRadius: '8px',
                    background: '#fef2f2',
                    color: '#b91c1c',
                    fontSize: '0.875rem',
                    marginBottom: '0.75rem',
                  }}
                >
                  {catalogError}
                  <div style={{ marginTop: '0.5rem', color: '#64748b', fontSize: '0.8rem' }}>
                    Fix <code style={{ background: '#fee2e2', padding: '0.1rem 0.3rem', borderRadius: '4px' }}>NANGO_BASE_URL</code> in repo-root{' '}
                    <code style={{ background: '#fee2e2', padding: '0.1rem 0.3rem', borderRadius: '4px' }}>.env</code> and ensure Nango is running, then refresh this page.
                  </div>
                </div>
              )}
              {!catalogLoading && !catalogError && catalogRows.length === 0 && (
                <p style={{ color: '#666', fontSize: '0.875rem' }}>No providers returned. Check hub configuration or open the full library to retry.</p>
              )}
              {!catalogLoading && !catalogError && catalogRows.length > 0 && globalCatalogRows.length === 0 && (
                <p style={{ color: '#666', fontSize: '0.875rem' }}>
                  The hub only returned providers that are Omini-only home cards (see <strong>Chinese Services</strong>). Add those there, or add more providers to your Nango environment.
                </p>
              )}
              {!catalogLoading && !catalogError && globalCatalogRows.length > 0 && categoryFiltered.length > 0 && (
                <>
                  {selectedCategory === null ? (
                    <>
                      {groupedByCategory.map(({ category, providers }) => {
                        const visible = visiblePerCategory[category] ?? PAGE;
                        const visibleProviders = providers.slice(0, visible);
                        return (
                          <div key={category} style={{ marginBottom: '1.75rem' }}>
                            <h3
                              style={{
                                fontSize: '0.875rem',
                                fontWeight: 600,
                                color: '#374151',
                                marginBottom: '0.65rem',
                                paddingBottom: '0.35rem',
                                borderBottom: '2px solid #e5e7eb',
                                display: 'flex',
                                alignItems: 'center',
                                gap: '0.5rem',
                              }}
                            >
                              {category.charAt(0).toUpperCase() + category.slice(1)}
                              <span style={{ fontSize: '0.72rem', fontWeight: 400, color: '#9ca3af', background: '#f1f5f9', padding: '0.05rem 0.45rem', borderRadius: '9999px' }}>
                                {providers.length}
                              </span>
                            </h3>
                            {viewMode === 'list' ? (
                              <div
                                style={{
                                  background: 'white',
                                  borderRadius: '10px',
                                  border: '1px solid #e2e8f0',
                                  maxHeight: '480px',
                                  overflowY: 'auto',
                                  scrollbarWidth: 'thin',
                                  scrollbarColor: '#cbd5e1 transparent',
                                }}
                              >
                                {visibleProviders.map((p) => (
                                  <CatalogProviderListItem
                                    key={`${category}-${p.name}`}
                                    row={p}
                                    existingPlatform={findConnectorPlatformForCatalogProvider(connectors, p.name)}
                                    onAddConnector={(providerKey) =>
                                      navigate(`/connectors/${encodeURIComponent(autoConnectorIdFromProvider(providerKey))}?provider_key=${encodeURIComponent(providerKey)}&engine=nango&new=1`)
                                    }
                                    onOpenConnector={(platform) =>
                                      navigate(`/connectors/${encodeURIComponent(platform)}`)
                                    }
                                  />
                                ))}
                                <div
                                  ref={(el) => { sentinelByCategory.current[category] = el; }}
                                  data-category={category}
                                  style={{ height: '1px', flexShrink: 0 }}
                                />
                              </div>
                            ) : (
                              <div
                                style={{
                                  maxHeight: '520px',
                                  overflowY: 'auto',
                                  scrollbarWidth: 'thin',
                                  scrollbarColor: '#cbd5e1 transparent',
                                }}
                              >
                                <div
                                  style={{
                                    display: 'grid',
                                    gridTemplateColumns: 'repeat(auto-fill, minmax(260px, 1fr))',
                                    gap: '0.75rem',
                                    alignItems: 'stretch',
                                  }}
                                >
                                  {visibleProviders.map((p) => (
                                    <div key={`${category}-${p.name}`} style={{ display: 'flex', flexDirection: 'column', flex: 1 }}>
                                      <CatalogProviderCard
                                        row={p}
                                        existingPlatform={findConnectorPlatformForCatalogProvider(connectors, p.name)}
                                        onAddConnector={(providerKey) =>
                                          navigate(`/connectors/${encodeURIComponent(autoConnectorIdFromProvider(providerKey))}?provider_key=${encodeURIComponent(providerKey)}&engine=nango&new=1`)
                                        }
                                        onOpenConnector={(platform) =>
                                          navigate(`/connectors/${encodeURIComponent(platform)}`)
                                        }
                                      />
                                    </div>
                                  ))}
                                  <div
                                    ref={(el) => { sentinelByCategory.current[category] = el; }}
                                    data-category={category}
                                    style={{ height: '1px', flexShrink: 0 }}
                                  />
                                </div>
                              </div>
                            )}
                          </div>
                        );
                      })}
                    </>
                  ) : (
                    <>
                      {viewMode === 'list' ? (
                        <div
                          style={{
                            background: 'white',
                            borderRadius: '10px',
                            border: '1px solid #e2e8f0',
                            overflow: 'hidden',
                          }}
                        >
                          {categoryFiltered.map((p) => (
                            <CatalogProviderListItem
                              key={p.name}
                              row={p}
                              existingPlatform={findConnectorPlatformForCatalogProvider(connectors, p.name)}
                              onAddConnector={(providerKey) =>
                                navigate(`/connectors/${encodeURIComponent(autoConnectorIdFromProvider(providerKey))}?provider_key=${encodeURIComponent(providerKey)}&engine=nango&new=1`)
                              }
                              onOpenConnector={(platform) =>
                                navigate(`/connectors/${encodeURIComponent(platform)}`)
                              }
                            />
                          ))}
                        </div>
                      ) : (
                        <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fill, minmax(260px, 1fr))', gap: '0.75rem' }}>
                          {categoryFiltered.map((p) => (
                            <CatalogProviderCard
                              key={p.name}
                              row={p}
                              existingPlatform={findConnectorPlatformForCatalogProvider(connectors, p.name)}
                              onAddConnector={(providerKey) =>
                                navigate(`/connectors/${encodeURIComponent(autoConnectorIdFromProvider(providerKey))}?provider_key=${encodeURIComponent(providerKey)}&engine=nango&new=1`)
                              }
                              onOpenConnector={(platform) =>
                                navigate(`/connectors/${encodeURIComponent(platform)}`)
                              }
                            />
                          ))}
                        </div>
                      )}
                    </>
                  )}
                </>
              )}
              {!catalogLoading && !catalogError && globalCatalogRows.length > 0 && categoryFiltered.length === 0 && (
                <p style={{ color: '#64748b', fontSize: '0.875rem', textAlign: 'center', padding: '1.5rem' }}>
                  No providers match your filter.
                </p>
              )}
            </>
          ) : (
            /* Chinese Services content */
            <>
              <p style={{ margin: '0 0 1rem', fontSize: '0.875rem', color: '#666', lineHeight: 1.45 }}>
                Omini-only integrations not in the managed hub. Do not add a card here for a service Nango already offers — use Global Services.
              </p>
              <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fill, minmax(260px, 1fr))', gap: '0.75rem' }}>
                {PLATFORMS.filter((p) => !configuredPlatforms.has(p.id)).map((p) => (
                  <div
                    key={p.id}
                    style={{
                      background: 'white',
                      borderRadius: '14px',
                      padding: '1.1rem',
                      border: '1px solid #e2e8f0',
                      boxShadow: '0 1px 3px rgba(15, 23, 42, 0.06)',
                      display: 'flex',
                      flexDirection: 'column',
                      gap: '0.75rem',
                    }}
                  >
                    <div style={{ display: 'flex', gap: '0.85rem', alignItems: 'center' }}>
                      <IntegrationProviderLogo
                        url={resolveBuiltinCatalogLogo(p.id, catalogLogoByProvider, p.logo_url || '')}
                        label={p.name}
                        size={52}
                        brandColor={p.color}
                      />
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
                          title={p.name}
                        >
                          {p.name}
                        </div>
                        <code
                          style={{
                            fontSize: '0.72rem',
                            color: '#64748b',
                            wordBreak: 'break-all',
                            fontFamily: 'ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace',
                          }}
                        >
                          {p.id}
                        </code>
                      </div>
                    </div>
                    <div style={{ marginTop: 'auto', display: 'flex', gap: '0.5rem', alignItems: 'center' }}>
                      <Link
                        to={`/connectors/${p.id}`}
                        style={{
                          padding: '0.45rem 0.9rem',
                          borderRadius: '8px',
                          border: 'none',
                          background: '#4f46e5',
                          color: 'white',
                          fontSize: '0.82rem',
                          fontWeight: 600,
                          textDecoration: 'none',
                          cursor: 'pointer',
                          boxShadow: '0 1px 2px rgba(79, 70, 229, 0.2)',
                        }}
                      >
                        Connect
                      </Link>
                    </div>
                  </div>
                ))}
              </div>
            </>
          )}
        </section>
      </main>
    </div>
  );
}
