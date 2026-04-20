//! Wasm plugin host for Panda (Phase 2).
//!
//! Current ABI v1:
//! - guest export `panda_abi_version() -> i32` (must equal [`PANDA_WASM_ABI_VERSION`])
//! - optional guest export `panda_on_request() -> i32`
//! - optional guest export `panda_on_request_body(ptr: i32, len: i32) -> i32`
//! - optional guest export `panda_on_response_chunk(ptr: i32, len: i32) -> i32`
//! - host import `panda_set_header(name_ptr, name_len, value_ptr, value_len)`
//! - host import `panda_set_body(ptr, len)` for request body replacement
//! - host import `panda_set_response_chunk(ptr, len)` for streaming response chunk replacement
//!
//! A plugin may call `panda_set_header` during `panda_on_request` to append request
//! headers before upstream forwarding. If request body hooks are enabled by the host,
//! `panda_on_request_body` can replace the buffered body through `panda_set_body`.
//!
//! Return code semantics for `panda_on_request*`:
//! - `0`: allow
//! - `1`: reject policy denied
//! - `2`: reject malformed request
//! - any other non-zero: reject plugin-specific

use std::path::Path;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::Instant;

use http::header::{HeaderMap, HeaderName, HeaderValue};
use wasmtime::{Caller, Engine, Instance, Linker, Memory, Module, Store};

/// ABI version negotiated between `panda-proxy` and guest modules.
pub const PANDA_WASM_ABI_VERSION: u32 = 1;
pub const RC_ALLOW: i32 = 0;
pub const RC_REJECT_POLICY_DENIED: i32 = 1;
pub const RC_REJECT_MALFORMED_REQUEST: i32 = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyCode {
    Denied,
    MalformedRequest,
    PluginSpecific(i32),
}

impl PolicyCode {
    fn from_rc(rc: i32) -> Option<Self> {
        if rc == RC_ALLOW {
            return None;
        }
        Some(match rc {
            RC_REJECT_POLICY_DENIED => Self::Denied,
            RC_REJECT_MALFORMED_REQUEST => Self::MalformedRequest,
            n => Self::PluginSpecific(n),
        })
    }
}

#[derive(Debug)]
pub enum HookFailure {
    PolicyReject {
        plugin: String,
        code: PolicyCode,
    },
    Runtime {
        plugin: String,
        reason: RuntimeReason,
        message: String,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeReason {
    Trap,
    MemoryViolation,
    InvalidInput,
    HostCallFailure,
    Internal,
}

impl std::fmt::Display for HookFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PolicyReject { plugin, code } => {
                write!(f, "plugin {plugin} rejected request code={code:?}")
            }
            Self::Runtime {
                plugin,
                reason,
                message,
            } => write!(
                f,
                "plugin {plugin} runtime failure reason={reason:?}: {message}"
            ),
        }
    }
}

impl std::error::Error for HookFailure {}

fn classify_runtime_reason(msg: &str) -> RuntimeReason {
    let m = msg.to_ascii_lowercase();
    if m.contains("trap") {
        return RuntimeReason::Trap;
    }
    if m.contains("out of bounds") || m.contains("negative ptr/len") || m.contains("overflow") {
        return RuntimeReason::MemoryViolation;
    }
    if m.contains("utf-8") || m.contains("invalid header") {
        return RuntimeReason::InvalidInput;
    }
    if m.contains("host") || m.contains("panda_set_") {
        return RuntimeReason::HostCallFailure;
    }
    RuntimeReason::Internal
}

#[derive(Default)]
struct HostState {
    pending_headers: Vec<(String, String)>,
    pending_body: Option<Vec<u8>>,
    pending_response_chunk: Option<Vec<u8>>,
}

struct PluginInner {
    store: Store<HostState>,
    instance: Instance,
}

enum HookCallError {
    Policy(PolicyCode),
    Runtime(anyhow::Error),
}

/// One guest module loaded from disk.
pub struct LoadedPlugin {
    pub name: String,
    inners: Vec<Mutex<PluginInner>>,
    next_idx: AtomicUsize,
    stats: std::sync::Arc<RuntimeStats>,
}

