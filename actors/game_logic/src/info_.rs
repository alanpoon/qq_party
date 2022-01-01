use wasmbus_rpc::serialize;
use wasmbus_rpc::actor::prelude::*;
use wasmbus_rpc::RpcError;
use wasmcloud_interface_logging::{LogEntry};
use crate::host_call::host_call;
use std::borrow::Cow;
pub fn info_(s:String)->std::result::Result<Vec<u8>, RpcError>{
  let l = LogEntry{
    level: String::from("info"),
    text: s,
  };
  let buf = serialize(&l)?;
  let msg = Message {
    method: "Logging.WriteLog",
    arg: Cow::Borrowed(&buf),
  };
  host_call("default","wasmcloud:builtin:logging",msg.method,msg.arg.as_ref())?;
  Ok(vec![])
}
