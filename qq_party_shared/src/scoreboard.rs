use std::collections::HashMap;

use serde::{Serialize, Deserialize};
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct ScoreBoard{
  pub scores: HashMap<u32,i16>
}