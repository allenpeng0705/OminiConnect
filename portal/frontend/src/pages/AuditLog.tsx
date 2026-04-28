import { useEffect, useState } from 'react';
import { Link } from 'react-router-dom';

interface AuditLogEntry {
  id: string;
  agent_id: string;
  tool_slug: string;
  platform: string;
  arguments: Record<string, unknown>;
  result: string;
  status: string;
  duration_ms: number;
  created_at: string;
}

interface AuditLogResponse {
  logs: AuditLogEntry[];
}

export default function AuditLog() {
  const [logs, setLogs] = useState<AuditLogEntry[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState('');
  const [platformFilter, setPlatformFilter] = useState('');
  const [agentFilter, setAgentFilter] = useState('');

  useEffect(() => {
    loadLogs();
  }, [platformFilter, agentFilter]);

  async function loadLogs() {
    setLoading(true);
    setError('');
    try {
      const params = new URLSearchParams();
      if (platformFilter) params.set('platform', platformFilter);
      if (agentFilter) params.set('agent_id', agentFilter);
      params.set('limit', '100');

      const res = await fetch(`/api/audit/logs?${params}`, {
        credentials: 'include',
      });
      if (!res.ok) {
        throw new Error('Failed to load audit logs');
      }
      const data: AuditLogResponse = await res.json();
      setLogs(data.logs);
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Failed to load');
    } finally {
      setLoading(false);
    }
  }

  function formatDate(isoString: string): string {
    try {
      const date = new Date(isoString);
      return date.toLocaleString();
    } catch {
      return isoString;
    }
  }

  function truncate(str: string, maxLen: number): string {
    if (str.length <= maxLen) return str;
    return str.substring(0, maxLen) + '...';
  }

  function getStatusBadge(status: string): { bg: string; text: string } {
    switch (status) {
      case 'success':
        return { bg: '#dcfce7', text: '#166534' };
      case 'error':
        return { bg: '#fee2e2', text: '#991b1b' };
      case 'pending':
        return { bg: '#fef3c7', text: '#92400e' };
      default:
        return { bg: '#f1f5f9', text: '#475569' };
    }
  }

  return (
    <div style={{ padding: '24px', maxWidth: '1200px', margin: '0 auto' }}>
      {/* Header */}
      <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '24px' }}>
        <div>
          <h1 style={{ fontSize: '24px', fontWeight: '600', margin: 0 }}>Audit Logs</h1>
          <p style={{ color: '#64748b', margin: '4px 0 0 0', fontSize: '14px' }}>
            Tool execution history for compliance and debugging
          </p>
        </div>
        <Link
          to="/"
          style={{
            padding: '8px 16px',
            background: '#f1f5f9',
            borderRadius: '6px',
            color: '#475569',
            textDecoration: 'none',
            fontSize: '14px',
          }}
        >
          ← Back to Dashboard
        </Link>
      </div>

      {/* Filters */}
      <div style={{ display: 'flex', gap: '16px', marginBottom: '24px', flexWrap: 'wrap' }}>
        <div>
          <label style={{ display: 'block', fontSize: '12px', color: '#64748b', marginBottom: '4px' }}>
            Platform Filter
          </label>
          <input
            type="text"
            placeholder="e.g., github, slack"
            value={platformFilter}
            onChange={(e) => setPlatformFilter(e.target.value)}
            style={{
              padding: '8px 12px',
              border: '1px solid #e2e8f0',
              borderRadius: '6px',
              fontSize: '14px',
              width: '200px',
            }}
          />
        </div>
        <div>
          <label style={{ display: 'block', fontSize: '12px', color: '#64748b', marginBottom: '4px' }}>
            Agent ID Filter
          </label>
          <input
            type="text"
            placeholder="e.g., agent-123"
            value={agentFilter}
            onChange={(e) => setAgentFilter(e.target.value)}
            style={{
              padding: '8px 12px',
              border: '1px solid #e2e8f0',
              borderRadius: '6px',
              fontSize: '14px',
              width: '200px',
            }}
          />
        </div>
        <div style={{ alignSelf: 'flex-end' }}>
          <button
            onClick={loadLogs}
            style={{
              padding: '8px 16px',
              background: '#3b82f6',
              color: 'white',
              border: 'none',
              borderRadius: '6px',
              fontSize: '14px',
              cursor: 'pointer',
            }}
          >
            Refresh
          </button>
        </div>
      </div>

      {/* Error */}
      {error && (
        <div style={{ padding: '12px', background: '#fee2e2', color: '#991b1b', borderRadius: '6px', marginBottom: '16px' }}>
          {error}
        </div>
      )}

      {/* Loading */}
      {loading && (
        <div style={{ textAlign: 'center', padding: '40px', color: '#64748b' }}>Loading...</div>
      )}

      {/* Table */}
      {!loading && logs.length === 0 && (
        <div style={{ textAlign: 'center', padding: '40px', color: '#64748b', background: '#f8fafc', borderRadius: '8px' }}>
          No audit logs found
        </div>
      )}

      {!loading && logs.length > 0 && (
        <div style={{ background: 'white', borderRadius: '8px', border: '1px solid #e2e8f0', overflow: 'hidden' }}>
          <table style={{ width: '100%', borderCollapse: 'collapse', fontSize: '14px' }}>
            <thead>
              <tr style={{ background: '#f8fafc', borderBottom: '1px solid #e2e8f0' }}>
                <th style={{ padding: '12px 16px', textAlign: 'left', fontWeight: '500', color: '#64748b' }}>Time</th>
                <th style={{ padding: '12px 16px', textAlign: 'left', fontWeight: '500', color: '#64748b' }}>Agent</th>
                <th style={{ padding: '12px 16px', textAlign: 'left', fontWeight: '500', color: '#64748b' }}>Tool</th>
                <th style={{ padding: '12px 16px', textAlign: 'left', fontWeight: '500', color: '#64748b' }}>Platform</th>
                <th style={{ padding: '12px 16px', textAlign: 'left', fontWeight: '500', color: '#64748b' }}>Status</th>
                <th style={{ padding: '12px 16px', textAlign: 'left', fontWeight: '500', color: '#64748b' }}>Duration</th>
                <th style={{ padding: '12px 16px', textAlign: 'left', fontWeight: '500', color: '#64748b' }}>Arguments</th>
              </tr>
            </thead>
            <tbody>
              {logs.map((log) => {
                const badge = getStatusBadge(log.status);
                return (
                  <tr key={log.id} style={{ borderBottom: '1px solid #f1f5f9' }}>
                    <td style={{ padding: '12px 16px', color: '#64748b', fontSize: '13px' }}>
                      {formatDate(log.created_at)}
                    </td>
                    <td style={{ padding: '12px 16px' }}>
                      <code style={{ background: '#f1f5f9', padding: '2px 6px', borderRadius: '4px', fontSize: '12px' }}>
                        {log.agent_id || '(none)'}
                      </code>
                    </td>
                    <td style={{ padding: '12px 16px', fontWeight: '500' }}>{log.tool_slug}</td>
                    <td style={{ padding: '12px 16px' }}>
                      <span style={{
                        background: '#f1f5f9',
                        padding: '2px 8px',
                        borderRadius: '4px',
                        fontSize: '12px',
                      }}>
                        {log.platform}
                      </span>
                    </td>
                    <td style={{ padding: '12px 16px' }}>
                      <span style={{
                        background: badge.bg,
                        color: badge.text,
                        padding: '2px 8px',
                        borderRadius: '4px',
                        fontSize: '12px',
                        fontWeight: '500',
                      }}>
                        {log.status}
                      </span>
                    </td>
                    <td style={{ padding: '12px 16px', color: '#64748b' }}>{log.duration_ms}ms</td>
                    <td style={{ padding: '12px 16px', fontSize: '12px', color: '#64748b', maxWidth: '200px' }}>
                      {truncate(JSON.stringify(log.arguments), 50)}
                    </td>
                  </tr>
                );
              })}
            </tbody>
          </table>
        </div>
      )}
    </div>
  );
}