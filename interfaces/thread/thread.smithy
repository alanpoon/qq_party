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
  operations: [ StartThread,HandleRequest,Now ]
}
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