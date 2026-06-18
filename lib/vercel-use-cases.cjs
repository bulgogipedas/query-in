const useCases = [
  {
    name: 'Operations Review',
    slug: 'query-in',
    summary:
      'Inspect exports from billing, support, product, or CRM tools without uploading sensitive files into another SaaS system.',
    status: 'Ready',
    stack: ['Private CSVs', 'SQL', 'Schema inference', 'Export'],
  },
  {
    name: 'Pre-Warehouse Triage',
    slug: 'local-query-engine',
    summary:
      'Validate file shape, inspect nulls, and answer quick questions before deciding whether data belongs in the warehouse.',
    status: 'Ready',
    stack: ['Schema checks', 'Local compute', 'Query history', 'Results table'],
  },
]

module.exports = {
  useCases,
}
