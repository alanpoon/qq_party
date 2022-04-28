extern crate std;
use std::string::{ToString,String};

use serde::__private::Vec;
pub type RpcResult<T> = std::result::Result<T, RpcError>;
use serde::{Serialize,Deserialize};
#[derive(thiserror::Error, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub enum RpcError {
    /// The request exceeded its deadline.
    #[error("the request exceeded its deadline: {0}")]
    DeadlineExceeded(String),

    /// A capability provider was called before its configure_dispatch was called.
    #[error("the capability provider has not been initialized: {0}")]
    NotInitialized(String),

    #[error("method not handled {0}")]
    MethodNotHandled(String),

    /// Error that can be returned if server has not implemented
    /// an optional interface method
    #[error("method not implemented")]
    NotImplemented,

    #[error("Host send error {0}")]
    HostError(String),

    #[error("deserialization: {0}")]
    Deser(String),

    #[error("serialization: {0}")]
    Ser(String),

    #[error("rpc: {0}")]
    Rpc(String),

    #[error("nats: {0}")]
    Nats(String),

    #[error("invalid parameter: {0}")]
    InvalidParameter(String),

    /// Error occurred in actor's rpc handler
    #[error("actor: {0}")]
    ActorHandler(String),

    /// Error occurred during provider initialization or put-link
    #[error("provider initialization or put-link: {0}")]
    ProviderInit(String),

    /// Timeout occurred
    #[error("timeout: {0}")]
    Timeout(String),

    //#[error("IO error")]
    //IO([from] std::io::Error)
    /// Anything else
    #[error("{0}")]
    Other(String),
}

impl From<String> for RpcError {
    fn from(s: String) -> RpcError {
        RpcError::Other(s)
    }
}

impl From<&str> for RpcError {
    fn from(s: &str) -> RpcError {
        RpcError::Other(s.to_string())
    }
}

impl From<std::io::Error> for RpcError {
    fn from(e: std::io::Error) -> RpcError {
        RpcError::Other(String::new())
    }
}
#[link(wasm_import_module = "wasmbus")]
extern "C" {
    pub fn __console_log(ptr: *const u8, len: usize);
    pub fn __host_call(
        bd_ptr: *const u8,
        bd_len: usize,
        ns_ptr: *const u8,
        ns_len: usize,
        op_ptr: *const u8,
        op_len: usize,
        ptr: *const u8,
        len: usize,
    ) -> usize;
    pub fn __host_response(ptr: *const u8);
    pub fn __host_response_len() -> usize;
    pub fn __host_error_len() -> usize;
    pub fn __host_error(ptr: *const u8);
    //pub fn __guest_response(ptr: *const u8, len: usize);
    //pub fn __guest_error(ptr: *const u8, len: usize);
    //pub fn __guest_request(op_ptr: *const u8, ptr: *const u8);
}
/// The function through which all host calls (from actors) take place.
pub fn host_call(binding: &str, ns: &str, op: &str, msg: &[u8]) -> RpcResult<Vec<u8>> {
  let callresult = unsafe {
      __host_call(
          binding.as_ptr() as _,
          binding.len() as _,
          ns.as_ptr() as _,
          ns.len() as _,
          op.as_ptr() as _,
          op.len() as _,
          msg.as_ptr() as _,
          msg.len() as _,
      )
  };
  if callresult != 1 {
      // call was not successful
      let errlen = unsafe { __host_error_len() };
      let buf = Vec::with_capacity(errlen as _);
      let retptr = buf.as_ptr();
      let slice = unsafe {
          __host_error(retptr);
          std::slice::from_raw_parts(retptr as _, errlen as _)
      };
      Err(RpcError::HostError(
          String::from_utf8_lossy(&slice.to_vec()).to_string(),
      ))
  } else {
      // call succeeded
      let len = unsafe { __host_response_len() };
      let buf = Vec::with_capacity(len as _);
      let retptr = buf.as_ptr();
      let slice = unsafe {
          __host_response(retptr);
          std::slice::from_raw_parts(retptr as _, len as _)
      };
      Ok(slice.to_vec())
  }
}