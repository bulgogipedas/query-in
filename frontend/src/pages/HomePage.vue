<script setup lang="ts">
import { ArrowRight, Lock, Zap, Braces } from 'lucide-vue-next'
import { RouterLink } from 'vue-router'
import WebGLCanvas from '../components/visual/WebGLCanvas.vue'

const highlights = [
  {
    title: 'Instant Local Queries',
    description: 'Open large CSV files and run SQL in the browser without waiting on uploads or warehouse setup.',
    icon: Zap,
  },
  {
    title: 'Private by Default',
    description: 'Files stay on the user device, giving teams a safer way to inspect sensitive operational data.',
    icon: Lock,
  },
  {
    title: 'Workflow Ready',
    description: 'Schema inference, query history, and result exports keep lightweight analysis moving end to end.',
    icon: Braces,
  },
]
</script>

<template>
  <section class="hero-band">
    <div class="mx-auto grid min-h-[calc(100svh-73px)] w-full max-w-7xl content-center gap-10 px-5 py-16 lg:grid-cols-[minmax(0,0.9fr)_minmax(24rem,1.1fr)] lg:items-center">
      <div class="max-w-3xl">
        <p class="eyebrow">Private CSV analytics workspace</p>
        <h1 class="mt-5 max-w-3xl font-display text-5xl font-bold leading-tight md:text-7xl">
          Query In
        </h1>
        <p class="mt-6 max-w-2xl text-xl leading-8 text-[#cccccc]">
          Turn CSV files into answers with SQL that runs directly in the browser.
          No upload queue, no warehouse dependency, no data leaving the device.
        </p>
        <div class="mt-8 flex flex-wrap gap-3">
          <RouterLink class="primary-action" to="/query">
            Start Querying
            <ArrowRight class="size-4" aria-hidden="true" />
          </RouterLink>
          <RouterLink class="secondary-action" to="/projects">Explore Use Cases</RouterLink>
        </div>
      </div>

      <WebGLCanvas />

      <div class="grid gap-4 md:grid-cols-3 lg:col-span-2">
        <article v-for="item in highlights" :key="item.title" class="feature-card">
          <component :is="item.icon" class="size-5 text-[#faff69]" aria-hidden="true" />
          <h2 class="mt-4 font-display text-xl font-bold text-white">{{ item.title }}</h2>
          <p class="mt-3 text-sm leading-6 text-[#cccccc]">{{ item.description }}</p>
        </article>
      </div>
    </div>
  </section>
</template>
