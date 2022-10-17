use wasmbus_rpc::actor::prelude::*;
use wasmbus_rpc::common::serialize;
use wasmcloud_interface_logging::{LogEntry};
use crate::host_call::host_call;
use std::borrow::Cow;

pub fn info_(s:String){
  let l = LogEntry{
    level: String::from("info"),
    text: s,
  };
  let buf:std::result::Result<Vec<u8>,_>= serialize(&l);
  match buf{
    Ok(buf)=>{
      let msg = Message {
        method: "Logging.WriteLog",
        arg: Cow::Borrowed(&buf),
      };
      match host_call("default","wasmcloud:builtin:logging",msg.method,msg.arg.as_ref()){
        Ok(_)=>{},
        Err(_)=>{}
      }
    }
    _=>{

    }
  }
  
}
