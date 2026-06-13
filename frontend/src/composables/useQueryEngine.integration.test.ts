import { afterEach, beforeEach, describe, expect, it } from 'vitest'
import { readFile } from 'node:fs/promises'
import { effectScope } from 'vue'
import { useQueryEngine } from './useQueryEngine'
import { handleQueryEngineRequest } from '../workers/queryEngineRuntime'
import type { QueryWorkerRequest, QueryWorkerResponse } from '../workers/queryWorkerProtocol'

const originalWorker = globalThis.Worker

describe('useQueryEngine', () => {
  beforeEach(async () => {
    globalThis.Worker = InlineQueryWorker as unknown as typeof Worker
    globalThis.__QUERY_IN_WASM_MODULE_OR_PATH__ = await readFile(
      new URL('../../../query-engine-wasm/pkg/query_engine_wasm_bg.wasm', import.meta.url),
    )
  })

  afterEach(() => {
    globalThis.Worker = originalWorker
    globalThis.__QUERY_IN_WASM_MODULE_OR_PATH__ = undefined
  })

  it('registers sample CSV data and executes a SELECT query through worker messaging', async () => {
    const scope = effectScope()
    const engine = scope.run(() => useQueryEngine())

    if (!engine) {
      throw new Error('Query engine composable was not created.')
    }

    await engine.initialize()

    const csv = encodeCsv(`id,name,score
1,Ada,42
2,Grace,37
`)

    const schema = await engine.registerCsv('scores', csv)
    const result = await engine.execute('SELECT name, score FROM scores LIMIT 1')

    expect(engine.isReady.value).toBe(true)
    expect(engine.tableCount.value).toBe(1)
    expect(schema).toMatchObject({
      name: 'scores',
      row_count: 2,
    })
    expect(schema.columns.map((column) => [column.name, column.data_type])).toEqual([
      ['id', 'Int64'],
      ['name', 'Utf8'],
      ['score', 'Int64'],
    ])
    expect(result.rows).toEqual([
      {
        name: 'Ada',
        score: '42',
      },
    ])
    expect(engine.latestResult.value).toEqual(result)

    scope.stop()
  })
})

class InlineQueryWorker {
  onmessage: ((event: MessageEvent<QueryWorkerResponse>) => void) | null = null
  onerror: ((event: ErrorEvent) => void) | null = null

  postMessage(request: QueryWorkerRequest) {
    void handleQueryEngineRequest(request)
      .then((response) => {
        this.onmessage?.({ data: response } as MessageEvent<QueryWorkerResponse>)
      })
      .catch((error: unknown) => {
        this.onerror?.({
          message: error instanceof Error ? error.message : 'Inline query worker failed.',
        } as ErrorEvent)
      })
  }

  terminate() {}
}

function encodeCsv(csv: string) {
  return new TextEncoder().encode(csv).buffer
}
