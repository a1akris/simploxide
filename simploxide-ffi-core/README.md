# simploxide-ffi-core

A fully asynchrouns raw SimpleX client backed by the SimpleX FFI bindings(see
[simploxide_sxcrt_sys] for setup instructions) that provides:

1. Multi-instance support: run many SimpleX-Chat instances from a single process. Each instance
   is fully isolated and all are served by a single shared worker thread with fair round-robin
   scheduling and per-instance execution caps to prevent starvation.

1. Complete asynchonisity: futures created by the same instance of a client are fully
   independent from each other. The event queue receives events independently from client
   actions.

1. Graceful shutdown with strong guarantees:
    - All commands enqueued before [`RawClient::disconnect`] are guaranteed to execute and
      return their responses.

    - All commands enqueued after [`RawClient::disconnect`] are guaranteed to return
      [`CallError::Failure`] without being executed.

    - You will receive events for as long as the chat instance is active. After disconnect the
      remaining buffered events are delivered and then the event queue closes.

### LICENSE

**Licensed under [AGPL-3.0](../LICENSE-AGPL3)**
