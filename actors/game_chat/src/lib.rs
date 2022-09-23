extern crate wasmcloud_interface_messaging as messaging;
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_logging::{info,error};
use wasmcloud_interface_messaging::{MessageSubscriber,SubMessage};
use rustrict::CensorStr;
use crate::messaging_::publish_;
use messaging::*;
mod host_call;
mod info_;
mod messaging_;
use host_call::host_call;
use info_::info_;

use serde::{Serialize,Deserialize};

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor,MessageSubscriber)]
struct ChatActor {}
#[async_trait]
impl MessageSubscriber for ChatActor{
  async fn handle_message(&self, ctx: &Context, req: &SubMessage) -> RpcResult<()> {
    info_(format!("some chat msg"));
    if req.subject.contains("chat_handler"){
      let mut client_message:Result<ChatMsg,_>= rmp_serde::from_slice(&req.body);
      match client_message{
        Ok(mut cm)=>{
          cm.data = cm.data.censor();
          info_(format!("some chat msg--{:?}",cm.data.clone()));
          let pMsg = PubMessage{
            body:rmp_serde::to_vec(&cm).unwrap(),
            reply_to: None,
            subject: String::from("chat_from_server")
          };
          publish_(pMsg);
        },
        Err(e)=>{
          info_(format!("game_chat{:?}",e));
        }
      }
    }
    Ok(())
  }
}
#[derive(Serialize,Deserialize)]
pub struct ChatMsg{
  pub user:String,
  pub data:String
}