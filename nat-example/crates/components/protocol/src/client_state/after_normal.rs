use crate::{ClientContext, ClientInput, ClientState, ClientStateDispatcher,Event,Command};
use crate::nats;
use log::*;
#[derive(Debug, PartialEq, Clone)]
pub struct AfterNormal {
  //user_id
}

impl ClientState for AfterNormal {
    fn handle(&self, commands: &mut ClientContext, event: &ClientInput) -> ClientStateDispatcher {
      info!("LZ afternormal{:?}",event);
      match event {
        ClientInput::Event(e) => {
          if let Event::Nats(client_name,s_op)=e{
            match s_op{
              nats::proto::ServerOp::Msg{subject,sid,reply_to,payload}=>{
                info!("recv msg {} payload:{}",subject,std::str::from_utf8(payload).unwrap());
              }
              _=>{}
            }
            
          }
        }
      }
      self.clone().into()
    }
}
