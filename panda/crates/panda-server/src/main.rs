//! Thin entrypoint: load YAML config, run async proxy (Phase 1.1 workspace boundary).

/// JSON Schema for WebSocket text frames on `GET /console/ws` (Live Trace v1).
const LIVE_TRACE_WS_SCHEMA_V1: &str = include_str!("../schemas/live_trace_ws.v1.schema.json");

#[cfg(feature = "grpc")]
mod grpc_health;

use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Context;
use clap::Parser;
use opentelemetry::global;
use opentelemetry::trace::TracerProvider;
use opentelemetry::KeyValue;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{trace::SdkTracerProvider, Resource};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

#[cfg(feature = "mimalloc")]
#[global_allocator]
static GLOBAL_ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

const ENV_HELP: &str = r#"Environment (non-exhaustive; see repository README):
  OTEL_EXPORTER_OTLP_ENDPOINT       OTLP HTTP traces endpoint (enables trace export when set)
  PANDA_OTEL_SERVICE_NAME           OpenTelemetry service.name (default: panda-gateway)
  PANDA_OTEL_TRACE_SAMPLING_RATIO   Trace sampling 0.0–1.0 (default: 1.0)
  PANDA_REDIS_URL                   Overrides Redis URL from YAML (TPM / shared counters)
  PANDA_SEMANTIC_CACHE_REDIS_URL    Semantic cache Redis URL
  PANDA_SEMANTIC_CACHE_TIMEOUT_MS   Semantic cache get timeout (default: 50)
  PANDA_UPSTREAM_FIRST_BYTE_TIMEOUT_MS  Max wait for first upstream response body byte after headers (0=off)
  PANDA_UPSTREAM_SSE_IDLE_TIMEOUT_MS    Max idle between SSE body chunks after first byte (0=off; default 120s)
  PANDA_DEV_CONSOLE_ENABLED         Set to `true` to expose the developer console
  PANDA_LISTEN_OVERRIDE             When set to host:port, overrides YAML `listen` (used by `--ui`)
  PANDA_CONSOLE_BLEND_PRICE_PER_MILLION_TOKENS  Optional float; enables $ estimates in /tpm/status + console
  RUST_LOG                          Tracing filter (default: info); e.g. panda_proxy=debug
"#;

/// Panda AI gateway: streaming HTTP proxy, MCP host, semantic cache, Wasm plugins.
#[derive(Parser)]
#[command(
    name = "panda",
    author,
    version,
    about = "Panda AI gateway: streaming HTTP proxy, MCP host, semantic cache, Wasm plugins.",
    long_about = "Loads a YAML configuration file and starts the HTTP listener. \
For an annotated template, see `panda.example.yaml` in the repository root.",
    after_long_help = ENV_HELP
)]
struct Cli {
    /// Path to the YAML configuration file
    #[arg(value_name = "CONFIG", default_value = "panda.yaml")]
    config: PathBuf,
    /// Print the Live Trace WebSocket JSON Schema (v1) to stdout and exit
    #[arg(long)]
    print_live_trace_schema: bool,
    /// Enable the Live Trace developer console and prefer `127.0.0.1:8081` (unless `PANDA_LISTEN_OVERRIDE` is set)
    #[arg(long)]
    ui: bool,
}

