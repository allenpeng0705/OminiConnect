//! Persistence for dynamic API-gateway ingress routes (control plane): JSON file, SQLite, PostgreSQL.
//!
//! **Postgres** here means any PostgreSQL-compatible server: self-hosted, **AWS RDS / Aurora PostgreSQL**,
//! **Azure Database for PostgreSQL**, **GCP Cloud SQL for PostgreSQL**, etc. Configure
//! [`panda_config::ControlPlaneStoreConfig::database_url`] with the provider’s connection string
//! (TLS via `sslmode` / URL options as supported by sqlx + rustls). **MySQL** and **SQL Server**
//! (e.g. Azure SQL Database) are different engines and are not implemented in this module yet.

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;
#[cfg(feature = "control-plane-sql")]
use std::time::{SystemTime, UNIX_EPOCH};

use async_trait::async_trait;
use futures_util::StreamExt;
use panda_config::{ApiGatewayIngressRoute, ControlPlaneConfig, ControlPlaneStoreKind};

#[cfg(feature = "control-plane-sql")]
use panda_config::{ApiGatewayIngressAuthMode, ApiGatewayIngressBackend, RouteRateLimitConfig};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use super::ingress::DynamicIngressRoutes;

#[async_trait]
pub trait ControlPlanePersist: Send + Sync {
    async fn upsert(&self, r: &ApiGatewayIngressRoute) -> Result<(), String>;
    async fn remove(&self, tenant_id: &str, path_prefix: &str) -> Result<bool, String>;
    async fn load_all(&self) -> Result<Vec<ApiGatewayIngressRoute>, String>;
    async fn replace_all(&self, routes: &[ApiGatewayIngressRoute]) -> Result<(), String>;
}

#[derive(Debug, Serialize, Deserialize)]
struct DynamicIngressExportV1 {
    version: u32,
    routes: Vec<ApiGatewayIngressRoute>,
}

pub fn export_routes_json(routes: &[ApiGatewayIngressRoute]) -> serde_json::Value {
    serde_json::to_value(DynamicIngressExportV1 {
        version: 1,
        routes: routes.to_vec(),
    })
    .unwrap_or_else(|_| serde_json::json!({ "version": 1, "routes": [] }))
}

pub fn parse_import_body(bytes: &[u8]) -> Result<Vec<ApiGatewayIngressRoute>, String> {
    let v: serde_json::Value =
        serde_json::from_slice(bytes).map_err(|e| format!("invalid JSON: {e}"))?;
    if let Some(arr) = v.as_array() {
        serde_json::from_value(serde_json::Value::Array(arr.clone()))
            .map_err(|e| format!("invalid routes array: {e}"))
    } else {
        let env: DynamicIngressExportV1 =
            serde_json::from_value(v).map_err(|e| format!("invalid import envelope: {e}"))?;
        if env.version != 1 {
            return Err(format!("unsupported export version {}", env.version));
        }
        Ok(env.routes)
    }
}

#[cfg(feature = "control-plane-sql")]
fn now_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}

#[cfg(feature = "control-plane-sql")]
fn backend_to_token(b: ApiGatewayIngressBackend) -> &'static str {
    use ApiGatewayIngressBackend as B;
    match b {
        B::Ai => "ai",
        B::Mcp => "mcp",
        B::Ops => "ops",
        B::Deny => "deny",
        B::Gone => "gone",
        B::NotFound => "not_found",
    }
}

#[cfg(feature = "control-plane-sql")]
fn auth_to_token(a: &ApiGatewayIngressAuthMode) -> &'static str {
    use ApiGatewayIngressAuthMode as M;
    match a {
        M::Inherit => "inherit",
        M::Required => "required",
        M::Optional => "optional",
    }
}

#[cfg(feature = "control-plane-sql")]
fn auth_from_token(s: &str) -> Result<ApiGatewayIngressAuthMode, String> {
    use ApiGatewayIngressAuthMode as M;
    match s.trim() {
        "inherit" => Ok(M::Inherit),
        "required" => Ok(M::Required),
        "optional" => Ok(M::Optional),
        _ => Err(format!("unknown ingress auth_mode in store: {s:?}")),
    }
}

#[cfg(feature = "control-plane-sql")]
fn backend_from_token(s: &str) -> Result<ApiGatewayIngressBackend, String> {
    use ApiGatewayIngressBackend as B;
    match s.trim() {
        "ai" => Ok(B::Ai),
        "mcp" => Ok(B::Mcp),
        "ops" => Ok(B::Ops),
        "deny" => Ok(B::Deny),
        "gone" => Ok(B::Gone),
        "not_found" => Ok(B::NotFound),
        _ => Err(format!("unknown ingress backend in store: {s:?}")),
    }
}

