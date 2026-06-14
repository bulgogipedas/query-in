declare module '@query-in/query-engine-wasm' {
  import type {
    InferredCsvSchema,
    QueryResult,
    RegisteredCsvSchema,
  } from './workers/queryWorkerProtocol'

  type WasmInitInput = RequestInfo | URL | WebAssembly.Module | BufferSource

  export default function init(options?: WasmInitInput | { module_or_path?: WasmInitInput }): Promise<unknown>

  export class QueryEngine {
    constructor()
    register_csv(name: string, data: Uint8Array): RegisteredCsvSchema
    execute(sql: string): QueryResult
  }

  export function infer_schema(data: Uint8Array): InferredCsvSchema
}
