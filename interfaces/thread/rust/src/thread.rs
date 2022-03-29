// This file is generated automatically using wasmcloud/weld-codegen 0.4.3

#[allow(unused_imports)]
use async_trait::async_trait;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::{borrow::Borrow, borrow::Cow, io::Write, string::ToString};
#[allow(unused_imports)]
use wasmbus_rpc::{
    cbor::*,
    common::{
        deserialize, message_format, serialize, Context, Message, MessageDispatch, MessageFormat,
        SendOpts, Transport,
    },
    error::{RpcError, RpcResult},
    Timestamp,
};

#[allow(dead_code)]
pub const SMITHY_VERSION: &str = "1.0";

/// Parameters sent for AuthorizePayment
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct StartThreadRequest {
    /// Token of the payment method to be used
    #[serde(default)]
    pub elapsed: u32,
    /// Amount of transaction, in cents.
    #[serde(default)]
    pub game_id: String,
    #[serde(default)]
    pub sleep_interval: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// Amount of tax applied to this transaction, in cents
    #[serde(default)]
    pub timestamp: u64,
}

// Encode StartThreadRequest as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_start_thread_request<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &StartThreadRequest,
) -> RpcResult<()> {
    e.map(5)?;
    e.str("elapsed")?;
    e.u32(val.elapsed)?;
    e.str("game_id")?;
    e.str(&val.game_id)?;
    e.str("sleep_interval")?;
    e.u32(val.sleep_interval)?;
    if let Some(val) = val.subject.as_ref() {
        e.str("subject")?;
        e.str(val)?;
    } else {
        e.null()?;
    }
    e.str("timestamp")?;
    e.u64(val.timestamp)?;
    Ok(())
}

// Decode StartThreadRequest from cbor input stream
#[doc(hidden)]
pub fn decode_start_thread_request(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<StartThreadRequest, RpcError> {
    let __result = {
        let mut elapsed: Option<u32> = None;
        let mut game_id: Option<String> = None;
        let mut sleep_interval: Option<u32> = None;
        let mut subject: Option<Option<String>> = Some(None);
        let mut timestamp: Option<u64> = None;

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct StartThreadRequest, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.fixed_array()?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => elapsed = Some(d.u32()?),
                    1 => game_id = Some(d.str()?.to_string()),
                    2 => sleep_interval = Some(d.u32()?),
                    3 => {
                        subject = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    4 => timestamp = Some(d.u64()?),
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.fixed_map()?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "elapsed" => elapsed = Some(d.u32()?),
                    "game_id" => game_id = Some(d.str()?.to_string()),
                    "sleep_interval" => sleep_interval = Some(d.u32()?),
                    "subject" => {
                        subject = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    "timestamp" => timestamp = Some(d.u64()?),
                    _ => d.skip()?,
                }
            }
        }
        StartThreadRequest {
            elapsed: if let Some(__x) = elapsed {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field StartThreadRequest.elapsed (#0)".to_string(),
                ));
            },

            game_id: if let Some(__x) = game_id {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field StartThreadRequest.game_id (#1)".to_string(),
                ));
            },

            sleep_interval: if let Some(__x) = sleep_interval {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field StartThreadRequest.sleep_interval (#2)".to_string(),
                ));
            },
            subject: subject.unwrap(),

            timestamp: if let Some(__x) = timestamp {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field StartThreadRequest.timestamp (#4)".to_string(),
                ));
            },
        }
    };
    Ok(__result)
}
/// Response to AuthorizePayment
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct StartThreadResponse {}

// Encode StartThreadResponse as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_start_thread_response<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    _val: &StartThreadResponse,
) -> RpcResult<()> {
    e.map(0)?;
    Ok(())
}

// Decode StartThreadResponse from cbor input stream
#[doc(hidden)]
pub fn decode_start_thread_response(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<StartThreadResponse, RpcError> {
    let __result = {
        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct StartThreadResponse, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.fixed_array()?;
            for __i in 0..(len as usize) {
                d.skip()?;
            }
        } else {
            let len = d.fixed_map()?;
            for __i in 0..(len as usize) {
                d.str()?;
                d.skip()?;
            }
        }
        StartThreadResponse {}
    };
    Ok(__result)
}
/// wasmbus.contractId: wasmcloud:thread
/// wasmbus.providerReceive
/// wasmbus.actorReceive
#[async_trait]
pub trait Thread {
    /// returns the capability contract id for this interface
    fn contract_id() -> &'static str {
        "wasmcloud:thread"
    }
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
    /// AuthorizePayment - Validates that a potential payment transaction
    /// can go through. If this succeeds then we should assume it is safe
    /// to complete a payment. Thread _cannot_ be completed without getting
    /// a validation code (in other words, all thread have to be pre-authorized).
    async fn now(&self, ctx: &Context, arg: &StartThreadRequest) -> RpcResult<u64>;
}

