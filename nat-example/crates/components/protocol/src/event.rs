use serde::{Deserialize, Serialize};
use crate::nats;
#[non_exhaustive]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
//#[serde(tag = "c")] // stands for code
pub enum Event {
    Nats(String,nats::proto::ServerOp),//server_name
    WSClose,
    NatSubOk(String),
    NatPubOk(String),
    #[serde(other)]
    Unknown,
}
pub type RawEvent = nats::proto::ServerOp;

#[derive(Clone, PartialEq, Debug, Default)]
pub struct Events(Vec<Event>);

impl Events {
    pub fn iter(&self) -> impl Iterator<Item = &Event> {
        self.0.iter()
    }

    pub fn push(&mut self, event: Event) {
        self.0.push(event);
    }
    pub fn retain<F:FnMut(&Event)->bool>(&mut self, f: F){
      self.0.retain(f);
    }
    pub fn clear(&mut self){
      self.0.clear();
    }
    pub fn truncate(&mut self) {
        self.0.truncate(32);
    }
}