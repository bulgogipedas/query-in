# Query In Project Plan

## Vision

Query In is a free open-source browser-based CSV query workspace for private, fast operational analysis. Users can upload CSV files, inspect inferred schemas, and run SQL queries directly in the browser. The core promise is privacy and speed: data stays local, while the query engine targets native-like performance through Rust and WebAssembly.

## Product Goals

- Give teams a usable local-first workflow for CSV analysis.
- Keep uploaded data on the user device.
- Support SQL-based exploration of CSV files.
- Present infrastructure, runtime, and systems choices clearly for technical users and contributors.
- Build the repository history through issues and pull requests.

## Core Pages

- Home: product positioning, technical highlights, and clear paths into the app.
- Query: CSV upload, schema preview, SQL editor, results table, export, and charting workflow.
- Use Cases: operational scenarios, data workflows, stack notes, and product outcomes.

## Technical Architecture

- `frontend/`: Vue 3 application built with Vite, TypeScript, Tailwind CSS v4, and Bun.
- `backend/`: Rust Axum service for health checks and product metadata APIs.
- `query-engine-wasm/`: Rust WebAssembly crate that will host CSV registration, schema inference, and SQL execution.

## Milestones

1. Foundation: monorepo, documentation, frontend routes, backend health endpoint, and WASM placeholder API.
2. WASM engine: DataFusion integration, CSV registration, basic SQL execution, and worker bridge.
3. Query UI: file dropzone, schema viewer, CodeMirror SQL editor, result table, export, and query history.
4. Product polish: black/electric-yellow WebGL or canvas hero, responsive UI, use case page, open-source product copy, and SEO metadata.
5. Deployment: Podman-based production build pipeline, static frontend serving, HTTPS, caching, and performance audit.

## Performance Targets

- First Contentful Paint below 1.2 seconds.
- WASM load and initialization below 800 milliseconds.
- Query 100,000 CSV rows in less than 500 milliseconds where practical.
- Lighthouse performance score of 90 or higher after a dedicated performance pass.
- Keep JavaScript and WASM payloads small enough for a fast first-run product experience.

## Risks

- DataFusion may produce a large WASM binary.
- Async query execution may need browser-specific handling.
- Large CSV files can overload browser memory without limits.
- WebGL visual polish must stay aligned with the black canvas and electric-yellow product UI direction and needs a graceful fallback.

## Mitigations

- Start with a narrow SQL path and measure bundle size early.
- Run query work in Web Workers.
- Apply client-side file size limits and batched result rendering.
- Provide a non-WebGL visual fallback for unsupported browsers.
- Use Containerfile, Podman, and Podman Compose for container workflows instead of Docker tooling.