#[cfg(feature = "control-plane-sql")]
fn route_to_row(
    r: &ApiGatewayIngressRoute,
) -> Result<
    (
        String,
        String,
        String,
        String,
        Option<String>,
        Option<i64>,
        String,
        i64,
    ),
    String,
> {
    let tenant_id = r
        .tenant_id
        .as_deref()
        .map(str::trim)
        .unwrap_or("")
        .to_string();
    let path_prefix = r.path_prefix.trim().to_string();
    if path_prefix.is_empty() {
        return Err("path_prefix must not be empty".to_string());
    }
    if !path_prefix.starts_with('/') {
        return Err("path_prefix must start with `/`".to_string());
    }
    let backend = backend_to_token(r.backend).to_string();
    let methods_json = serde_json::to_string(&r.methods).map_err(|e| e.to_string())?;
    let backend_base = r
        .backend_base
        .as_ref()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());
    let rate_limit_rps = r
        .rate_limit
        .as_ref()
        .map(|rl| rl.rps as i64)
        .filter(|&n| n > 0);
    let auth_mode = auth_to_token(&r.auth).to_string();
    Ok((
        tenant_id,
        path_prefix,
        backend,
        methods_json,
        backend_base,
        rate_limit_rps,
        auth_mode,
        now_ms(),
    ))
}

#[cfg(feature = "control-plane-sql")]
fn row_to_route(
    tenant_id: String,
    path_prefix: String,
    backend: String,
    methods_json: String,
    backend_base: Option<String>,
    rate_limit_rps: Option<i64>,
    auth_mode: String,
) -> Result<ApiGatewayIngressRoute, String> {
    let backend = backend_from_token(&backend)?;
    let methods: Vec<String> =
        serde_json::from_str(&methods_json).map_err(|e| format!("invalid methods_json: {e}"))?;
    let rate_limit = if let Some(n) = rate_limit_rps.filter(|&n| n > 0) {
        let u: u32 = n
            .try_into()
            .map_err(|_| "rate_limit_rps out of range".to_string())?;
        Some(RouteRateLimitConfig { rps: u })
    } else {
        None
    };
    Ok(ApiGatewayIngressRoute {
        tenant_id: if tenant_id.trim().is_empty() {
            None
        } else {
            Some(tenant_id)
        },
        path_prefix,
        backend,
        methods,
        backend_base,
        rate_limit,
        auth: auth_from_token(&auth_mode)?,
    })
}

/// Build dynamic ingress table + optional persistence from config.
pub async fn init_dynamic_ingress(
    cfg: &ControlPlaneConfig,
) -> anyhow::Result<(Arc<DynamicIngressRoutes>, Option<String>)> {
    if !cfg.enabled {
        return Ok((DynamicIngressRoutes::new_arc(), None));
    }

    let persist: Option<Arc<dyn ControlPlanePersist>> = match cfg.store.kind {
        ControlPlaneStoreKind::Memory => None,
        ControlPlaneStoreKind::JsonFile => {
            let path = cfg
                .store
                .json_file
                .as_deref()
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .ok_or_else(|| {
                    anyhow::anyhow!("control_plane.store.json_file required for kind json_file")
                })?;
            Some(Arc::new(JsonFilePersist::new(PathBuf::from(path))) as Arc<dyn ControlPlanePersist>)
        }
        ControlPlaneStoreKind::Sqlite | ControlPlaneStoreKind::Postgres => {
            let url = cfg
                .store
                .database_url
                .as_deref()
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .ok_or_else(|| {
                    anyhow::anyhow!(
                        "control_plane.store.database_url required for sqlite or postgres"
                    )
                })?;
            #[cfg(feature = "control-plane-sql")]
            {
                Some(
                    Arc::new(SqlControlPlanePersist::connect(url, cfg.store.kind).await?)
                        as Arc<dyn ControlPlanePersist>,
                )
            }
            #[cfg(not(feature = "control-plane-sql"))]
            {
                let _ = url;
                anyhow::bail!(
                    "control_plane SQL store requires building panda-proxy with feature `control-plane-sql`"
                );
            }
        }
    };

    let initial = if let Some(p) = &persist {
        p.load_all().await.map_err(anyhow::Error::msg)?
    } else {
        Vec::new()
    };

    let d =
        DynamicIngressRoutes::new_arc_with(persist.clone(), initial).map_err(anyhow::Error::msg)?;

    let note = match cfg.store.kind {
        ControlPlaneStoreKind::Memory => None,
        ControlPlaneStoreKind::JsonFile => Some(format!(
            "control_plane store: json_file ({})",
            cfg.store.json_file.as_deref().unwrap_or("")
        )),
        ControlPlaneStoreKind::Sqlite => Some("control_plane store: sqlite".to_string()),
        ControlPlaneStoreKind::Postgres => Some(
            "control_plane store: postgres (incl. Cloud SQL for PostgreSQL when using a postgres URL)"
                .to_string(),
        ),
    };

    Ok((d, note))
}

