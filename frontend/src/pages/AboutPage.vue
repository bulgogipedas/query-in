<script setup lang="ts">
const skillGroups = [
  {
    title: 'Systems Engineering',
    summary: 'Backend and runtime work focused on clear boundaries, observable APIs, and practical performance.',
    skills: ['Rust', 'Axum', 'Tokio', 'WebAssembly', 'Web Workers'],
  },
  {
    title: 'Data Engineering',
    summary: 'Analytical workflows shaped around local files, schema clarity, and SQL-first exploration.',
    skills: ['CSV parsing', 'Schema inference', 'SQL', 'Apache Arrow', 'DataFusion roadmap'],
  },
  {
    title: 'Frontend Product',
    summary: 'Browser interfaces that expose technical depth through usable, polished product surfaces.',
    skills: ['Vue 3', 'TypeScript', 'Tailwind CSS', 'CodeMirror', 'Responsive UI'],
  },
  {
    title: 'Delivery',
    summary: 'Reviewable engineering habits that make portfolio work easy to inspect and extend.',
    skills: ['GitHub issues', 'Pull requests', 'Podman', 'CI planning', 'Documentation'],
  },
]
</script>

<template>
  <section class="page-shell">
    <div class="page-heading">
      <p class="eyebrow">About</p>
      <h1>Built as a product, presented as a portfolio.</h1>
      <p>
        Query In is designed to demonstrate practical data engineering judgment through a usable CSV analysis tool, clear system boundaries, and production-oriented documentation.
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
