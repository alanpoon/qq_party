use std::collections::HashMap;
use crate::*;
use serde::{Serialize, Deserialize};
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct ScoreBoard{
  pub scores: HashMap<u32,(i16,BallLabel)>
}