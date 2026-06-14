FROM rust:1.91 AS wasm-builder

WORKDIR /workspace

RUN rustup target add wasm32-unknown-unknown \
  && cargo install wasm-pack --locked

COPY Cargo.toml Cargo.lock ./
COPY backend/Cargo.toml backend/Cargo.toml
COPY backend/src backend/src
COPY query-engine-wasm/Cargo.toml query-engine-wasm/Cargo.toml
COPY query-engine-wasm query-engine-wasm

RUN wasm-pack build query-engine-wasm --target web --out-dir pkg --release

FROM oven/bun:1 AS frontend-builder

WORKDIR /workspace

COPY frontend/package.json frontend/bun.lock frontend/

WORKDIR /workspace/frontend

RUN bun install --frozen-lockfile --omit optional

WORKDIR /workspace

COPY frontend frontend
COPY --from=wasm-builder /workspace/query-engine-wasm/pkg query-engine-wasm/pkg

RUN cd frontend && bun run build

FROM rust:1.91 AS backend-builder

WORKDIR /workspace

COPY Cargo.toml Cargo.lock ./
COPY backend/Cargo.toml backend/Cargo.toml
COPY query-engine-wasm/Cargo.toml query-engine-wasm/Cargo.toml
COPY backend backend
COPY query-engine-wasm query-engine-wasm

RUN cargo build -p backend --release

FROM caddy:2-alpine AS runtime

RUN apk add --no-cache ca-certificates

COPY --from=frontend-builder /workspace/frontend/dist /srv/query-in
COPY --from=backend-builder /workspace/target/release/backend /usr/local/bin/query-in-backend
COPY deploy/Caddyfile /etc/caddy/Caddyfile
COPY scripts/container-entrypoint.sh /usr/local/bin/query-in-entrypoint

RUN chmod +x /usr/local/bin/query-in-entrypoint

ENV QUERY_IN_BACKEND_ADDR=127.0.0.1:3001
ENV QUERY_IN_SITE_ADDRESS=:8080
ENV RUST_LOG=info

EXPOSE 8080

HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
  CMD wget -q --spider http://127.0.0.1:8080/api/health || exit 1

ENTRYPOINT ["query-in-entrypoint"]
