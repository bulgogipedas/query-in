# Contributing to Query In

Query In is a free open-source project for private, browser-based CSV analysis. Contributions are welcome when they keep the project local-first, understandable, and practical for self-hosted use.

## Development Workflow

1. Open or reference a GitHub issue before starting non-trivial work.
2. Create a focused branch named by purpose, such as `feature/query-history-export` or `infra/podman-deploy-docs`.
3. Keep commits scoped and reference the related issue number.
4. Open a pull request with a summary, validation notes, and risk or rollback notes.
5. Wait for CI to pass before requesting review or merge.

## Local Setup

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

For WebAssembly packaging:

```bash
rustup target add wasm32-unknown-unknown
WASM_PACK=/path/to/wasm-pack scripts/check-wasm-package.sh
```

## Coding Guidelines

- Keep user data local to the browser by default.
- Prefer simple, readable implementation over speculative abstraction.
- Use Bun for frontend package management.
- Use Podman for container workflows.
- Keep UI copy and documentation in English.
- Avoid commercial SaaS, pricing, or sales-led language in project documentation.

## Pull Request Checklist

- The change references an issue.
- Relevant frontend, Rust, or workflow checks were run.
- User-facing copy is clear and open-source friendly.
- Deployment or performance changes update the relevant docs.
- The PR remains small enough to review.
