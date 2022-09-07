use crate::{ClientContext, ClientInput, ClientState, ClientStateDispatcher,Event,Command};
use crate::nats;
use super::after_normal::AfterNormal;
use qq_party_shared::{Position,TargetVelocity,Velocity,BallId,UserInfo,ClientMessage,ServerMessage};
use rand::Rng;
use log::*;
#[derive(Debug, PartialEq, Clone)]
pub struct Normal {
  //user_id
}

impl ClientState for Normal {
    fn handle(&self, commands: &mut ClientContext, event: &ClientInput) -> ClientStateDispatcher {
      info!("LZ{:?}",event);
      match event {
          ClientInput::Event(e) => {
            if let Event::Nats(client_name,s_op)=e{
              let p = nats::proto::ClientOp::Pub{
                subject:String::from("hello"),
                reply_to:None,
                payload:b"bbb".to_vec(),
              };
              commands.commands.push(Command::Nats(String::from("default"),p));
              let mut rand_rng = rand::thread_rng();
              let x = rand_rng.gen_range(10000..99999);
              let n = nats::proto::ClientOp::Sub{
                subject: format!("channel.{:?}",x),
                queue_group:None,
                sid:18,
              };
              commands.commands.push(Command::Nats(String::from("default"),n));
              let tv = ClientMessage::Welcome{
                game_id:String::from("hello"),
                ball_id:BallId(x,0),
              };
              info!("Welcome Welcome");
              let tv_= rmp_serde::to_vec(&tv).unwrap();
              let n = nats::proto::ClientOp::Pub{
                subject: String::from("client_handler.hello"),
                reply_to: None,
                payload: tv_,
              };
              commands.commands.push(Command::Nats(String::from("default"),n));
              commands.commands.push(Command::StoreLocal(UserInfo{
                ball_id:BallId(x,0),
                sub_map:String::from(""),
              }));
              
              info!("normal client_name: {}, {:?}",client_name,s_op);
              match s_op{
                nats::proto::ServerOp::Msg{subject,sid,reply_to,payload}=>{
                  info!("msg {} payload:{}",subject,std::str::from_utf8(payload).unwrap());
                  info!("pub going to afternormal");
                  return AfterNormal{
                    
                  }
                  .into()
                }
                nats::proto::ServerOp::Ping=>{
                  let p = nats::proto::ClientOp::Pong;
                  commands.commands.push(Command::Nats(String::from("default"),p));
                }
               
                _=>{}
              }
              return AfterNormal{
                  
              }
              .into()
              } else if &Event::NatPubOk(String::from("hello"))== e{
                info!("pub going to afternormal");
                return AfterNormal{
                  
                }
                .into()
            }    
          }
        }
        
        self.clone().into()
    }
}
