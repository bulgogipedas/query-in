<script setup lang="ts">
import { onMounted, ref } from 'vue'

type UseCase = {
  name: string
  slug: string
  summary: string
  status: string
  stack: string[]
}

const useCases = ref<UseCase[]>([])
const isLoading = ref(true)
const errorMessage = ref('')

onMounted(async () => {
  try {
    const response = await fetch('/api/use-cases')

    if (!response.ok) {
      throw new Error(`Use case request failed with ${response.status}`)
    }

    useCases.value = await response.json() as UseCase[]
  } catch (error) {
    errorMessage.value = error instanceof Error
      ? error.message
      : 'Use case data could not be loaded.'
  } finally {
    isLoading.value = false
  }
})
</script>

<template>
  <section class="page-shell">
    <div class="page-heading">
      <p class="eyebrow">Use cases</p>
      <h1>Fast answers for teams working with file-based data.</h1>
      <p>Query In fits the messy middle between spreadsheets and a full analytics stack.</p>
    </div>

    <div v-if="isLoading" class="feature-card">
      <p class="font-mono text-sm text-[#faff69]">Loading use cases...</p>
    </div>

    <div v-else-if="errorMessage" class="feature-card border-[#faff69]">
      <p class="font-display text-xl font-bold text-white">Use cases are unavailable.</p>
      <p class="mt-3 font-mono text-sm text-[#cccccc]">{{ errorMessage }}</p>
    </div>

    <div v-else class="grid gap-4 md:grid-cols-2">
      <article v-for="useCase in useCases" :key="useCase.name" class="feature-card">
        <div class="flex items-start justify-between gap-4">
          <h2 class="font-display text-2xl font-bold text-white">{{ useCase.name }}</h2>
          <span class="badge">{{ useCase.status }}</span>
        </div>
        <p class="mt-3 leading-7 text-[#cccccc]">{{ useCase.summary }}</p>
        <div class="mt-5 flex flex-wrap gap-2">
          <span v-for="item in useCase.stack" :key="item" class="badge">{{ item }}</span>
        </div>
      </article>
    </div>
  </section>
</template>