// --- JSON file -----------------------------------------------------------------

struct JsonFilePersist {
    path: PathBuf,
    io: Mutex<()>,
}

impl JsonFilePersist {
    fn new(path: PathBuf) -> Self {
        Self {
            path,
            io: Mutex::new(()),
        }
    }

    async fn read_file(&self) -> Result<Vec<ApiGatewayIngressRoute>, String> {
        let _g = self.io.lock().await;
        if !Path::new(&self.path).exists() {
            return Ok(Vec::new());
        }
        let raw = tokio::fs::read_to_string(&self.path)
            .await
            .map_err(|e| format!("read {}: {e}", self.path.display()))?;
        if raw.trim().is_empty() {
            return Ok(Vec::new());
        }
        parse_import_body(raw.as_bytes())
    }

    async fn write_file(&self, routes: &[ApiGatewayIngressRoute]) -> Result<(), String> {
        let _g = self.io.lock().await;
        let v = export_routes_json(routes);
        let s = serde_json::to_string_pretty(&v).map_err(|e| e.to_string())?;
        let dir = self.path.parent().filter(|p| !p.as_os_str().is_empty());
        if let Some(d) = dir {
            tokio::fs::create_dir_all(d)
                .await
                .map_err(|e| format!("create_dir_all: {e}"))?;
        }
        let tmp = self.path.with_extension("json.tmp");
        tokio::fs::write(&tmp, s.as_bytes())
            .await
            .map_err(|e| format!("write tmp: {e}"))?;
        tokio::fs::rename(&tmp, &self.path)
            .await
            .map_err(|e| format!("rename: {e}"))?;
        Ok(())
    }
}

#[async_trait]
impl ControlPlanePersist for JsonFilePersist {
    async fn upsert(&self, r: &ApiGatewayIngressRoute) -> Result<(), String> {
        let mut all = self.read_file().await?;
        let t = r
            .tenant_id
            .as_deref()
            .map(str::trim)
            .unwrap_or("")
            .to_string();
        let p = r.path_prefix.trim().to_string();
        if let Some(i) = all.iter().position(|x| {
            x.path_prefix.trim() == p
                && x.tenant_id.as_deref().map(str::trim).unwrap_or("") == t.as_str()
        }) {
            all[i] = r.clone();
        } else {
            all.push(r.clone());
        }
        self.write_file(&all).await
    }

    async fn remove(&self, tenant_id: &str, path_prefix: &str) -> Result<bool, String> {
        let tid = tenant_id.trim();
        let p = path_prefix.trim();
        let mut all = self.read_file().await?;
        let before = all.len();
        all.retain(|x| {
            x.path_prefix.trim() != p || x.tenant_id.as_deref().map(str::trim).unwrap_or("") != tid
        });
        if all.len() == before {
            return Ok(false);
        }
        self.write_file(&all).await?;
        Ok(true)
    }

    async fn load_all(&self) -> Result<Vec<ApiGatewayIngressRoute>, String> {
        self.read_file().await
    }

    async fn replace_all(&self, routes: &[ApiGatewayIngressRoute]) -> Result<(), String> {
        self.write_file(routes).await
    }
}

// --- SQL (SQLite / PostgreSQL; Cloud SQL for PostgreSQL uses the Postgres driver + URL) ---------

#[cfg(feature = "control-plane-sql")]
enum SqlDb {
    Sqlite(sqlx::SqlitePool),
    Postgres(sqlx::PgPool),
}

#[cfg(feature = "control-plane-sql")]
struct SqlControlPlanePersist {
    db: SqlDb,
}

