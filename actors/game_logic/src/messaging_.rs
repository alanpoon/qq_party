use wasmbus_rpc::serialize;
use wasmbus_rpc::actor::prelude::*;
use wasmbus_rpc::RpcError;
use wasmcloud_interface_messaging::{PubMessage};
use crate::host_call::host_call;
use std::borrow::Cow;
pub fn publish_(s:PubMessage)->std::result::Result<Vec<u8>, RpcError>{
  let buf = serialize(&s)?;
  let msg = Message {
    method: "Messaging.Publish",
    arg: Cow::Borrowed(&buf),
  };
  host_call("default","wasmcloud:messaging",msg.method,msg.arg.as_ref())?;
  Ok(vec![])
}
