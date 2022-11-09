use qq_party_shared::*;
use nats_lite::nats;
use protocol::{Command};
use chrono::prelude::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy::utils::Duration;
use crate::*;
use bevy::math::Vec2;
pub fn target_velocity(ball_id:BallId,target_velocity_x:f32,target_velocity_y:f32,sa:String,vel:&mut Velocity)->Vec<Command>{
  let t_v= TargetVelocity(Vec2::new(target_velocity_x,target_velocity_y));
  let tv = ClientMessage::TargetVelocity{
    game_id:String::from("hello"),
    ball_id:ball_id,
    target_velocity:t_v.clone(),
  };
 
  let tv_= rmp_serde::to_vec(&tv).unwrap();
  let n1 = nats::proto::ClientOp::Pub{
    subject: String::from("client_handler.hello"),
    reply_to: None,
    payload: tv_,
  };
  let tv = ServerMessage::TargetVelocity{
    ball_id:ball_id,
    target_velocity:t_v.clone(),
  };
  let tv_= rmp_serde::to_vec(&tv).unwrap();
  let n2 = nats::proto::ClientOp::Pub{
    subject: format!("game_logic.peer.{}",sa),
    reply_to: None,
    payload: tv_,
  };
  update::target_velocity::velocity(vel, t_v.clone());
  vec![Command::Nats(String::from("default"),n1),Command::Nats(String::from("default"),n2)]
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

pub fn fire(ball_id:BallId,velocity_x:f32,velocity_y:f32,cmd: &mut Commands,cooldown_query:&Query<&CoolDownTimer>)->Option<Command>{
  let mut can_send=true;
  for cd in cooldown_query.iter(){
    if cd.1==String::from("fire"){
      can_send = false;
      break;
    }
  }
  if can_send{
    cmd.spawn().insert(CoolDownTimer(Timer::new(Duration::from_secs(3),false),String::from("fire")));
    match serde_json::to_string(&CoolDownMessage::HideUI(String::from("fire"))){
      Ok(j)=>{
        push_web_bevy_events_fn2(&j);
      }
      Err(e)=>{
        info!("push_web_bevy_events_fn2 error {:?}",e);
      }
    }
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
    Some(Command::Nats(String::from("default"),n))
  }else{
    None
  }
  
}
pub fn dash(ball_id:BallId,cmd: &mut Commands,cooldown_query:&Query<&CoolDownTimer>)->Option<Command>{
  let mut can_send=true;
  for cd in cooldown_query.iter(){
    if cd.1==String::from("dash"){
      can_send = false;
      break;
    }
  }
  
  if can_send{
    cmd.spawn().insert(CoolDownTimer(Timer::new(Duration::from_secs(3),false),String::from("dash")));
    match serde_json::to_string(&CoolDownMessage::HideUI(String::from("dash"))){
      Ok(j)=>{
        push_web_bevy_events_fn2(&j);
      }
      Err(e)=>{
        info!("push_web_bevy_events_fn2 error {:?}",e);
      }
    }
    let tv = ClientMessage::Dash{
      ball_id:ball_id
    };
    let tv_= rmp_serde::to_vec(&tv).unwrap();
    let n = nats::proto::ClientOp::Pub{
      subject: String::from("client_handler.hello"),
      reply_to: None,
      payload: tv_,
    };
    Some(Command::Nats(String::from("default"),n))
    
  }else{
    None
  }
  
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