<script setup lang="ts">
import { Database, Rows3 } from 'lucide-vue-next'
import type { RegisteredCsvSchema } from '../../workers/queryWorkerProtocol'

defineProps<{
  schemas: RegisteredCsvSchema[]
  isLoading?: boolean
  error?: string | null
}>()

function formatNullCount(nullCount: number, rowCount: number) {
  if (rowCount === 0) {
    return '0'
  }

  return `${nullCount} (${Math.round((nullCount / rowCount) * 100)}%)`
}
</script>

<template>
  <div class="grid gap-4">
    <div v-if="error" class="rounded-md border border-[#3a1a1a] bg-[#1a0f0f] p-3 text-sm text-[#ef4444]">
      {{ error }}
    </div>

    <div v-if="isLoading" class="result-empty">Inferring CSV schema...</div>

    <div v-else-if="schemas.length === 0" class="result-empty">
      Schema details will appear after CSV files are selected.
    </div>

    <template v-else>
      <section v-for="schema in schemas" :key="schema.name" class="grid gap-3 border-t border-[#2a2a2a] pt-4">
        <div class="flex flex-wrap items-center justify-between gap-3">
          <div class="flex min-w-0 items-center gap-3">
            <Database class="size-5 shrink-0 text-[#faff69]" aria-hidden="true" />
            <div class="min-w-0">
              <h3 class="truncate font-display text-lg font-semibold text-white">{{ schema.name }}</h3>
              <p class="mt-1 text-sm text-[#888888]">{{ schema.columns.length }} columns</p>
            </div>
          </div>
          <div class="flex items-center gap-2 text-sm text-[#cccccc]">
            <Rows3 class="size-4 text-[#faff69]" aria-hidden="true" />
            <span>{{ schema.row_count.toLocaleString() }} rows</span>
          </div>
        </div>

        <div class="overflow-x-auto">
          <table class="w-full min-w-[36rem] text-left text-sm">
            <thead class="border-b border-[#2a2a2a] text-xs uppercase text-[#888888]">
              <tr>
                <th class="py-2 pr-4 font-semibold">Column</th>
                <th class="px-4 py-2 font-semibold">Type</th>
                <th class="px-4 py-2 font-semibold">Nullable</th>
                <th class="py-2 pl-4 font-semibold">Null Count</th>
              </tr>
            </thead>
            <tbody class="divide-y divide-[#2a2a2a]">
              <tr v-for="column in schema.columns" :key="column.name">
                <td class="py-3 pr-4 font-mono text-white">{{ column.name }}</td>
                <td class="px-4 py-3">
                  <span class="badge">{{ column.data_type }}</span>
                </td>
                <td class="px-4 py-3 text-[#cccccc]">{{ column.nullable ? 'Yes' : 'No' }}</td>
                <td class="py-3 pl-4 text-[#cccccc]">
                  {{ formatNullCount(column.null_count, schema.row_count) }}
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </section>
    </template>
  </div>
</template>
