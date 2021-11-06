// This file is generated automatically using wasmcloud/weld-codegen and smithy model definitions
//

#![allow(unused_imports, clippy::ptr_arg, clippy::needless_lifetimes)]
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, io::Write, string::ToString};
use wasmbus_rpc::{
    deserialize, serialize, Context, Message, MessageDispatch, RpcError, RpcResult, SendOpts,
    Timestamp, Transport,
};

pub const SMITHY_VERSION: &str = "1.0";

/// Parameters sent for AuthorizePayment
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct StartThreadRequest {
    /// Token of the payment method to be used
    pub elapsed: u32,
    /// Amount of transaction, in cents.
    #[serde(default)]
    pub game_id: String,
    /// Amount of tax applied to this transaction, in cents
    pub timestamp: u64,
}

/// Response to AuthorizePayment
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct StartThreadResponse {}

/// wasmbus.contractId: wasmcloud:example:thread
/// wasmbus.providerReceive
#[async_trait]
pub trait Thread {
    /// returns the capability contract id for this interface
    fn contract_id() -> &'static str {
        "wasmcloud:example:thread"
    }
    /// AuthorizePayment - Validates that a potential payment transaction
    /// can go through. If this succeeds then we should assume it is safe
    /// to complete a payment. Thread _cannot_ be completed without getting
    /// a validation code (in other words, all thread have to be pre-authorized).
    async fn start_thread(
        &self,
        ctx: &Context,
        arg: &StartThreadRequest,
    ) -> RpcResult<StartThreadResponse>;
    async fn handle_request(
        &self,
        ctx: &Context,
        arg: &StartThreadRequest,
    ) -> RpcResult<StartThreadResponse>;
}

/// ThreadReceiver receives messages defined in the Thread service trait
#[doc(hidden)]
#[async_trait]
pub trait ThreadReceiver: MessageDispatch + Thread {
    async fn dispatch(&self, ctx: &Context, message: &Message<'_>) -> RpcResult<Message<'_>> {
        match message.method {
            "StartThread" => {
                let value: StartThreadRequest = deserialize(message.arg.as_ref())
                    .map_err(|e| RpcError::Deser(format!("message '{}': {}", message.method, e)))?;
                let resp = Thread::start_thread(self, ctx, &value).await?;
                let buf = serialize(&resp)?;
                Ok(Message {
                    method: "Thread.StartThread",
                    arg: Cow::Owned(buf),
                })
            }
            "HandleRequest" => {
                let value: StartThreadRequest = deserialize(message.arg.as_ref())
                    .map_err(|e| RpcError::Deser(format!("message '{}': {}", message.method, e)))?;
                let resp = Thread::handle_request(self, ctx, &value).await?;
                let buf = serialize(&resp)?;
                Ok(Message {
                    method: "Thread.HandleRequest",
                    arg: Cow::Owned(buf),
                })
            }
            _ => Err(RpcError::MethodNotHandled(format!(
                "Thread::{}",
                message.method
            ))),
        }
    }
}

/// ThreadSender sends messages to a Thread service
/// client for sending Thread messages
#[derive(Debug)]
pub struct ThreadSender<T: Transport> {
    transport: T,
}

impl<T: Transport> ThreadSender<T> {
    /// Constructs a ThreadSender with the specified transport
    pub fn via(transport: T) -> Self {
        Self { transport }
    }

    pub fn set_timeout(&self, interval: std::time::Duration) {
        self.transport.set_timeout(interval);
    }
}

#[cfg(target_arch = "wasm32")]
impl ThreadSender<wasmbus_rpc::actor::prelude::WasmHost> {
    /// Constructs a client for sending to a Thread provider
    /// implementing the 'wasmcloud:example:thread' capability contract, with the "default" link
    pub fn new() -> Self {
        let transport = wasmbus_rpc::actor::prelude::WasmHost::to_provider(
            "wasmcloud:example:thread",
            "default",
        )
        .unwrap();
        Self { transport }
    }

    /// Constructs a client for sending to a Thread provider
    /// implementing the 'wasmcloud:example:thread' capability contract, with the specified link name
    pub fn new_with_link(link_name: &str) -> wasmbus_rpc::RpcResult<Self> {
        let transport = wasmbus_rpc::actor::prelude::WasmHost::to_provider(
            "wasmcloud:example:thread",
            link_name,
        )?;
        Ok(Self { transport })
    }
}
#[async_trait]
impl<T: Transport + std::marker::Sync + std::marker::Send> Thread for ThreadSender<T> {
    #[allow(unused)]
    /// AuthorizePayment - Validates that a potential payment transaction
    /// can go through. If this succeeds then we should assume it is safe
    /// to complete a payment. Thread _cannot_ be completed without getting
    /// a validation code (in other words, all thread have to be pre-authorized).
    async fn start_thread(
        &self,
        ctx: &Context,
        arg: &StartThreadRequest,
    ) -> RpcResult<StartThreadResponse> {
        let buf = serialize(arg)?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Thread.StartThread",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;
        let value = deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("response to {}: {}", "StartThread", e)))?;
        Ok(value)
    }
    #[allow(unused)]
    async fn handle_request(
        &self,
        ctx: &Context,
        arg: &StartThreadRequest,
    ) -> RpcResult<StartThreadResponse> {
        let buf = serialize(arg)?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Thread.HandleRequest",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;
        let value = deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("response to {}: {}", "HandleRequest", e)))?;
        Ok(value)
    }
}
