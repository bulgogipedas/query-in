# Query In

Query In is a browser-based CSV query engine and data engineering portfolio. It is designed as a working product: visitors can upload CSV files, inspect schemas, and run SQL in the browser while their data stays on their device.

> Your data stays in your browser. Your queries run at native speed.

## Stack

- Frontend: Vue 3, Vite, TypeScript, Tailwind CSS v4, Bun
- Backend: Rust, Axum, Tokio
- WASM: Rust, wasm-bindgen, planned Apache DataFusion integration
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
bun install
bun run dev
```

```bash
cargo run -p backend
```

```bash
cargo test --workspace
```

The frontend development server runs on Vite's default port. The backend listens on `127.0.0.1:3001` and exposes `GET /api/health`.

## GitHub Workflow

Every non-trivial change must start with a GitHub issue, use a focused branch, include commits that reference the issue, and be delivered through a pull request. Direct merges to `main` are not part of the default workflow.

See [docs/github-workflow.md](docs/github-workflow.md) for the full workflow.
