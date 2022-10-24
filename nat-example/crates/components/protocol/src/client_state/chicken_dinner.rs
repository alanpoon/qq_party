use crate::{ClientContext, ClientInput, ClientState, ClientStateDispatcher,Event,Command};
use crate::nats;
use log::*;
#[derive(Debug, PartialEq, Clone)]
pub struct ChickenDinner {
  //user_id
}
pub fn pre_chicken_dinner_unsub_all(){

}
impl ClientState for ChickenDinner {
    fn handle(&self, commands: &mut ClientContext, event: &ClientInput) -> ClientStateDispatcher {
      info!("LZ ChickenDinner{:?}",event);
      match event {
        ClientInput::Event(e) => {
          if let Event::Nats(_client_name,s_op)=e{
            let n = nats::proto::ClientOp::Unsub{
              sid:16,//welcome
              max_msgs:None,
            };
            commands.commands.push(Command::Nats(String::from("default"),n));
            let n = nats::proto::ClientOp::Unsub{
              sid:19,//game_logic.scores
              max_msgs:None,
            };

            commands.commands.push(Command::Nats(String::from("default"),n.clone()));
            let n = nats::proto::ClientOp::Unsub{
              sid:23,//game_logic.reset
              max_msgs:None,
            };

            commands.commands.push(Command::Nats(String::from("default"),n.clone()));
            let n = nats::proto::ClientOp::Unsub{
              sid:21,//game_logic_specify.{}
              max_msgs:None,
            };
            commands.commands.push(Command::Nats(String::from("default"),n));
            let n = nats::proto::ClientOp::Unsub{
              sid:22,//game_logic_storm_rings
              max_msgs:None,
            };
            commands.commands.push(Command::Nats(String::from("default"),n));
            let n = nats::proto::ClientOp::Unsub{
              sid:17,//game_logic.submap
              max_msgs:None,
            };
            commands.commands.push(Command::Nats(String::from("default"),n));

            match s_op{
              nats::proto::ServerOp::Msg{subject:_,sid:_,reply_to:_,payload:_}=>{
                // info!("recv msg {} payloadlen:{:?}",subject,payload.len());
              }
              _=>{}
            }
            
          }
        }
      }
      self.clone().into()
    }
}
