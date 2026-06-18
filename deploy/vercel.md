# Vercel Deployment

Query In can be deployed to Vercel as a static Vite application with small serverless API compatibility routes. CSV query execution still runs in the browser through the committed WebAssembly package.

## Project Settings

Create the Vercel project from the repository root and keep these settings:

- Framework Preset: Vite
- Root Directory: repository root
- Install Command: `cd frontend && bun install --frozen-lockfile`
- Build Command: `bash scripts/verify-vercel-wasm.sh && cd frontend && bun run build`
- Output Directory: `frontend/dist`

These values are also captured in the root `vercel.json`.

## API Routes

Vercel serves lightweight Node.js functions for:

- `GET /api/health`
- `GET /api/use-cases`
- `GET /api/projects`

The Rust Axum backend remains the container/self-hosting runtime. The Vercel functions mirror the product metadata endpoints needed by the static frontend.

## WASM Package

The generated browser package under `query-engine-wasm/pkg` is committed for Vercel so the build sandbox does not need to compile Rust or install `wasm-pack`.

Before updating the committed package, regenerate and verify it:

```bash
wasm-pack build query-engine-wasm --target web --out-dir pkg --release
bash scripts/verify-vercel-wasm.sh
```

The current budget keeps `query_engine_wasm_bg.wasm` below 5 MiB.

## SPA Routing

The root `vercel.json` rewrites application routes such as `/query` and `/use-cases` to `index.html`, while preserving the `/api/*` serverless routes.
