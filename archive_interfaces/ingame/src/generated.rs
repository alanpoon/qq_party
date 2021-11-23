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
    pub fn spend_bean_request(
        &self,
        request: SpendBeanRequest,
    ) -> HandlerResult<SpendBeanResponse> {
        host_call(
            &self.binding,
            "qq:ingame",
            "SpendBeanRequest",
            &serialize(request)?,
        )
        .map(|vec| {
            let resp = deserialize::<SpendBeanResponse>(vec.as_ref()).unwrap();
            resp
        })
        .map_err(|e| e.into())
    }
}

pub struct Handlers {}

impl Handlers {
    pub fn register_spend_bean_request(
        f: fn(SpendBeanRequest) -> HandlerResult<SpendBeanResponse>,
    ) {
        *SPEND_BEAN_REQUEST.write().unwrap() = Some(f);
        register_function(&"SpendBeanRequest", spend_bean_request_wrapper);
    }
}

lazy_static! {
    static ref SPEND_BEAN_REQUEST: RwLock<Option<fn(SpendBeanRequest) -> HandlerResult<SpendBeanResponse>>> =
        RwLock::new(None);
}

fn spend_bean_request_wrapper(input_payload: &[u8]) -> CallResult {
    let input = deserialize::<SpendBeanRequest>(input_payload)?;
    let lock = SPEND_BEAN_REQUEST.read().unwrap().unwrap();
    let result = lock(input)?;
    Ok(serialize(result)?)
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct SpendBeanRequest {
    #[serde(rename = "amount")]
    pub amount: u16,
    #[serde(rename = "user_id")]
    pub user_id: u32,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct SpendBeanResponse {
    #[serde(rename = "error_code")]
    pub error_code: u32,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct UserBeanAmount {
    #[serde(rename = "user_id")]
    pub user_id: u32,
    #[serde(rename = "amount")]
    pub amount: u32,
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
