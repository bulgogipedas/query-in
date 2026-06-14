import { computed, onScopeDispose, ref, shallowRef } from 'vue'
import type {
  QueryResult,
  QueryWorkerRequest,
  QueryWorkerRequestType,
  QueryWorkerResponse,
  QueryWorkerSuccessPayload,
  RegisteredCsvSchema,
} from '../workers/queryWorkerProtocol'

type PendingRequest<T extends QueryWorkerRequestType> = {
  resolve: (payload: QueryWorkerSuccessPayload[T]) => void
  reject: (error: Error) => void
}

type PendingRequests = Map<string, PendingRequest<QueryWorkerRequestType>>

let requestSequence = 0

export function useQueryEngine() {
  const worker = shallowRef<Worker | null>(null)
  const pendingRequests: PendingRequests = new Map()
  const isReady = ref(false)
  const isBusy = ref(false)
  const error = ref<string | null>(null)
  const registeredSchemas = ref<RegisteredCsvSchema[]>([])
  const latestResult = ref<QueryResult | null>(null)

  const tableCount = computed(() => registeredSchemas.value.length)

  async function initialize() {
    await sendRequest({
      id: nextRequestId(),
      type: 'initialize',
    })

    isReady.value = true
  }

  async function registerCsv(name: string, data: ArrayBuffer) {
    const schema = await sendRequest({
      id: nextRequestId(),
      type: 'registerCsv',
      payload: {
        name,
        data,
      },
    })

    registeredSchemas.value = [
      ...registeredSchemas.value.filter((registeredSchema) => registeredSchema.name !== schema.name),
      schema,
    ]

    return schema
  }

  async function inferSchema(data: ArrayBuffer) {
    return sendRequest({
      id: nextRequestId(),
      type: 'inferSchema',
      payload: {
        data,
      },
    })
  }

  async function execute(sql: string) {
    const result = await sendRequest({
      id: nextRequestId(),
      type: 'execute',
      payload: {
        sql,
      },
    })

    latestResult.value = result

    return result
  }

  function resetError() {
    error.value = null
  }

  function terminate() {
    rejectPendingRequests(new Error('Query engine worker was terminated.'))
    worker.value?.terminate()
    worker.value = null
    isReady.value = false
    isBusy.value = false
  }

  function getWorker() {
    if (worker.value) {
      return worker.value
    }

    const nextWorker = new Worker(new URL('../workers/query.worker.ts', import.meta.url), {
      type: 'module',
    })

    nextWorker.onmessage = (event: MessageEvent<QueryWorkerResponse>) => {
      handleWorkerResponse(event.data)
    }

    nextWorker.onerror = (event) => {
      rejectPendingRequests(new Error(event.message || 'Query engine worker failed.'))
      error.value = event.message || 'Query engine worker failed.'
      isBusy.value = false
    }

    worker.value = nextWorker

    return nextWorker
  }

  async function sendRequest<T extends QueryWorkerRequest>(request: T) {
    resetError()
    isBusy.value = true

    const targetWorker = getWorker()

    return new Promise<QueryWorkerSuccessPayload[T['type']]>((resolve, reject) => {
      pendingRequests.set(request.id, {
        resolve: resolve as PendingRequest<QueryWorkerRequestType>['resolve'],
        reject,
      })

      targetWorker.postMessage(request, transferListFor(request))
    }).finally(() => {
      isBusy.value = pendingRequests.size > 0
    })
  }

  function handleWorkerResponse(response: QueryWorkerResponse) {
    const pendingRequest = pendingRequests.get(response.id)

    if (!pendingRequest) {
      return
    }

    pendingRequests.delete(response.id)

    if (response.type === 'error') {
      const requestError = new Error(response.error)
      error.value = response.error
      pendingRequest.reject(requestError)
      return
    }

    pendingRequest.resolve(response.payload)
  }

  function rejectPendingRequests(reason: Error) {
    for (const pendingRequest of pendingRequests.values()) {
      pendingRequest.reject(reason)
    }

    pendingRequests.clear()
  }

  onScopeDispose(() => {
    terminate()
  })

  return {
    isReady,
    isBusy,
    error,
    registeredSchemas,
    latestResult,
    tableCount,
    initialize,
    registerCsv,
    inferSchema,
    execute,
    resetError,
    terminate,
  }
}

function nextRequestId() {
  requestSequence += 1

  return `query-request-${requestSequence}`
}

function transferListFor(request: QueryWorkerRequest): Transferable[] {
  if (request.type === 'registerCsv' || request.type === 'inferSchema') {
    return [request.payload.data]
  }

  return []
}