impl LoadedPlugin {
    fn with_inner<R>(
        &self,
        f: impl FnOnce(&mut PluginInner) -> Result<R, HookCallError>,
    ) -> Result<R, HookCallError> {
        let idx = self.next_idx.fetch_add(1, Ordering::Relaxed) % self.inners.len().max(1);
        self.stats
            .pool_acquire_total
            .fetch_add(1, Ordering::Relaxed);
        match self.inners[idx].try_lock() {
            Ok(mut guard) => f(&mut guard),
            Err(_) => {
                self.stats
                    .pool_contended_total
                    .fetch_add(1, Ordering::Relaxed);
                let started = Instant::now();
                let mut guard = self.inners[idx]
                    .lock()
                    .map_err(|e| HookCallError::Runtime(anyhow::anyhow!("plugin mutex: {e}")))?;
                self.stats
                    .pool_wait_ns_total
                    .fetch_add(started.elapsed().as_nanos() as u64, Ordering::Relaxed);
                f(&mut guard)
            }
        }
    }

    /// If the module exports `add(i32, i32) -> i32`, call `add(1, 2)` and log; otherwise no-op.
    pub fn invoke_add_smoke(&self) -> anyhow::Result<()> {
        self.with_inner(|g| {
            let add = {
                match g
                    .instance
                    .get_typed_func::<(i32, i32), i32>(&mut g.store, "add")
                {
                    Ok(f) => f,
                    Err(_) => return Ok(()),
                }
            };
            let r = add
                .call(&mut g.store, (1, 2))
                .map_err(|e| HookCallError::Runtime(anyhow::anyhow!("{e}")))?;
            eprintln!("panda wasm plugin {}: add(1,2) = {}", self.name, r);
            Ok(())
        })
        .map_err(|e| match e {
            HookCallError::Policy(code) => anyhow::anyhow!("unexpected policy in smoke: {code:?}"),
            HookCallError::Runtime(err) => err,
        })
    }

    fn apply_request_headers(&self, headers: &mut HeaderMap) -> Result<usize, HookCallError> {
        self.with_inner(|g| {
            g.store.data_mut().pending_headers.clear();
            let on_request = match g
                .instance
                .get_typed_func::<(), i32>(&mut g.store, "panda_on_request")
            {
                Ok(f) => f,
                Err(_) => return Ok(0),
            };
            let rc = on_request.call(&mut g.store, ()).map_err(|e| {
                HookCallError::Runtime(anyhow::anyhow!("panda_on_request call: {e}"))
            })?;
            if let Some(code) = PolicyCode::from_rc(rc) {
                return Err(HookCallError::Policy(code));
            }
            let mut applied = 0usize;
            let pending = std::mem::take(&mut g.store.data_mut().pending_headers);
            for (name_raw, value_raw) in pending {
                let name = HeaderName::from_bytes(name_raw.as_bytes()).map_err(|_| {
                    HookCallError::Runtime(anyhow::anyhow!(
                        "invalid header name from plugin: {name_raw:?}"
                    ))
                })?;
                let value = HeaderValue::from_str(&value_raw).map_err(|_| {
                    HookCallError::Runtime(anyhow::anyhow!("invalid header value from plugin"))
                })?;
                headers.append(name, value);
                applied += 1;
            }
            Ok(applied)
        })
    }

    fn apply_request_body(&self, request_body: &[u8]) -> Result<Option<Vec<u8>>, HookCallError> {
        self.with_inner(|g| {
            g.store.data_mut().pending_body = None;
            let on_request_body = match g
                .instance
                .get_typed_func::<(i32, i32), i32>(&mut g.store, "panda_on_request_body")
            {
                Ok(f) => f,
                Err(_) => return Ok(None),
            };
            let memory = memory_from_store(g).map_err(HookCallError::Runtime)?;
            write_guest_bytes(&mut g.store, &memory, 0, request_body)
                .map_err(HookCallError::Runtime)?;
            let rc = on_request_body
                .call(&mut g.store, (0, request_body.len() as i32))
                .map_err(|e| {
                    HookCallError::Runtime(anyhow::anyhow!("panda_on_request_body call: {e}"))
                })?;
            if let Some(code) = PolicyCode::from_rc(rc) {
                return Err(HookCallError::Policy(code));
            }
            Ok(g.store.data_mut().pending_body.take())
        })
    }

