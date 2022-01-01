extern crate wasmcloud_interface_messaging as messaging;
mod host_call;
mod info_;
mod messaging_;
use host_call::host_call;
use info_::info_;
use messaging_::publish_;
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_logging::{info,error,debug};
use wasmcloud_interface_messaging::PubMessage;
use wasmcloud_interface_thread::{StartThreadRequest, StartThreadResponse,Thread,ThreadReceiver,ThreadSender};
use messaging::*;
use lazy_static::lazy_static; // 1.4.0
use bevy_ecs_wasm::prelude::{Schedule,World,Query,SystemStage,IntoSystem,Res};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use serde::{Serialize,Deserialize};
use qq_party_shared::update_velocity_system;

//use arugio_shared::update_velocity_system;
lazy_static! {
  static ref MAP: Arc<Mutex<HashMap<String,(Schedule,World)>>> = Arc::new(Mutex::new(HashMap::new()));
}
#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor,Thread)]
struct GameLogicActor {}

#[async_trait]
impl Thread for GameLogicActor{
  async fn start_thread(&self, ctx: &Context, start_thread_request: &StartThreadRequest) -> RpcResult<StartThreadResponse> {
    info!("start_thread----");
    let mut world = World::default();
    world.spawn().insert(A{position:0});
    {
    let mut map = MAP.clone();
    let mut m = map.lock().unwrap();
    let mut schedule = Schedule::default();
    let mut update = SystemStage::single_threaded();
    update.add_system(sys.system());
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
    let mut n = String::from("");
    {
      let mut guard = match map.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
          n = format!("{:?}",poisoned);
          poisoned.into_inner()
        },
      };
      if let Some((ref mut s, ref mut w))= guard.get_mut(&start_thread_request.game_id){
          if let Some(mut t) = w.get_resource_mut::<Time>(){
            n = String::from("can find time");
            t.update(start_thread_request.elapsed as f32);
          }else{
            w.insert_resource(Time{elapsed:start_thread_request.elapsed as f32});
          }
        
        // /w.spawn().insert_bundle(arugio_shared::BallBundle);
        s.run_once(w);
      }else{
        n = String::from("can't find");
      }
    }
    info!("{}",n);
    Ok(StartThreadResponse{})
  }
}

// fn stop_thread(req: game_engine::StartThreadRequest) -> HandlerResult<game_engine::StartThreadResponse> {
//   //logging::default().write_log("LOGGING_ACTORINFO", "info", "Stop thread")?;
//   game_engine::stop_thread(req)
// }
#[derive(Debug, Eq, PartialEq, Default,Serialize, Deserialize,Clone)]
struct A{
  position: i32,
}
#[derive(Debug, PartialEq, Default)]
struct Time{pub elapsed:f32}
impl Time{
  pub fn update(&mut self,t:f32){
    self.elapsed = t;
  }
}


fn sys(mut query: Query<&mut A>,time: Res<Time>) {
  //logging::default().write_log("LOGGING_ACTORINFO", "info", "sysing").unwrap();
  for  mut a in query.iter_mut() {
      let n = format!("sys a >{:?}, t >{:?}",a,2);
      info_(n);
      //logging::default().write_log("LOGGING_ACTORINFO", "info", &n).unwrap();
      a.position = a.position + 1;
      let b = serde_json::to_vec(&a.clone())
                   .unwrap();
      let pMsg = PubMessage{
        body:b,
        reply_to: None,
        subject: "game_logic".to_owned()
      };
      publish_(pMsg);
  }
}
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

