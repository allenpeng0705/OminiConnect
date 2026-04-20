//! Stream `text/event-stream` responses; count completion tokens (tiktoken + optional `usage`).

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use std::time::Duration;

use bytes::{Buf, BufMut, Bytes, BytesMut};
use hyper::body::{Body, Frame, Incoming};
use panda_wasm::PluginRuntime;
use serde_json::Value;
use tiktoken_rs::CoreBPE;

use crate::shared::tpm::TpmCounters;
use crate::PandaBodyError;

/// Optional tap for assistant-visible text extracted from OpenAI-style SSE `data:` lines.
pub trait LlmStreamTap: Send + Sync {
    fn on_assistant_delta(&self, chunk: &str);
}

/// After the first response body chunk, fail if the upstream stays `Pending` longer than `idle`
/// before sending the next chunk (async workers that accept the connection but stall mid-stream).
/// When `idle` is `None`, forwards to `inner` unchanged.
pub struct UpstreamIdleBetweenChunksBody<B> {
    inner: B,
    idle: Option<Duration>,
    sleep: Option<Pin<Box<tokio::time::Sleep>>>,
    /// True after we have observed at least one `Ready` from `inner` (any frame kind).
    saw_progress: bool,
}

impl<B> UpstreamIdleBetweenChunksBody<B> {
    pub fn new(inner: B, idle: Option<Duration>) -> Self {
        Self {
            inner,
            idle,
            sleep: None,
            saw_progress: false,
        }
    }
}

