use std::collections::HashMap;
use crate::*;
use serde::{Serialize, Deserialize};
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct ToDespawn{
  pub entity: HashMap<u32,(Entity,u32)>//npc_id, (npc_entity,elapsed)
}