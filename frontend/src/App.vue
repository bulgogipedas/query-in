<script setup lang="ts">
import { RouterView } from 'vue-router'
import Footer from './components/layout/Footer.vue'
import Navbar from './components/layout/Navbar.vue'
</script>

<template>
  <div class="min-h-screen bg-[#0a0a0f] text-[#f0f0ff]">
    <Navbar />
    <main>
      <RouterView />
    </main>
    <Footer />
  </div>
</template>
