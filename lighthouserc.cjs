module.exports = {
  ci: {
    collect: {
      numberOfRuns: 1,
      startServerCommand: 'cd frontend && bun run preview --host 127.0.0.1 --port 4173',
      startServerReadyPattern: 'Local:',
      startServerReadyTimeout: 30000,
      url: [
        'http://127.0.0.1:4173/',
        'http://127.0.0.1:4173/query',
        'http://127.0.0.1:4173/projects',
        'http://127.0.0.1:4173/about',
      ],
      settings: {
        chromeFlags: '--headless=new --no-sandbox',
        preset: 'desktop',
      },
    },
    assert: {
      assertions: {
        'categories:performance': ['error', { minScore: 0.9 }],
        'categories:accessibility': ['warn', { minScore: 0.9 }],
        'categories:best-practices': ['warn', { minScore: 0.9 }],
        'categories:seo': ['warn', { minScore: 0.9 }],
      },
    },
    upload: {
      target: 'temporary-public-storage',
    },
  },
}
