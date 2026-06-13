<script setup lang="ts">
const skillGroups = [
  {
    title: 'Browser-Native Processing',
    summary: 'Query In uses a local execution model so teams can analyze CSV files without uploading raw data to a remote service.',
    skills: ['Local execution', 'WebAssembly', 'Web Workers', 'Fast startup', 'Large files'],
  },
  {
    title: 'SQL Analysis Flow',
    summary: 'The workspace is designed for analysts and operators who need schema visibility, repeatable queries, and exportable answers.',
    skills: ['CSV parsing', 'Schema inference', 'SQL editor', 'Query history', 'CSV and JSON export'],
  },
  {
    title: 'Privacy Controls',
    summary: 'Sensitive files remain on the device by default, reducing the risk and friction of quick data inspection.',
    skills: ['No file uploads', 'Device-local data', 'Clear limits', 'Explicit exports', 'Private sessions'],
  },
  {
    title: 'Deployment Ready',
    summary: 'The product is packaged for teams that want a simple hosted entry point backed by observable API health checks.',
    skills: ['Podman runtime', 'Caddy proxy', 'Health endpoint', 'CI checks', 'Performance gate'],
  },
]
</script>

<template>
  <section class="page-shell">
    <div class="page-heading">
      <p class="eyebrow">Platform</p>
      <h1>Private CSV analysis for teams that need answers now.</h1>
      <p>
        Query In gives analysts, founders, finance teams, and operators a secure workspace for ad hoc CSV exploration without forcing every question through a warehouse or shared spreadsheet.
      </p>
    </div>

    <div class="grid gap-4 lg:grid-cols-2">
      <article v-for="group in skillGroups" :key="group.title" class="feature-card">
        <div class="flex items-start justify-between gap-4">
          <h2 class="font-display text-xl font-bold text-white">{{ group.title }}</h2>
          <span class="font-mono text-sm text-[#faff69]">{{ group.skills.length }}</span>
        </div>
        <p class="mt-3 text-sm leading-6 text-[#cccccc]">{{ group.summary }}</p>
        <div class="mt-5 flex flex-wrap gap-2">
          <span v-for="skill in group.skills" :key="skill" class="badge">{{ skill }}</span>
        </div>
      </article>
    </div>
  </section>
</template>
