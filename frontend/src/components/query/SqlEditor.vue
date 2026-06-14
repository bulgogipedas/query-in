<script setup lang="ts">
import { autocompletion, type CompletionContext, type CompletionResult } from '@codemirror/autocomplete'
import { defaultKeymap, history, historyKeymap } from '@codemirror/commands'
import { sql } from '@codemirror/lang-sql'
import { syntaxHighlighting, HighlightStyle } from '@codemirror/language'
import { EditorState, Compartment } from '@codemirror/state'
import { EditorView, keymap, lineNumbers, placeholder } from '@codemirror/view'
import { tags } from '@lezer/highlight'
import { computed, onBeforeUnmount, onMounted, ref, shallowRef, watch } from 'vue'
import type { RegisteredCsvSchema } from '../../workers/queryWorkerProtocol'

const props = defineProps<{
  modelValue: string
  schemas: RegisteredCsvSchema[]
}>()

const emit = defineEmits<{
  'update:modelValue': [value: string]
}>()

const editorRoot = ref<HTMLDivElement | null>(null)
const editorView = shallowRef<EditorView | null>(null)
const autocompleteCompartment = new Compartment()

const tableNames = computed(() => props.schemas.map((schema) => schema.name))

const sqlHighlightStyle = HighlightStyle.define([
  { tag: tags.keyword, color: '#faff69', fontWeight: '700' },
  { tag: tags.string, color: '#a7f3d0' },
  { tag: tags.number, color: '#fbbf24' },
  { tag: tags.operator, color: '#cccccc' },
  { tag: tags.variableName, color: '#ffffff' },
  { tag: tags.comment, color: '#888888', fontStyle: 'italic' },
])

onMounted(() => {
  if (!editorRoot.value) {
    return
  }

  editorView.value = new EditorView({
    parent: editorRoot.value,
    state: createEditorState(props.modelValue),
  })
})

onBeforeUnmount(() => {
  editorView.value?.destroy()
})

watch(
  () => props.modelValue,
  (nextValue) => {
    const view = editorView.value

    if (!view || nextValue === view.state.doc.toString()) {
      return
    }

    view.dispatch({
      changes: {
        from: 0,
        to: view.state.doc.length,
        insert: nextValue,
      },
    })
  },
)

watch(
  () => props.schemas,
  () => {
    editorView.value?.dispatch({
      effects: autocompleteCompartment.reconfigure(autocompletion({ override: [completeSql] })),
    })
  },
  { deep: true },
)

function createEditorState(value: string) {
  return EditorState.create({
    doc: value,
    extensions: [
      lineNumbers(),
      history(),
      keymap.of([...defaultKeymap, ...historyKeymap]),
      sql(),
      syntaxHighlighting(sqlHighlightStyle),
      placeholder('select * from uploaded_csv limit 100;'),
      autocompleteCompartment.of(autocompletion({ override: [completeSql] })),
      EditorView.lineWrapping,
      EditorView.updateListener.of((update) => {
        if (update.docChanged) {
          emit('update:modelValue', update.state.doc.toString())
        }
      }),
      queryInEditorTheme,
    ],
  })
}

function completeSql(context: CompletionContext): CompletionResult | null {
  const word = context.matchBefore(/[A-Za-z_][A-Za-z0-9_]*$/)

  if (!word && !context.explicit) {
    return null
  }

  const from = word?.from ?? context.pos
  const options = [
    ...props.schemas.map((schema) => ({
      label: schema.name,
      type: 'variable',
      detail: `${schema.row_count.toLocaleString()} rows`,
    })),
    ...props.schemas.flatMap((schema) =>
      schema.columns.map((column) => ({
        label: column.name,
        type: 'property',
        detail: `${schema.name}.${column.data_type}`,
      })),
    ),
    ...['select', 'from', 'where', 'limit', 'as', 'and', 'or'].map((keyword) => ({
      label: keyword,
      type: 'keyword',
    })),
  ]

  return {
    from,
    options,
    validFor: /^[A-Za-z_][A-Za-z0-9_]*$/,
  }
}

const queryInEditorTheme = EditorView.theme({
  '&': {
    minHeight: '13rem',
    border: '1px solid #2a2a2a',
    borderRadius: '8px',
    backgroundColor: '#1a1a1a',
    color: '#ffffff',
    fontSize: '0.92rem',
  },
  '.cm-scroller': {
    fontFamily: 'JetBrains Mono, ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace',
    lineHeight: '1.65',
  },
  '.cm-content': {
    minHeight: '13rem',
    padding: '1rem 0.75rem',
  },
  '.cm-gutters': {
    backgroundColor: '#121212',
    borderRight: '1px solid #2a2a2a',
    color: '#5a5a5a',
  },
  '.cm-activeLineGutter': {
    backgroundColor: '#242424',
    color: '#cccccc',
  },
  '.cm-activeLine': {
    backgroundColor: '#24242466',
  },
  '.cm-selectionBackground, &.cm-focused .cm-selectionBackground': {
    backgroundColor: '#faff6940',
  },
  '&.cm-focused': {
    outline: '2px solid #faff6966',
    outlineOffset: '2px',
  },
  '.cm-tooltip': {
    border: '1px solid #2a2a2a',
    borderRadius: '8px',
    backgroundColor: '#1a1a1a',
    color: '#ffffff',
  },
  '.cm-tooltip-autocomplete ul li[aria-selected]': {
    backgroundColor: '#faff69',
    color: '#0a0a0a',
  },
})
</script>

<template>
  <div class="grid gap-3">
    <div ref="editorRoot" />
    <p class="text-xs leading-5 text-[#888888]">
      Autocomplete includes SQL keywords, registered table names, and inferred column names.
      <span v-if="tableNames.length">Available tables: {{ tableNames.join(', ') }}.</span>
    </p>
  </div>
</template>
