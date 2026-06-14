# Query In

Query In is a browser-based CSV query workspace for teams that need fast, private analysis without moving files into a warehouse first. Users can upload CSV files, inspect schemas, and run SQL in the browser while their data stays on their device.

> Your data stays in your browser. Your queries run at native speed.

## Stack

- Frontend: Vue 3, Vite, TypeScript, Tailwind CSS v4, Bun
- Backend: Rust, Axum, Tokio
- WASM: Rust, wasm-bindgen, Apache DataFusion 53.1.0 integration
- Containers: Podman, Podman Compose, and future production Containerfile
- Workflow: GitHub issues, focused branches, reviewed pull requests

## Repository Layout

```text
frontend/           Vue application and browser UI
backend/            Axum API service
query-engine-wasm/  Rust WebAssembly query engine crate
docs/               Product plan and engineering workflow
```

## Local Development

```bash
cd frontend
bun install --omit optional
bun run dev
```

```bash
cargo run -p backend
```

```bash
cargo test --workspace
```

The frontend development server runs on Vite's default port. The backend listens on `127.0.0.1:3001` by default and exposes `GET /api/health`.

Verify the WebAssembly crate setup with:

```bash
rustup target add wasm32-unknown-unknown
cargo check -p query-engine-wasm --target wasm32-unknown-unknown
```

The WASM crate currently pins DataFusion to 53.1.0 with a minimal SQL feature set and enables `getrandom`'s `wasm_js` feature for browser-compatible `wasm32-unknown-unknown` builds.

Build and size-check the browser package with:

```bash
WASM_PACK=/path/to/wasm-pack scripts/check-wasm-package.sh
```

The generated `query-engine-wasm/pkg/` directory is ignored by Git and must stay below 5MB for the current browser delivery budget.

You can also run both services through Podman Compose:

```bash
podman compose up
```

Production container work uses `Containerfile` and Podman commands. Docker and Docker Compose are not part of the project workflow.

```bash
podman build -t query-in:local -f Containerfile .
podman run --rm -p 8080:8080 query-in:local
```

The runtime image starts the Axum API on `127.0.0.1:3001` and serves the built frontend through Caddy on `:8080`. Set `QUERY_IN_SITE_ADDRESS` to a real production domain, for example `query-in.example.com`, when deploying behind public DNS so Caddy can manage HTTPS automatically.

## GitHub Workflow

Every non-trivial change must start with a GitHub issue, use a focused branch, include commits that reference the issue, and be delivered through a pull request. Direct merges to `main` are not part of the default workflow.

See [docs/github-workflow.md](docs/github-workflow.md) for the full workflow.
