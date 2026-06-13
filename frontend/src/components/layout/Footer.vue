<script setup lang="ts">
import { Github, Linkedin } from 'lucide-vue-next'
</script>

<template>
  <footer class="border-t border-[#2a2a2a]">
    <div
      class="mx-auto flex w-full max-w-6xl flex-col gap-4 px-5 py-8 text-sm text-[#888888] md:flex-row md:items-center md:justify-between"
    >
      <p>Private CSV analysis, running locally in the browser.</p>
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
</template>
