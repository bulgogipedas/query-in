# Query In Project Plan

## Vision

Query In is a portfolio project that behaves like a real product. Visitors can upload CSV files, inspect inferred schemas, and run SQL queries directly in the browser. The core promise is privacy and speed: data stays local, while the query engine targets native-like performance through Rust and WebAssembly.

## Product Goals

- Demonstrate data engineering skill through a usable browser experience.
- Keep uploaded data on the visitor device.
- Support SQL-based exploration of CSV files.
- Present backend, frontend, and systems engineering choices clearly.
- Build the repository history through issues and pull requests.

## Core Pages

- Home: product positioning, technical highlights, and portfolio entry points.
- Query: CSV upload, schema preview, SQL editor, results table, export, and charting workflow.
- Projects: data engineering project cards with stack badges and links.
- About: professional profile, skills, experience, and contact links.

## Technical Architecture

- `frontend/`: Vue 3 application built with Vite, TypeScript, Tailwind CSS v4, and Bun.
- `backend/`: Rust Axum service for health checks and future project metadata APIs.
- `query-engine-wasm/`: Rust WebAssembly crate that will host CSV registration, schema inference, and SQL execution.

## Milestones

1. Foundation: monorepo, documentation, frontend routes, backend health endpoint, and WASM placeholder API.
2. WASM engine: DataFusion integration, CSV registration, basic SQL execution, and worker bridge.
3. Query UI: file dropzone, schema viewer, CodeMirror SQL editor, result table, export, and query history.
4. Portfolio polish: black/electric-yellow WebGL or canvas hero, responsive UI, projects page, about page, and SEO metadata.
5. Deployment: Podman-based production build pipeline, static frontend serving, HTTPS, caching, and performance audit.

## Performance Targets

- First Contentful Paint below 1.2 seconds.
- WASM load and initialization below 800 milliseconds.
- Query 100,000 CSV rows in less than 500 milliseconds where practical.
- Lighthouse performance score of 90 or higher.
- Keep JavaScript and WASM payloads small enough for a portfolio visitor experience.

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
