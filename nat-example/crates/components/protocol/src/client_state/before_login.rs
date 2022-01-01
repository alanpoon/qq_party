use tracing::error;

use crate::{ClientContext, ClientInput, ClientState, ClientStateDispatcher, Event,Command};
use crate::nats;
use super::normal::Normal;
use wasm_bindgen::prelude::*;
use log::*;
#[derive(Debug, PartialEq, Clone)]
pub struct BeforeLogin {}

impl ClientState for BeforeLogin {
    fn handle(&self, commands: &mut ClientContext, event: &ClientInput) -> ClientStateDispatcher {
      info!("LZ{:?}",event);
        match event {
            ClientInput::Event(e) => {
                if let Event::Nats(client_name,s_op)=e{
                  // let n = nats::proto::ClientOp::Sub{
                  //   subject:String::from("hello"),
                  //   queue_group:None,
                  //   sid:3,
                  // };
                  // commands.commands.push(Command::Nats(String::from("default"),n));
                  let n = nats::proto::ClientOp::Sub{
                    subject:String::from("game_logic"),
                    queue_group:None,
                    sid:17,
                  };
                  commands.commands.push(Command::Nats(String::from("default"),n));
                  info!("subscribe game_logic");
                  return Normal{
                    
                  }
                  .into()
                }               
            }
            event => {
                error!("unexpected event: {:?}", event);
            }
        }
       
        self.clone().into()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::client_state::normal::Normal;
    use crate::{ClientContext, UserId};

    #[test]
    fn handles_logged_in_event() {
        let state = BeforeLogin {};
        let mut context = ClientContext {
            ..Default::default()
        };
        let user_id = UserId::generate();
        let event = ClientInput::Event(Event::LoggedIn {
            user_id: user_id.clone(),
        });

        assert_eq!(
            state.handle(&mut context, &event),
            Normal { user_id }.into()
        );
    }
}