#[cfg(feature = "control-plane-sql")]
const UPSERT_SQL: &str = r#"
INSERT INTO panda_control_plane_ingress_route (tenant_id, path_prefix, backend, methods_json, backend_base, rate_limit_rps, auth_mode, updated_at_ms)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
ON CONFLICT(tenant_id, path_prefix) DO UPDATE SET
  backend = excluded.backend,
  methods_json = excluded.methods_json,
  backend_base = excluded.backend_base,
  rate_limit_rps = excluded.rate_limit_rps,
  auth_mode = excluded.auth_mode,
  updated_at_ms = excluded.updated_at_ms
"#;

#[cfg(feature = "control-plane-sql")]
const PG_INGRESS_NOTIFY_CHANNEL: &str = "panda_cp_ingress";

#[cfg(feature = "control-plane-sql")]
impl SqlControlPlanePersist {
    async fn connect(url: &str, kind: ControlPlaneStoreKind) -> anyhow::Result<Self> {
        let url = url.trim();
        let db = match kind {
            ControlPlaneStoreKind::Sqlite => {
                if !url.starts_with("sqlite:") {
                    anyhow::bail!("sqlite store kind expects database_url to start with sqlite:");
                }
                let p = sqlx::sqlite::SqlitePoolOptions::new()
                    .max_connections(5)
                    .connect(url)
                    .await?;
                sqlx::migrate!("./migrations").run(&p).await?;
                SqlDb::Sqlite(p)
            }
            ControlPlaneStoreKind::Postgres => {
                if !url.starts_with("postgres://") && !url.starts_with("postgresql://") {
                    anyhow::bail!(
                        "postgres store kind expects database_url to start with postgres:// or postgresql://"
                    );
                }
                let p = sqlx::postgres::PgPoolOptions::new()
                    .max_connections(5)
                    .connect(url)
                    .await?;
                sqlx::migrate!("./migrations").run(&p).await?;
                SqlDb::Postgres(p)
            }
            _ => anyhow::bail!("internal: sql persist only for sqlite or postgres kind"),
        };
        Ok(Self { db })
    }

    async fn pg_notify_ingress_changed(&self) {
        let SqlDb::Postgres(p) = &self.db else {
            return;
        };
        let q = format!("SELECT pg_notify('{PG_INGRESS_NOTIFY_CHANNEL}', '')");
        if let Err(e) = sqlx::query(&q).execute(p).await {
            tracing::warn!(
                target: "panda::control_plane",
                "pg_notify failed (replicas may lag until reload_from_store_ms): {e}"
            );
        }
    }

    async fn insert_or_update(&self, r: &ApiGatewayIngressRoute) -> Result<(), String> {
        let (
            tenant_id,
            path_prefix,
            backend,
            methods_json,
            backend_base,
            rate_limit_rps,
            auth_mode,
            ts,
        ) = route_to_row(r)?;
        match &self.db {
            SqlDb::Sqlite(p) => {
                sqlx::query(UPSERT_SQL)
                    .bind(&tenant_id)
                    .bind(&path_prefix)
                    .bind(&backend)
                    .bind(&methods_json)
                    .bind(&backend_base)
                    .bind(rate_limit_rps)
                    .bind(&auth_mode)
                    .bind(ts)
                    .execute(p)
                    .await
                    .map_err(|e| e.to_string())?;
            }
            SqlDb::Postgres(p) => {
                sqlx::query(UPSERT_SQL)
                    .bind(&tenant_id)
                    .bind(&path_prefix)
                    .bind(&backend)
                    .bind(&methods_json)
                    .bind(&backend_base)
                    .bind(rate_limit_rps)
                    .bind(&auth_mode)
                    .bind(ts)
                    .execute(p)
                    .await
                    .map_err(|e| e.to_string())?;
            }
        }
        self.pg_notify_ingress_changed().await;
        Ok(())
    }
}

#[cfg(feature = "control-plane-sql")]
#[async_trait]
impl ControlPlanePersist for SqlControlPlanePersist {
    async fn upsert(&self, r: &ApiGatewayIngressRoute) -> Result<(), String> {
        self.insert_or_update(r).await
    }

