use qq_party_shared::*;
use bevy::prelude::*;
use bevy::utils::Duration;
use crate::messaging_::publish_;
use crate::info_::info_;
use crate::startup::{RunningTimer,NotRunningTimer,IsRunning};
use wasmcloud_interface_messaging::{PubMessage};
pub fn system(
    mut cmd: Commands,
    mut ball_query: Query<(Entity,&BallId)>,
    mut npc_query: Query<(Entity,&NPCId)>,
    mut storm_query: Query<(Entity,&StormRingId)>,
    mut timer_query: Query<(Entity,&mut RunningTimer)>,
    mut scoreboard: ResMut<ScoreBoard>,
    mut is_running: ResMut<IsRunning>,
    time:Res<Time>){
    if (*is_running).0{
      for (e,mut timer) in timer_query.iter_mut(){
          if timer.0.tick(Duration::from_millis((time.delta_seconds() as f32 * 1000.0) as u64)).just_finished() {
            (*is_running).0 = false;
              cmd.spawn().insert(NotRunningTimer(Timer::new(Duration::new(10,0),false)));
              cmd.entity(e).despawn();
              for (e,_) in ball_query.iter(){
                  cmd.entity(e).despawn();
              }
              for (e,_) in npc_query.iter(){
                  cmd.entity(e).despawn();
              }
              for (e,_) in storm_query.iter(){
                  cmd.entity(e).despawn();
              }
              let mut score_vec:Vec<(i16,BallLabel)> = vec![];
              for (k,v) in (*scoreboard).scores.iter(){
                score_vec.push(v.clone());
              }
              score_vec.sort_by(|a,b|{
                b.0.cmp(&a.0)
              });
              if score_vec.len() >3{
                score_vec.clone().split_off(3);
              }
              *scoreboard = ScoreBoard::default();
              score_vec = vec![(2222,BallLabel(String::from("hello"),String::from(".cn")))];
              let server_message = ServerMessage::ResetGame{scoreboard:score_vec};
              match rmp_serde::to_vec(&server_message){
                Ok(b)=>{
                  let p_msg = PubMessage{
                    body:b,
                    reply_to: None,
                    subject: String::from("game_logic.reset")
                  };
                  publish_(p_msg);
                }
                _=>{}
              }
          }
      }
  }

}