    fn apply_response_chunk(&self, chunk: &[u8]) -> Result<Option<Vec<u8>>, HookCallError> {
        self.with_inner(|g| {
            g.store.data_mut().pending_response_chunk = None;
            let on_response_chunk = match g
                .instance
                .get_typed_func::<(i32, i32), i32>(&mut g.store, "panda_on_response_chunk")
            {
                Ok(f) => f,
                Err(_) => return Ok(None),
            };
            let memory = memory_from_store(g).map_err(HookCallError::Runtime)?;
            write_guest_bytes(&mut g.store, &memory, 0, chunk).map_err(HookCallError::Runtime)?;
            let rc = on_response_chunk
                .call(&mut g.store, (0, chunk.len() as i32))
                .map_err(|e| {
                    HookCallError::Runtime(anyhow::anyhow!("panda_on_response_chunk call: {e}"))
                })?;
            if let Some(code) = PolicyCode::from_rc(rc) {
                return Err(HookCallError::Policy(code));
            }
            Ok(g.store.data_mut().pending_response_chunk.take())
        })
    }
}

/// All plugins for one process (shared across connections).
pub struct PluginRuntime {
    #[allow(dead_code)]
    engine: Engine,
    plugins: Vec<LoadedPlugin>,
    stats: std::sync::Arc<RuntimeStats>,
}

struct RuntimeStats {
    module_instantiate_total: AtomicU64,
    pool_instances_total: AtomicU64,
    pool_acquire_total: AtomicU64,
    pool_contended_total: AtomicU64,
    pool_wait_ns_total: AtomicU64,
}