    async fn remove(&self, tenant_id: &str, path_prefix: &str) -> Result<bool, String> {
        let tid = tenant_id.trim();
        let pfx = path_prefix.trim();
        let n = match &self.db {
            SqlDb::Sqlite(pool) => {
                sqlx::query(
                    "DELETE FROM panda_control_plane_ingress_route WHERE tenant_id = $1 AND path_prefix = $2",
                )
                .bind(tid)
                .bind(pfx)
                .execute(pool)
                .await
                .map_err(|e| e.to_string())?
                .rows_affected()
            }
            SqlDb::Postgres(pool) => {
                sqlx::query(
                    "DELETE FROM panda_control_plane_ingress_route WHERE tenant_id = $1 AND path_prefix = $2",
                )
                .bind(tid)
                .bind(pfx)
                .execute(pool)
                .await
                .map_err(|e| e.to_string())?
                .rows_affected()
            }
        };
        if n > 0 {
            self.pg_notify_ingress_changed().await;
        }
        Ok(n > 0)
    }

    async fn load_all(&self) -> Result<Vec<ApiGatewayIngressRoute>, String> {
        use sqlx::Row;
        const Q: &str = "SELECT tenant_id, path_prefix, backend, methods_json, backend_base, rate_limit_rps, auth_mode FROM panda_control_plane_ingress_route ORDER BY tenant_id, path_prefix";
        match &self.db {
            SqlDb::Sqlite(pool) => {
                let rows = sqlx::query(Q)
                    .fetch_all(pool)
                    .await
                    .map_err(|e| e.to_string())?;
                let mut out = Vec::with_capacity(rows.len());
                for row in rows {
                    let tenant_id: String = row.try_get("tenant_id").map_err(|e| e.to_string())?;
                    let path_prefix: String =
                        row.try_get("path_prefix").map_err(|e| e.to_string())?;
                    let backend: String = row.try_get("backend").map_err(|e| e.to_string())?;
                    let methods_json: String =
                        row.try_get("methods_json").map_err(|e| e.to_string())?;
                    let backend_base: Option<String> =
                        row.try_get("backend_base").map_err(|e| e.to_string())?;
                    let rate_limit_rps: Option<i64> =
                        row.try_get("rate_limit_rps").map_err(|e| e.to_string())?;
                    let auth_mode: String =
                        row.try_get("auth_mode").map_err(|e| e.to_string())?;
                    out.push(row_to_route(
                        tenant_id,
                        path_prefix,
                        backend,
                        methods_json,
                        backend_base,
                        rate_limit_rps,
                        auth_mode,
                    )?);
                }
                Ok(out)
            }
            SqlDb::Postgres(pool) => {
                let rows = sqlx::query(Q)
                    .fetch_all(pool)
                    .await
                    .map_err(|e| e.to_string())?;
                let mut out = Vec::with_capacity(rows.len());
                for row in rows {
                    let tenant_id: String = row.try_get("tenant_id").map_err(|e| e.to_string())?;
                    let path_prefix: String =
                        row.try_get("path_prefix").map_err(|e| e.to_string())?;
                    let backend: String = row.try_get("backend").map_err(|e| e.to_string())?;
                    let methods_json: String =
                        row.try_get("methods_json").map_err(|e| e.to_string())?;
                    let backend_base: Option<String> =
                        row.try_get("backend_base").map_err(|e| e.to_string())?;
                    let rate_limit_rps: Option<i64> =
                        row.try_get("rate_limit_rps").map_err(|e| e.to_string())?;
                    let auth_mode: String =
                        row.try_get("auth_mode").map_err(|e| e.to_string())?;
                    out.push(row_to_route(
                        tenant_id,
                        path_prefix,
                        backend,
                        methods_json,
                        backend_base,
                        rate_limit_rps,
                        auth_mode,
                    )?);
                }
                Ok(out)
            }
        }
    }

