use crate::{ClientContext, ClientInput, ClientState, ClientStateDispatcher,Event};
#[derive(Debug, PartialEq, Clone)]
pub struct AfterNormal {
  //user_id
}

impl ClientState for AfterNormal {
    fn handle(&self, _commands: &mut ClientContext, event: &ClientInput) -> ClientStateDispatcher {
      //info!("LZ afternormal{:?}",event);
      match event {
        ClientInput::Event(e) => {
          if let Event::Nats(_client_name,s_op)=e{
            
            
          }
        }
      }
      self.clone().into()
    }
}
