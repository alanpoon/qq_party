extern crate wasmcloud_interface_messaging as messaging;
mod host_call;
mod info_;
mod messaging_;
mod spawn_;
mod bevy_wasmcloud_time;
mod thread;
mod client_message_handlers;
mod systems;
<<<<<<< HEAD
=======
mod startup;
>>>>>>> develop
mod update_client_state;
use info_::info_;
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_logging::{info,error,debug};
use wasmcloud_interface_messaging::{MessageSubscriber,SubMessage};
use wasmcloud_interface_thread::{StartThreadRequest, StartThreadResponse,Thread,ThreadReceiver,ThreadSender};
use messaging::*;
use lazy_static::lazy_static; // 1.4.0
use bevy_ecs_wasm::prelude::{Schedule,World,Query,SystemStage,IntoSystem,Res,Component};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use serde::{Serialize,Deserialize};
use qq_party_shared::*;
lazy_static! {
  static ref MAP: Arc<Mutex<HashMap<String,(Schedule,World)>>> = Arc::new(Mutex::new(HashMap::new()));
}
#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor,Thread,MessageSubscriber)]
struct GameLogicActor {}

#[async_trait]
impl Thread for GameLogicActor{
  async fn start_thread(&self, ctx: &Context, start_thread_request: &StartThreadRequest) -> RpcResult<StartThreadResponse> {
    info!("start_thread----");
    let mut world = World::default();
    world.spawn().insert(A{position:0});
<<<<<<< HEAD
=======
    startup::npc::spawn(&mut world).await?;
>>>>>>> develop
    world.insert_resource(Time{elapsed:0.0});
    {
    let mut map = MAP.clone();
    let mut m = map.lock().unwrap();
    let mut schedule = Schedule::default();
    let mut update = SystemStage::single_threaded();
<<<<<<< HEAD
    
=======
>>>>>>> develop
    update.add_system(systems::sys.system());
    update.add_system(systems::sys_ball_bundle_debug.system());
    update.add_system(systems::sys_bevy_wasmcloud_time.system());
    update.add_system(systems::publish::sys_publish_game_state.system());
    update.add_system(qq_party_shared::systems::update_state_position::<bevy_wasmcloud_time::Time>.system());
    update.add_system(qq_party_shared::systems::update_state_velocity.system());
<<<<<<< HEAD
=======
    update.add_system(qq_party_shared::systems::set_state_chasetarget_npc.system());
    update.add_system(qq_party_shared::systems::update_state_velocity_npc.system());
>>>>>>> develop
    update.add_system(systems::sys_health_check_despawn.system());
    schedule.add_stage("update", update);
    
    m.insert(start_thread_request.game_id.clone(),(schedule,world));
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
    Ok(StartThreadResponse{})
  }
  async fn handle_request(&self, ctx: &Context, start_thread_request: &StartThreadRequest) -> RpcResult<StartThreadResponse> {
    info!("handle_request----");
    let mut map = MAP.clone();
    thread::thread_handle_request(map,start_thread_request).await
  }
}
#[async_trait]
impl MessageSubscriber for GameLogicActor{
  async fn handle_message(&self, ctx: &Context, req: &SubMessage) -> RpcResult<()> {
    if req.subject.contains("client_handler"){
      let client_message: Result<ClientMessage,_> = serde_json::from_slice(&req.body);
      match client_message{
        Ok(ClientMessage::TargetVelocity{game_id,ball_id,target_velocity})=>{
          let mut map = MAP.clone();
          client_message_handlers::target_velocity_handler::_fn(map,game_id,ball_id,target_velocity);  
        }
        Ok(ClientMessage::Welcome{game_id,ball_id})=>{
          let mut map = MAP.clone();
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
<<<<<<< HEAD
#[derive(Debug, Eq, PartialEq, Default,Serialize, Deserialize,Clone)]
=======
#[derive(Component,Debug, Eq, PartialEq, Default,Serialize, Deserialize,Clone)]
>>>>>>> develop
pub struct A{
  position: i32,
}
#[derive(Debug, PartialEq, Default)]
pub struct Time{pub elapsed:f32}
impl Time{
  pub fn update(&mut self,t:f32){
    self.elapsed = t;
  }
}


// fn sys(mut query: Query<&mut A>,time: Res<Time>) {
//   //logging::default().write_log("LOGGING_ACTORINFO", "info", "sysing").unwrap();
//   for  mut a in query.iter_mut() {
//       let n = format!("sys a >{:?}, t >{:?}",a,2);
//       info_(n);
//       //logging::default().write_log("LOGGING_ACTORINFO", "info", &n).unwrap();
//       a.position = a.position + 1;
//   }
// }
// fn sys_bevy_wasmcloud_time(time: Res<bevy_wasmcloud_time::Time>) {
//     let n = format!("bevy_wasmcloud_time sys t >{:?}",*time);
//     info_(n);
// }
// fn spawn_ball_system(mut cmd: Commands, unowned_balls: Query<&BallId, Without<NetworkHandle>>) {
//   let mut count = 0;
//   let mut highest_id = 0;
//   for ball in unowned_balls.iter() {
//       count += 1;
//       highest_id = highest_id.max(ball.0);
//   }

//   if count < 3 {
//       cmd.spawn_bundle((
//           BallId(highest_id + 1),
//           Position(vec2(
//               rand::random::<f32>() * 10.0 - 5.0,
//               rand::random::<f32>() * 10.0 - 5.0,
//           )),
//           Velocity::default(),
//           TargetVelocity::default(),
//       ));

//       println!("Spawned ball {:?}", highest_id + 1);
//   }
// }