    async fn replace_all(&self, routes: &[ApiGatewayIngressRoute]) -> Result<(), String> {
        match &self.db {
            SqlDb::Sqlite(p) => {
                let mut tx = p.begin().await.map_err(|e| e.to_string())?;
                sqlx::query("DELETE FROM panda_control_plane_ingress_route")
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;
                for r in routes {
                    let (
                        tenant_id,
                        path_prefix,
                        backend,
                        methods_json,
                        backend_base,
                        rate_limit_rps,
                        auth_mode,
                        ts,
                    ) = route_to_row(r)?;
                    sqlx::query(
                        r#"INSERT INTO panda_control_plane_ingress_route (tenant_id, path_prefix, backend, methods_json, backend_base, rate_limit_rps, auth_mode, updated_at_ms)
                           VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"#,
                    )
                    .bind(&tenant_id)
                    .bind(&path_prefix)
                    .bind(&backend)
                    .bind(&methods_json)
                    .bind(&backend_base)
                    .bind(rate_limit_rps)
                    .bind(&auth_mode)
                    .bind(ts)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;
                }
                tx.commit().await.map_err(|e| e.to_string())?;
            }
            SqlDb::Postgres(p) => {
                let mut tx = p.begin().await.map_err(|e| e.to_string())?;
                sqlx::query("DELETE FROM panda_control_plane_ingress_route")
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;
                for r in routes {
                    let (
                        tenant_id,
                        path_prefix,
                        backend,
                        methods_json,
                        backend_base,
                        rate_limit_rps,
                        auth_mode,
                        ts,
                    ) = route_to_row(r)?;
                    sqlx::query(
                        r#"INSERT INTO panda_control_plane_ingress_route (tenant_id, path_prefix, backend, methods_json, backend_base, rate_limit_rps, auth_mode, updated_at_ms)
                           VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"#,
                    )
                    .bind(&tenant_id)
                    .bind(&path_prefix)
                    .bind(&backend)
                    .bind(&methods_json)
                    .bind(&backend_base)
                    .bind(rate_limit_rps)
                    .bind(&auth_mode)
                    .bind(ts)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;
                }
                tx.commit().await.map_err(|e| e.to_string())?;
                self.pg_notify_ingress_changed().await;
            }
        }
        Ok(())
    }
}

/// Re-read routes from the backing store and replace the in-memory dynamic table (no store writes).
pub async fn reload_dynamic_ingress_from_store(
    dynamic: &DynamicIngressRoutes,
) -> Result<(), String> {
    let Some(p) = dynamic.persist_handle() else {
        return Ok(());
    };
    let routes = p.load_all().await?;
    dynamic.replace_all_from_routes(routes)
}

/// Background poll so replicas pick up changes from another instance or GitOps edits to `json_file`.
pub fn spawn_control_plane_store_reload_loop(
    dynamic: Arc<DynamicIngressRoutes>,
    interval: Duration,
) {
    tokio::spawn(async move {
        let mut tick = tokio::time::interval(interval);
        tick.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
        loop {
            tick.tick().await;
            if let Err(e) = reload_dynamic_ingress_from_store(dynamic.as_ref()).await {
                tracing::warn!(target: "panda::control_plane", "reload_from_store_ms tick failed: {e}");
            }
        }
    });
}

#[cfg(feature = "control-plane-sql")]
pub fn spawn_postgres_control_plane_listener(url: String, dynamic: Arc<DynamicIngressRoutes>) {
    tokio::spawn(async move {
        loop {
            match sqlx::postgres::PgListener::connect(&url).await {
                Ok(mut listener) => {
                    if let Err(e) = listener.listen(PG_INGRESS_NOTIFY_CHANNEL).await {
                        tracing::warn!(
                            target: "panda::control_plane",
                            "postgres LISTEN failed: {e}; retrying in 5s"
                        );
                        tokio::time::sleep(Duration::from_secs(5)).await;
                        continue;
                    }
                    tracing::info!(target: "panda::control_plane", "postgres NOTIFY listener active ({})", PG_INGRESS_NOTIFY_CHANNEL);
                    loop {
                        match listener.recv().await {
                            Ok(_) => {
                                if let Err(e) =
                                    reload_dynamic_ingress_from_store(dynamic.as_ref()).await
                                {
                                    tracing::warn!(
                                        target: "panda::control_plane",
                                        "reload after NOTIFY failed: {e}"
                                    );
                                }
                            }
                            Err(e) => {
                                tracing::warn!(
                                    target: "panda::control_plane",
                                    "postgres listener recv error: {e}; reconnecting"
                                );
                                break;
                            }
                        }
                    }
                }
                Err(e) => {
                    tracing::warn!(
                        target: "panda::control_plane",
                        "postgres listener connect failed: {e}; retrying in 5s"
                    );
                    tokio::time::sleep(Duration::from_secs(5)).await;
                }
            }
        }
    });
}

// --- Redis pub/sub (cross-replica reload; optional) -----------------------------------------------

use redis::AsyncCommands;
use sha2::{Digest, Sha256};

/// Redis key for a control-plane API token (`prefix` + SHA-256 hex of raw token).
pub fn control_plane_api_key_storage_key(prefix: &str, token: &str) -> String {
    let mut h = Sha256::new();
    h.update(token.as_bytes());
    format!("{}{:x}", prefix, h.finalize())
}

/// Stored control-plane API key metadata (Redis value; legacy keys use `"1"`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlPlaneApiKeyRecord {
    #[serde(default)]
    pub scopes: Vec<String>,
    #[serde(default)]
    pub tenant_id: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlPlaneApiKeyAccessLevel {
    ReadOnly,
    ReadWrite,
}

