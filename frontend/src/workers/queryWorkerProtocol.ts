export type QueryWorkerRequest =
  | {
      id: string
      type: 'initialize'
    }
  | {
      id: string
      type: 'registerCsv'
      payload: {
        name: string
        data: ArrayBuffer
      }
    }
  | {
      id: string
      type: 'inferSchema'
      payload: {
        data: ArrayBuffer
      }
    }
  | {
      id: string
      type: 'execute'
      payload: {
        sql: string
      }
    }

export type QueryWorkerRequestType = QueryWorkerRequest['type']

export interface QueryColumnSchema {
  name: string
  data_type: 'Boolean' | 'Int64' | 'Float64' | 'Utf8'
  nullable: boolean
}

export interface RegisteredCsvSchema {
  name: string
  columns: QueryColumnSchema[]
  row_count: number
}

export interface InferredCsvSchema {
  columns: QueryColumnSchema[]
  row_count: number
}

export type QueryRow = Record<string, string | null>

export interface QueryResult {
  rows: QueryRow[]
  schema: QueryColumnSchema[]
  row_count: number
  elapsed_ms: number
}

export type QueryWorkerSuccessPayload = {
  initialize: { ready: true }
  registerCsv: RegisteredCsvSchema
  inferSchema: InferredCsvSchema
  execute: QueryResult
}

export type QueryWorkerResponse =
  | {
      id: string
      type: 'success'
      requestType: QueryWorkerRequestType
      payload: QueryWorkerSuccessPayload[QueryWorkerRequestType]
    }
  | {
      id: string
      type: 'error'
      requestType: QueryWorkerRequestType
      error: string
    }
