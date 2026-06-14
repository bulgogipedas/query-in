# Query In Frontend

Vue 3, Vite, TypeScript, and Tailwind CSS v4 power the Query In browser interface.

## Commands

```bash
bun install --omit optional
bun run dev
bun run build
```

The frontend currently provides the home, query workspace, and use case pages. Follow-up issues will deepen the WebAssembly worker bridge, SQL editor, schema viewer, result table, and product polish.

The project aliases Rollup to `@rollup/wasm-node`, omits optional native packages for local macOS installs, and removes `fsevents` in frontend scripts because those native packages can hang on this local macOS/Node 24 toolchain.
