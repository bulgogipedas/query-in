# Release Checklist

Use this checklist before tagging a Query In release or publishing a production image.

## Pre-Release

- Confirm all intended issues are closed or linked to the release PR.
- Confirm the PR has passed Frontend, Rust, and Lighthouse CI.
- Run the frontend production build:

```bash
cd frontend
bun run build
```

- Run Rust checks:

```bash
cargo fmt --all -- --check
cargo test --workspace
cargo clippy --workspace -- -D warnings
```

- Run the WASM release package size check:

```bash
WASM_PACK=/path/to/wasm-pack scripts/check-wasm-package.sh
```

## Container

Build the production image:

```bash
podman build -t query-in:release-candidate -f Containerfile .
```

The container build runs the Vite production bundle step only. Frontend type safety is validated by the CI `Frontend` job before release.

Run it locally:

```bash
podman run --rm -p 8080:8080 query-in:release-candidate
```

Verify:

```bash
curl -fsS http://127.0.0.1:8080/api/health
curl -fsS http://127.0.0.1:8080/
```

## Documentation

- Update `CHANGELOG.md`.
- Confirm `README.md`, `deploy/README.md`, and `docs/performance.md` match the release.
- Confirm no secrets or generated build artifacts are staged.

## Tagging

Use annotated tags:

```bash
git tag -a v0.1.0 -m "Release v0.1.0"
git push origin v0.1.0
```

Pushing a `v*` tag runs the Container Publish workflow. The workflow can also be started manually from GitHub Actions when a maintainer needs to rebuild an image without creating a release tag.
