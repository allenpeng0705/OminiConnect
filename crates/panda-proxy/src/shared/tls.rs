//! TLS acceptor (optional client CA = mTLS).

use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::Arc;

use anyhow::Context;
use panda_config::TlsListenConfig;
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use rustls::server::WebPkiClientVerifier;
use rustls::RootCertStore;
use rustls::ServerConfig;
use rustls_pemfile::{certs, private_key};

pub fn server_config(tls: &TlsListenConfig) -> anyhow::Result<Arc<ServerConfig>> {
    let certs = load_certs(Path::new(&tls.cert_pem))?;
    let key = load_private_key(Path::new(&tls.key_pem))?;

    let config = if let Some(ref ca_path) = tls.client_ca_pem {
        let ca_certs = load_certs(Path::new(ca_path))?;
        let mut roots = RootCertStore::empty();
        for c in ca_certs {
            roots.add(c).context("add client CA to root store")?;
        }
        let verifier = WebPkiClientVerifier::builder(Arc::new(roots)).build()?;
        ServerConfig::builder()
            .with_client_cert_verifier(verifier)
            .with_single_cert(certs, key)
            .context("rustls server config (mTLS)")?
    } else {
        ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(certs, key)
            .context("rustls server config")?
    };

    Ok(Arc::new(config))
}

fn load_certs(path: &Path) -> anyhow::Result<Vec<CertificateDer<'static>>> {
    let mut reader = BufReader::new(File::open(path)?);
    let certs: Vec<CertificateDer<'static>> = certs(&mut reader)
        .filter_map(|r| r.ok())
        .map(CertificateDer::from)
        .collect();
    anyhow::ensure!(!certs.is_empty(), "no certificates in {}", path.display());
    Ok(certs)
}

fn load_private_key(path: &Path) -> anyhow::Result<PrivateKeyDer<'static>> {
    let mut reader = BufReader::new(File::open(path)?);
    private_key(&mut reader)
        .transpose()
        .context("single private key PEM")?
        .context("no private key in PEM")
}