/// ThreadReceiver receives messages defined in the Thread service trait
#[doc(hidden)]
#[async_trait]
pub trait ThreadReceiver: MessageDispatch + Thread {
    async fn dispatch<'disp__, 'ctx__, 'msg__>(
        &'disp__ self,
        ctx: &'ctx__ Context,
        message: &Message<'msg__>,
    ) -> Result<Message<'msg__>, RpcError> {
        match message.method {
            "StartThread" => {
                let value: StartThreadRequest = wasmbus_rpc::common::deserialize(&message.arg)
                    .map_err(|e| RpcError::Deser(format!("'StartThreadRequest': {}", e)))?;
                let resp = Thread::start_thread(self, ctx, &value).await?;
                let buf = wasmbus_rpc::common::serialize(&resp)?;
                Ok(Message {
                    method: "Thread.StartThread",
                    arg: Cow::Owned(buf),
                })
            }
            "HandleRequest" => {
                let value: StartThreadRequest = wasmbus_rpc::common::deserialize(&message.arg)
                    .map_err(|e| RpcError::Deser(format!("'StartThreadRequest': {}", e)))?;
                let resp = Thread::handle_request(self, ctx, &value).await?;
                let buf = wasmbus_rpc::common::serialize(&resp)?;
                Ok(Message {
                    method: "Thread.HandleRequest",
                    arg: Cow::Owned(buf),
                })
            }
            "Now" => {
                let value: StartThreadRequest = wasmbus_rpc::common::deserialize(&message.arg)
                    .map_err(|e| RpcError::Deser(format!("'StartThreadRequest': {}", e)))?;
                let resp = Thread::now(self, ctx, &value).await?;
                let buf = wasmbus_rpc::common::serialize(&resp)?;
                Ok(Message {
                    method: "Thread.Now",
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

#[cfg(not(target_arch = "wasm32"))]
impl<'send> ThreadSender<wasmbus_rpc::provider::ProviderTransport<'send>> {
    /// Constructs a Sender using an actor's LinkDefinition,
    /// Uses the provider's HostBridge for rpc
    pub fn for_actor(ld: &'send wasmbus_rpc::core::LinkDefinition) -> Self {
        Self {
            transport: wasmbus_rpc::provider::ProviderTransport::new(ld, None),
        }
    }
}
#[cfg(target_arch = "wasm32")]
impl ThreadSender<wasmbus_rpc::actor::prelude::WasmHost> {
    /// Constructs a client for actor-to-actor messaging
    /// using the recipient actor's public key
    pub fn to_actor(actor_id: &str) -> Self {
        let transport =
            wasmbus_rpc::actor::prelude::WasmHost::to_actor(actor_id.to_string()).unwrap();
        Self { transport }
    }
}

#[cfg(target_arch = "wasm32")]
impl ThreadSender<wasmbus_rpc::actor::prelude::WasmHost> {
    /// Constructs a client for sending to a Thread provider
    /// implementing the 'wasmcloud:thread' capability contract, with the "default" link
    pub fn new() -> Self {
        let transport =
            wasmbus_rpc::actor::prelude::WasmHost::to_provider("wasmcloud:thread", "default")
                .unwrap();
        Self { transport }
    }

    /// Constructs a client for sending to a Thread provider
    /// implementing the 'wasmcloud:thread' capability contract, with the specified link name
    pub fn new_with_link(link_name: &str) -> wasmbus_rpc::error::RpcResult<Self> {
        let transport =
            wasmbus_rpc::actor::prelude::WasmHost::to_provider("wasmcloud:thread", link_name)?;
        Ok(Self { transport })
    }
}
#[async_trait]
impl<T: Transport + std::marker::Sync + std::marker::Send> Thread for ThreadSender<T> {
    #[allow(unused)]
    async fn start_thread(
        &self,
        ctx: &Context,
        arg: &StartThreadRequest,
    ) -> RpcResult<StartThreadResponse> {
        let buf = wasmbus_rpc::common::serialize(arg)?;
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

        let value: StartThreadResponse = wasmbus_rpc::common::deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("'{}': StartThreadResponse", e)))?;
        Ok(value)
    }
    #[allow(unused)]
    async fn handle_request(
        &self,
        ctx: &Context,
        arg: &StartThreadRequest,
    ) -> RpcResult<StartThreadResponse> {
        let buf = wasmbus_rpc::common::serialize(arg)?;
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

        let value: StartThreadResponse = wasmbus_rpc::common::deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("'{}': StartThreadResponse", e)))?;
        Ok(value)
    }
    #[allow(unused)]
    /// AuthorizePayment - Validates that a potential payment transaction
    /// can go through. If this succeeds then we should assume it is safe
    /// to complete a payment. Thread _cannot_ be completed without getting
    /// a validation code (in other words, all thread have to be pre-authorized).
    async fn now(&self, ctx: &Context, arg: &StartThreadRequest) -> RpcResult<u64> {
        let buf = wasmbus_rpc::common::serialize(arg)?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Thread.Now",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: u64 = wasmbus_rpc::common::deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("'{}': U64", e)))?;
        Ok(value)
    }
}