/// When OTLP is enabled, returns a clone of the SDK provider for shutdown after the runtime stops
/// (OpenTelemetry 0.31 removed `global::shutdown_tracer_provider`).
fn init_observability() -> Option<SdkTracerProvider> {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let fmt_layer = tracing_subscriber::fmt::layer().json();

    if let Some(endpoint) = std::env::var("OTEL_EXPORTER_OTLP_ENDPOINT")
        .ok()
        .filter(|s| !s.trim().is_empty())
    {
        let service_name = std::env::var("PANDA_OTEL_SERVICE_NAME")
            .ok()
            .filter(|s| !s.trim().is_empty())
            .unwrap_or_else(|| "panda-gateway".to_string());
        let trace_sampling_ratio = std::env::var("PANDA_OTEL_TRACE_SAMPLING_RATIO")
            .ok()
            .and_then(|s| s.parse::<f64>().ok())
            .map(|r| r.clamp(0.0, 1.0))
            .unwrap_or(1.0);
        match build_otel_provider(&endpoint, &service_name, trace_sampling_ratio) {
            Ok(provider) => {
                let shutdown_handle = provider.clone();
                let tracer = provider.tracer("panda");
                global::set_tracer_provider(provider);
                let _ = tracing_subscriber::registry()
                    .with(filter)
                    .with(fmt_layer)
                    .with(tracing_opentelemetry::layer().with_tracer(tracer))
                    .try_init();
                return Some(shutdown_handle);
            }
            Err(e) => {
                eprintln!(
                    "panda: failed to init OTLP exporter ({e}); falling back to logs-only tracing"
                );
                let _ = tracing_subscriber::registry()
                    .with(filter)
                    .with(fmt_layer)
                    .try_init();
            }
        }
    } else {
        let _ = tracing_subscriber::registry()
            .with(filter)
            .with(fmt_layer)
            .try_init();
    }
    None
}

fn build_otel_provider(
    endpoint: &str,
    service_name: &str,
    trace_sampling_ratio: f64,
) -> anyhow::Result<SdkTracerProvider> {
    let exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_http()
        .with_endpoint(endpoint)
        .build()?;
    Ok(SdkTracerProvider::builder()
        .with_resource(
            Resource::builder_empty()
                .with_attributes([KeyValue::new("service.name", service_name.to_string())])
                .build(),
        )
        .with_sampler(opentelemetry_sdk::trace::Sampler::ParentBased(Box::new(
            opentelemetry_sdk::trace::Sampler::TraceIdRatioBased(trace_sampling_ratio),
        )))
        .with_batch_exporter(exporter)
        .build())
}

fn main() -> anyhow::Result<()> {
    let otel_provider = init_observability();

    let cli = Cli::parse();
    if cli.print_live_trace_schema {
        print!("{LIVE_TRACE_WS_SCHEMA_V1}");
        return Ok(());
    }
    if cli.ui {
        std::env::set_var("PANDA_DEV_CONSOLE_ENABLED", "true");
        let existing = std::env::var("PANDA_LISTEN_OVERRIDE").unwrap_or_default();
        if existing.trim().is_empty() {
            std::env::set_var("PANDA_LISTEN_OVERRIDE", "127.0.0.1:8081");
        }
        eprintln!("panda: --ui enabled the Live Trace console (PANDA_DEV_CONSOLE_ENABLED=true)");
        eprintln!(
            "panda: open http://{}/console after startup (override bind with PANDA_LISTEN_OVERRIDE)",
            std::env::var("PANDA_LISTEN_OVERRIDE").unwrap_or_default()
        );
    }
    let config_path = cli.config;

    let config = Arc::new(
        panda_config::PandaConfig::load_from_path(&config_path)
            .with_context(|| format!("failed to load config from {}", config_path.display()))?,
    );

    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;

    let out = rt.block_on(async move {
        #[cfg(feature = "grpc")]
        let grpc_shutdown_tx = {
            if let Ok(raw) = std::env::var("PANDA_GRPC_HEALTH_LISTEN") {
                let raw = raw.trim();
                if raw.is_empty() {
                    None
                } else {
                    match raw.parse::<std::net::SocketAddr>() {
                        Ok(addr) => {
                            let (tx, rx) = tokio::sync::oneshot::channel();
                            tokio::spawn(async move {
                                if let Err(e) = grpc_health::serve_health(addr, rx).await {
                                    eprintln!("panda: gRPC health server exited: {e}");
                                }
                            });
                            eprintln!("panda: gRPC health listening on grpc://{addr} (stops with HTTP gateway)");
                            Some(tx)
                        }
                        Err(_) => {
                            eprintln!("panda: PANDA_GRPC_HEALTH_LISTEN invalid: {raw}");
                            None
                        }
                    }
                }
            } else {
                None
            }
        };

        let run_result = panda_proxy::run(config).await;

        #[cfg(feature = "grpc")]
        if let Some(tx) = grpc_shutdown_tx {
            let _ = tx.send(());
        }

        run_result
    });
    if let Some(p) = otel_provider {
        let _ = p.shutdown();
    }
    out
}