impl Default for RuntimeStats {
    fn default() -> Self {
        Self {
            module_instantiate_total: AtomicU64::new(0),
            pool_instances_total: AtomicU64::new(0),
            pool_acquire_total: AtomicU64::new(0),
            pool_contended_total: AtomicU64::new(0),
            pool_wait_ns_total: AtomicU64::new(0),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PluginRuntimeStats {
    pub module_instantiate_total: u64,
    pub pool_instances_total: u64,
    pub pool_acquire_total: u64,
    pub pool_contended_total: u64,
    pub pool_wait_ns_total: u64,
}

impl PluginRuntime {
    /// Load `*.wasm` from `dir` if `Some`, else return `None`. Empty directory is allowed (warn only).
    pub fn load_optional(dir: Option<&Path>) -> anyhow::Result<Option<Self>> {
        let Some(dir) = dir else {
            return Ok(None);
        };
        if !dir.is_dir() {
            anyhow::bail!("plugins directory is not a directory: {}", dir.display());
        }
        let engine = Engine::default();
        let stats = std::sync::Arc::new(RuntimeStats::default());
        let pool_size = instance_pool_size_from_env();
        let plugins =
            Self::load_from_directory(&engine, dir, std::sync::Arc::clone(&stats), pool_size)?;
        if plugins.is_empty() {
            eprintln!("panda: no .wasm files in {}", dir.display());
        }
        Ok(Some(Self {
            engine,
            plugins,
            stats,
        }))
    }

    fn load_from_directory(
        engine: &Engine,
        dir: &Path,
        stats: std::sync::Arc<RuntimeStats>,
        pool_size: usize,
    ) -> anyhow::Result<Vec<LoadedPlugin>> {
        let mut out = Vec::new();
        for e in std::fs::read_dir(dir)? {
            let e = e?;
            let path = e.path();
            if path.extension().and_then(|s| s.to_str()) != Some("wasm") {
                continue;
            }
            let name = path
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_else(|| path.display().to_string());
            let module = Module::from_file(engine, &path)
                .map_err(|err| anyhow::anyhow!("{}: {err}", path.display()))?;
            let mut linker: Linker<HostState> = Linker::new(engine);
            linker.func_wrap(
                "panda_host",
                "panda_set_header",
                |mut caller: Caller<'_, HostState>,
                 name_ptr: i32,
                 name_len: i32,
                 value_ptr: i32,
                 value_len: i32| {
                    let Ok(memory) = memory_from_caller(&mut caller) else {
                        return;
                    };
                    let Ok(name) = read_utf8(&caller, &memory, name_ptr, name_len) else {
                        return;
                    };
                    let Ok(value) = read_utf8(&caller, &memory, value_ptr, value_len) else {
                        return;
                    };
                    caller.data_mut().pending_headers.push((name, value));
                },
            )?;
            linker.func_wrap(
                "panda_host",
                "panda_set_body",
                |mut caller: Caller<'_, HostState>, ptr: i32, len: i32| {
                    let Ok(memory) = memory_from_caller(&mut caller) else {
                        return;
                    };
                    let Ok(body) = read_bytes(&caller, &memory, ptr, len) else {
                        return;
                    };
                    caller.data_mut().pending_body = Some(body);
                },
            )?;
            linker.func_wrap(
                "panda_host",
                "panda_set_response_chunk",
                |mut caller: Caller<'_, HostState>, ptr: i32, len: i32| {
                    let Ok(memory) = memory_from_caller(&mut caller) else {
                        return;
                    };
                    let Ok(chunk) = read_bytes(&caller, &memory, ptr, len) else {
                        return;
                    };
                    caller.data_mut().pending_response_chunk = Some(chunk);
                },
            )?;

            let mut inners = Vec::with_capacity(pool_size);
            for _ in 0..pool_size {
                let mut store = Store::new(engine, HostState::default());
                let instance = linker
                    .instantiate(&mut store, &module)
                    .map_err(|err| anyhow::anyhow!("{} instantiate: {err}", path.display()))?;
                validate_plugin_abi(&mut store, &instance, &path)?;
                stats
                    .module_instantiate_total
                    .fetch_add(1, Ordering::Relaxed);
                stats.pool_instances_total.fetch_add(1, Ordering::Relaxed);
                inners.push(Mutex::new(PluginInner { store, instance }));
            }
            out.push(LoadedPlugin {
                name,
                inners,
                next_idx: AtomicUsize::new(0),
                stats: std::sync::Arc::clone(&stats),
            });
        }
        out.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(out)
    }

    /// Call `add` smoke on each plugin that exports it (logs errors, does not fail the gateway).
    pub fn smoke_test(&self) {
        for p in &self.plugins {
            if let Err(e) = p.invoke_add_smoke() {
                eprintln!("panda wasm plugin {} smoke: {e:#}", p.name);
            }
        }
    }

    pub fn plugin_count(&self) -> usize {
        self.plugins.len()
    }

    pub fn plugin_names(&self) -> Vec<String> {
        self.plugins.iter().map(|p| p.name.clone()).collect()
    }

    pub fn stats_snapshot(&self) -> PluginRuntimeStats {
        PluginRuntimeStats {
            module_instantiate_total: self.stats.module_instantiate_total.load(Ordering::Relaxed),
            pool_instances_total: self.stats.pool_instances_total.load(Ordering::Relaxed),
            pool_acquire_total: self.stats.pool_acquire_total.load(Ordering::Relaxed),
            pool_contended_total: self.stats.pool_contended_total.load(Ordering::Relaxed),
            pool_wait_ns_total: self.stats.pool_wait_ns_total.load(Ordering::Relaxed),
        }
    }

    pub fn apply_request_plugins_strict(
        &self,
        headers: &mut HeaderMap,
    ) -> Result<usize, HookFailure> {
        let mut total = 0usize;
        for p in &self.plugins {
            let n = p.apply_request_headers(headers).map_err(|e| match e {
                HookCallError::Policy(code) => HookFailure::PolicyReject {
                    plugin: p.name.clone(),
                    code,
                },
                HookCallError::Runtime(e) => {
                    let message = format!("plugin {} request hook failed: {e:#}", p.name);
                    HookFailure::Runtime {
                        plugin: p.name.clone(),
                        reason: classify_runtime_reason(&message),
                        message,
                    }
                }
            })?;
            total += n;
        }
        Ok(total)
    }

    pub fn apply_request_body_plugins_strict(
        &self,
        request_body: &[u8],
        max_output_bytes: usize,
    ) -> Result<Option<Vec<u8>>, HookFailure> {
        let mut current: Option<Vec<u8>> = None;
        for p in &self.plugins {
            let input = current.as_deref().unwrap_or(request_body);
            match p.apply_request_body(input) {
                Ok(Some(next)) => {
                    if next.len() > max_output_bytes {
                        let message = format!(
                            "plugin {} body hook output too large: {} > {}",
                            p.name,
                            next.len(),
                            max_output_bytes
                        );
                        return Err(HookFailure::Runtime {
                            plugin: p.name.clone(),
                            reason: RuntimeReason::InvalidInput,
                            message,
                        });
                    }
                    current = Some(next);
                }
                Ok(None) => {}
                Err(e) => {
                    return Err(match e {
                        HookCallError::Policy(code) => HookFailure::PolicyReject {
                            plugin: p.name.clone(),
                            code,
                        },
                        HookCallError::Runtime(e) => {
                            let message = format!("plugin {} body hook failed: {e:#}", p.name);
                            HookFailure::Runtime {
                                plugin: p.name.clone(),
                                reason: classify_runtime_reason(&message),
                                message,
                            }
                        }
                    });
                }
            }
        }
        Ok(current)
    }

    pub fn apply_response_chunk_plugins_strict(
        &self,
        response_chunk: &[u8],
        max_output_bytes: usize,
    ) -> Result<Option<Vec<u8>>, HookFailure> {
        let mut current: Option<Vec<u8>> = None;
        for p in &self.plugins {
            let input = current.as_deref().unwrap_or(response_chunk);
            match p.apply_response_chunk(input) {
                Ok(Some(next)) => {
                    if next.len() > max_output_bytes {
                        let message = format!(
                            "plugin {} response chunk hook output too large: {} > {}",
                            p.name,
                            next.len(),
                            max_output_bytes
                        );
                        return Err(HookFailure::Runtime {
                            plugin: p.name.clone(),
                            reason: RuntimeReason::InvalidInput,
                            message,
                        });
                    }
                    current = Some(next);
                }
                Ok(None) => {}
                Err(e) => {
                    return Err(match e {
                        HookCallError::Policy(code) => HookFailure::PolicyReject {
                            plugin: p.name.clone(),
                            code,
                        },
                        HookCallError::Runtime(e) => {
                            let message =
                                format!("plugin {} response chunk hook failed: {e:#}", p.name);
                            HookFailure::Runtime {
                                plugin: p.name.clone(),
                                reason: classify_runtime_reason(&message),
                                message,
                            }
                        }
                    });
                }
            }
        }
        Ok(current)
    }
}

fn validate_plugin_abi(
    store: &mut Store<HostState>,
    instance: &Instance,
    path: &Path,
) -> anyhow::Result<()> {
    let f = instance
        .get_typed_func::<(), i32>(&mut *store, "panda_abi_version")
        .map_err(|_| anyhow::anyhow!("{} missing export panda_abi_version", path.display()))?;
    let abi = f.call(&mut *store, ())?;
    if abi != PANDA_WASM_ABI_VERSION as i32 {
        anyhow::bail!(
            "{} ABI mismatch: guest={} host={}",
            path.display(),
            abi,
            PANDA_WASM_ABI_VERSION
        );
    }
    Ok(())
}

fn memory_from_caller(caller: &mut Caller<'_, HostState>) -> anyhow::Result<Memory> {
    match caller.get_export("memory") {
        Some(wasmtime::Extern::Memory(m)) => Ok(m),
        _ => anyhow::bail!("guest missing exported memory"),
    }
}

fn memory_from_store(inner: &mut PluginInner) -> anyhow::Result<Memory> {
    match inner.instance.get_export(&mut inner.store, "memory") {
        Some(wasmtime::Extern::Memory(m)) => Ok(m),
        _ => anyhow::bail!("guest missing exported memory"),
    }
}

fn read_utf8(
    caller: &Caller<'_, HostState>,
    memory: &Memory,
    ptr: i32,
    len: i32,
) -> anyhow::Result<String> {
    if ptr < 0 || len < 0 {
        anyhow::bail!("negative ptr/len from guest");
    }
    let start = ptr as usize;
    let len = len as usize;
    let end = start
        .checked_add(len)
        .ok_or_else(|| anyhow::anyhow!("ptr/len overflow"))?;
    let data = memory.data(caller);
    if end > data.len() {
        anyhow::bail!("ptr/len out of bounds");
    }
    Ok(std::str::from_utf8(&data[start..end])?.to_string())
}

fn read_bytes(
    caller: &Caller<'_, HostState>,
    memory: &Memory,
    ptr: i32,
    len: i32,
) -> anyhow::Result<Vec<u8>> {
    if ptr < 0 || len < 0 {
        anyhow::bail!("negative ptr/len from guest");
    }
    let start = ptr as usize;
    let len = len as usize;
    let end = start
        .checked_add(len)
        .ok_or_else(|| anyhow::anyhow!("ptr/len overflow"))?;
    let data = memory.data(caller);
    if end > data.len() {
        anyhow::bail!("ptr/len out of bounds");
    }
    Ok(data[start..end].to_vec())
}

fn write_guest_bytes(
    store: &mut Store<HostState>,
    memory: &Memory,
    offset: usize,
    data: &[u8],
) -> anyhow::Result<()> {
    let needed = offset
        .checked_add(data.len())
        .ok_or_else(|| anyhow::anyhow!("guest write overflow"))?;
    let page_size = 65_536usize;
    let cur_len = memory.data_size(&*store);
    if needed > cur_len {
        let to_grow = needed - cur_len;
        let pages = to_grow.div_ceil(page_size);
        memory.grow(&mut *store, pages as u64)?;
    }
    memory.write(&mut *store, offset, data)?;
    Ok(())
}

fn instance_pool_size_from_env() -> usize {
    let raw = std::env::var("PANDA_WASM_INSTANCE_POOL_SIZE")
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(4);
    raw.max(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_inline_wat_add() {
        let wasm = wat::parse_str(
            r#"(module (func (export "add") (param i32 i32) (result i32) local.get 0 local.get 1 i32.add))"#,
        )
        .unwrap();
        let engine = Engine::default();
        let module = Module::new(&engine, wasm).unwrap();
        let mut store = Store::new(&engine, ());
        let instance = Instance::new(&mut store, &module, &[]).unwrap();
        let add = instance
            .get_typed_func::<(i32, i32), i32>(&mut store, "add")
            .unwrap();
        assert_eq!(add.call(&mut store, (40, 2)).unwrap(), 42);
    }

    #[test]
    fn request_hook_can_set_header() {
        let wasm = wat::parse_str(
            r#"(module
                (import "panda_host" "panda_set_header"
                    (func $seth (param i32 i32 i32 i32)))
                (memory (export "memory") 1)
                (data (i32.const 0) "x-from-wasm")
                (data (i32.const 32) "enabled")
                (func (export "panda_abi_version") (result i32)
                    i32.const 1)
                (func (export "panda_on_request") (result i32)
                    i32.const 0 i32.const 11 i32.const 32 i32.const 7
                    call $seth
                    i32.const 0)
            )"#,
        )
        .unwrap();
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("demo.wasm");
        std::fs::write(&path, wasm).unwrap();

        let engine = Engine::default();
        let stats = std::sync::Arc::new(RuntimeStats::default());
        let plugins = PluginRuntime::load_from_directory(
            &engine,
            dir.path(),
            std::sync::Arc::clone(&stats),
            1,
        )
        .unwrap();
        assert_eq!(plugins.len(), 1);

        let runtime = PluginRuntime {
            engine,
            plugins,
            stats,
        };
        let mut headers = HeaderMap::new();
        let n = runtime.apply_request_plugins_strict(&mut headers).unwrap();
        assert_eq!(n, 1);
        assert_eq!(
            headers
                .get("x-from-wasm")
                .and_then(|v| v.to_str().ok())
                .unwrap(),
            "enabled"
        );
    }

    #[test]
    fn body_hook_can_replace_body() {
        let wasm = wat::parse_str(
            r#"(module
                (import "panda_host" "panda_set_body"
                    (func $setb (param i32 i32)))
                (memory (export "memory") 1)
                (data (i32.const 0) "original")
                (data (i32.const 32) "sanitized")
                (func (export "panda_abi_version") (result i32)
                    i32.const 1)
                (func (export "panda_on_request_body") (param i32 i32) (result i32)
                    i32.const 32 i32.const 9
                    call $setb
                    i32.const 0)
            )"#,
        )
        .unwrap();
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("body.wasm");
        std::fs::write(&path, wasm).unwrap();

        let engine = Engine::default();
        let stats = std::sync::Arc::new(RuntimeStats::default());
        let plugins = PluginRuntime::load_from_directory(
            &engine,
            dir.path(),
            std::sync::Arc::clone(&stats),
            1,
        )
        .unwrap();
        let runtime = PluginRuntime {
            engine,
            plugins,
            stats,
        };

        let out = runtime
            .apply_request_body_plugins_strict(b"original", 1024)
            .unwrap()
            .expect("plugin body replacement");
        assert_eq!(out, b"sanitized");
    }

