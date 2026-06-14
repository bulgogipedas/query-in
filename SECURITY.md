# Security Policy

Query In is designed around a local-first privacy model: CSV files should stay in the browser unless a user explicitly exports or deploys their own infrastructure.

## Supported Versions

Security fixes target the current `main` branch. Tagged releases will document supported versions once the project starts publishing stable releases.

## Reporting a Vulnerability

Please do not open public issues for vulnerabilities. Use GitHub private vulnerability reporting when available, or contact the repository owner through GitHub.

Useful reports include:

- Affected commit or release.
- Steps to reproduce.
- Expected and actual behavior.
- Browser, operating system, and deployment mode.
- Any relevant logs with secrets removed.

## Security Expectations

- Uploaded CSV data should be processed in the browser.
- The backend should not receive file contents for query execution.
- Production deployments should use HTTPS.
- Secrets must not be committed to the repository.
- Container images should be rebuilt from current dependencies before public deployment.
