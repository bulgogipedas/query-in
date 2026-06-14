import { computed, ref } from 'vue'

export interface QueryHistoryEntry {
  id: string
  sql: string
  executedAt: string
  elapsedMs: number
  rowCount: number
}

export interface QueryHistoryInput {
  sql: string
  elapsedMs: number
  rowCount: number
}

const QUERY_HISTORY_STORAGE_KEY = 'query-in:query-history'
const MAX_HISTORY_ENTRIES = 50

export function useQueryHistory() {
  const entries = ref<QueryHistoryEntry[]>(readStoredEntries())
  const hasEntries = computed(() => entries.value.length > 0)

  function recordQuery(input: QueryHistoryInput) {
    const trimmedSql = input.sql.trim()

    if (!trimmedSql) {
      return
    }

    const nextEntry: QueryHistoryEntry = {
      id: crypto.randomUUID(),
      sql: trimmedSql,
      executedAt: new Date().toISOString(),
      elapsedMs: input.elapsedMs,
      rowCount: input.rowCount,
    }

    entries.value = [
      nextEntry,
      ...entries.value.filter((entry) => entry.sql !== trimmedSql),
    ].slice(0, MAX_HISTORY_ENTRIES)

    writeStoredEntries(entries.value)
  }

  function clearHistory() {
    entries.value = []
    removeStoredEntries()
  }

  return {
    entries,
    hasEntries,
    recordQuery,
    clearHistory,
  }
}

function readStoredEntries() {
  if (typeof localStorage === 'undefined') {
    return []
  }

  try {
    const storedEntries = localStorage.getItem(QUERY_HISTORY_STORAGE_KEY)

    if (!storedEntries) {
      return []
    }

    const parsedEntries = JSON.parse(storedEntries)

    if (!Array.isArray(parsedEntries)) {
      return []
    }

    return parsedEntries.filter(isQueryHistoryEntry).slice(0, MAX_HISTORY_ENTRIES)
  } catch {
    return []
  }
}

function writeStoredEntries(entries: QueryHistoryEntry[]) {
  if (typeof localStorage === 'undefined') {
    return
  }

  localStorage.setItem(QUERY_HISTORY_STORAGE_KEY, JSON.stringify(entries))
}

function removeStoredEntries() {
  if (typeof localStorage === 'undefined') {
    return
  }

  localStorage.removeItem(QUERY_HISTORY_STORAGE_KEY)
}

function isQueryHistoryEntry(entry: unknown): entry is QueryHistoryEntry {
  if (!entry || typeof entry !== 'object') {
    return false
  }

  const candidate = entry as QueryHistoryEntry

  return (
    typeof candidate.id === 'string' &&
    typeof candidate.sql === 'string' &&
    typeof candidate.executedAt === 'string' &&
    typeof candidate.elapsedMs === 'number' &&
    typeof candidate.rowCount === 'number'
  )
}
