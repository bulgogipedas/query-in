<script setup lang="ts">
const projects = [
  {
    name: 'Query In',
    description: 'A browser-based CSV query engine and data engineering portfolio built with Rust, WASM, Vue, and Axum.',
    stack: ['Rust', 'WASM', 'Vue', 'Axum', 'SQL'],
  },
]
</script>

<template>
  <section class="page-shell">
    <div class="page-heading">
      <p class="eyebrow">Portfolio</p>
      <h1>Data engineering projects with working demos.</h1>
      <p>Project metadata will be served by the Axum API after the backend persistence milestone.</p>
    </div>

    <div class="grid gap-4 md:grid-cols-2">
      <article v-for="project in projects" :key="project.name" class="feature-card">
        <h2 class="font-display text-2xl font-bold text-white">{{ project.name }}</h2>
        <p class="mt-3 leading-7 text-[#cccccc]">{{ project.description }}</p>
        <div class="mt-5 flex flex-wrap gap-2">
          <span v-for="item in project.stack" :key="item" class="badge">{{ item }}</span>
        </div>
      </article>
    </div>
  </section>
</template>
