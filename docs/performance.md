# Performance Budget

Query In should feel fast on first load while still shipping a Rust WebAssembly query engine.

## Current CI Gate

Pull requests run Lighthouse CI on:

- `/`
- `/query`
- `/projects`

The bootstrap gate is:

- Performance: `0.80` minimum
- Accessibility: `0.90` warning
- Best practices: `0.90` warning
- SEO: `0.90` warning

The `0.80` performance threshold is intentional for the current open-source bootstrap. The long-term target is `0.90+` after a dedicated optimization pass.

## Local Checks

Build the frontend:

```bash
cd frontend
bun run build
```

Run Lighthouse CI when Chrome is available:

```bash
./frontend/node_modules/.bin/lhci autorun --config=lighthouserc.cjs
```

Build and size-check the release WebAssembly package:

```bash
WASM_PACK=/path/to/wasm-pack scripts/check-wasm-package.sh
```

## Budgets

- Keep the generated WASM package below 5 MiB until the engine needs a documented increase.
- Keep the home route free from eager query-engine loading.
- Keep WebGL and other visual enhancements off the first critical path.
- Keep route-level code splitting for query-heavy UI.

## Raising the Lighthouse Gate

Before raising the performance gate to `0.90`, verify:

- The home route has no eager WASM or query editor loading.
- The query route lazy-loads editor and worker code.
- Static assets are cached correctly in the production container.
- The latest Lighthouse reports pass consistently across at least three runs.
