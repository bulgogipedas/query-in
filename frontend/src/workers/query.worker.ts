import type {
  InferredCsvSchema,
  QueryResult,
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
  default: () => Promise<unknown>
  QueryEngine: WasmQueryEngineConstructor
  infer_schema(data: Uint8Array): InferredCsvSchema
}

interface WasmRuntime {
  module: WasmQueryEngineModule
  engine: WasmQueryEngine
}

interface QueryWorkerScope {
  onmessage: ((event: MessageEvent<QueryWorkerRequest>) => void) | null
  postMessage(response: QueryWorkerResponse): void
}

const workerScope = self as unknown as QueryWorkerScope

let runtimePromise: Promise<WasmRuntime> | undefined

workerScope.onmessage = (event: MessageEvent<QueryWorkerRequest>) => {
  void handleRequest(event.data)
}

async function handleRequest(request: QueryWorkerRequest) {
  try {
    const runtime = await getRuntime()
    const payload = executeRequest(runtime, request)

    postResponse({
      id: request.id,
      type: 'success',
      requestType: request.type,
      payload,
    } as QueryWorkerResponse)
  } catch (error) {
    postResponse({
      id: request.id,
      type: 'error',
      requestType: request.type,
      error: toErrorMessage(error),
    })
  }
}

async function getRuntime() {
  runtimePromise ??= loadRuntime()

  return runtimePromise
}

async function loadRuntime(): Promise<WasmRuntime> {
  const wasmModule = await import('@query-in/query-engine-wasm')

  await wasmModule.default()

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
      return runtime.engine.execute(request.payload.sql)
  }
}

function toBytes(data: ArrayBuffer) {
  return new Uint8Array(data)
}

function postResponse(response: QueryWorkerResponse) {
  workerScope.postMessage(response)
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
