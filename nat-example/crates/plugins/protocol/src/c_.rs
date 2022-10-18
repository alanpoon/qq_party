use qq_party_shared::{Position,TargetVelocity,QQVelocity,BallId,ClientMessage};
use protocol::{Command,nats};
use chrono::prelude::*;
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

pub fn fire(ball_id:BallId,velocity_x:f32,velocity_y:f32)->Command{
  let tv = ClientMessage::Fire{
    ball_id:ball_id,
    velocity:QQVelocity(Vec2::new(velocity_x,velocity_y)),
    sprite_enum:0,
  };
  
  let tv_= rmp_serde::to_vec(&tv).unwrap();
  let n = nats::proto::ClientOp::Pub{
    subject: String::from("client_handler.hello"),
    reply_to: None,
    payload: tv_,
  };
  Command::Nats(String::from("default"),n)
}
pub fn dash(ball_id:BallId)->Command{
  let tv = ClientMessage::Dash{
    ball_id:ball_id
  };
  
  let tv_= rmp_serde::to_vec(&tv).unwrap();
  let n = nats::proto::ClientOp::Pub{
    subject: String::from("client_handler.hello"),
    reply_to: None,
    payload: tv_,
  };
  Command::Nats(String::from("default"),n)
}
pub fn ping(ball_id:u32)->Command{
  let now: DateTime<Utc> = Utc::now();
  let timestamp: i64 = now.timestamp();
 
  let tv = ClientMessage::Ping{
    ball_id_secret:ball_id.to_string(),
    timestamp:timestamp as u32
  };
  
  let tv_= rmp_serde::to_vec(&tv).unwrap();
  let n = nats::proto::ClientOp::Pub{
    subject: String::from("player_health_check_handler.hello"),
    reply_to: None,
    payload: tv_,
  };
  Command::Nats(String::from("default"),n)
}