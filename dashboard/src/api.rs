//! Dashboard API server

use crate::model::*;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::Utc;

/// Application state
#[derive(Debug, Clone)]
pub struct AppState {
    /// Recent audit events (in-memory for demo)
    events: Arc<RwLock<VecDeque<AuditEvent>>>,
    /// Dashboard statistics
    stats: Arc<RwLock<DashboardStats>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            events: Arc::new(RwLock::new(VecDeque::with_capacity(1000))),
            stats: Arc::new(RwLock::new(DashboardStats {
                total_events: 0,
                policy_violations: 0,
                pii_scrubbed: 0,
                events_today: 0,
                events_by_severity: SeverityCounts {
                    info: 0,
                    warning: 0,
                    error: 0,
                    critical: 0,
                },
                events_by_platform: PlatformCounts {
                    feishu: 0,
                    dingtalk: 0,
                    wechatwork: 0,
                },
            })),
        }
    }
}

impl AppState {
    /// Add a new audit event
    pub async fn add_event(&self, event: AuditEvent) {
        let mut events = self.events.write().await;
        events.push_front(event.clone());

        // Keep only last 1000 events
        if events.len() > 1000 {
            events.pop_back();
        }

        // Update stats
        let mut stats = self.stats.write().await;
        stats.total_events += 1;
        stats.events_today += 1;

        match event.severity.as_str() {
            "info" => stats.events_by_severity.info += 1,
            "warning" => stats.events_by_severity.warning += 1,
            "error" => stats.events_by_severity.error += 1,
            "critical" => stats.events_by_severity.critical += 1,
            _ => {}
        }

        match event.category.as_str() {
            "policy_violation" | "data_residency_violation" => stats.policy_violations += 1,
            "pii_scrubbed" => stats.pii_scrubbed += 1,
            _ => {}
        }

        match event.platform.as_str() {
            "feishu" => stats.events_by_platform.feishu += 1,
            "dingtalk" | "ding_talk" => stats.events_by_platform.dingtalk += 1,
            "wechatwork" | "we_chat_work" => stats.events_by_platform.wechatwork += 1,
            _ => {}
        }
    }
}

/// API response wrapper
#[derive(Serialize)]
struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    error: Option<String>,
}

impl<T> ApiResponse<T> {
    fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    fn error(msg: &str) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(msg.to_string()),
        }
    }
}

/// Query parameters for listing events
#[derive(Debug, Deserialize)]
pub struct ListEventsQuery {
    limit: Option<usize>,
    offset: Option<usize>,
    severity: Option<String>,
    platform: Option<String>,
}

/// GET /api/stats - Get dashboard statistics
async fn get_stats(State(state): State<AppState>) -> Json<ApiResponse<DashboardStats>> {
    let stats = state.stats.read().await.clone();
    Json(ApiResponse::success(stats))
}

/// GET /api/events - List recent events
async fn list_events(
    State(state): State<AppState>,
    Query(query): Query<ListEventsQuery>,
) -> Json<ApiResponse<Vec<AuditEvent>>> {
    let limit = query.limit.unwrap_or(50).min(100);
    let offset = query.offset.unwrap_or(0);

    let events = state.events.read().await;
    let filtered: Vec<AuditEvent> = events
        .iter()
        .skip(offset)
        .take(limit)
        .filter(|e| {
            if let Some(ref sev) = query.severity {
                if &e.severity != sev {
                    return false;
                }
            }
            if let Some(ref plat) = query.platform {
                if &e.platform != plat {
                    return false;
                }
            }
            true
        })
        .cloned()
        .collect();

    Json(ApiResponse::success(filtered))
}

/// GET /api/violations - List recent policy violations
async fn get_violations(
    State(state): State<AppState>,
    Query(query): Query<ListEventsQuery>,
) -> Json<ApiResponse<Vec<AuditEvent>>> {
    let limit = query.limit.unwrap_or(50).min(100);

    let events = state.events.read().await;
    let violations: Vec<AuditEvent> = events
        .iter()
        .filter(|e| {
            e.category.contains("violation")
        })
        .take(limit)
        .cloned()
        .collect();

    Json(ApiResponse::success(violations))
}

