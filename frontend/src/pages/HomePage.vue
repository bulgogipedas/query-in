<script setup lang="ts">
import { defineAsyncComponent, ref } from 'vue'
import { RouterLink } from 'vue-router'

const WebGLCanvas = defineAsyncComponent(() => import('../components/visual/WebGLCanvas.vue'))
const showInteractiveHero = ref(false)

const highlights = [
  {
    title: 'Instant Local Queries',
    description: 'Open large CSV files and run SQL in the browser without waiting on uploads or warehouse setup.',
    marker: '01',
  },
  {
    title: 'Private by Default',
    description: 'Files stay on the user device, giving teams a safer way to inspect sensitive operational data.',
    marker: '02',
  },
  {
    title: 'Workflow Ready',
    description: 'Schema inference, query history, and result exports keep lightweight analysis moving end to end.',
    marker: '03',
  },
]

function activateHero() {
  showInteractiveHero.value = true
}
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
            <span aria-hidden="true">-&gt;</span>
          </RouterLink>
          <RouterLink class="secondary-action" to="/projects">Explore Use Cases</RouterLink>
        </div>
      </div>

      <WebGLCanvas v-if="showInteractiveHero" start-on-mount />

      <div
        v-else
        class="relative min-h-[24rem] overflow-hidden rounded-xl border border-[#2a2a2a] bg-[#1a1a1a]"
        @click="activateHero"
        @focusin="activateHero"
        @pointerenter="activateHero"
      >
        <div
          class="absolute inset-0 bg-[radial-gradient(circle_at_50%_28%,rgba(250,255,105,0.16),transparent_34%),linear-gradient(90deg,rgba(250,255,105,0.08)_1px,transparent_1px),linear-gradient(0deg,rgba(250,255,105,0.06)_1px,transparent_1px)] bg-[length:auto,2rem_2rem,2rem_2rem]"
          aria-hidden="true"
        />

        <div class="relative grid h-full min-h-[24rem] content-between p-5">
          <div
            class="flex items-center justify-between border-b border-[#2a2a2a] pb-3 font-mono text-xs text-[#888888]"
          >
            <span>query-in://local-engine</span>
            <span class="text-[#faff69]">WASM READY</span>
          </div>

          <pre class="overflow-hidden font-mono text-sm leading-7 text-white"><code><span class="text-[#888888]">01</span> select region, revenue
<span class="text-[#888888]">02</span> from uploaded_csv
<span class="text-[#888888]">03</span> where browser_only = true
<span class="text-[#888888]">04</span> limit 100;</code></pre>

          <div class="grid gap-3 sm:grid-cols-3">
            <div class="border-l border-[#faff69] pl-3">
              <p class="font-mono text-2xl font-bold text-[#faff69]">0</p>
              <p class="text-xs text-[#888888]">server uploads</p>
            </div>
            <div class="border-l border-[#faff69] pl-3">
              <p class="font-mono text-2xl font-bold text-[#faff69]">WASM</p>
              <p class="text-xs text-[#888888]">query runtime</p>
            </div>
            <div class="border-l border-[#faff69] pl-3">
              <p class="font-mono text-2xl font-bold text-[#faff69]">SQL</p>
              <p class="text-xs text-[#888888]">local analysis</p>
            </div>
          </div>
        </div>
      </div>

      <div class="grid gap-4 md:grid-cols-3 lg:col-span-2">
        <article v-for="item in highlights" :key="item.title" class="feature-card">
          <span class="font-mono text-sm font-bold text-[#faff69]" aria-hidden="true">{{ item.marker }}</span>
          <h2 class="mt-4 font-display text-xl font-bold text-white">{{ item.title }}</h2>
          <p class="mt-3 text-sm leading-6 text-[#cccccc]">{{ item.description }}</p>
        </article>
      </div>
    </div>
  </section>
</template>
