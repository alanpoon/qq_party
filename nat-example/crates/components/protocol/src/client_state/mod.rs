mod before_login;
mod normal;
mod after_normal;
mod chicken_dinner;
pub use after_normal::*;
pub use before_login::*;
pub use chicken_dinner::*;
use enum_dispatch::enum_dispatch;
pub use normal::*;

use crate::{Commands, Event};

#[enum_dispatch(ClientState)]
#[derive(Debug, PartialEq, Clone)]
pub enum ClientStateDispatcher {
    BeforeLogin,
    Normal,
    AfterNormal,
    ChickenDinner,
}

impl Default for ClientStateDispatcher {
    fn default() -> Self {
        Self::BeforeLogin(BeforeLogin {})
    }
}

#[enum_dispatch]
pub trait ClientState {
    fn handle(&self, commands: &mut ClientContext, event: &ClientInput) -> ClientStateDispatcher;
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct ClientContext {
    pub commands: Commands,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ClientInput {
    Event(Event),
}
