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
    pub fn gateway_publish(&self, request: BrokerMessage) -> HandlerResult<Vec<u8>> {
        host_call(
            &self.binding,
            "qq:gateway",
            "GatewayPublish",
            &serialize(request)?,
        )
        .map(|vec| {
            let resp = deserialize::<Vec<u8>>(vec.as_ref()).unwrap();
            resp
        })
        .map_err(|e| e.into())
    }
}

pub struct Handlers {}

impl Handlers {
    pub fn register_gateway_publish(f: fn(BrokerMessage) -> HandlerResult<Vec<u8>>) {
        *GATEWAY_PUBLISH.write().unwrap() = Some(f);
        register_function(&"GatewayPublish", gateway_publish_wrapper);
    }
}

lazy_static! {
    static ref GATEWAY_PUBLISH: RwLock<Option<fn(BrokerMessage) -> HandlerResult<Vec<u8>>>> =
        RwLock::new(None);
}

fn gateway_publish_wrapper(input_payload: &[u8]) -> CallResult {
    let input = deserialize::<BrokerMessage>(input_payload)?;
    let lock = GATEWAY_PUBLISH.read().unwrap().unwrap();
    let result = lock(input)?;
    Ok(serialize(result)?)
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct BrokerMessage {
    #[serde(rename = "subject")]
    pub subject: String,
    #[serde(rename = "replyTo")]
    pub reply_to: String,
    #[serde(with = "serde_bytes")]
    #[serde(rename = "body")]
    pub body: Vec<u8>,
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
