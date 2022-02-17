use qq_party_shared::*;
use crate::info_::info_;
use crate::messaging_::publish_;
use crate::spawn_::spawn;
use wasmcloud_interface_messaging::{MessageSubscriber,PubMessage,SubMessage};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use bevy_ecs_wasm::prelude::{Schedule,World,Query,SystemStage,IntoSystem,Res};
use wasmcloud_interface_logging::{info,error,debug};
use bevy_math::Vec2;
use wasmbus_rpc::RpcResult;
use wasmcloud_interface_numbergen::random_in_range;
pub async fn _fn (map:Arc<Mutex<HashMap<String,(Schedule,World)>>>,game_id:String,ball_id:BallId)-> RpcResult<()>{
    info!("handle_message map");
    let x = random_in_range(3300,3800).await?;
    let y = random_in_range(3500,4000).await?;
    let mut n = String::from("");
    let ball_bundle = BallBundle{
      ball_id:ball_id,
      position:Position(Vec2::new(x as f32,y as f32)),
      velocity:Velocity(Vec2::new(0.0 as f32,2.0 as f32)),
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
      
      if let Some((ref mut s, ref mut w))= guard.get_mut(&game_id){
        n = String::from("spawning");
        n.push_str("spawning");
        n.push_str(&x.to_string());
        n.push_str("y:");
        n.push_str(&y.to_string());
        let mut query = w.query::<(&BallId,&Position, &Velocity,&TargetVelocity)>();
        for (ball_id,position, velocity,target_velocity) in query.iter(&w){
          ball_bundles.push(BallBundle{
            ball_id:ball_id.clone(),position:position.clone(),velocity:velocity.clone(),target_velocity:target_velocity.clone()
          });
        }
        spawn(w,ball_bundle.clone());
      }
    }
    info!("handle_message {:?}",n);
    let server_message = ServerMessage::Welcome{ball_bundle};
    match serde_json::to_vec(&server_message){
      Ok(b)=>{
        let pMsg = PubMessage{
          body:b,
          reply_to: None,
          subject: "game_logic".to_owned()
          };
        publish_(pMsg);
      }
      _=>{}
    }
    let channel_message_back = ServerMessage::GameState{ball_bundles};
    match serde_json::to_vec(&channel_message_back){
      Ok(b)=>{
        let pMsg = PubMessage{
          body:b,
          reply_to: None,
          subject: format!("channel.{:?}",ball_id.0)
          };
        publish_(pMsg);
      }
      _=>{}
    }
    Ok(())
}