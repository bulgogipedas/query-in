<script setup lang="ts">
import { computed, ref } from 'vue'
import { AlertCircle, Download, FileJson, Loader2, TableProperties } from 'lucide-vue-next'
import { useVirtualizer } from '@tanstack/vue-virtual'
import type { QueryResult, QueryRow } from '../../workers/queryWorkerProtocol'

const props = defineProps<{
  result: QueryResult | null
  isLoading?: boolean
  error?: string | null
}>()

const scrollElement = ref<HTMLDivElement | null>(null)

const columns = computed(() => props.result?.schema ?? [])
const rows = computed(() => props.result?.rows ?? [])

const rowVirtualizer = useVirtualizer(
  computed(() => ({
    count: rows.value.length,
    getScrollElement: () => scrollElement.value,
    estimateSize: () => 44,
    overscan: 8,
  })),
)

const virtualRows = computed(() => rowVirtualizer.value.getVirtualItems())
const totalSize = computed(() => rowVirtualizer.value.getTotalSize())
const canExport = computed(() => Boolean(props.result && rows.value.length > 0))

function cellValue(row: QueryRow, columnName: string) {
  return row[columnName] ?? 'NULL'
}

function exportCsv() {
  if (!props.result) {
    return
  }

  const columnNames = props.result.schema.map((column) => column.name)
  const csvRows = [
    columnNames.map(escapeCsvCell).join(','),
    ...props.result.rows.map((row) =>
      columnNames.map((columnName) => escapeCsvCell(row[columnName] ?? '')).join(','),
    ),
  ]

  downloadFile(csvRows.join('\n'), 'text/csv;charset=utf-8', 'csv')
}

function exportJson() {
  if (!props.result) {
    return
  }

  const payload = {
    row_count: props.result.row_count,
    elapsed_ms: props.result.elapsed_ms,
    schema: props.result.schema,
    rows: props.result.rows,
  }

  downloadFile(JSON.stringify(payload, null, 2), 'application/json;charset=utf-8', 'json')
}

function escapeCsvCell(value: string | null) {
  const cell = value ?? ''

  if (/[",\n\r]/.test(cell)) {
    return `"${cell.replace(/"/g, '""')}"`
  }

  return cell
}

function downloadFile(content: string, mimeType: string, extension: 'csv' | 'json') {
  const blob = new Blob([content], { type: mimeType })
  const url = URL.createObjectURL(blob)
  const link = document.createElement('a')

  link.href = url
  link.download = `query-in-results-${timestampForFilename()}.${extension}`
  link.click()
  URL.revokeObjectURL(url)
}

function timestampForFilename() {
  return new Date().toISOString().replace(/:/g, '-').replace(/\.\d{3}Z$/, 'Z')
}
</script>

<template>
  <div class="grid gap-4">
    <div v-if="error" class="rounded-md border border-[#3a1a1a] bg-[#1a0f0f] p-3 text-sm text-[#ef4444]">
      <div class="flex items-start gap-2">
        <AlertCircle class="mt-0.5 size-4 shrink-0" aria-hidden="true" />
        <span>{{ error }}</span>
      </div>
    </div>

    <div v-if="isLoading" class="result-empty flex items-center gap-2">
      <Loader2 class="size-4 animate-spin text-[#faff69]" aria-hidden="true" />
      <span>Running query...</span>
    </div>

    <div v-else-if="!result" class="result-empty">
      Query results will appear after a SQL statement runs.
    </div>

    <div v-else-if="rows.length === 0" class="result-empty">
      The query completed successfully and returned no rows.
    </div>

    <div v-else class="grid gap-3">
      <div class="flex flex-wrap items-center justify-between gap-3 text-sm text-[#cccccc]">
        <div class="flex items-center gap-2">
          <TableProperties class="size-4 text-[#faff69]" aria-hidden="true" />
          <span>{{ result.row_count.toLocaleString() }} rows</span>
        </div>
        <div class="flex flex-wrap items-center gap-2">
          <span>{{ result.elapsed_ms.toFixed(2) }} ms</span>
          <button
            type="button"
            class="secondary-action min-h-10 px-3 py-2 text-sm"
            :disabled="!canExport"
            :class="!canExport ? 'cursor-not-allowed opacity-60' : ''"
            @click="exportCsv"
          >
            <Download class="size-4" aria-hidden="true" />
            CSV
          </button>
          <button
            type="button"
            class="secondary-action min-h-10 px-3 py-2 text-sm"
            :disabled="!canExport"
            :class="!canExport ? 'cursor-not-allowed opacity-60' : ''"
            @click="exportJson"
          >
            <FileJson class="size-4" aria-hidden="true" />
            JSON
          </button>
        </div>
      </div>

      <div ref="scrollElement" class="max-h-[28rem] overflow-auto rounded-md border border-[#2a2a2a] bg-[#1a1a1a]">
        <div class="sticky top-0 z-10 grid min-w-max border-b border-[#2a2a2a] bg-[#121212]">
          <div class="flex">
            <div
              v-for="column in columns"
              :key="column.name"
              class="w-48 shrink-0 border-r border-[#2a2a2a] px-3 py-2 font-mono text-xs font-semibold uppercase text-[#888888]"
            >
              {{ column.name }}
            </div>
          </div>
        </div>

        <div class="relative min-w-max" :style="{ height: `${totalSize}px` }">
          <div
            v-for="virtualRow in virtualRows"
            :key="`row-${virtualRow.index}`"
            class="absolute left-0 top-0 flex border-b border-[#2a2a2a]"
            :style="{ transform: `translateY(${virtualRow.start}px)` }"
          >
            <div
              v-for="column in columns"
              :key="column.name"
              class="w-48 shrink-0 truncate border-r border-[#2a2a2a] px-3 py-2 font-mono text-sm text-[#ffffff]"
              :class="cellValue(rows[virtualRow.index], column.name) === 'NULL' ? 'text-[#5a5a5a]' : ''"
              :title="cellValue(rows[virtualRow.index], column.name)"
            >
              {{ cellValue(rows[virtualRow.index], column.name) }}
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