/// Legacy `"1"` or JSON [`ControlPlaneApiKeyRecord`]; returns `None` when the key is missing or invalid.
pub fn parse_control_plane_api_key_value(v: &str) -> Option<(ControlPlaneApiKeyAccessLevel, Option<String>)> {
    let v = v.trim();
    if v == "1" || v.is_empty() {
        return Some((ControlPlaneApiKeyAccessLevel::ReadWrite, None));
    }
    let rec: ControlPlaneApiKeyRecord = serde_json::from_str(v).ok()?;
    let tenant = rec
        .tenant_id
        .as_ref()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string());
    let scopes: Vec<String> = rec
        .scopes
        .iter()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    if scopes.is_empty() {
        return Some((ControlPlaneApiKeyAccessLevel::ReadWrite, tenant));
    }
    let rw = scopes.iter().any(|s| {
        let s = s.as_str();
        s == "write"
            || s == "*"
            || s == "control_plane:write"
            || s.eq_ignore_ascii_case("rw")
    });
    let ro = scopes.iter().any(|s| {
        let s = s.as_str();
        s == "read" || s == "control_plane:read" || s.eq_ignore_ascii_case("ro")
    });
    if rw {
        Some((ControlPlaneApiKeyAccessLevel::ReadWrite, tenant))
    } else if ro {
        Some((ControlPlaneApiKeyAccessLevel::ReadOnly, tenant))
    } else {
        None
    }
}

pub async fn control_plane_api_key_access(
    conn: &mut redis::aio::ConnectionManager,
    prefix: &str,
    token: &str,
) -> Option<(ControlPlaneApiKeyAccessLevel, Option<String>)> {
    let key = control_plane_api_key_storage_key(prefix, token);
    let v: Option<String> = conn.get(&key).await.ok().flatten();
    let v = v?;
    parse_control_plane_api_key_value(&v)
}

pub async fn control_plane_api_key_issue(
    conn: &mut redis::aio::ConnectionManager,
    prefix: &str,
    ttl_seconds: Option<u64>,
    record: &ControlPlaneApiKeyRecord,
) -> Result<String, String> {
    let token = uuid::Uuid::new_v4().to_string();
    let key = control_plane_api_key_storage_key(prefix, &token);
    let value = if record.scopes.is_empty()
        && record
            .tenant_id
            .as_ref()
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .is_none()
    {
        "1".to_string()
    } else {
        serde_json::to_string(record).map_err(|e| e.to_string())?
    };
    match ttl_seconds.filter(|t| *t > 0) {
        Some(t) => conn
            .set_ex::<_, _, ()>(&key, &value, t)
            .await
            .map_err(|e| e.to_string())?,
        None => conn
            .set::<_, _, ()>(&key, &value)
            .await
            .map_err(|e| e.to_string())?,
    }
    Ok(token)
}

pub async fn control_plane_api_key_revoke(
    conn: &mut redis::aio::ConnectionManager,
    prefix: &str,
    token: &str,
) -> Result<bool, String> {
    let key = control_plane_api_key_storage_key(prefix, token);
    let n: u64 = conn.del(&key).await.map_err(|e| e.to_string())?;
    Ok(n > 0)
}

/// After mutating dynamic ingress, publish so other replicas reload from the backing store.
pub async fn publish_control_plane_ingress_reload(redis_url: &str, channel: &str) {
    let Ok(client) = redis::Client::open(redis_url) else {
        tracing::warn!(target: "panda::control_plane", "redis PUBLISH: invalid URL");
        return;
    };
    let Ok(mut conn) = redis::aio::ConnectionManager::new(client).await else {
        tracing::warn!(target: "panda::control_plane", "redis PUBLISH: connection failed");
        return;
    };
    let r: redis::RedisResult<i64> = redis::cmd("PUBLISH")
        .arg(channel)
        .arg("reload")
        .query_async(&mut conn)
        .await;
    if let Err(e) = r {
        tracing::warn!(target: "panda::control_plane", "redis PUBLISH failed: {e}");
    }
}

