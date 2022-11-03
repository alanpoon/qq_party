use crate::{ClientContext, ClientInput, ClientState, ClientStateDispatcher,Event,Command};
use crate::nats;
use super::after_normal::AfterNormal;
use qq_party_shared::*;
// use rand::Rng;
use log::*;
#[derive(Debug, PartialEq, Clone)]
pub struct Normal {
  //user_id
}

impl ClientState for Normal {
    fn handle(&self, commands: &mut ClientContext, event: &ClientInput) -> ClientStateDispatcher {

      match event {
          ClientInput::Event(e) => {
            match e{
              Event::Nats(_client_name,s_op)=>{
                match s_op{
                  nats::proto::ServerOp::Msg{subject:_,sid:_,reply_to:_,payload:_}=>{
                 
                  }
                  nats::proto::ServerOp::Ping=>{
                    let p = nats::proto::ClientOp::Pong;
                    commands.commands.push(Command::Nats(String::from("default"),p));
                  }
                 
                  _=>{}
                }
                //return AfterNormal{}.into()
              },
              Event::NatPubOk(p) =>{
                if p==&String::from("hello"){
                  return AfterNormal{}.into()
                }
              },
              Event::BevyWeb(json_value) =>{
                let m: Result<ClientMessage,_> = serde_json::from_value(json_value.clone());
                match m{
                  Ok(ClientMessage::Welcome{game_id:_,ball_id,ball_label})=>{
                    let tv = ClientMessage::Welcome{
                      game_id:String::from("hello"),
                      ball_id:ball_id.clone(),
                      ball_label:ball_label,
                    };
                    info!("--- Welcome");
                    let tv_= rmp_serde::to_vec(&tv).unwrap();
                    let n = nats::proto::ClientOp::Pub{
                      subject: String::from("client_handler.hello"),
                      reply_to: None,
                      payload: tv_,
                    };
                    commands.commands.push(Command::Nats(String::from("default"),n));
                    commands.commands.push(Command::StoreLocal(UserInfo{
                      ball_id:ball_id,
                      sub_map:String::from(""),
                    }));
                    let n = nats::proto::ClientOp::Sub{
                      subject: format!("game_logic_specify.{}",ball_id.0),
                      queue_group:None,
                      sid:21,
                    };
                    commands.commands.push(Command::Nats(String::from("default"),n));
                    let n = nats::proto::ClientOp::Sub{
                      subject: String::from("game_logic_storm_rings"),
                      queue_group:None,
                      sid:22,
                    };
                    commands.commands.push(Command::Nats(String::from("default"),n));
                    
                    return AfterNormal{}.into()
                  }
                  
                  Err(e)=>{
                    info!("---ClientMessage de err{:?}",e);
                  }
                  _=>{

                  }
                }
              }
              _=>{}
            }    
          }
      }
        
      self.clone().into()
    }
}
