// thread.smithy
// A simple service that calculates the factorial of a whole number


// Tell the code generator how to reference symbols defined in this namespace
metadata package = [ { namespace: "org.example.interfaces.thread", crate: "thread" } ]

namespace org.example.interfaces.thread

use org.wasmcloud.model#wasmbus
use org.wasmcloud.model#U32
use org.wasmcloud.model#U64

/// The Thread service has a single method, calculate, which
/// calculates the factorial of its whole number parameter.
@wasmbus(
    contractId: "wasmcloud:thread",
    actorReceive: true,
    providerReceive: true )
service Thread {
  version: "0.1",
  operations: [ StartThread,Tick ]
}

operation StartThread {
    input: StartThreadRequest,
    output: StartThreadResponse,
}
operation Tick{
    //timestamp
    input: U64,
    output: U32,
}
/// Parameters sent for StartThreadRequest
structure StartThreadRequest {
    
    @required
    @n(0)
    game_id: String,
    /// sleep_interval in millisecond
    @required
    @n(1)
    sleep_interval: U32,
}

/// Response to AuthorizePayment
structure StartThreadResponse {
    
}