/// POST /api/events - Add a new event (for testing/integration)
async fn add_event(
    State(state): State<AppState>,
    Json(event): Json<AuditEvent>,
) -> Json<ApiResponse<()>> {
    state.add_event(event).await;
    Json(ApiResponse::success(()))
}

/// GET / - Serve dashboard HTML
async fn dashboard() -> Html<&'static str> {
    Html(DASHBOARD_HTML)
}

/// Create the dashboard router
pub fn create_router() -> Router {
    let state = AppState::default();

    Router::new()
        .route("/", get(dashboard))
        .route("/api/stats", get(get_stats))
        .route("/api/events", get(list_events))
        .route("/api/violations", get(get_violations))
        .route("/api/events", post(add_event))
        .with_state(state)
}

/// Start the dashboard server
pub async fn start(addr: &str) -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = addr.parse()?;
    let listener = tokio::net::TcpListener::bind(addr).await?;

    println!("Dashboard running at http://{}", addr);
    println!("Open http://{}/ in your browser", addr);

    axum::serve(listener, create_router()).await?;
    Ok(())
}

/// Simple dashboard HTML
static DASHBOARD_HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>OminiConnect Dashboard</title>
    <style>
        * { box-sizing: border-box; margin: 0; padding: 0; }
        body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; background: #f5f5f5; color: #333; }
        .header { background: #2c3e50; color: white; padding: 1rem 2rem; }
        .header h1 { font-size: 1.5rem; }
        .container { max-width: 1200px; margin: 2rem auto; padding: 0 1rem; }
        .stats-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 1rem; margin-bottom: 2rem; }
        .stat-card { background: white; padding: 1.5rem; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1); }
        .stat-card h3 { font-size: 0.875rem; color: #666; margin-bottom: 0.5rem; }
        .stat-card .value { font-size: 2rem; font-weight: bold; color: #2c3e50; }
        .stat-card.warning .value { color: #f39c12; }
        .stat-card.danger .value { color: #e74c3c; }
        .panel { background: white; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1); margin-bottom: 1rem; }
        .panel-header { padding: 1rem 1.5rem; border-bottom: 1px solid #eee; font-weight: 600; }
        .panel-body { padding: 1rem 1.5rem; }
        table { width: 100%; border-collapse: collapse; }
        th, td { text-align: left; padding: 0.75rem; border-bottom: 1px solid #eee; }
        th { font-weight: 600; color: #666; font-size: 0.875rem; }
        .badge { display: inline-block; padding: 0.25rem 0.5rem; border-radius: 4px; font-size: 0.75rem; font-weight: 500; }
        .badge-info { background: #e3f2fd; color: #1976d2; }
        .badge-warning { background: #fff3e0; color: #f57c00; }
        .badge-error { background: #ffebee; color: #c62828; }
        .badge-critical { background: #f8d7da; color: #721c24; }
        .refresh-btn { background: #3498db; color: white; border: none; padding: 0.5rem 1rem; border-radius: 4px; cursor: pointer; }
        .refresh-btn:hover { background: #2980b9; }
    </style>
</head>
<body>
    <div class="header">
        <h1>OminiConnect Compliance Dashboard</h1>
    </div>
    <div class="container">
        <div class="stats-grid">
            <div class="stat-card">
                <h3>Total Events</h3>
                <div class="value" id="total-events">-</div>
            </div>
            <div class="stat-card warning">
                <h3>Policy Violations</h3>
                <div class="value" id="policy-violations">-</div>
            </div>
            <div class="stat-card danger">
                <h3>PII Scrubbed</h3>
                <div class="value" id="pii-scrubbed">-</div>
            </div>
            <div class="stat-card">
                <h3>Events Today</h3>
                <div class="value" id="events-today">-</div>
            </div>
        </div>

        <div class="panel">
            <div class="panel-header">
                Recent Events
                <button class="refresh-btn" style="float: right;" onclick="loadEvents()">Refresh</button>
            </div>
            <div class="panel-body">
                <table>
                    <thead>
                        <tr>
                            <th>Time</th>
                            <th>Severity</th>
                            <th>Category</th>
                            <th>Platform</th>
                            <th>Action</th>
                            <th>Preview</th>
                        </tr>
                    </thead>
                    <tbody id="events-table">
                        <tr><td colspan="6" style="text-align: center; color: #999;">Loading...</td></tr>
                    </tbody>
                </table>
            </div>
        </div>

        <div class="panel">
            <div class="panel-header">Policy Violations</div>
            <div class="panel-body">
                <table>
                    <thead>
                        <tr>
                            <th>Time</th>
                            <th>Platform</th>
                            <th>User</th>
                            <th>Policy</th>
                            <th>Action</th>
                        </tr>
                    </thead>
                    <tbody id="violations-table">
                        <tr><td colspan="5" style="text-align: center; color: #999;">Loading...</td></tr>
                    </tbody>
                </table>
            </div>
        </div>
    </div>

    <script>
        async function loadStats() {
            try {
                const res = await fetch('/api/stats');
                const data = await res.json();
                if (data.success) {
                    document.getElementById('total-events').textContent = data.data.total_events;
                    document.getElementById('policy-violations').textContent = data.data.policy_violations;
                    document.getElementById('pii-scrubbed').textContent = data.data.pii_scrubbed;
                    document.getElementById('events-today').textContent = data.data.events_today;
                }
            } catch (e) {
                console.error('Failed to load stats:', e);
            }
        }

        async function loadEvents() {
            try {
                const res = await fetch('/api/events?limit=20');
                const data = await res.json();
                if (data.success) {
                    const tbody = document.getElementById('events-table');
                    if (data.data.length === 0) {
                        tbody.innerHTML = '<tr><td colspan="6" style="text-align: center; color: #999;">No events</td></tr>';
                    } else {
                        tbody.innerHTML = data.data.map(e => `
                            <tr>
                                <td>${new Date(e.timestamp).toLocaleString()}</td>
                                <td><span class="badge badge-${e.severity}">${e.severity}</span></td>
                                <td>${e.category}</td>
                                <td>${e.platform}</td>
                                <td>${e.action}</td>
                                <td>${e.content_preview || '-'}</td>
                            </tr>
                        `).join('');
                    }
                }
            } catch (e) {
                console.error('Failed to load events:', e);
            }
        }

        async function loadViolations() {
            try {
                const res = await fetch('/api/violations?limit=10');
                const data = await res.json();
                if (data.success) {
                    const tbody = document.getElementById('violations-table');
                    if (data.data.length === 0) {
                        tbody.innerHTML = '<tr><td colspan="5" style="text-align: center; color: #999;">No violations</td></tr>';
                    } else {
                        tbody.innerHTML = data.data.map(e => `
                            <tr>
                                <td>${new Date(e.timestamp).toLocaleString()}</td>
                                <td>${e.platform}</td>
                                <td>${e.user_id || '-'}</td>
                                <td>${e.policy_type || e.category}</td>
                                <td><span class="badge badge-error">${e.action}</span></td>
                            </tr>
                        `).join('');
                    }
                }
            } catch (e) {
                console.error('Failed to load violations:', e);
            }
        }

        // Load initial data
        loadStats();
        loadEvents();
        loadViolations();

        // Refresh every 30 seconds
        setInterval(() => { loadStats(); loadEvents(); loadViolations(); }, 30000);
    </script>
</body>
</html>"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_app_state_add_event() {
        let state = AppState::default();

        let event = AuditEvent {
            id: "test-1".to_string(),
            timestamp: Utc::now(),
            category: "policy_violation".to_string(),
            severity: "error".to_string(),
            platform: "feishu".to_string(),
            user_id: Some("user123".to_string()),
            tenant_id: None,
            policy_type: Some("content_moderation".to_string()),
            action: "blocked".to_string(),
            content_preview: Some("test content".to_string()),
        };

        state.add_event(event).await;

        let stats = state.stats.read().await;
        assert_eq!(stats.total_events, 1);
        assert_eq!(stats.policy_violations, 1);
        assert_eq!(stats.events_by_severity.error, 1);
        assert_eq!(stats.events_by_platform.feishu, 1);
    }
}
