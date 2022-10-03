use wasmbus_rpc::actor::prelude::*;
use wasmbus_rpc::common::serialize;
use wasmcloud_interface_messaging::{PubMessage};
use crate::host_call::host_call;
use std::borrow::Cow;
pub fn publish_(s:PubMessage){
  let buf:std::result::Result<Vec<u8>,_>= serialize(&s);
  match buf{
    Ok(buf)=>{
      let msg = Message {
        method: "Messaging.Publish",
        arg: Cow::Borrowed(&buf),
        //arg: buf,
      };
      match host_call("default","wasmcloud:messaging",msg.method,msg.arg.as_ref()){
        _=>{}
      }
    },
    _=>{

    }
  }
}
