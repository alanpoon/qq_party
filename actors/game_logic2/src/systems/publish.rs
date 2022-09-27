use wasmcloud_interface_messaging::PubMessage;
use bevy_ecs::prelude::*;
use qq_party_shared::*;
use crate::info_::info_;
use crate::{Time,TimeV2};
use crate::bevy_wasmcloud_time;
use crate::messaging_::publish_;
use crate::util::sub_map_area;
use std::collections::HashMap;
pub fn sys_publish_game_state(mut elapsed_time:ResMut<Time>,bevy_wasmcloud_time_val:Res<bevy_wasmcloud_time::Time>,
  query: Query<(&BallId,&BallLabel,&Position,&Velocity,&TargetVelocity)>,
  npc_query: Query<(&NPCId,&Position,&Velocity,&ChaseTargetId)>) {
  if (*elapsed_time).elapsed > 5.0{
    (*elapsed_time).elapsed = 0.0;
    let mut ball_bundles_hashmap :HashMap<String,Vec<BallBundle>> = HashMap::new();
    ball_bundles_hashmap.insert(String::from("A"),vec![]);
    ball_bundles_hashmap.insert(String::from("B"),vec![]);
    ball_bundles_hashmap.insert(String::from("C"),vec![]);
    ball_bundles_hashmap.insert(String::from("D"),vec![]);
    let mut npc_bundles_hashmap :HashMap<String,Vec<NPCBundle>> = HashMap::new();
    npc_bundles_hashmap.insert(String::from("A"),vec![]);
    npc_bundles_hashmap.insert(String::from("B"),vec![]);
    npc_bundles_hashmap.insert(String::from("C"),vec![]);
    npc_bundles_hashmap.insert(String::from("D"),vec![]);
    for (ball_id,ball_label,position,velocity,target_velocity) in query.iter(){
      let sa = sub_map_area(position.clone());
      if let Some(x) = ball_bundles_hashmap.get_mut(&sa) {
        x.push(BallBundle{ball_id:ball_id.clone(),ball_label:ball_label.clone(),position:position.clone(),velocity:velocity.clone(),target_velocity:target_velocity.clone()});
      }
    }
    for (npc_id,position,velocity,chase_target) in npc_query.iter(){
      let sa = sub_map_area(position.clone());
      if let Some(x) = npc_bundles_hashmap.get_mut(&sa) {
        x.push(NPCBundle{npc_id:npc_id.clone(),position:position.clone(),velocity:velocity.clone(),chase_target:chase_target.clone()});
      }
    }
    let mut m_ = vec![String::from("A"), String::from("B"), String::from("C"),String::from("D")];
    for m in m_.iter(){
      let ball_bundles = ball_bundles_hashmap.get(m).unwrap();
      let npc_bundles = npc_bundles_hashmap.get(m).unwrap();
      for (i,npc_chunck) in npc_bundles.chunks(20).enumerate(){
        let mut bb= vec![];
        if i==0{
          bb = ball_bundles.clone();    
        }
        let channel_message_back = ServerMessage::GameState{ball_bundles:bb,npc_bundles:npc_chunck.to_vec(),timestamp:(*bevy_wasmcloud_time_val).timestamp};
        match rmp_serde::to_vec(&channel_message_back){
          Ok(b)=>{
            let pMsg = PubMessage{
              body:b,
              reply_to: None,
              subject: format!("game_logic.{}",m)
            };
            publish_(pMsg);
          }
          Err(e)=>{
            info_(format!("m iter ....error{}",e));
          }
        }
      }

      
    }
    
    return
  }
  // info_(format!("bevy_wasmcloud_time delta_seconds {:?} {:?}",(*bevy_wasmcloud_time_val).delta_seconds, (*elapsed_time).elapsed));
  (*elapsed_time).elapsed += (*bevy_wasmcloud_time_val).delta_seconds;
}

pub fn sys_publish_game_state_by_sub_map(mut elapsed_time:ResMut<TimeV2>,bevy_wasmcloud_time_val:Res<bevy_wasmcloud_time::Time>,
  query: Query<(&BallId,&BallLabel,&Position,&Velocity,&TargetVelocity)>,
  npc_query: Query<(&NPCId,&Position,&Velocity,&ChaseTargetId)>,
  scoreboard:Res<ScoreBoard>) {
  let mut once = true;
  for (key,elapsed) in (*elapsed_time).elapsed.iter_mut(){
    if *elapsed >5.0{
      *elapsed = 0.0;
      
      if key =="scoreboard"{
        let mut score_vec:Vec<(i16,BallLabel)> = vec![];
        for (k,v) in (*scoreboard).scores.iter(){
          score_vec.push(v.clone());
        }
        score_vec.sort_by(|a,b|{
          b.0.cmp(&a.0)
        });
         if score_vec.len() >0{
          if score_vec.len() >10{
            score_vec.clone().split_off(10);
          }
        }
        let msg = ServerMessage::Scores{scoreboard:score_vec};
        match rmp_serde::to_vec(&msg){
          Ok(b)=>{
            let pMsg = PubMessage{
              body:b,
              reply_to: None,
              subject: String::from("game_logic.scores"),
            };
            publish_(pMsg);
          }
          Err(e)=>{
            info_(format!("m iter ....error{}",e));
          }
        }
        continue;
      }
      let mut ball_bundles =vec![];
      let mut npc_bundles = vec![];
      for (ball_id,ball_label,position,velocity,target_velocity) in query.iter(){
        let sa = sub_map_area(position.clone());
        if &sa ==key{
          ball_bundles.push(BallBundle{ball_id:ball_id.clone(),ball_label:ball_label.clone(),position:position.clone(),velocity:velocity.clone(),target_velocity:target_velocity.clone()});
        }
        
      }
      
      for (npc_id,position,velocity,chase_target) in npc_query.iter(){
        let sa = sub_map_area(position.clone());
        if &sa ==key{
          npc_bundles.push(NPCBundle{npc_id:npc_id.clone(),position:position.clone(),velocity:velocity.clone(),chase_target:ChaseTargetId(chase_target.0.clone(),0)});
        }
      }
      for (i,npc_chunck) in npc_bundles.chunks(20).enumerate(){
        let mut bb= vec![];
        if i==0{
          bb = ball_bundles.clone();    
        }
        let channel_message_back = ServerMessage::GameState{ball_bundles:bb,npc_bundles:npc_chunck.to_vec(),timestamp:(*bevy_wasmcloud_time_val).timestamp};
        match rmp_serde::to_vec(&channel_message_back){
          Ok(b)=>{
            let pMsg = PubMessage{
              body:b,
              reply_to: None,
              subject: format!("game_logic.{}",key)
            };
            publish_(pMsg);
          }
          Err(e)=>{
            info_(format!("m iter ....error{}",e));
          }
        }
      }
      
    }else{
      *elapsed += (*bevy_wasmcloud_time_val).delta_seconds;
    }
  }
}