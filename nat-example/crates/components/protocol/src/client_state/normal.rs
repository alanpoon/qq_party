use crate::{ClientContext, ClientInput, ClientState, ClientStateDispatcher,Event,Command};
use crate::nats;
use super::after_normal::AfterNormal;

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
