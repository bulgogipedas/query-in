<script setup lang="ts">
import { Database, Github, Linkedin } from 'lucide-vue-next'
import { RouterLink, RouterView } from 'vue-router'
</script>

<template>
  <div class="min-h-screen bg-[#0a0a0f] text-[#f0f0ff]">
    <header class="border-b border-[#2a2a4a] bg-[#0a0a0f]/90 backdrop-blur">
      <nav class="mx-auto flex w-full max-w-6xl items-center justify-between px-5 py-4">
        <RouterLink to="/" class="flex items-center gap-3 font-display text-lg font-semibold">
          <span class="grid size-9 place-items-center rounded bg-[#00d9ff] text-[#0a0a0f]">
            <Database class="size-5" aria-hidden="true" />
          </span>
          <span>Query In</span>
        </RouterLink>

        <div class="flex items-center gap-2 text-sm text-[#8888aa]">
          <RouterLink class="nav-link" to="/query">Query</RouterLink>
          <RouterLink class="nav-link" to="/projects">Projects</RouterLink>
          <RouterLink class="nav-link" to="/about">About</RouterLink>
        </div>
      </nav>
    </header>

    <main>
      <RouterView />
    </main>

    <footer class="border-t border-[#2a2a4a]">
      <div class="mx-auto flex w-full max-w-6xl flex-col gap-4 px-5 py-8 text-sm text-[#8888aa] md:flex-row md:items-center md:justify-between">
        <p>Query In keeps CSV exploration local, fast, and portfolio-ready.</p>
        <div class="flex gap-3">
          <a class="icon-link" href="https://github.com/bulgogipedas/query-in" aria-label="GitHub repository">
            <Github class="size-5" aria-hidden="true" />
          </a>
          <a class="icon-link" href="https://www.linkedin.com" aria-label="LinkedIn profile">
            <Linkedin class="size-5" aria-hidden="true" />
          </a>
        </div>
      </div>
    </footer>
  </div>
</template>
