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
use wasmbus_rpc::RpcResult;
use wasmcloud_interface_numbergen::random_in_range;
pub async fn _fn (map:Arc<Mutex<App>>,game_id:String,ball_id:BallId,ball_label:BallLabel)-> RpcResult<()>{
    info!("handle_message map");
    let x = random_in_range(3300,3800).await?;
    let y = random_in_range(3500,3800).await?;
    let pos = Position(Vec2::new(x as f32,y as f32));
    let sa = sub_map_area(pos.clone());
    let mut n = String::from("");
    let ball_bundle = BallBundle{
      ball_id:ball_id,
      ball_label:ball_label,
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
      
      n = String::from("spawning");
      n.push_str("spawning");
      n.push_str(&x.to_string());
      n.push_str("y:");
      n.push_str(&y.to_string());
      let mut query = app.world.query::<(&BallId,&Position, &Velocity,&TargetVelocity)>();
      for (ball_id,position, velocity,target_velocity) in query.iter(&app.world){
        ball_bundles.push(BallBundle{
          ball_id:ball_id.clone(),position:position.clone(),velocity:velocity.clone(),target_velocity:target_velocity.clone(),
        });
      }
      spawn(&mut app.world,ball_bundle.clone());
      let mut scoreboard = app.world.get_resource_mut::<ScoreBoard>().unwrap();
      init_score(ball_id.0,&mut scoreboard.scores);
      ball_bundles.push(ball_bundle.clone());
      
    }
    info!("game_logic ....{:?}",sa);
    let server_message = ServerMessage::Welcome{ball_bundle,sub_map:sa};
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
    Ok(())
}

pub fn init_score(ball_id:u32,mut scores:&mut HashMap<u32,i16>){
  scores.insert(ball_id,0);
}