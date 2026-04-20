//! Optional [`grpc.health.v1`] server (overall status via empty service name).
//! Shuts down with the HTTP gateway: [`serve_health`] uses tonic `serve_with_shutdown` and completes
//! when `shutdown` fires (typically after `panda_proxy::run` returns).

use std::net::SocketAddr;

use tonic_health::server::health_reporter;
use tonic_health::ServingStatus;
use tokio::sync::oneshot;

pub async fn serve_health(
    addr: SocketAddr,
    shutdown: oneshot::Receiver<()>,
) -> Result<(), tonic::transport::Error> {
    let (mut reporter, service) = health_reporter();
    reporter
        .set_service_status("", ServingStatus::Serving)
        .await;
    tonic::transport::Server::builder()
        .add_service(service)
        .serve_with_shutdown(addr, async move {
            let _ = shutdown.await;
        })
        .await
}
