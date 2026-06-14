# Podman Deployment Runbook

This runbook covers self-hosting Query In with Podman. Query In is free open-source software, and the default deployment keeps the browser UI and API in one container.

## Image Build

From the repository root:

```bash
podman build -t query-in:local -f Containerfile .
```

The image builds:

- The Rust WebAssembly package with `wasm-pack`.
- The Vite frontend with Bun.
- The Axum backend release binary.
- A Caddy runtime serving static assets and proxying `/api/*`.

## Local Production Run

```bash
podman run --rm \
  -p 8080:8080 \
  -e QUERY_IN_SITE_ADDRESS=:8080 \
  query-in:local
```

Verify the deployment:

```bash
curl -fsS http://127.0.0.1:8080/api/health
curl -fsS http://127.0.0.1:8080/
```

## Podman Compose

Copy the example environment file and adjust values as needed:

```bash
cp .env.example .env
podman compose -f deploy/compose.production.yml up --build
```

## Public HTTPS Deployment

Point DNS at the host and set `QUERY_IN_SITE_ADDRESS` to the public domain:

```bash
podman run -d \
  --name query-in \
  --restart unless-stopped \
  -p 80:80 \
  -p 443:443 \
  -e QUERY_IN_SITE_ADDRESS=query-in.example.com \
  ghcr.io/bulgogipedas/query-in:latest
```

Caddy manages HTTPS automatically when the container can bind ports `80` and `443` and the domain resolves to the host.

## Health Checks

The runtime container exposes `GET /api/health` through Caddy. Use it for uptime checks:

```bash
curl -fsS https://query-in.example.com/api/health
```

The container image also includes a local health check against `http://127.0.0.1:8080/api/health`.

## Caching

Caddy applies:

- Long-lived immutable caching for fingerprinted frontend assets under `/assets/*`.
- Short caching for application shell routes.
- `no-store` for API responses.

This keeps repeat loads fast without caching API status responses.

## Rollback

Use the immutable commit SHA image tag published to GitHub Container Registry:

```bash
podman pull ghcr.io/bulgogipedas/query-in:<commit-sha>
podman stop query-in
podman rm query-in
podman run -d \
  --name query-in \
  --restart unless-stopped \
  -p 80:80 \
  -p 443:443 \
  -e QUERY_IN_SITE_ADDRESS=query-in.example.com \
  ghcr.io/bulgogipedas/query-in:<commit-sha>
```

## Operational Notes

- Do not mount user CSV files into the container; CSV analysis runs in the browser.
- Keep secrets out of `.env` unless they are needed for future deployment integrations.
- Rebuild images regularly to pick up base image updates.
