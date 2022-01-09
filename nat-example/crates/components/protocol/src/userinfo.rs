use qq_party_shared::BallId;
use serde::{Serialize,Deserialize};
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone,Default)]
pub struct UserInfo{
  pub ball_id:BallId,
}