use wasmbus_rpc::actor::prelude::*;
use wasmbus_rpc::common::{serialize,deserialize};
use wasmcloud_interface_numbergen::{RangeLimit};
use crate::host_call::host_call;
use std::borrow::Cow;

pub fn random_in_range_(min:u32,max:u32)->RpcResult<u32>{
  let l = RangeLimit{
    min,max
  };
  let buf:std::result::Result<Vec<u8>,_>= serialize(&l);
  match buf{
    Ok(buf)=>{
      let msg = Message {
        method: "NumberGen.RandomInRange",
        arg: Cow::Borrowed(&buf),
      };
      // match host_call("default","wasmcloud:builtin:numbergen",msg.method,msg.arg.as_ref()){
      //   Ok(result)=>{Ok(result)},
      //   Err(_)=>{}
      // }
      
      let s = host_call("default","wasmcloud:builtin:numbergen",msg.method,msg.arg.as_ref());
      match s{
        Ok(s)=>{
          deserialize(&s)
        },
        _=>{
          Ok(0)
        }
      }
    }
    _=>{
      Ok(0)
    }
  }
}
