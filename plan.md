# Query In Project Plan

Query In is a browser-based CSV query engine and data engineering portfolio app. The product goal is a working portfolio experience where visitors can upload CSV files, query them with SQL in the browser, and review supporting data engineering projects.

## Stack

- Frontend: Vue 3, Vite, TypeScript, Tailwind CSS v4, Bun
- Backend: Rust, Axum, Tokio
- WASM engine: Rust, wasm-bindgen, planned DataFusion integration
- Development workflow: GitHub issues, focused branches, conventional commits, and pull requests

## Architecture

```text
frontend/           Vue application and browser UI
backend/            Axum API service
query-engine-wasm/  Rust WebAssembly query engine crate
docs/               Supporting product and workflow documentation
```

## Phase 1 — Foundation

- [x] Init Vite + Vue 3 TS in `frontend/` using `bun create vite`
- [x] Install and configure Tailwind CSS v4 with `@theme` design tokens
- [x] Create Cargo workspace with `backend/` and `query-engine-wasm/` crates
- [x] Axum skeleton with `GET /api/health` returning `{ status, version, uptime }`
- [x] Vue Router with 4 routes: `/`, `/query`, `/projects`, `/about`
- [x] Navbar and Footer components
- [x] Docker Compose for local dev

## Phase 2 — WASM Engine

- [x] `query-engine-wasm` crate with wasm-bindgen setup
- [x] DataFusion dependency compiled to `wasm32-unknown-unknown` target
- [x] `QueryEngine::register_csv(name, bytes)` returning inferred schema
- [x] `QueryEngine::execute(sql)` returning `QueryResult` with rows and `elapsed_ms`
- [x] `wasm-pack build` succeeds, output under 5MB
- [x] `query.worker.ts` Web Worker wrapping WASM calls
- [x] `useQueryEngine` composable with Worker message passing
- [x] Integration test: upload sample CSV, run SELECT, verify results

## Phase 3 — Query UI

- [x] `FileDropzone.vue` with drag and drop, multi-file support, and 50MB limit
- [x] `SchemaViewer.vue` with inferred types and null count
- [x] `SqlEditor.vue` with CodeMirror 6 SQL mode and column autocomplete
- [x] `ResultTable.vue` virtualized with `@tanstack/vue-virtual`
- [ ] Export results to CSV and JSON
- [ ] `useQueryHistory` composable using localStorage with max 50 entries
- [x] Query execution time display in milliseconds
- [x] Error state for invalid SQL

## Phase 4 — UI & Pages

- [ ] `WebGLCanvas.vue` animated noise gradient shader using violet and cyan
- [ ] Hero section with WebGL background, headline, and CTA button
- [ ] Projects page fetching from `GET /api/projects`
- [ ] About page with skills matrix
- [ ] Page transitions with Vue `<Transition>`
- [ ] Mobile responsive at 768px breakpoint

## Phase 5 — Deploy

- [ ] Multi-stage Dockerfile: WASM build, frontend build, backend build, runtime
- [ ] Caddy config for reverse proxy and HTTPS
- [ ] GitHub Actions CI: lint and test on every PR
- [ ] GitHub Actions CD: Docker build and push on merge to main
- [ ] Lighthouse CI asserting Performance >= 90

## Engineering Rules

- Use Bun only for frontend package management.
- Use Vue `<script setup>` Composition API only.
- Use Tailwind v4 `@theme`; do not add `tailwind.config.js`.
- Keep Rust formatted with rustfmt and clean under clippy.
- Keep TypeScript strict and avoid `any`.
- Run WASM work in a Web Worker, not on the main thread.
- Commit after each completed goal with a Conventional Commit message that references the GitHub issue.
