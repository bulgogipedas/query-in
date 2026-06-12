<script setup lang="ts">
import { FileUp, Play, TableProperties } from 'lucide-vue-next'
</script>

<template>
  <section class="page-shell">
    <div class="page-heading">
      <p class="eyebrow">Query workspace</p>
      <h1>Upload CSV files, inspect schema, then run SQL.</h1>
      <p>
        This bootstrap screen reserves the core workflow while the WebAssembly engine and worker integration are built in follow-up issues.
      </p>
    </div>

    <div class="grid gap-4 lg:grid-cols-[1fr_1.2fr]">
      <section class="panel">
        <div class="panel-title">
          <FileUp class="size-5 text-[#00d9ff]" aria-hidden="true" />
          <h2>CSV Input</h2>
        </div>
        <div class="dropzone">
          <p class="font-semibold text-white">Drop CSV files here</p>
          <p class="mt-2 text-sm text-[#8888aa]">Multi-file joins and schema overrides are planned for the query engine milestone.</p>
        </div>
      </section>

      <section class="panel">
        <div class="panel-title">
          <Play class="size-5 text-[#00d9ff]" aria-hidden="true" />
          <h2>SQL Editor</h2>
        </div>
        <pre class="code-surface"><code>select *
from uploaded_csv
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
