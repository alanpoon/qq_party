use qq_party_shared::{Position,TargetVelocity,Velocity,BallId,LocalUserInfo,ClientMessage,ServerMessage,BallBundle};
use protocol::{Command,Event,nats};
use bevy::math::Vec2;
pub fn target_velocity(ball_id:BallId,target_velocity_x:f32,target_velocity_y:f32)->Command{
  let tv = ClientMessage::TargetVelocity{
    game_id:String::from("hello"),
    ball_id:ball_id,
    target_velocity:TargetVelocity(Vec2::new(target_velocity_x,target_velocity_y)),
  };
  
  let tv_= rmp_serde::to_vec(&tv).unwrap();
  let n = nats::proto::ClientOp::Pub{
    subject: String::from("client_handler.hello"),
    reply_to: None,
    payload: tv_,
  };
  Command::Nats(String::from("default"),n)
}
pub fn change_sub_map(ball_id:BallId,position:Position)->Command{
  let tv = ClientMessage::ChangeSubMap{
    game_id:String::from("hello"),
    ball_id:ball_id,
    position:position
  };
  
  let tv_= rmp_serde::to_vec(&tv).unwrap();
  let n = nats::proto::ClientOp::Pub{
    subject: String::from("client_handler.hello"),
    reply_to: None,
    payload: tv_,
  };
  Command::Nats(String::from("default"),n)
}