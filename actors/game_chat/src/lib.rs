extern crate wasmcloud_interface_messaging as messaging;
mod host_call;
mod info_;
mod systems;
mod thread;
mod bevy_wasmcloud_time;
mod messaging_;
mod spawn_;
mod client_message_handlers;
mod plugins;
mod startup;
mod util;
use host_call::host_call;
use std::borrow::Borrow;
use info_::info_;
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_logging::{info,error};
use wasmcloud_interface_messaging::{MessageSubscriber,SubMessage};
use wasmcloud_interface_thread::{StartThreadRequest, StartThreadResponse,Thread,ThreadReceiver,ThreadSender};
use messaging::*;
use lazy_static::lazy_static; // 1.4.0
use bevy_app::{App};
use bevy_ecs::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_utils::Duration;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use serde::{Serialize,Deserialize};
use std::boxed::Box;
use qq_party_shared::*;
use std::io::Write;
use std::borrow::Cow;
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
      let mut map = APP.clone();
      let mut m = map.lock().unwrap();
      
      m.world.spawn_batch(npc_bundles);
      m.init_resource::<Time>()
      .init_resource::<ScoreBoard>()
      .add_plugin(bevy_transform::TransformPlugin::default())
      .add_plugin(PhysicsPlugin)
      //.add_system(systems::publish::sys_publish_game_state.system())
      .add_system(systems::publish::sys_publish_game_state_by_sub_map.system())
     // .add_system(systems::sys_time_debug.system())
      //.add_system(systems::sys.system())
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
    let mut map = APP.clone();
    //Ok(StartThreadResponse{})
     thread::thread_handle_request(map,start_thread_request).await
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
        Ok(ClientMessage::TargetVelocity{game_id,ball_id,target_velocity})=>{
          let mut map = APP.clone();
          client_message_handlers::target_velocity_handler::_fn(map,game_id,ball_id,target_velocity);  
        }
        Ok(ClientMessage::Welcome{game_id,ball_id,ball_label})=>{
          let mut map = APP.clone();
          client_message_handlers::welcome_handler::_fn(map,game_id,ball_id,ball_label).await;
        }
        Ok(ClientMessage::ChangeSubMap{game_id,ball_id,position})=>{
          let mut map = APP.clone();
          client_message_handlers::change_sub_map_handler::_fn(map,game_id,ball_id,position);
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
#[derive(Debug, PartialEq, Default)]
pub struct Time{pub elapsed:f32}
impl Time{
  pub fn update(&mut self,t:f32){
    self.elapsed = t;
  }
  pub fn elapsed(&self)->f32{
    self.elapsed
  }
}
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
// trait TimeAble {
//   fn update(&mut self,f32);
//   fn elapsed(&self) -> f32;
// }
