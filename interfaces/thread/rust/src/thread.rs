// This file is @generated by wasmcloud/weld-codegen 0.4.6.
// It is not intended for manual editing.
// namespace: org.example.interfaces.thread

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

/// Parameters sent for StartThreadRequest
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct StartThreadRequest {
    #[serde(default)]
    pub game_id: String,
    /// sleep_interval in millisecond
    #[serde(default)]
    pub sleep_interval: u32,
}

// Encode StartThreadRequest as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_start_thread_request<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &StartThreadRequest,
) -> RpcResult<()>
where
    <W as wasmbus_rpc::cbor::Write>::Error: std::fmt::Display,
{
    e.map(2)?;
    e.str("game_id")?;
    e.str(&val.game_id)?;
    e.str("sleep_interval")?;
    e.u32(val.sleep_interval)?;
    Ok(())
}

// Decode StartThreadRequest from cbor input stream
#[doc(hidden)]
pub fn decode_start_thread_request(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<StartThreadRequest, RpcError> {
    let __result = {
        let mut game_id: Option<String> = None;
        let mut sleep_interval: Option<u32> = None;

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
                    0 => game_id = Some(d.str()?.to_string()),
                    1 => sleep_interval = Some(d.u32()?),
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.fixed_map()?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "game_id" => game_id = Some(d.str()?.to_string()),
                    "sleep_interval" => sleep_interval = Some(d.u32()?),
                    _ => d.skip()?,
                }
            }
        }
        StartThreadRequest {
            game_id: if let Some(__x) = game_id {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field StartThreadRequest.game_id (#0)".to_string(),
                ));
            },

            sleep_interval: if let Some(__x) = sleep_interval {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field StartThreadRequest.sleep_interval (#1)".to_string(),
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
) -> RpcResult<()>
where
    <W as wasmbus_rpc::cbor::Write>::Error: std::fmt::Display,
{
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
/// The Thread service has a single method, calculate, which
/// calculates the factorial of its whole number parameter.
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
    async fn tick(&self, ctx: &Context, arg: &u64) -> RpcResult<u32>;
}

/// ThreadReceiver receives messages defined in the Thread service trait
/// The Thread service has a single method, calculate, which
/// calculates the factorial of its whole number parameter.
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
            "Tick" => {
                let value: u64 = wasmbus_rpc::common::deserialize(&message.arg)
                    .map_err(|e| RpcError::Deser(format!("'U64': {}", e)))?;

                let resp = Thread::tick(self, ctx, &value).await?;
                let buf = wasmbus_rpc::common::serialize(&resp)?;

                Ok(Message {
                    method: "Thread.Tick",
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
/// The Thread service has a single method, calculate, which
/// calculates the factorial of its whole number parameter.
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
    async fn tick(&self, ctx: &Context, arg: &u64) -> RpcResult<u32> {
        let buf = wasmbus_rpc::common::serialize(arg)?;

        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Thread.Tick",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: u32 = wasmbus_rpc::common::deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("'{}': U32", e)))?;
        Ok(value)
    }
}
