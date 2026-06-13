<script setup lang="ts">
import { Database } from 'lucide-vue-next'
import { RouterLink } from 'vue-router'
</script>

<template>
  <header class="border-b border-[#2a2a2a] bg-[#0a0a0a]/90 backdrop-blur">
    <nav class="mx-auto flex w-full max-w-6xl flex-wrap items-center justify-between gap-3 px-5 py-4">
      <RouterLink to="/" class="flex shrink-0 items-center gap-3 font-display text-lg font-semibold">
        <span class="grid size-9 place-items-center rounded bg-[#faff69] text-[#0a0a0a]">
          <Database class="size-5" aria-hidden="true" />
        </span>
        <span>Query In</span>
      </RouterLink>

      <div class="flex flex-wrap items-center justify-end gap-2 text-sm text-[#888888]">
        <RouterLink class="nav-link" to="/query">Query</RouterLink>
        <RouterLink class="nav-link" to="/projects">Projects</RouterLink>
        <RouterLink class="nav-link" to="/about">About</RouterLink>
      </div>
    </nav>
  </header>
</template>
