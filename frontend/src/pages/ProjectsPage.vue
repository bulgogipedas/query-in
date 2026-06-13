<script setup lang="ts">
import { onMounted, ref } from 'vue'

type Project = {
  name: string
  slug: string
  summary: string
  status: string
  stack: string[]
}

const projects = ref<Project[]>([])
const isLoading = ref(true)
const errorMessage = ref('')

onMounted(async () => {
  try {
    const response = await fetch('/api/projects')

    if (!response.ok) {
      throw new Error(`Project metadata request failed with ${response.status}`)
    }

    projects.value = await response.json() as Project[]
  } catch (error) {
    errorMessage.value = error instanceof Error
      ? error.message
      : 'Project metadata could not be loaded.'
  } finally {
    isLoading.value = false
  }
})
</script>

<template>
  <section class="page-shell">
    <div class="page-heading">
      <p class="eyebrow">Portfolio</p>
      <h1>Data engineering projects with working demos.</h1>
      <p>Project metadata is served by the Query In Axum API and rendered by the browser client.</p>
    </div>

    <div v-if="isLoading" class="feature-card">
      <p class="font-mono text-sm text-[#faff69]">Loading project metadata...</p>
    </div>

    <div v-else-if="errorMessage" class="feature-card border-[#faff69]">
      <p class="font-display text-xl font-bold text-white">Project metadata is unavailable.</p>
      <p class="mt-3 font-mono text-sm text-[#cccccc]">{{ errorMessage }}</p>
    </div>

    <div v-else class="grid gap-4 md:grid-cols-2">
      <article v-for="project in projects" :key="project.name" class="feature-card">
        <div class="flex items-start justify-between gap-4">
          <h2 class="font-display text-2xl font-bold text-white">{{ project.name }}</h2>
          <span class="badge">{{ project.status }}</span>
        </div>
        <p class="mt-3 leading-7 text-[#cccccc]">{{ project.summary }}</p>
        <div class="mt-5 flex flex-wrap gap-2">
          <span v-for="item in project.stack" :key="item" class="badge">{{ item }}</span>
        </div>
      </article>
    </div>
  </section>
</template>
