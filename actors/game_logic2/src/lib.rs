extern crate wasmcloud_interface_messaging as messaging;
mod host_call;
mod info_;
mod systems;
mod thread;
mod bevy_wasmcloud_time;
use host_call::host_call;
use std::borrow::Borrow;
use info_::info_;
use wasmbus_rpc::actor::prelude::*;
use wasmbus_rpc::common::serialize;
use wasmcloud_interface_logging::{info,error,debug};
use wasmcloud_interface_messaging::{MessageSubscriber,SubMessage};
use wasmcloud_interface_thread::{StartThreadRequest, StartThreadResponse,Thread,ThreadReceiver,ThreadSender};
use messaging::*;
use lazy_static::lazy_static; // 1.4.0
use bevy_app::{ScheduleRunnerSettings,App};
use bevy_ecs::prelude::*;
use bevy_utils::Duration;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use serde::{Serialize,Deserialize};
use std::boxed::Box;
use qq_party_shared::*;
use std::io::Write;
use std::borrow::Cow;
lazy_static! {
  //static ref MAP: Arc<Mutex<HashMap<String,(Schedule,World)>>> = Arc::new(Mutex::new(HashMap::new()));
  //static ref APPMAP: Arc<Mutex<HashMap<String,App>>> = Arc::new(Mutex::new(HashMap::new()));
  static ref APP: Arc<Mutex<App>> = Arc::new(Mutex::new(App::new()));
}

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor,Thread,MessageSubscriber)]
struct GameLogicActor {}

fn my_runner(mut app: App) {
  loop {
      info_("In main loop".to_string());
      //let now = instant::Instant::now();
      let bytes:Vec<u8> =vec![];
      info_(String::from("nowing"));
      let mut m = StartThreadRequest::default();
      m.game_id =String::from("hello");
      let mb = serialize(&m).unwrap();
      let data = Cow::Borrowed(&mb);
      let mut buf = host_call("default","wasmcloud:thread","Thread.Now",&data).unwrap();
      let mut r_buf = [0_u8;8];
      buf.write(&mut r_buf);
      let now = u64::from_le_bytes(r_buf) as f64;
      info_(format!("now {:?}",now));
      app.update();
  }
}
#[async_trait]
impl Thread for GameLogicActor{
  async fn start_thread(&self, ctx: &Context, start_thread_request: &StartThreadRequest) -> RpcResult<StartThreadResponse> {
    info!("start_thread----");
    
    {
      // let a = App::new()
      // .init_resource::<Time>()
      // .add_startup_system(systems::spawn.system())
      // .add_system(systems::sys.system());
      let mut map = APP.clone();
      let mut m = map.lock().unwrap();
      m.init_resource::<Time>()
      .add_startup_system(systems::spawn.system())
      .add_system(systems::sys.system());
      //let ra = *a;
      // m.insert(start_thread_request.game_id.clone(),App::new()
      // .init_resource::<Time>()
      // .add_startup_system(systems::spawn.system())
      // .add_system(systems::sys.system()).borrow());
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
    // let mut map = APP.clone();
    Ok(StartThreadResponse{})
     //thread::thread_handle_request(map,start_thread_request).await
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
          //let mut map = MAP.clone();
          //client_message_handlers::target_velocity_handler::_fn(map,game_id,ball_id,target_velocity);  
        }
        Ok(ClientMessage::Welcome{game_id,ball_id})=>{
          //let mut map = MAP.clone();
          //client_message_handlers::welcome_handler::_fn(map,game_id,ball_id).await;
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

