<script setup lang="ts">
import { FileUp, ListTree, Play, TableProperties } from 'lucide-vue-next'
import { computed, ref } from 'vue'
import FileDropzone, { type CsvFileSelection } from '../components/query/FileDropzone.vue'
import SchemaViewer from '../components/query/SchemaViewer.vue'
import SqlEditor from '../components/query/SqlEditor.vue'
import { useQueryEngine } from '../composables/useQueryEngine'
import type { RegisteredCsvSchema } from '../workers/queryWorkerProtocol'

const selectedFiles = ref<CsvFileSelection[]>([])
const sqlQuery = ref('select *\nfrom uploaded_csv\nlimit 100;')
const schemaEntries = ref<SchemaEntry[]>([])
const schemaError = ref<string | null>(null)
const isRegisteringSchemas = ref(false)
const queryEngine = useQueryEngine()

let initializePromise: Promise<void> | null = null

interface SchemaEntry {
  fileId: string
  schema: RegisteredCsvSchema
}

const selectedSchemas = computed(() => {
  const selectedFileIds = new Set(selectedFiles.value.map((file) => file.id))

  return schemaEntries.value
    .filter((entry) => selectedFileIds.has(entry.fileId))
    .map((entry) => entry.schema)
})

async function handleFilesSelected(files: CsvFileSelection[]) {
  selectedFiles.value = files
  schemaError.value = null

  await registerSchemas(files)
}

async function registerSchemas(files: CsvFileSelection[]) {
  const pendingFiles = files.filter(
    (file) => !schemaEntries.value.some((entry) => entry.fileId === file.id),
  )

  if (pendingFiles.length === 0) {
    return
  }

  isRegisteringSchemas.value = true

  try {
    await ensureQueryEngineReady()

    const reservedNames = new Set(schemaEntries.value.map((entry) => entry.schema.name))

    for (const file of pendingFiles) {
      const tableName = tableNameForFile(file, reservedNames)
      const schema = await queryEngine.registerCsv(tableName, await file.file.arrayBuffer())
      reservedNames.add(tableName)
      schemaEntries.value = [...schemaEntries.value, { fileId: file.id, schema }]
      applyDefaultSqlForTable(tableName)
    }
  } catch (error) {
    schemaError.value = error instanceof Error ? error.message : 'CSV schema inference failed.'
  } finally {
    isRegisteringSchemas.value = false
  }
}

async function ensureQueryEngineReady() {
  initializePromise ??= queryEngine.initialize()

  await initializePromise
}

function tableNameForFile(file: CsvFileSelection, reservedNames: Set<string>) {
  const basename = file.name.replace(/\.csv$/i, '')
  const normalizedName = basename.replace(/[^A-Za-z0-9_]+/g, '_').replace(/^_+|_+$/g, '')
  let tableName = normalizedName || 'uploaded_csv'

  if (/^[0-9]/.test(tableName)) {
    tableName = `table_${tableName}`
  }

  if (!reservedNames.has(tableName)) {
    return tableName
  }

  let suffix = 2

  while (reservedNames.has(`${tableName}_${suffix}`)) {
    suffix += 1
  }

  return `${tableName}_${suffix}`
}

function applyDefaultSqlForTable(tableName: string) {
  if (sqlQuery.value.includes('uploaded_csv')) {
    sqlQuery.value = `select *\nfrom ${tableName}\nlimit 100;`
  }
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
        <SqlEditor v-model="sqlQuery" :schemas="selectedSchemas" />
      </section>
    </div>

    <section class="panel">
      <div class="panel-title">
        <ListTree class="size-5 text-[#00d9ff]" aria-hidden="true" />
        <h2>Schema</h2>
      </div>
      <SchemaViewer :schemas="selectedSchemas" :is-loading="isRegisteringSchemas" :error="schemaError" />
    </section>

    <section class="panel">
      <div class="panel-title">
        <TableProperties class="size-5 text-[#00d9ff]" aria-hidden="true" />
        <h2>Results</h2>
      </div>
      <div class="result-empty">Query results will appear after SQL execution is connected to the editor.</div>
    </section>
  </section>
</template>
