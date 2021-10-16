extern crate rmp_serde as rmps;
use rmps::{Deserializer, Serializer};
use serde::{Deserialize, Serialize};
use std::io::Cursor;

#[cfg(feature = "guest")]
extern crate wapc_guest as guest;
#[cfg(feature = "guest")]
use guest::prelude::*;

#[cfg(feature = "guest")]
pub struct Host {
    binding: String,
}

#[cfg(feature = "guest")]
impl Default for Host {
    fn default() -> Self {
        Host {
            binding: "default".to_string(),
        }
    }
}

/// Creates a named host binding
#[cfg(feature = "guest")]
pub fn host(binding: &str) -> Host {
    Host {
        binding: binding.to_string(),
    }
}

/// Creates the default host binding
#[cfg(feature = "guest")]
pub fn default() -> Host {
    Host::default()
}

#[cfg(feature = "guest")]
impl Host {
    pub fn start_thread(&self, request: StartThreadRequest) -> HandlerResult<StartThreadResponse> {
        host_call(
            &self.binding,
            "wasmcloud:game",
            "StartThread",
            &serialize(request)?,
        )
        .map(|vec| {
            let resp = deserialize::<StartThreadResponse>(vec.as_ref()).unwrap();
            resp
        })
        .map_err(|e| e.into())
    }
    pub fn stop_thread(&self, request: StartThreadRequest) -> HandlerResult<StartThreadResponse> {
        host_call(
            &self.binding,
            "wasmcloud:game",
            "StopThread",
            &serialize(request)?,
        )
        .map(|vec| {
            let resp = deserialize::<StartThreadResponse>(vec.as_ref()).unwrap();
            resp
        })
        .map_err(|e| e.into())
    }
    pub fn start_thread_request(
        &self,
        request: StartThreadRequest,
    ) -> HandlerResult<StartThreadResponse> {
        host_call(
            &self.binding,
            "wasmcloud:game",
            "StartThreadRequest",
            &serialize(request)?,
        )
        .map(|vec| {
            let resp = deserialize::<StartThreadResponse>(vec.as_ref()).unwrap();
            resp
        })
        .map_err(|e| e.into())
    }
    pub fn handle_thread(&self, request: StartThreadRequest) -> HandlerResult<StartThreadResponse> {
        host_call(
            &self.binding,
            "wasmcloud:game",
            "HandleThread",
            &serialize(request)?,
        )
        .map(|vec| {
            let resp = deserialize::<StartThreadResponse>(vec.as_ref()).unwrap();
            resp
        })
        .map_err(|e| e.into())
    }
}

#[cfg(feature = "guest")]
pub struct Handlers {}

#[cfg(feature = "guest")]
impl Handlers {
    pub fn register_start_thread(f: fn(StartThreadRequest) -> HandlerResult<StartThreadResponse>) {
        *START_THREAD.write().unwrap() = Some(f);
        register_function(&"StartThread", start_thread_wrapper);
    }
    pub fn register_stop_thread(f: fn(StartThreadRequest) -> HandlerResult<StartThreadResponse>) {
        *STOP_THREAD.write().unwrap() = Some(f);
        register_function(&"StopThread", stop_thread_wrapper);
    }
    pub fn register_start_thread_request(
        f: fn(StartThreadRequest) -> HandlerResult<StartThreadResponse>,
    ) {
        *START_THREAD_REQUEST.write().unwrap() = Some(f);
        register_function(&"StartThreadRequest", start_thread_request_wrapper);
    }
    pub fn register_handle_thread(f: fn(StartThreadRequest) -> HandlerResult<StartThreadResponse>) {
        *HANDLE_THREAD.write().unwrap() = Some(f);
        register_function(&"HandleThread", handle_thread_wrapper);
    }
}

#[cfg(feature = "guest")]
lazy_static::lazy_static! {
static ref START_THREAD: std::sync::RwLock<Option<fn(StartThreadRequest) -> HandlerResult<StartThreadResponse>>> = std::sync::RwLock::new(None);
static ref STOP_THREAD: std::sync::RwLock<Option<fn(StartThreadRequest) -> HandlerResult<StartThreadResponse>>> = std::sync::RwLock::new(None);
static ref START_THREAD_REQUEST: std::sync::RwLock<Option<fn(StartThreadRequest) -> HandlerResult<StartThreadResponse>>> = std::sync::RwLock::new(None);
static ref HANDLE_THREAD: std::sync::RwLock<Option<fn(StartThreadRequest) -> HandlerResult<StartThreadResponse>>> = std::sync::RwLock::new(None);
}

#[cfg(feature = "guest")]
fn start_thread_wrapper(input_payload: &[u8]) -> CallResult {
    let input = deserialize::<StartThreadRequest>(input_payload)?;
    let lock = START_THREAD.read().unwrap().unwrap();
    let result = lock(input)?;
    serialize(result)
}

#[cfg(feature = "guest")]
fn stop_thread_wrapper(input_payload: &[u8]) -> CallResult {
    let input = deserialize::<StartThreadRequest>(input_payload)?;
    let lock = STOP_THREAD.read().unwrap().unwrap();
    let result = lock(input)?;
    serialize(result)
}

#[cfg(feature = "guest")]
fn start_thread_request_wrapper(input_payload: &[u8]) -> CallResult {
    let input = deserialize::<StartThreadRequest>(input_payload)?;
    let lock = START_THREAD_REQUEST.read().unwrap().unwrap();
    let result = lock(input)?;
    serialize(result)
}

#[cfg(feature = "guest")]
fn handle_thread_wrapper(input_payload: &[u8]) -> CallResult {
    let input = deserialize::<StartThreadRequest>(input_payload)?;
    let lock = HANDLE_THREAD.read().unwrap().unwrap();
    let result = lock(input)?;
    serialize(result)
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct StartThreadRequest {
    #[serde(rename = "game_id")]
    pub game_id: String,
    #[serde(rename = "timestamp")]
    pub timestamp: Option<i64>,
    #[serde(rename = "elapsed")]
    pub elapsed: Option<f32>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct StartThreadResponse {}

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
