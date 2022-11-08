use crate::{ClientContext, ClientInput, ClientState, ClientStateDispatcher, Event,Command};
use nats_lite::nats;
use super::normal::Normal;
#[derive(Debug, PartialEq, Clone)]
pub struct BeforeLogin {}

impl ClientState for BeforeLogin {
    fn handle(&self, commands: &mut ClientContext, event: &ClientInput) -> ClientStateDispatcher {
        //info!("LZ{:?}",event);
        match event {
            ClientInput::Event(e) => {
                if let Event::Nats(_client_name,_s_op)=e{
                  // let n = nats::proto::ClientOp::Sub{
                  //   subject:String::from("hello"),
                  //   queue_group:None,
                  //   sid:3,
                  // };
                  // commands.commands.push(Command::Nats(String::from("default"),n));

                  let n = nats::proto::ClientOp::Sub{
                    subject:String::from("welcome"),
                    queue_group:None,
                    sid:16,
                  };
                  commands.commands.push(Command::Nats(String::from("default"),n));
                  let n = nats::proto::ClientOp::Sub{
                    subject:String::from("game_logic.scores"),
                    queue_group:None,
                    sid:19,
                  };
                  commands.commands.push(Command::Nats(String::from("default"),n.clone()));
                  let n = nats::proto::ClientOp::Sub{
                    subject: String::from("game_logic.state"),
                    queue_group:None,
                    sid:23,
                  };
                  commands.commands.push(Command::Nats(String::from("default"),n));
                  //info!("subscribe welcome client_name {:?} s_op {:?}",client_name,s_op);
                  return Normal{
                    
                  }
                  .into()
                }               
            }
        }
       
        self.clone().into()
    }
}
