import wasmUrl from '../../../query-engine-wasm/pkg/query_engine_wasm_bg.wasm?url'
import type {
  InferredCsvSchema,
  QueryResult,
  QueryRow,
  QueryWorkerRequest,
  QueryWorkerResponse,
  RegisteredCsvSchema,
} from './queryWorkerProtocol'

interface WasmQueryEngine {
  register_csv(name: string, data: Uint8Array): RegisteredCsvSchema
  execute(sql: string): QueryResult
}

interface WasmQueryEngineConstructor {
  new (): WasmQueryEngine
}

interface WasmQueryEngineModule {
  default: (options?: WasmInitInput | { module_or_path?: WasmInitInput }) => Promise<unknown>
  QueryEngine: WasmQueryEngineConstructor
  infer_schema(data: Uint8Array): InferredCsvSchema
}

type WasmInitInput = RequestInfo | URL | WebAssembly.Module | BufferSource

declare global {
  var __QUERY_IN_WASM_MODULE_OR_PATH__: WasmInitInput | undefined
}

interface WasmRuntime {
  module: WasmQueryEngineModule
  engine: WasmQueryEngine
}

let runtimePromise: Promise<WasmRuntime> | undefined

export async function handleQueryEngineRequest(request: QueryWorkerRequest): Promise<QueryWorkerResponse> {
  try {
    const runtime = await getRuntime()
    const payload = executeRequest(runtime, request)

    return {
      id: request.id,
      type: 'success',
      requestType: request.type,
      payload,
    } as QueryWorkerResponse
  } catch (error) {
    return {
      id: request.id,
      type: 'error',
      requestType: request.type,
      error: toErrorMessage(error),
    }
  }
}

async function getRuntime() {
  runtimePromise ??= loadRuntime()

  return runtimePromise
}

async function loadRuntime(): Promise<WasmRuntime> {
  const wasmModule = await import('@query-in/query-engine-wasm')
  const moduleOrPath = globalThis.__QUERY_IN_WASM_MODULE_OR_PATH__ ?? wasmUrl

  await wasmModule.default({ module_or_path: moduleOrPath })

  return {
    module: wasmModule,
    engine: new wasmModule.QueryEngine(),
  }
}

function executeRequest(runtime: WasmRuntime, request: QueryWorkerRequest) {
  switch (request.type) {
    case 'initialize':
      return { ready: true }
    case 'registerCsv':
      return runtime.engine.register_csv(request.payload.name, toBytes(request.payload.data))
    case 'inferSchema':
      return runtime.module.infer_schema(toBytes(request.payload.data))
    case 'execute':
      return normalizeQueryResult(runtime.engine.execute(request.payload.sql))
  }
}

function toBytes(data: ArrayBuffer) {
  return new Uint8Array(data)
}

function normalizeQueryResult(result: QueryResult): QueryResult {
  return {
    ...result,
    rows: result.rows.map(normalizeQueryRow),
  }
}

function normalizeQueryRow(row: QueryRow | Map<string, string | null>): QueryRow {
  if (row instanceof Map) {
    return Object.fromEntries(row)
  }

  return row
}

function toErrorMessage(error: unknown) {
  if (error instanceof Error) {
    return error.message
  }

  if (typeof error === 'string') {
    return error
  }

  return 'Query worker request failed.'
}
