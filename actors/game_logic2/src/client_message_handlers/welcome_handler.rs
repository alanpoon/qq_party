use qq_party_shared::*;
use crate::info_::info_;
use crate::messaging_::publish_;
use crate::spawn_::spawn;
use qq_party_shared::sub_map::sub_map_area;
use wasmcloud_interface_messaging::{PubMessage};
use std::collections::HashMap;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::sync::{Arc, Mutex};
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_numbergen::random_in_range;
pub async fn _fn (map:Arc<Mutex<App>>,game_id:String,ball_id:BallId,ball_label:BallLabel)-> RpcResult<()>{
    let x = random_in_range(3300,3800).await? as f32;
    let y = random_in_range(3500,3800).await? as f32;
    let key = sub_map_area(x,y);
    info_(format!("welcome ball_id {:?}",ball_id));
    let ball_bundle = BallBundle{
      ball_id:ball_id,
      ball_label:ball_label.clone(),
      transform:Transform { translation: [x,y,3.0].into(), ..Default::default() },
      global_transform:GlobalTransform::identity(),
      velocity:Velocity::zero(),
      rigid_body:RigidBody::Dynamic,
      locked_axes:LockedAxes::ROTATION_LOCKED,
      last_npc:LastNPC(0, None, false),
      interpolated:TransformInterpolation::default()
    };
    {
      let guard = match map.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
          poisoned.into_inner()
        },
      };
      let mut app = guard;
      let mut state_from_transform= None;
      if let Some(st) = app.world.get_resource::<StateTransformer>(){
        state_from_transform = Some(st.1.clone());
      }
      info_(format!("before spawn"));
      spawn(&mut app.world,ball_bundle.clone());
      info_(format!("before spawn"));
      let mut scoreboard = app.world.get_resource_mut::<ScoreBoard>().unwrap();
      init_score(ball_id.0,ball_label,&mut scoreboard.scores);
      let qq_state = app.world.get_resource::<StateTransformer>().unwrap().1.clone();
      let server_message = ServerMessage::Welcome{ball_bundle,sub_map:key.clone(),qq_state};
      match rmp_serde::to_vec(&server_message){
        Ok(b)=>{
          let p_msg = PubMessage{
            body:b,
            reply_to: None,
            subject: String::from("welcome")
            };
          publish_(p_msg);
        }
        _=>{}
      }
      info_(format!("after publish"));
      if let Some(st) = state_from_transform{
        match st{
          QQState::Running|QQState::StopNotification=>{
            //continue
          },
          QQState::Stop|QQState::RunNotification=>{
            if let Some(winners) = app.world.get_resource::<crate::Winners>(){
              let channel_message_back = ServerMessage::StateChange{state:QQState::Stop,scoreboard:winners.scores.clone()};
              match rmp_serde::to_vec(&channel_message_back){
                Ok(b)=>{
                  let p_msg = PubMessage{
                    body:b,
                    reply_to: None,
                    subject: format!("game_logic_specify.{}",ball_id.0)
                  };
                  publish_(p_msg);
                }
                Err(e)=>{
                  info_(format!("m iter ....error{}",e));
                }
              }
              return Ok(());
            }
          }
            
          _=>{}
        }
      }
      let mut ball_bundles =vec![];
      let mut npc_bundles = vec![];
      let time = app.world.get_resource::<Time>().unwrap();
      let time_clone = time.clone();
      let mut query = app.world.query::<(&BallId,&BallLabel,&Transform, &Velocity,&LastNPC)>();
      //let mut query = app.world.query::<(&BallBundle)>();
      for (gball_id,ball_label,transform,velocity,last_npc) in query.iter(&app.world){
        if gball_id.0!=ball_id.0{//don't send yourself
          let sa = sub_map_area(transform.translation.x,transform.translation.y);
          if sa ==key{
            ball_bundles.push(BallBundle{ball_id:gball_id.clone(),ball_label:ball_label.clone(),
              transform:transform.clone(),global_transform:GlobalTransform::identity(),
              velocity:velocity.clone(),rigid_body:RigidBody::Dynamic,
              locked_axes:LockedAxes::ROTATION_LOCKED,last_npc:last_npc.clone(),
              interpolated:TransformInterpolation::default()});
          }
        }
      }
      let mut npc_query = app.world.query::<(&NPCId,&Transform,&Velocity,&ChaseTargetId)>();

      for (npc_id,transform,velocity,chase_target) in npc_query.iter(&app.world){
        let sa = sub_map_area(transform.translation.x,transform.translation.x);
        if sa ==key{
          npc_bundles.push(NPCBundle{npc_id:npc_id.clone(),
            transform:transform.clone(),
            global_transform:GlobalTransform::identity(),
            velocity:velocity.clone(),chase_target:ChaseTargetId(chase_target.0.clone(),0),
            rigid_body:RigidBody::Dynamic
          });
        }
      }
      let storm_timing = app.world.get_resource::<StormTiming>().unwrap().clone();
      // info_(format!("npc_bundles {:?}",npc_bundles));
      for (i,npc_chunck) in npc_bundles.chunks(20).enumerate(){
        let mut bb= vec![];
        if i==0{
          bb = ball_bundles.clone();
        }
        let channel_message_back = ServerMessage::GameState{ball_bundles:bb,npc_bundles:npc_chunck.to_vec(),
          storm_timing:storm_timing.clone(),timestamp:time_clone.timestamp};
        match rmp_serde::to_vec(&channel_message_back){
          Ok(b)=>{
            let p_msg = PubMessage{
              body:b,
              reply_to: None,
              subject: format!("game_logic_specify.{}",ball_id.0)
            };
            publish_(p_msg);
          }
          Err(e)=>{
            info_(format!("m iter ....error{}",e));
          }
        }
      }
      
    }
    
    Ok(())
}

pub fn init_score(ball_id:u32,ball_label:BallLabel,scores:&mut HashMap<u32,(i16,BallLabel)>){
  scores.insert(ball_id,(0,ball_label));
}