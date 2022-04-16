extern crate wasmcloud_interface_messaging as messaging;
mod host_call;
mod info_;
mod systems;
mod thread;
mod bevy_wasmcloud_time;
mod messaging_;
mod spawn_;
mod client_message_handlers;
use host_call::host_call;
use std::borrow::Borrow;
use info_::info_;
use wasmbus_rpc::actor::prelude::*;
use wasmbus_rpc::serialize;
use wasmcloud_interface_logging::{info,error,debug};
use wasmcloud_interface_messaging::{MessageSubscriber,SubMessage};
use wasmcloud_interface_thread::{StartThreadRequest, StartThreadResponse,Thread,ThreadReceiver,ThreadSender};
use messaging::*;
use lazy_static::lazy_static; // 1.4.0
use bevy_app::{ScheduleRunnerSettings,App};
use bevy_ecs::prelude::*;
use bevy_transform::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_utils::Duration;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use serde::{Serialize,Deserialize};
use std::boxed::Box;
use qq_party_shared::*;
use std::io::Write;
use std::borrow::Cow;

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
    {
      let mut map = APP.clone();
      let mut m = map.lock().unwrap();
      m.init_resource::<Time>()
      .add_plugin(bevy_transform::TransformPlugin::default())
      //.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
      //.add_startup_system(systems::spawn.system())
      .add_system(systems::sys_bevy_wasmcloud_time.system())
      //.add_system(qq_party_shared::systems::update_state_position::<bevy_wasmcloud_time::Time>.system())
      .add_system(qq_party_shared::systems::update_state_position_physics::<bevy_wasmcloud_time::Time>.system())
      .add_system(qq_party_shared::systems::update_state_velocity_physics.system())
      .add_system(qq_party_shared::systems::physics::spawn_player_collider.system())
      .add_system(systems::sys.system());
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
      let client_message: Result<ClientMessage,_> = serde_json::from_slice(&req.body);
      match client_message{
        Ok(ClientMessage::TargetVelocity{game_id,ball_id,target_velocity})=>{
          let mut map = APP.clone();
          client_message_handlers::target_velocity_handler::_fn(map,game_id,ball_id,target_velocity);  
        }
        Ok(ClientMessage::Welcome{game_id,ball_id})=>{
          let mut map = APP.clone();
          client_message_handlers::welcome_handler::_fn(map,game_id,ball_id).await;
        }
        _=>{}
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
}

