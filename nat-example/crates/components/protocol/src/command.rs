use serde::{Deserialize, Serialize};
use crate::nats;
#[non_exhaustive]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
//#[serde(tag = "c")] // stands for code
pub enum Command {
    Nats(String,nats::proto::ClientOp),
    #[serde(other)]
    Unknown,
}
#[cfg(test)]
mod test {
    use super::*;
}
pub type RawCommand = nats::proto::ClientOp;
#[derive(Clone, PartialEq, Debug, Default)]
pub struct Commands(Vec<Command>);
impl Commands {
    pub fn iter(&self) -> impl Iterator<Item = &Command> {
        self.0.iter()
    }

    pub fn push(&mut self, event: Command) {
        self.0.push(event);
    }

    pub fn clear(&mut self) {
        self.0.clear();
        self.0.truncate(32);
    }
}
