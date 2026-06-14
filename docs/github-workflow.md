# GitHub Workflow

Query In uses an issue-first, pull-request-driven workflow so product and engineering changes stay reviewable and professional.

## Rules

- Create or reference a GitHub issue before starting product, bug, or engineering work.
- Use focused branches named by purpose, such as `bootstrap/monorepo-foundation`, `feature/query-engine-ui`, or `infra/wasm-build-pipeline`.
- Reference the related issue in every commit message.
- Open a pull request for every branch before merge.
- Keep pull requests small, scoped, and reviewable.
- Include a concise PR summary, validation notes, and risk or rollback notes when relevant.
- Do not merge directly into `main` unless explicitly requested.

## Bootstrap Tracking

The initial bootstrap work is tracked by Issue #1: `Bootstrap Query In monorepo foundation`.

The initial branch is `bootstrap/monorepo-foundation`.

The bootstrap PR should include:

- Monorepo scaffold
- Frontend route shells
- Backend health API
- WASM placeholder API
- English product and workflow documentation
