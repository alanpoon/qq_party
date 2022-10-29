use wasmcloud_interface_messaging::PubMessage;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy::math::Vec2;
use qq_party_shared::*;
use crate::info_::info_;
use crate::{TimeV2};
use crate::bevy_wasmcloud_time;
use crate::messaging_::publish_;
use qq_party_shared::sub_map::sub_map_area;

pub fn sys_publish_game_state_by_sub_map(mut cmd:Commands,mut elapsed_time:ResMut<TimeV2>,bevy_wasmcloud_time_val:Res<bevy_wasmcloud_time::Time>,
  query: Query<(&BallId,&BallLabel,&Transform,&Velocity,&LastNPC)>,
  npc_query: Query<(&NPCId,&Transform,&Velocity,&ChaseTargetId)>,
  storm_ring_query: Query<(Entity,&StormRingId)>,
  scoreboard:Res<ScoreBoard>,
  mut storm_timing:ResMut<StormTiming>,
  state_transform: Res<StateTransformer>) {
  for (key,elapsed) in (*elapsed_time).elapsed.iter_mut(){
    if key =="scoreboard"{
      if *elapsed >3.0{
        *elapsed = 0.0;
        let mut score_vec:Vec<(u32,i16,BallLabel)> = vec![];
        for (k,v) in (*scoreboard).scores.iter(){
          score_vec.push((k.clone(),v.0.clone(),v.1.clone()));
        }
        score_vec.sort_by(|a,b|{
          b.1.cmp(&a.1)
        });
        if score_vec.len() >8{
          score_vec.clone().truncate(8);
        }
        let server_message = ServerMessage::Scores{scoreboard:score_vec};
        match rmp_serde::to_vec(&server_message){
          Ok(b)=>{
            let p_msg = PubMessage{
              body:b,
              reply_to: None,
              subject: String::from("game_logic.scores"),
            };
            publish_(p_msg);
          }
          Err(e)=>{
            info_(format!("m iter ....error{}",e));
          }
        }
        
      }else{
        *elapsed += (*bevy_wasmcloud_time_val).delta_seconds;
      }
      continue;
    }
    if key == "storm_ring"{
      // match (*state_transform).1{
      //   QQState::Running|QQState::StopNotification=>{

      //   }
      //   _=>{
      //     continue
      //   }
      // }
      let mut storm_rings = vec![];
      for (e,_storm_ring_id) in storm_ring_query.iter(){
        storm_rings.push(e);
      }
      if *elapsed >(STORM_INTERVAL+STORM_DURATION) as f32{
        // for e in storm_rings{
        //   cmd.entity(e).despawn();
        // }
        *elapsed =0.0;        
        *storm_timing = StormTiming(bevy_wasmcloud_time_val.timestamp+STORM_INTERVAL,STORM_DURATION);
        let channel_message_back = ServerMessage::StormRings{storm_rings:vec![],next_storm_timing:Some(storm_timing.clone())};
        match rmp_serde::to_vec(&channel_message_back){
          Ok(b)=>{
            let p_msg = PubMessage{
              body:b,
              reply_to: None,
              subject: String::from("game_logic_storm_rings")
            };
            publish_(p_msg);
          }
          Err(e)=>{
            info_(format!("m iter ....error{}",e));
          }
        }
      } else if *elapsed >STORM_INTERVAL as f32 && *elapsed <= (STORM_INTERVAL+STORM_DURATION) as f32{
        if storm_rings.len()==0{
          let storm_ring_id = StormRingId(Vec2::new(3600.0,3500.0),90);
          cmd.spawn().insert(storm_ring_id.clone());
          let channel_message_back = ServerMessage::StormRings{storm_rings:vec![storm_ring_id],next_storm_timing:None};
          match rmp_serde::to_vec(&channel_message_back){
            Ok(b)=>{
              let p_msg = PubMessage{
                body:b,
                reply_to: None,
                subject: String::from("game_logic_storm_rings")
              };
              publish_(p_msg);
            }
            Err(e)=>{
              info_(format!("m iter ....error{}",e));
            }
          }
        }
      }
      if storm_timing.0==0{
        let next_storm_timing = bevy_wasmcloud_time_val.timestamp - *elapsed as u64 + STORM_INTERVAL;
        *storm_timing = StormTiming(next_storm_timing,STORM_DURATION);
      }
      *elapsed += (*bevy_wasmcloud_time_val).delta_seconds;
      
      continue;
    }
    if *elapsed >30.0{
      *elapsed = 0.0; 
      let mut ball_bundles =vec![];
      let mut npc_bundles = vec![];
      for (ball_id,ball_label,transform,velocity,last_npc) in query.iter(){
        let sa = sub_map_area(transform.translation.x,transform.translation.y);
        if &sa ==key{
          ball_bundles.push(BallBundle{ball_id:ball_id.clone(),ball_label:ball_label.clone(),
            transform:transform.clone(),
            global_transform:GlobalTransform::identity(),
            velocity:velocity.clone(),
            rigid_body:RigidBody::Dynamic,
            locked_axes:LockedAxes::ROTATION_LOCKED,last_npc:last_npc.clone()
          });
        }
        
      }
      
      for (npc_id,transform,velocity,chase_target) in npc_query.iter(){
        let sa = sub_map_area(transform.translation.x,transform.translation.y);
        if &sa ==key{
          npc_bundles.push(NPCBundle{npc_id:npc_id.clone(),transform:transform.clone(),
            global_transform:GlobalTransform::identity(),
            velocity:velocity.clone(),chase_target:ChaseTargetId(chase_target.0.clone(),0),rigid_body:RigidBody::Dynamic});
        }
      }
      for (i,npc_chunck) in npc_bundles.chunks(20).enumerate(){
        let mut bb= vec![];
        if i==0{
          bb = ball_bundles.clone();    
        }
        let channel_message_back = ServerMessage::GameState{ball_bundles:bb,npc_bundles:npc_chunck.to_vec(),
          storm_timing:storm_timing.clone(),
          timestamp:(*bevy_wasmcloud_time_val).timestamp};
        match rmp_serde::to_vec(&channel_message_back){
          Ok(b)=>{
            let p_msg = PubMessage{
              body:b,
              reply_to: None,
              subject: format!("game_logic.{}",key)
            };
            publish_(p_msg);
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