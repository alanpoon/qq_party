use qq_party_shared::*;
use crate::info_::info_;
use crate::messaging_::publish_;
use crate::spawn_::spawn;
use crate::util::sub_map_area;
use wasmcloud_interface_messaging::{MessageSubscriber,PubMessage,SubMessage};
use std::collections::HashMap;
use bevy_app::App;
use std::sync::{Arc, Mutex};
use bevy_ecs::prelude::*;
use wasmcloud_interface_logging::{info,error,debug};
use bevy_math::Vec2;
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_numbergen::random_in_range;
pub async fn _fn (map:Arc<Mutex<App>>,game_id:String,ball_id:BallId,ball_label:BallLabel)-> RpcResult<()>{
    let x = random_in_range(3300,3800).await?;
    let y = random_in_range(3500,3800).await?;
    let pos = Position(Vec2::new(x as f32,y as f32));
    let key = sub_map_area(pos.clone());
    let mut n = String::from("");
    info_(format!("welcome ball_id {:?}",ball_id));
    let ball_bundle = BallBundle{
      ball_id:ball_id,
      ball_label:ball_label.clone(),
      position:pos,
      velocity:Velocity(Vec2::new(0.0 as f32,0.0 as f32)),
      target_velocity: TargetVelocity(Vec2::ZERO),
    };
    let mut ball_bundles:Vec<BallBundle> = vec![];
    {
      let mut guard = match map.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
          poisoned.into_inner()
        },
      };
      let mut app = guard;

      spawn(&mut app.world,ball_bundle.clone());
      let mut scoreboard = app.world.get_resource_mut::<ScoreBoard>().unwrap();
      init_score(ball_id.0,ball_label,&mut scoreboard.scores);
      let server_message = ServerMessage::Welcome{ball_bundle,sub_map:key.clone()};
      match rmp_serde::to_vec(&server_message){
        Ok(b)=>{
          let pMsg = PubMessage{
            body:b,
            reply_to: None,
            subject: String::from("welcome")
            };
          publish_(pMsg);
        }
        _=>{}
      }
      let mut ball_bundles =vec![];
      let mut npc_bundles = vec![];
      let bevy_wasmcloud_time_val = app.world.get_resource::<crate::bevy_wasmcloud_time::Time>().unwrap();
      let bevy_wasmcloud_time_val_clone = bevy_wasmcloud_time_val.clone();
      let mut query = app.world.query::<(&BallId,&BallLabel,&Position, &Velocity,&TargetVelocity)>();
      for (ball_id,ball_label,position,velocity,target_velocity) in query.iter(&app.world){
        let sa = sub_map_area(position.clone());
        if sa ==key{
          ball_bundles.push(BallBundle{ball_id:ball_id.clone(),ball_label:ball_label.clone(),position:position.clone(),velocity:velocity.clone(),target_velocity:target_velocity.clone()});
        }
        
      }
      let mut npc_query = app.world.query::<(&NPCId,&Position,&Velocity,&ChaseTargetId)>();

      for (npc_id,position,velocity,chase_target) in npc_query.iter(&app.world){
        let sa = sub_map_area(position.clone());
        if sa ==key{
          npc_bundles.push(NPCBundle{npc_id:npc_id.clone(),position:position.clone(),velocity:velocity.clone(),chase_target:ChaseTargetId(chase_target.0.clone(),0)});
        }
      }
      for (i,npc_chunck) in npc_bundles.chunks(20).enumerate(){
        let mut bb= vec![];
        if i==0{
          bb = ball_bundles.clone();    
        }
        let channel_message_back = ServerMessage::GameState{ball_bundles:bb,npc_bundles:npc_chunck.to_vec(),timestamp:bevy_wasmcloud_time_val_clone.timestamp};
        match rmp_serde::to_vec(&channel_message_back){
          Ok(b)=>{
            let pMsg = PubMessage{
              body:b,
              reply_to: None,
              subject: format!("game_logic_specify.{}",ball_id.0)
            };
            publish_(pMsg);
          }
          Err(e)=>{
            info_(format!("m iter ....error{}",e));
          }
        }
      }
      
    }
    
    Ok(())
}

pub fn init_score(ball_id:u32,ball_label:BallLabel,mut scores:&mut HashMap<u32,(i16,BallLabel)>){
  scores.insert(ball_id,(0,ball_label));
}