extern crate wasmcloud_interface_messaging as messaging;
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_logging::{info,error};
use wasmcloud_interface_messaging::{MessageSubscriber,SubMessage};
use wasmcloud_interface_thread::{StartThreadRequest, StartThreadResponse,Thread,ThreadSender};
use lazy_static::lazy_static; // 1.4.0
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use crate::messaging_::publish_;
use messaging::*;
mod host_call;
mod info_;
mod messaging_;
//use host_call::host_call;
//use info_::info_;

use serde::{Serialize,Deserialize};
//ball_id,timestamp
lazy_static! {
  static ref MAP: Arc<Mutex<HashMap<String,u32>>> = Arc::new(Mutex::new(HashMap::new()));
}
#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor,MessageSubscriber)]
struct PlayerHealthCheckActor {}
#[async_trait]
impl MessageSubscriber for PlayerHealthCheckActor{
  async fn handle_message(&self, ctx: &Context, req: &SubMessage) -> RpcResult<()> {
    if req.subject.contains("player_health_check_handler"){
      let client_message:Result<PlayerHealthCheckMsg,_>= rmp_serde::from_slice(&req.body);
      match client_message{
        Ok(cm)=>{
          let map = MAP.clone();
          let mut m = map.lock().unwrap();
          if let Some(last_timestamp) = m.get_mut(&cm.ball_id_secret){
            *last_timestamp = cm.timestamp;
          }else{
            m.insert(cm.ball_id_secret,cm.timestamp);
          }
        },
        Err(_)=>{
        }
      }
    }
    Ok(())
  }
}
#[async_trait]
impl Thread for PlayerHealthCheckActor{
  async fn start_thread(&self, ctx: &Context, start_thread_request: &StartThreadRequest) -> RpcResult<StartThreadResponse> {
    info!("start_thread----");
    let provider = ThreadSender::new();
    if let Err(e) = provider
        .start_thread(
            ctx,
            start_thread_request,
        )
        .await
    {
        error!("sending reply: {}",e.to_string());
    }
    info!("end_thread----");
    Ok(StartThreadResponse{})
  }
  async fn handle_request(&self, _ctx: &Context, start_thread_request: &StartThreadRequest) -> RpcResult<StartThreadResponse> {
    let map = MAP.clone();
    let mut m = map.lock().unwrap();
    m.retain(|k,v|{
      if  *v - start_thread_request.timestamp as u32 >70000{
        let cm = ClientMessage::Disconnect{
          ball_id_secret:k.clone()
        };
        let p_msg = PubMessage{
          body:rmp_serde::to_vec(&cm).unwrap(),
          reply_to: None,
          subject: String::from("client_handler.hello")
        };
        publish_(p_msg);
        false
      }else{
        true
      }
    });
    
    Ok(StartThreadResponse{})
  }
  async fn now(&self,_ctx:&Context,_: &StartThreadRequest)  -> RpcResult<u64>{
    Ok(2)
  }
}

#[derive(Serialize,Deserialize)]
pub struct PlayerHealthCheckMsg{
  pub ball_id_secret:String,
  pub timestamp:u32
}
#[derive(Serialize, Deserialize, Clone)]
pub enum ClientMessage {
    Disconnect{ball_id_secret:String},
}