pub fn spawn_control_plane_redis_reload_subscriber(
    redis_url: String,
    channel: String,
    dynamic: Arc<DynamicIngressRoutes>,
) {
    tokio::spawn(async move {
        loop {
            match run_redis_ingress_reload_subscriber(&redis_url, &channel, &dynamic).await {
                Ok(()) => {
                    tracing::warn!(target: "panda::control_plane", "redis pub/sub stream ended; reconnecting");
                }
                Err(e) => {
                    tracing::warn!(target: "panda::control_plane", "redis pub/sub: {e}; reconnecting");
                }
            }
            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    });
}

async fn run_redis_ingress_reload_subscriber(
    redis_url: &str,
    channel: &str,
    dynamic: &Arc<DynamicIngressRoutes>,
) -> Result<(), String> {
    let client = redis::Client::open(redis_url).map_err(|e| e.to_string())?;
    let mut pubsub = client.get_async_pubsub().await.map_err(|e| e.to_string())?;
    pubsub.subscribe(channel).await.map_err(|e| e.to_string())?;
    tracing::info!(target: "panda::control_plane", "redis SUBSCRIBE {channel} (dynamic ingress reload)");
    let mut stream = pubsub.on_message();
    while let Some(msg) = stream.next().await {
        let _: String = msg.get_payload().unwrap_or_default();
        if let Err(e) = reload_dynamic_ingress_from_store(dynamic.as_ref()).await {
            tracing::warn!(target: "panda::control_plane", "reload after redis PUBLISH failed: {e}");
        }
    }
    Ok(())
}

/// Import merge/replace from HTTP handler: updates backing store then in-memory table.
pub async fn import_dynamic_routes(
    dynamic: &DynamicIngressRoutes,
    routes: Vec<ApiGatewayIngressRoute>,
    replace: bool,
) -> Result<usize, String> {
    if replace {
        if let Some(p) = dynamic.persist_handle() {
            p.replace_all(&routes).await?;
        }
        dynamic.replace_all_from_routes(routes.clone())?;
        return Ok(routes.len());
    }
    let mut n = 0usize;
    for r in routes {
        if let Some(p) = dynamic.persist_handle() {
            p.upsert(&r).await?;
        }
        dynamic.upsert_route_memory_only(&r)?;
        n += 1;
    }
    Ok(n)
}

#[cfg(test)]
mod api_key_parse_tests {
    use super::*;

    #[test]
    fn parse_legacy_and_json_scopes() {
        assert_eq!(
            parse_control_plane_api_key_value("1"),
            Some((ControlPlaneApiKeyAccessLevel::ReadWrite, None))
        );
        let ro = r#"{"scopes":["read"],"tenant_id":"acme"}"#;
        assert_eq!(
            parse_control_plane_api_key_value(ro),
            Some((
                ControlPlaneApiKeyAccessLevel::ReadOnly,
                Some("acme".to_string())
            ))
        );
        let rw = r#"{"scopes":["write"],"tenant_id":"acme"}"#;
        assert_eq!(
            parse_control_plane_api_key_value(rw),
            Some((
                ControlPlaneApiKeyAccessLevel::ReadWrite,
                Some("acme".to_string())
            ))
        );
        assert_eq!(
            parse_control_plane_api_key_value("{}"),
            Some((ControlPlaneApiKeyAccessLevel::ReadWrite, None))
        );
        assert_eq!(
            parse_control_plane_api_key_value(r#"{"scopes":["nope"]}"#),
            None
        );
    }
}

#[cfg(test)]
mod reload_tests {
    use super::*;
    use panda_config::ApiGatewayIngressBackend;

    #[tokio::test]
    async fn reload_dynamic_ingress_from_json_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("routes.json");
        let json_a = r#"{"version":1,"routes":[{"path_prefix":"/a","backend":"ai","methods":[],"backend_base":null}]}"#;
        tokio::fs::write(&path, json_a).await.unwrap();
        let persist: Arc<dyn ControlPlanePersist> = Arc::new(JsonFilePersist::new(path.clone()));
        let initial = persist.load_all().await.unwrap();
        let dynamic =
            DynamicIngressRoutes::new_arc_with(Some(Arc::clone(&persist)), initial).unwrap();
        assert_eq!(dynamic.route_count(), 1);
        let json_b = r#"{"version":1,"routes":[{"path_prefix":"/bee","backend":"mcp","methods":[],"backend_base":null}]}"#;
        tokio::fs::write(&path, json_b).await.unwrap();
        reload_dynamic_ingress_from_store(&dynamic).await.unwrap();
        assert_eq!(dynamic.route_count(), 1);
        let snap = dynamic.api_routes_snapshot();
        assert_eq!(snap[0].path_prefix, "/bee");
        assert_eq!(snap[0].backend, ApiGatewayIngressBackend::Mcp);
    }
}
