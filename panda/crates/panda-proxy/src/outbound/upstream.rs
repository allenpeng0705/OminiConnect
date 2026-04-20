//! Join client URIs with configured upstream base and filter hop-by-hop headers.

use http::header::{self, HeaderMap, HeaderName};
use hyper::Uri;

/// Append the request path and query to `upstream_base` (no trailing slash).
pub fn join_upstream_uri(upstream_base: &str, req_uri: &Uri) -> anyhow::Result<Uri> {
    let base = upstream_base.trim_end_matches('/');
    let suffix = req_uri
        .path_and_query()
        .map(|pq| pq.as_str())
        .unwrap_or("/");
    let joined = format!("{base}{suffix}");
    joined
        .parse()
        .map_err(|e| anyhow::anyhow!("invalid joined upstream URI: {e}"))
}

/// Replace the path on `uri`, preserving the query string if present (`new_path` is e.g. `/v1/messages`).
pub fn rewrite_uri_path_preserving_query(uri: &Uri, new_path: &str) -> anyhow::Result<Uri> {
    let mut parts = uri.clone().into_parts();
    let pq = parts
        .path_and_query
        .as_ref()
        .and_then(|pq| pq.query())
        .map(|q| format!("{new_path}?{q}"))
        .unwrap_or_else(|| new_path.to_string());
    parts.path_and_query = Some(
        pq.parse()
            .map_err(|e| anyhow::anyhow!("path_and_query: {e}"))?,
    );
    Uri::from_parts(parts).map_err(|e| anyhow::anyhow!("uri parts: {e}"))
}

/// Request headers we never forward (hop-by-hop or set from target URI).
pub fn filter_request_headers(src: &HeaderMap, dst: &mut HeaderMap) {
    for (name, value) in src.iter() {
        if is_hop_by_hop_request(name) {
            continue;
        }
        if name == header::HOST {
            continue;
        }
        dst.append(name.clone(), value.clone());
    }
}

/// Strip hop-by-hop and framing headers from upstream response; Hyper sets framing for the client.
pub fn filter_response_headers(src: &HeaderMap, dst: &mut HeaderMap) {
    for (name, value) in src.iter() {
        if is_hop_by_hop_response(name) {
            continue;
        }
        if is_provider_internal_response_header(name) {
            continue;
        }
        dst.append(name.clone(), value.clone());
    }
}

fn is_hop_by_hop_request(name: &HeaderName) -> bool {
    matches!(
        name.as_str(),
        "connection"
            | "keep-alive"
            | "proxy-authenticate"
            | "proxy-authorization"
            | "te"
            | "trailer"
            | "transfer-encoding"
            | "upgrade"
    )
}

fn is_hop_by_hop_response(name: &HeaderName) -> bool {
    matches!(
        name.as_str(),
        "connection"
            | "keep-alive"
            | "proxy-authenticate"
            | "proxy-authorization"
            | "te"
            | "trailer"
            | "transfer-encoding"
            | "upgrade"
    ) || name == header::CONTENT_LENGTH
}

/// Strip provider/internal debug headers so clients only see normalized gateway headers.
fn is_provider_internal_response_header(name: &HeaderName) -> bool {
    let n = name.as_str().to_ascii_lowercase();
    n == "openai-organization"
        || n == "x-request-id"
        || n == "server"
        || n.starts_with("x-openai-")
        || n.starts_with("x-anthropic-")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn join_preserves_path_and_query() {
        let u = join_upstream_uri(
            "http://127.0.0.1:11434",
            &"/v1/chat/completions?x=1".parse().unwrap(),
        )
        .unwrap();
        assert_eq!(
            u.to_string(),
            "http://127.0.0.1:11434/v1/chat/completions?x=1"
        );
    }

    #[test]
    fn join_strips_trailing_slash_on_base() {
        let u = join_upstream_uri(
            "https://api.openai.com/v1/",
            &"/chat/completions".parse().unwrap(),
        )
        .unwrap();
        assert_eq!(u.to_string(), "https://api.openai.com/v1/chat/completions");
    }

    #[test]
    fn rewrite_uri_path_preserving_query_keeps_query() {
        let u = "https://api.example/v1/chat/completions?x=1"
            .parse::<Uri>()
            .unwrap();
        let out = super::rewrite_uri_path_preserving_query(&u, "/v1/messages").unwrap();
        assert_eq!(out.path(), "/v1/messages");
        assert_eq!(out.query(), Some("x=1"));
    }

    #[test]
    fn filter_response_headers_strips_hop_by_hop_and_provider_internal_headers() {
        let mut src = HeaderMap::new();
        src.insert("content-type", "application/json".parse().unwrap());
        src.insert("transfer-encoding", "chunked".parse().unwrap());
        src.insert("x-request-id", "upstream-id".parse().unwrap());
        src.insert("x-openai-processing-ms", "123".parse().unwrap());
        src.insert("openai-organization", "org1".parse().unwrap());

        let mut dst = HeaderMap::new();
        filter_response_headers(&src, &mut dst);

        assert!(dst.get("content-type").is_some());
        assert!(dst.get("transfer-encoding").is_none());
        assert!(dst.get("x-request-id").is_none());
        assert!(dst.get("x-openai-processing-ms").is_none());
        assert!(dst.get("openai-organization").is_none());
    }
}