impl<B> Body for UpstreamIdleBetweenChunksBody<B>
where
    B: Body<Data = Bytes, Error = PandaBodyError> + Unpin,
{
    type Data = Bytes;
    type Error = PandaBodyError;

    fn poll_frame(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>> {
        let this = self.as_mut().get_mut();
        if this.idle.is_none() {
            return Pin::new(&mut this.inner).poll_frame(cx);
        }
        match Pin::new(&mut this.inner).poll_frame(cx) {
            Poll::Ready(item) => {
                this.saw_progress = true;
                this.sleep = None;
                Poll::Ready(item)
            }
            Poll::Pending => {
                if !this.saw_progress {
                    return Poll::Pending;
                }
                if this.sleep.is_none() {
                    let dur = *this.idle.as_ref().unwrap();
                    this.sleep = Some(Box::pin(tokio::time::sleep(dur)));
                }
                match this.sleep.as_mut().unwrap().as_mut().poll(cx) {
                    Poll::Ready(()) => {
                        Poll::Ready(Some(Err(PandaBodyError::Io(std::io::Error::new(
                            std::io::ErrorKind::TimedOut,
                            "upstream SSE idle timeout (no further chunks)",
                        )))))
                    }
                    Poll::Pending => Poll::Pending,
                }
            }
        }
    }
}

/// After response headers, fail the body stream if the upstream sends no bytes before `limit`
/// elapses. When `limit` is `None`, forwards `poll_frame` to `inner` unchanged.
pub struct FirstUpstreamByteTimeoutBody<B> {
    inner: B,
    limit: Option<Duration>,
    sleep: Option<Pin<Box<tokio::time::Sleep>>>,
    passed_first: bool,
}

impl<B> FirstUpstreamByteTimeoutBody<B> {
    pub fn new(inner: B, limit: Option<Duration>) -> Self {
        Self {
            inner,
            limit,
            sleep: None,
            passed_first: false,
        }
    }
}

impl<B> Body for FirstUpstreamByteTimeoutBody<B>
where
    B: Body<Data = Bytes, Error = PandaBodyError> + Unpin,
{
    type Data = Bytes;
    type Error = PandaBodyError;

    fn poll_frame(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>> {
        let this = self.as_mut().get_mut();
        if this.limit.is_none() {
            return Pin::new(&mut this.inner).poll_frame(cx);
        }
        if this.passed_first {
            return Pin::new(&mut this.inner).poll_frame(cx);
        }
        match Pin::new(&mut this.inner).poll_frame(cx) {
            Poll::Ready(Some(Ok(frame))) => {
                this.passed_first = true;
                this.sleep = None;
                Poll::Ready(Some(Ok(frame)))
            }
            Poll::Ready(Some(Err(e))) => {
                this.passed_first = true;
                this.sleep = None;
                Poll::Ready(Some(Err(e)))
            }
            Poll::Ready(None) => {
                this.passed_first = true;
                this.sleep = None;
                Poll::Ready(None)
            }
            Poll::Pending => {
                if this.sleep.is_none() {
                    let dur = *this.limit.as_ref().unwrap();
                    this.sleep = Some(Box::pin(tokio::time::sleep(dur)));
                }
                match this.sleep.as_mut().unwrap().as_mut().poll(cx) {
                    Poll::Ready(()) => {
                        Poll::Ready(Some(Err(PandaBodyError::Io(std::io::Error::new(
                            std::io::ErrorKind::TimedOut,
                            "upstream first response byte timeout",
                        )))))
                    }
                    Poll::Pending => Poll::Pending,
                }
            }
        }
    }
}

pub struct PrefixedBody<B = Incoming> {
    prefix: Option<Bytes>,
    inner: B,
}

impl<B> PrefixedBody<B>
where
    B: Body<Data = Bytes, Error = PandaBodyError> + Unpin,
{
    pub fn new(prefix: Bytes, inner: B) -> Self {
        Self {
            prefix: if prefix.is_empty() {
                None
            } else {
                Some(prefix)
            },
            inner,
        }
    }
}

pub struct SseCountingBody<B = Incoming> {
    inner: B,
    buf: BytesMut,
    bucket: String,
    tpm: Arc<TpmCounters>,
    bpe: Option<Arc<CoreBPE>>,
    llm_tap: Option<Arc<dyn LlmStreamTap>>,
    usage_completion: Option<u64>,
    delta_tokens: u64,
    finished: bool,
}

pub struct WasmChunkHookBody<B = Incoming> {
    inner: B,
    runtime: Arc<PluginRuntime>,
    max_output_bytes: usize,
}

impl<B> SseCountingBody<B>
where
    B: Body<Data = Bytes, Error = PandaBodyError> + Unpin,
{
    pub fn new(
        inner: B,
        tpm: Arc<TpmCounters>,
        bucket: String,
        bpe: Option<Arc<CoreBPE>>,
        llm_tap: Option<Arc<dyn LlmStreamTap>>,
    ) -> Self {
        Self {
            inner,
            buf: BytesMut::new(),
            bucket,
            tpm,
            bpe,
            llm_tap,
            usage_completion: None,
            delta_tokens: 0,
            finished: false,
        }
    }

    fn process_buffer(&mut self) {
        while let Some(pos) = self.buf.iter().position(|&b| b == b'\n') {
            let mut line = self.buf.split_to(pos + 1);
            if line.last() == Some(&b'\n') {
                line.truncate(line.len().saturating_sub(1));
            }
            if line.last() == Some(&b'\r') {
                line.truncate(line.len().saturating_sub(1));
            }
            if !line.is_empty() {
                self.process_sse_line(&line[..]);
            }
        }
    }

    fn process_sse_line(&mut self, line: &[u8]) {
        let rest = if line.len() >= 5 && &line[..5] == b"data:" {
            trim_bytes(&line[5..])
        } else {
            return;
        };
        if rest == b"[DONE]" {
            return;
        }
        let Ok(v) = serde_json::from_slice::<Value>(rest) else {
            return;
        };
        if let Some(u) = v
            .get("usage")
            .and_then(|u| u.get("completion_tokens"))
            .and_then(|x| x.as_u64())
        {
            self.usage_completion = Some(u);
            return;
        }
        if let Some(arr) = v.get("choices").and_then(|c| c.as_array()) {
            for ch in arr {
                if let Some(s) = ch
                    .get("delta")
                    .and_then(|d| d.get("content"))
                    .and_then(|c| c.as_str())
                {
                    let n = self
                        .bpe
                        .as_ref()
                        .map(|b| b.encode_with_special_tokens(s).len() as u64)
                        .unwrap_or_else(|| s.len() as u64);
                    self.delta_tokens = self.delta_tokens.saturating_add(n);
                    if let Some(tap) = &self.llm_tap {
                        tap.on_assistant_delta(s);
                    }
                }
            }
        }
    }

    fn completion_total(&self) -> u64 {
        self.usage_completion.unwrap_or(self.delta_tokens)
    }

    fn spawn_completion_flush(&self, n: u64) {
        if n == 0 {
            return;
        }
        let tpm = self.tpm.clone();
        let bucket = self.bucket.clone();
        tokio::spawn(async move {
            tpm.add_completion_tokens(&bucket, n).await;
        });
    }
}

impl<B> WasmChunkHookBody<B>
where
    B: Body<Data = Bytes, Error = PandaBodyError> + Unpin,
{
    pub fn new(inner: B, runtime: Arc<PluginRuntime>, max_output_bytes: usize) -> Self {
        Self {
            inner,
            runtime,
            max_output_bytes,
        }
    }
}

fn trim_bytes(b: &[u8]) -> &[u8] {
    let mut s = b;
    while s.first().is_some_and(|x| x.is_ascii_whitespace()) {
        s = &s[1..];
    }
    while s.last().is_some_and(|x| x.is_ascii_whitespace()) {
        s = &s[..s.len() - 1];
    }
    s
}

impl<B> Body for SseCountingBody<B>
where
    B: Body<Data = Bytes, Error = PandaBodyError> + Unpin,
{
    type Data = Bytes;
    type Error = PandaBodyError;

    fn poll_frame(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>> {
        let this = self.as_mut().get_mut();
        if this.finished {
            return Poll::Ready(None);
        }

        let polled = Pin::new(&mut this.inner).poll_frame(cx);
        match polled {
            Poll::Pending => Poll::Pending,
            Poll::Ready(Some(Err(e))) => Poll::Ready(Some(Err(e))),
            Poll::Ready(None) => {
                this.finished = true;
                if !this.buf.is_empty() {
                    let tail = this.buf.clone().freeze();
                    this.buf.clear();
                    if !tail.is_empty() {
                        this.process_sse_line(&tail[..]);
                    }
                }
                let n = this.completion_total();
                this.spawn_completion_flush(n);
                Poll::Ready(None)
            }
            Poll::Ready(Some(Ok(frame))) => match frame.into_data() {
                Ok(mut data) => {
                    let data = data.copy_to_bytes(data.remaining());
                    this.buf.put_slice(&data);
                    this.process_buffer();
                    Poll::Ready(Some(Ok(Frame::data(data))))
                }
                Err(frame) => Poll::Ready(Some(Ok(frame))),
            },
        }
    }
}

impl<B> Body for PrefixedBody<B>
where
    B: Body<Data = Bytes, Error = PandaBodyError> + Unpin,
{
    type Data = Bytes;
    type Error = PandaBodyError;

    fn poll_frame(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>> {
        let this = self.as_mut().get_mut();
        if let Some(p) = this.prefix.take() {
            return Poll::Ready(Some(Ok(Frame::data(p))));
        }
        Pin::new(&mut this.inner).poll_frame(cx)
    }
}

impl<B> Body for WasmChunkHookBody<B>
where
    B: Body<Data = Bytes, Error = PandaBodyError> + Unpin,
{
    type Data = Bytes;
    type Error = PandaBodyError;

    fn poll_frame(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>> {
        let this = self.as_mut().get_mut();
        match Pin::new(&mut this.inner).poll_frame(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(Some(Ok(frame))) => match frame.into_data() {
                Ok(mut data) => {
                    let data = data.copy_to_bytes(data.remaining());
                    match this
                        .runtime
                        .apply_response_chunk_plugins_strict(&data, this.max_output_bytes)
                    {
                        Ok(Some(next)) => Poll::Ready(Some(Ok(Frame::data(Bytes::from(next))))),
                        Ok(None) => Poll::Ready(Some(Ok(Frame::data(data)))),
                        Err(e) => {
                            eprintln!("panda: wasm response chunk hook fail-open: {e}");
                            Poll::Ready(Some(Ok(Frame::data(data))))
                        }
                    }
                }
                Err(frame) => Poll::Ready(Some(Ok(frame))),
            },
            other => other,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use http_body_util::BodyExt;

    struct NoopBody;

    impl Body for NoopBody {
        type Data = Bytes;
        type Error = PandaBodyError;

        fn poll_frame(
            self: Pin<&mut Self>,
            _cx: &mut Context<'_>,
        ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>> {
            let _ = self;
            Poll::Ready(None)
        }
    }

    struct HangingBody;

    impl Body for HangingBody {
        type Data = Bytes;
        type Error = PandaBodyError;

        fn poll_frame(
            self: Pin<&mut Self>,
            _cx: &mut Context<'_>,
        ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>> {
            let _ = self;
            Poll::Pending
        }
    }

    struct OneThenHangBody {
        sent: bool,
    }

    impl Body for OneThenHangBody {
        type Data = Bytes;
        type Error = PandaBodyError;

        fn poll_frame(
            mut self: Pin<&mut Self>,
            _cx: &mut Context<'_>,
        ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>> {
            let this = self.as_mut().get_mut();
            if !this.sent {
                this.sent = true;
                Poll::Ready(Some(Ok(Frame::data(Bytes::from_static(b"x")))))
            } else {
                Poll::Pending
            }
        }
    }

    #[tokio::test]
    async fn idle_between_chunks_times_out_after_first_frame() {
        let inner = FirstUpstreamByteTimeoutBody::new(OneThenHangBody { sent: false }, None);
        let mut body = UpstreamIdleBetweenChunksBody::new(inner, Some(Duration::from_millis(30)));
        assert!(body.frame().await.unwrap().is_ok());
        match body.frame().await {
            Some(Err(PandaBodyError::Io(e))) => {
                assert_eq!(e.kind(), std::io::ErrorKind::TimedOut);
            }
            other => panic!("expected idle Io timeout, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn first_upstream_byte_timeout_returns_timed_out_io() {
        let mut body =
            FirstUpstreamByteTimeoutBody::new(HangingBody, Some(Duration::from_millis(25)));
        match body.frame().await {
            Some(Err(PandaBodyError::Io(e))) => {
                assert_eq!(e.kind(), std::io::ErrorKind::TimedOut);
            }
            other => panic!("expected Io timeout error, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn sse_counting_buffers_fragmented_data_lines() {
        let tpm = Arc::new(TpmCounters::connect(None).await.unwrap());
        let bpe = Arc::new(tiktoken_rs::cl100k_base().unwrap());
        let mut body = SseCountingBody::new(NoopBody, tpm, "b1".to_string(), Some(bpe), None);

        body.buf
            .put_slice(br#"data: {"choices":[{"delta":{"content":"hel"#);
        body.process_buffer();
        assert_eq!(body.delta_tokens, 0, "must not parse incomplete line");

        body.buf.put_slice(
            br#"lo"}}]}
"#,
        );
        body.process_buffer();
        assert!(body.delta_tokens > 0, "must parse once full line arrives");
    }

    #[tokio::test]
    async fn sse_counting_handles_utf8_multibyte_split_across_chunks() {
        let tpm = Arc::new(TpmCounters::connect(None).await.unwrap());
        let bpe = Arc::new(tiktoken_rs::cl100k_base().unwrap());
        let mut body = SseCountingBody::new(NoopBody, tpm, "b2".to_string(), Some(bpe), None);

        let mut part1 = br#"data: {"choices":[{"delta":{"content":""#.to_vec();
        part1.extend_from_slice(&[0xF0]); // first byte of 😀
        body.buf.put_slice(&part1);
        body.process_buffer();
        assert_eq!(
            body.delta_tokens, 0,
            "must not parse incomplete utf8/json line"
        );

        let mut part2 = vec![0x9F, 0x98, 0x80]; // remaining bytes of 😀
        part2.extend_from_slice(
            br#""}}]}
"#,
        );
        body.buf.put_slice(&part2);
        body.process_buffer();
        assert!(
            body.delta_tokens > 0,
            "must parse once utf8 sequence is complete"
        );
    }
}
