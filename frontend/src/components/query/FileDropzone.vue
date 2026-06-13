<script setup lang="ts">
import { computed, ref } from 'vue'
import { AlertCircle, FileText, Upload, X } from 'lucide-vue-next'

export interface CsvFileSelection {
  id: string
  file: File
  name: string
  size: number
  lastModified: number
}

const MAX_FILE_SIZE_BYTES = 50 * 1024 * 1024
const CSV_MIME_TYPES = new Set(['text/csv', 'application/csv', 'application/vnd.ms-excel'])

const emit = defineEmits<{
  filesSelected: [files: CsvFileSelection[]]
}>()

const fileInput = ref<HTMLInputElement | null>(null)
const selectedFiles = ref<CsvFileSelection[]>([])
const errors = ref<string[]>([])
const isDragging = ref(false)

const hasFiles = computed(() => selectedFiles.value.length > 0)

function openFilePicker() {
  fileInput.value?.click()
}

function handleInputChange(event: Event) {
  const input = event.target as HTMLInputElement

  addFiles(input.files)
  input.value = ''
}

function handleDragEnter() {
  isDragging.value = true
}

function handleDragLeave(event: DragEvent) {
  if (event.currentTarget === event.target) {
    isDragging.value = false
  }
}

function handleDrop(event: DragEvent) {
  isDragging.value = false
  addFiles(event.dataTransfer?.files ?? null)
}

function addFiles(fileList: FileList | null) {
  if (!fileList) {
    return
  }

  const nextErrors: string[] = []
  const nextFiles: CsvFileSelection[] = []

  for (const file of Array.from(fileList)) {
    const error = validateFile(file)

    if (error) {
      nextErrors.push(error)
      continue
    }

    nextFiles.push(toSelection(file))
  }

  const mergedFiles = mergeFiles(selectedFiles.value, nextFiles)

  selectedFiles.value = mergedFiles
  errors.value = nextErrors
  emit('filesSelected', mergedFiles)
}

function removeFile(fileId: string) {
  selectedFiles.value = selectedFiles.value.filter((file) => file.id !== fileId)
  emit('filesSelected', selectedFiles.value)
}

function validateFile(file: File) {
  if (!isCsvFile(file)) {
    return `${file.name} is not a CSV file.`
  }

  if (file.size > MAX_FILE_SIZE_BYTES) {
    return `${file.name} is larger than the 50MB limit.`
  }

  return null
}

function isCsvFile(file: File) {
  return file.name.toLowerCase().endsWith('.csv') || CSV_MIME_TYPES.has(file.type)
}

function mergeFiles(currentFiles: CsvFileSelection[], nextFiles: CsvFileSelection[]) {
  const filesById = new Map(currentFiles.map((file) => [file.id, file]))

  for (const file of nextFiles) {
    filesById.set(file.id, file)
  }

  return Array.from(filesById.values())
}

function toSelection(file: File): CsvFileSelection {
  return {
    id: `${file.name}-${file.size}-${file.lastModified}`,
    file,
    name: file.name,
    size: file.size,
    lastModified: file.lastModified,
  }
}

function formatFileSize(bytes: number) {
  if (bytes < 1024 * 1024) {
    return `${Math.max(1, Math.round(bytes / 1024))} KB`
  }

  return `${(bytes / 1024 / 1024).toFixed(1)} MB`
}
</script>

<template>
  <div class="grid gap-4">
    <button
      type="button"
      class="dropzone group grid min-h-48 w-full place-items-center border-dashed text-left transition"
      :class="isDragging ? 'border-[#00d9ff] bg-[#102437]' : 'hover:border-[#00d9ff]/70 hover:bg-[#14142f]'"
      @click="openFilePicker"
      @dragenter.prevent="handleDragEnter"
      @dragover.prevent
      @dragleave.prevent="handleDragLeave"
      @drop.prevent="handleDrop"
    >
      <span class="grid max-w-md justify-items-center gap-3 text-center">
        <span class="grid size-12 place-items-center rounded-md bg-[#00d9ff] text-[#0a0a0f]">
          <Upload class="size-6" aria-hidden="true" />
        </span>
        <span class="font-display text-lg font-semibold text-white">Drop CSV files or browse</span>
        <span class="text-sm leading-6 text-[#a6a6c2]">Multiple files are supported. Each file can be up to 50MB.</span>
      </span>
    </button>

    <input
      ref="fileInput"
      class="sr-only"
      type="file"
      accept=".csv,text/csv,application/csv,application/vnd.ms-excel"
      multiple
      @change="handleInputChange"
    />

    <div v-if="errors.length" class="grid gap-2 rounded-md border border-[#7f2534] bg-[#2d1118] p-3">
      <div v-for="error in errors" :key="error" class="flex items-start gap-2 text-sm text-[#ffb3c0]">
        <AlertCircle class="mt-0.5 size-4 shrink-0" aria-hidden="true" />
        <span>{{ error }}</span>
      </div>
    </div>

    <div v-if="hasFiles" class="grid gap-2">
      <div
        v-for="file in selectedFiles"
        :key="file.id"
        class="flex min-h-14 items-center justify-between gap-3 rounded-md border border-[#2a2a4a] bg-[#101024] px-3 py-2"
      >
        <div class="flex min-w-0 items-center gap-3">
          <FileText class="size-5 shrink-0 text-[#00d9ff]" aria-hidden="true" />
          <div class="min-w-0">
            <p class="truncate text-sm font-semibold text-white">{{ file.name }}</p>
            <p class="mt-0.5 text-xs text-[#8888aa]">{{ formatFileSize(file.size) }}</p>
          </div>
        </div>
        <button
          type="button"
          class="grid size-8 shrink-0 place-items-center rounded-md border border-[#2a2a4a] text-[#c7c7dd] transition hover:border-[#00d9ff]/70 hover:text-white"
          :aria-label="`Remove ${file.name}`"
          @click.stop="removeFile(file.id)"
        >
          <X class="size-4" aria-hidden="true" />
        </button>
      </div>
    </div>
  </div>
</template>
