extern crate rmp_serde as rmps;
use rmps::{Deserializer, Serializer};
use serde::{Deserialize, Serialize};
use std::io::Cursor;

extern crate log;
extern crate wapc_guest as guest;
use guest::prelude::*;

use lazy_static::lazy_static;
use std::sync::RwLock;

pub struct Host {
    binding: String,
}

impl Default for Host {
    fn default() -> Self {
        Host {
            binding: "default".to_string(),
        }
    }
}

/// Creates a named host binding
pub fn host(binding: &str) -> Host {
    Host {
        binding: binding.to_string(),
    }
}

/// Creates the default host binding
pub fn default() -> Host {
    Host::default()
}

impl Host {
    pub fn gift_request(&self, request: GiftRequest) -> HandlerResult<GiftResponse> {
        host_call(
            &self.binding,
            "qq:room",
            "GiftRequest",
            &serialize(request)?,
        )
        .map(|vec| {
            let resp = deserialize::<GiftResponse>(vec.as_ref()).unwrap();
            resp
        })
        .map_err(|e| e.into())
    }

    pub fn message_request(&self, request: MessageRequest) -> HandlerResult<MessageResponse> {
        host_call(
            &self.binding,
            "qq:room",
            "MessageRequest",
            &serialize(request)?,
        )
        .map(|vec| {
            let resp = deserialize::<MessageResponse>(vec.as_ref()).unwrap();
            resp
        })
        .map_err(|e| e.into())
    }
}

pub struct Handlers {}

impl Handlers {
    pub fn register_gift_request(f: fn(GiftRequest) -> HandlerResult<GiftResponse>) {
        *GIFT_REQUEST.write().unwrap() = Some(f);
        register_function(&"GiftRequest", gift_request_wrapper);
    }
    pub fn register_message_request(f: fn(MessageRequest) -> HandlerResult<MessageResponse>) {
        *MESSAGE_REQUEST.write().unwrap() = Some(f);
        register_function(&"MessageRequest", message_request_wrapper);
    }
}

lazy_static! {
    static ref GIFT_REQUEST: RwLock<Option<fn(GiftRequest) -> HandlerResult<GiftResponse>>> =
        RwLock::new(None);
    static ref MESSAGE_REQUEST: RwLock<Option<fn(MessageRequest) -> HandlerResult<MessageResponse>>> =
        RwLock::new(None);
}

fn gift_request_wrapper(input_payload: &[u8]) -> CallResult {
    let input = deserialize::<GiftRequest>(input_payload)?;
    let lock = GIFT_REQUEST.read().unwrap().unwrap();
    let result = lock(input)?;
    Ok(serialize(result)?)
}

fn message_request_wrapper(input_payload: &[u8]) -> CallResult {
    let input = deserialize::<MessageRequest>(input_payload)?;
    let lock = MESSAGE_REQUEST.read().unwrap().unwrap();
    let result = lock(input)?;
    Ok(serialize(result)?)
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct GiftRequest {
    #[serde(rename = "room_number")]
    pub room_number: String,
    #[serde(rename = "gift_id")]
    pub gift_id: u32,
    #[serde(rename = "user_id")]
    pub user_id: u32,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct GiftResponse {
    #[serde(rename = "gift_id")]
    pub gift_id: u32,
    #[serde(rename = "user_id")]
    pub user_id: u32,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct MessageRequest {
    #[serde(rename = "room_number")]
    pub room_number: String,
    #[serde(rename = "user_id")]
    pub user_id: u32,
    #[serde(rename = "msg")]
    pub msg: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct MessageResponse {
    #[serde(rename = "user_id")]
    pub user_id: u32,
    #[serde(rename = "msg")]
    pub msg: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct JoinRoomRequest {
    #[serde(rename = "user_id")]
    pub user_id: u32,
    #[serde(rename = "room_number")]
    pub room_number: String,
}

/// The standard function for serializing codec structs into a format that can be
/// used for message exchange between actor and host. Use of any other function to
/// serialize could result in breaking incompatibilities.
pub fn serialize<T>(
    item: T,
) -> ::std::result::Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>>
where
    T: Serialize,
{
    let mut buf = Vec::new();
    item.serialize(&mut Serializer::new(&mut buf).with_struct_map())?;
    Ok(buf)
}

/// The standard function for de-serializing codec structs from a format suitable
/// for message exchange between actor and host. Use of any other function to
/// deserialize could result in breaking incompatibilities.
pub fn deserialize<'de, T: Deserialize<'de>>(
    buf: &[u8],
) -> ::std::result::Result<T, Box<dyn std::error::Error + Send + Sync>> {
    let mut de = Deserializer::new(Cursor::new(buf));
    match Deserialize::deserialize(&mut de) {
        Ok(t) => Ok(t),
        Err(e) => Err(format!("Failed to de-serialize: {}", e).into()),
    }
}
