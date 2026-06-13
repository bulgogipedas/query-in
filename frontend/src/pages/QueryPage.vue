<script setup lang="ts">
import { FileUp, Play, TableProperties } from 'lucide-vue-next'
import { ref } from 'vue'
import FileDropzone, { type CsvFileSelection } from '../components/query/FileDropzone.vue'

const selectedFiles = ref<CsvFileSelection[]>([])

function handleFilesSelected(files: CsvFileSelection[]) {
  selectedFiles.value = files
}
</script>

<template>
  <section class="page-shell">
    <div class="page-heading">
      <p class="eyebrow">Query workspace</p>
      <h1>Upload CSV files, inspect schema, then run SQL.</h1>
      <p>
        This workspace connects the browser query engine to focused UI pieces as the Phase 3 workflow lands.
      </p>
    </div>

    <div class="grid gap-4 lg:grid-cols-[1fr_1.2fr]">
      <section class="panel">
        <div class="panel-title">
          <FileUp class="size-5 text-[#00d9ff]" aria-hidden="true" />
          <h2>CSV Input</h2>
        </div>
        <FileDropzone @files-selected="handleFilesSelected" />
      </section>

      <section class="panel">
        <div class="panel-title">
          <Play class="size-5 text-[#00d9ff]" aria-hidden="true" />
          <h2>SQL Editor</h2>
        </div>
        <pre class="code-surface"><code>select *
from {{ selectedFiles[0]?.name.replace(/\.csv$/i, '') || 'uploaded_csv' }}
limit 100;</code></pre>
      </section>
    </div>

    <section class="panel">
      <div class="panel-title">
        <TableProperties class="size-5 text-[#00d9ff]" aria-hidden="true" />
        <h2>Results</h2>
      </div>
      <div class="result-empty">Query results will appear here after the WASM execution layer lands.</div>
    </section>
  </section>
</template>
