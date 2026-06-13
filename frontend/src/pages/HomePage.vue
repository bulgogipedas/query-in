<script setup lang="ts">
import { ArrowRight, Lock, Zap, Braces } from 'lucide-vue-next'
import { RouterLink } from 'vue-router'

const highlights = [
  {
    title: 'Native Speed',
    description: 'The query engine is designed to run in WebAssembly instead of sending CSV data to a server.',
    icon: Zap,
  },
  {
    title: 'Privacy First',
    description: 'Uploaded files stay on the visitor device during browser-based analysis.',
    icon: Lock,
  },
  {
    title: 'Real SQL',
    description: 'The roadmap targets Apache DataFusion for practical joins, aggregates, and analytical SQL.',
    icon: Braces,
  },
]
</script>

<template>
  <section class="hero-band">
    <div class="mx-auto grid min-h-[calc(100svh-73px)] w-full max-w-6xl content-center gap-12 px-5 py-16">
      <div class="max-w-3xl">
        <p class="eyebrow">Browser-based CSV query engine</p>
        <h1 class="mt-5 max-w-3xl font-display text-5xl font-bold leading-tight md:text-7xl">
          Query In
        </h1>
        <p class="mt-6 max-w-2xl text-xl leading-8 text-[#cccccc]">
          Your data stays in your browser. Your queries run at native speed.
        </p>
        <div class="mt-8 flex flex-wrap gap-3">
          <RouterLink class="primary-action" to="/query">
            Try the Query Engine
            <ArrowRight class="size-4" aria-hidden="true" />
          </RouterLink>
          <RouterLink class="secondary-action" to="/projects">View Projects</RouterLink>
        </div>
      </div>

      <div class="grid gap-4 md:grid-cols-3">
        <article v-for="item in highlights" :key="item.title" class="feature-card">
          <component :is="item.icon" class="size-5 text-[#faff69]" aria-hidden="true" />
          <h2 class="mt-4 font-display text-xl font-bold text-white">{{ item.title }}</h2>
          <p class="mt-3 text-sm leading-6 text-[#cccccc]">{{ item.description }}</p>
        </article>
      </div>
    </div>
  </section>
</template>
