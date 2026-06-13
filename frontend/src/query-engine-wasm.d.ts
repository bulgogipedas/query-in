declare module '@query-in/query-engine-wasm' {
  import type {
    InferredCsvSchema,
    QueryResult,
    RegisteredCsvSchema,
  } from './workers/queryWorkerProtocol'

  export default function init(): Promise<unknown>

  export class QueryEngine {
    constructor()
    register_csv(name: string, data: Uint8Array): RegisteredCsvSchema
    execute(sql: string): QueryResult
  }

  export function infer_schema(data: Uint8Array): InferredCsvSchema
}
