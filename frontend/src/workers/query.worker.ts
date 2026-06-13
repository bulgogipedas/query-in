import { handleQueryEngineRequest } from './queryEngineRuntime'
import type { QueryWorkerRequest, QueryWorkerResponse } from './queryWorkerProtocol'

interface QueryWorkerScope {
  onmessage: ((event: MessageEvent<QueryWorkerRequest>) => void) | null
  postMessage(response: QueryWorkerResponse): void
}

const workerScope = self as unknown as QueryWorkerScope

workerScope.onmessage = (event: MessageEvent<QueryWorkerRequest>) => {
  void handleQueryEngineRequest(event.data).then(postResponse)
}

function postResponse(response: QueryWorkerResponse) {
  workerScope.postMessage(response)
}
