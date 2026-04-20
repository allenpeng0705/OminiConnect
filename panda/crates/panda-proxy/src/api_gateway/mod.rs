//! Built-in API gateway (ingress / egress). Phase A: flags; Phase B: egress; Phase C: ingress path routing.

pub mod control_plane_store;
pub mod egress;
pub mod ingress;

use panda_config::PandaConfig;

/// Snapshot of API gateway feature flags for [`crate::ProxyState`].
#[derive(Debug, Clone, Copy, Default)]
pub struct ApiGatewayState {
    pub ingress_enabled: bool,
    pub egress_enabled: bool,
}

impl ApiGatewayState {
    pub fn from_config(cfg: &PandaConfig) -> Self {
        Self {
            ingress_enabled: cfg.api_gateway.ingress.enabled,
            egress_enabled: cfg.api_gateway.egress.enabled,
        }
    }
}