    #[test]
    fn response_chunk_hook_can_replace_chunk() {
        let wasm = wat::parse_str(
            r#"(module
                (import "panda_host" "panda_set_response_chunk"
                    (func $setc (param i32 i32)))
                (memory (export "memory") 1)
                (data (i32.const 32) "chunk-safe")
                (func (export "panda_abi_version") (result i32)
                    i32.const 1)
                (func (export "panda_on_response_chunk") (param i32 i32) (result i32)
                    i32.const 32 i32.const 10
                    call $setc
                    i32.const 0)
            )"#,
        )
        .unwrap();
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("resp.wasm");
        std::fs::write(&path, wasm).unwrap();

        let engine = Engine::default();
        let stats = std::sync::Arc::new(RuntimeStats::default());
        let plugins = PluginRuntime::load_from_directory(
            &engine,
            dir.path(),
            std::sync::Arc::clone(&stats),
            1,
        )
        .unwrap();
        let runtime = PluginRuntime {
            engine,
            plugins,
            stats,
        };

        let out = runtime
            .apply_response_chunk_plugins_strict(b"hello", 1024)
            .unwrap()
            .expect("plugin chunk replacement");
        assert_eq!(out, b"chunk-safe");
    }

    #[test]
    fn request_hook_policy_rc_is_classified() {
        let wasm = wat::parse_str(
            r#"(module
                (memory (export "memory") 1)
                (func (export "panda_abi_version") (result i32) i32.const 1)
                (func (export "panda_on_request") (result i32) i32.const 2)
            )"#,
        )
        .unwrap();
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("reject.wasm"), wasm).unwrap();
        let engine = Engine::default();
        let stats = std::sync::Arc::new(RuntimeStats::default());
        let plugins = PluginRuntime::load_from_directory(
            &engine,
            dir.path(),
            std::sync::Arc::clone(&stats),
            1,
        )
        .unwrap();
        let runtime = PluginRuntime {
            engine,
            plugins,
            stats,
        };
        let mut headers = HeaderMap::new();
        let err = runtime
            .apply_request_plugins_strict(&mut headers)
            .unwrap_err();
        match err {
            HookFailure::PolicyReject { code, .. } => {
                assert_eq!(code, PolicyCode::MalformedRequest)
            }
            HookFailure::Runtime { message, .. } => panic!("unexpected runtime error: {message}"),
        }
    }

    #[test]
    fn missing_abi_export_fails_load() {
        let wasm = wat::parse_str(
            r#"(module
                (memory (export "memory") 1)
                (func (export "panda_on_request") (result i32) i32.const 0)
            )"#,
        )
        .unwrap();
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("bad.wasm"), wasm).unwrap();
        let engine = Engine::default();
        let err = PluginRuntime::load_from_directory(
            &engine,
            dir.path(),
            std::sync::Arc::new(RuntimeStats::default()),
            1,
        )
        .err()
        .expect("missing abi should fail");
        assert!(err.to_string().contains("missing export panda_abi_version"));
    }

    #[test]
    fn abi_mismatch_fails_load() {
        let wasm = wat::parse_str(
            r#"(module
                (memory (export "memory") 1)
                (func (export "panda_abi_version") (result i32) i32.const 99)
            )"#,
        )
        .unwrap();
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("mismatch.wasm"), wasm).unwrap();
        let engine = Engine::default();
        let err = PluginRuntime::load_from_directory(
            &engine,
            dir.path(),
            std::sync::Arc::new(RuntimeStats::default()),
            1,
        )
        .err()
        .expect("abi mismatch should fail");
        assert!(err.to_string().contains("ABI mismatch"));
    }
}
