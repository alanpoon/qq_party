// thread.smithy
//
// Sample api for a simple thread provider
//

// Tell the code generator how to reference symbols defined in this namespace
metadata package = [{
    namespace: "org.wasmcloud.examples.thread",
    crate: "wasmcloud_example_thread",
    py_module: "wasmcloud_example_thread",
}]

namespace org.wasmcloud.examples.thread

use org.wasmcloud.model#wasmbus
use org.wasmcloud.model#U32
use org.wasmcloud.model#U64

@wasmbus(
    contractId: "wasmcloud:thread",
    actorReceive: true,
    providerReceive: true )
service Thread {
  version: "0.1",
  operations: [ StartThread,HandleRequest,Now ]
}

/// AuthorizePayment - Validates that a potential payment transaction
/// can go through. If this succeeds then we should assume it is safe
/// to complete a payment. Thread _cannot_ be completed without getting
/// a validation code (in other words, all thread have to be pre-authorized).
operation Now{
  input: StartThreadRequest,
  output: U64,
}
operation StartThread {
    input: StartThreadRequest,
    output: StartThreadResponse,
}
operation HandleRequest{
    input: StartThreadRequest,
    output: StartThreadResponse,
}
/// Parameters sent for AuthorizePayment
structure StartThreadRequest {
    /// Amount of transaction, in cents.
    @required
    @n(0)
    game_id: String,

    /// Amount of tax applied to this transaction, in cents
    @required
    @n(1)
    timestamp: U64,

    /// Token of the payment method to be used
    @required
    @n(2)
    elapsed: U32,

    @required
    @n(3)
    sleep_interval: U32,

    @n(4)
    subject: String,
}


/// Response to AuthorizePayment
structure StartThreadResponse {
    
}
