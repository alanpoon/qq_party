extern crate wasmcloud_interface_messaging as messaging;
mod host_call;
mod info_;
mod numbergen_;
mod systems;
mod thread;
mod bevy_wasmcloud_time;
mod messaging_;
mod spawn_;
mod client_message_handlers;
mod plugins;
mod startup;
mod util;
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_logging::{info,error};
use wasmcloud_interface_messaging::{MessageSubscriber,SubMessage};
use wasmcloud_interface_thread::{StartThreadRequest, StartThreadResponse,Thread,ThreadReceiver,ThreadSender};
use messaging::*;
use crate::thread::thread_handle_request;
use lazy_static::lazy_static; // 1.4.0
use bevy::prelude::*;
use bevy::utils::Duration;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use serde::{Serialize,Deserialize};
use std::boxed::Box;
use qq_party_shared::*;
use crate::plugins::physics::PhysicsPlugin;
lazy_static! {
  static ref APP: Arc<Mutex<App>> = Arc::new(Mutex::new(App::new()));
}

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor,Thread,MessageSubscriber)]
struct GameLogicActor {}
#[async_trait]
impl Thread for GameLogicActor{
  async fn start_thread(&self, ctx: &Context, start_thread_request: &StartThreadRequest) -> RpcResult<StartThreadResponse> {
    info!("start_thread----");
    let npc_bundles = startup::npc::spawn_npc_bundles().await?;

    {
      let map = APP.clone();
      let mut m = map.lock().unwrap();
      
      m.world.spawn_batch(npc_bundles);
      m.world.spawn().insert(startup::storm_ring::spawn_storm_ring(3400.0,3400.0,80));
      //m.world.spawn().insert(startup::StateTransformer(Timer::new(Duration::from_secs(20),false),State::Running));
      m.init_resource::<Time>()
      .init_resource::<StormTiming>()
      .init_resource::<bevy_wasmcloud_time::Time>()
      .add_plugin(TransformPlugin::default())
      .add_plugin(PhysicsPlugin)
      .add_plugin(QQSharedPlugin)
      .add_system(systems::publish::sys_publish_game_state_by_sub_map)
      ;
      
    }
    let provider = ThreadSender::new();
    if let Err(e) = provider
        .start_thread(
            ctx,
            start_thread_request,
        )
        .await
    {
        error!("sending reply: {}",e.to_string());
    }
    info!("end_thread----");
    Ok(StartThreadResponse{})
  }
  async fn handle_request(&self, ctx: &Context, start_thread_request: &StartThreadRequest) -> RpcResult<StartThreadResponse> {
    //info!("handle_request----");
    let map = APP.clone();
    //Ok(StartThreadResponse{})
    thread_handle_request(map,start_thread_request).await
  }
  async fn now(&self,ctx:&Context,start_thread_request: &StartThreadRequest)  -> RpcResult<u64>{
    Ok(2)
  }
}
#[async_trait]
impl MessageSubscriber for GameLogicActor{
  async fn handle_message(&self, ctx: &Context, req: &SubMessage) -> RpcResult<()> {
    if req.subject.contains("client_handler"){
      let client_message: Result<ClientMessage,_> = rmp_serde::from_slice(&req.body);
      match client_message{
        Ok(ClientMessage::ChangeSubMap{game_id,ball_id,position})=>{
          let map = APP.clone();
          client_message_handlers::change_sub_map_handler::_fn(map,game_id,ball_id,position);
        }
        Ok(ClientMessage::Dash{ball_id})=>{
          let map = APP.clone();
          client_message_handlers::dash_handler::_fn(map,ball_id);
        }
        Ok(ClientMessage::Disconnect{ball_id_secret})=>{
          let map = APP.clone();
          client_message_handlers::disconnect_handler::_fn(map,ball_id_secret);
        }
        Ok(ClientMessage::Fire{ball_id,velocity,sprite_enum})=>{
          let map = APP.clone();
          client_message_handlers::fire_handler::_fn(map,ball_id,velocity,sprite_enum);
        }
        Ok(ClientMessage::Ping{..})=>{
        }
        Ok(ClientMessage::TargetVelocity{game_id,ball_id,target_velocity})=>{
          let map = APP.clone();
          client_message_handlers::target_velocity_handler::_fn(map,game_id,ball_id,target_velocity);  
        }
        Ok(ClientMessage::Welcome{game_id,ball_id,ball_label})=>{
          let map = APP.clone();
          client_message_handlers::welcome_handler::_fn(map,game_id,ball_id,ball_label).await;
        }
        
        Err(e)=>{
          info!("client_message err {:?}",e);
        }
      }
    }
    Ok(())
  }
}
// fn stop_thread(req: game_engine::StartThreadRequest) -> HandlerResult<game_engine::StartThreadResponse> {
//   //logging::default().write_log("LOGGING_ACTORINFO", "info", "Stop thread")?;
//   game_engine::stop_thread(req)
// }
#[derive(Component,Debug, Eq, PartialEq, Default,Serialize, Deserialize,Clone)]
pub struct A{
  position: i32,
}
#[derive(Bundle,Serialize, Deserialize,Clone,Debug)]
pub struct BallBundle {
    pub a: A,
}
#[derive(Debug, PartialEq, Default,Component)]
pub struct QQTime{pub elapsed:f32}
// impl Time{
//   pub fn update(&mut self,t:f32){
//     self.elapsed = t;
//   }
//   pub fn elapsed(&self)->f32{
//     self.elapsed
//   }
// }
#[derive(Debug, PartialEq, Default)]
pub struct TimeV2{pub elapsed:HashMap<String,f32>}
impl TimeV2{
  pub fn update(&mut self,t:f32,key:String){
    self.elapsed.insert(key,t);
  }
  pub fn elapsed(&self,key:String)->Option<&f32>{
    self.elapsed.get(&key)
  }
}
#[derive(Debug,Default,Clone)]
pub struct Winners{
  scores: Vec<(u32,i16,BallLabel)> //ball_id,score,ball_label
}

