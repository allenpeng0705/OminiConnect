# syntax=docker/dockerfile:1.7

FROM rust:1.88-bookworm AS builder
WORKDIR /work

# Prime dependency build cache first.
COPY Cargo.toml Cargo.lock ./
COPY crates ./crates
ARG PANDA_BUILD_FEATURES="mimalloc"
RUN cargo build --release -p panda-server --features "${PANDA_BUILD_FEATURES}"

FROM debian:bookworm-slim
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && groupadd --system --gid 10001 panda \
    && useradd --system --uid 10001 --gid 10001 --home-dir /app --shell /usr/sbin/nologin panda

WORKDIR /app
COPY --from=builder /work/target/release/panda /usr/local/bin/panda
COPY panda.example.yaml /app/panda.yaml
RUN chown -R panda:panda /app
USER panda:panda
ENV PANDA_LISTEN_OVERRIDE=0.0.0.0:8080

EXPOSE 8080
ENTRYPOINT ["/usr/local/bin/panda"]
CMD ["/app/panda.yaml"]
