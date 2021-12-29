extern crate wasmcloud_interface_messaging as messaging;
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_logging::{info,error,debug};
use wasmcloud_interface_thread::{StartThreadRequest, StartThreadResponse,Thread,ThreadReceiver,ThreadSender};
use messaging::*;
use lazy_static::lazy_static; // 1.4.0
use bevy_ecs::prelude::{Schedule,World,Component,Query,SystemStage,IntoSystem,Res};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use serde::{Serialize,Deserialize};
// mod generated;
// pub use generated::*;
// use crayon::prelude::*;

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
    debug!("start_thread----");
    let mut world = World::default();
    world.spawn().insert(A{position:0});
    //world.spawn().insert(ContextWrapper(ctx));
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
    debug!("handle_request----");
    Ok(StartThreadResponse{})
  }
}

// fn start_thread(req: game_engine::StartThreadRequest) -> HandlerResult<game_engine::StartThreadResponse> {
//   //logging::default().write_log("LOGGING_ACTORINFO", "info", "Start Thread")?;
//   let mut world = World::default();
//   world.spawn().insert(A{position:0});
//   let mut map = MAP.clone();
//   let mut m = map.lock().unwrap();
//   let mut schedule = Schedule::default();
//   let mut update = SystemStage::single_threaded();
//   update.add_system(sys.system());
//   schedule.add_stage("update", update);
//   m.insert(req.game_id.clone(),(schedule,world));
//   game_engine::start_thread(req)
// }
// fn stop_thread(req: game_engine::StartThreadRequest) -> HandlerResult<game_engine::StartThreadResponse> {
//   //logging::default().write_log("LOGGING_ACTORINFO", "info", "Stop thread")?;
//   game_engine::stop_thread(req)
// }
// fn poll_from_thread(req: game_engine::StartThreadRequest) -> HandlerResult<game_engine::StartThreadResponse>{
//   let b = format!("handle_thread {:?}",req.game_id);
//   //logging::default().write_log("LOGGING_ACTORINFO", "info", &b)?;
//   let mut map = MAP.clone();
//   let mut m = map.lock().unwrap();
//   if let Some((ref mut s, ref mut w))= m.get_mut(&req.game_id){
//     if let Some(e) = req.elapsed{
//       if let Some(mut t) = w.get_resource_mut::<Time>(){
//         t.update(e);
//       }else{
//         w.insert_resource(Time{elapsed:e});
//       }
//     }else{
//       w.insert_resource(Time{elapsed:100.0});
//     }
//     // /w.spawn().insert_bundle(arugio_shared::BallBundle);
//     s.run_once(w);
    
//   }else{
//     //logging::default().write_log("LOGGING_ACTORINFO", "info", "cannot find")?;
//   }
//   Ok(game_engine::StartThreadResponse{})
// }
#[derive(Component,Debug, Eq, PartialEq, Default,Serialize, Deserialize,Clone)]
struct A{
  position: i32,
}
#[derive(Component,Debug, PartialEq, Default)]
struct Time{pub elapsed:f32}
impl Time{
  pub fn update(&mut self,t:f32){
    self.elapsed = t;
  }
}

#[derive(Component)]
struct ContextWrapper(pub Context);

//fn sys(mut query: Query<(&mut A,ContextWrapper)>,time: Res<Time>) {
fn sys(mut query: Query<&mut A>,time: Res<Time>) {
  //logging::default().write_log("LOGGING_ACTORINFO", "info", "sysing").unwrap();
  for mut a in query.iter_mut() {
  //for mut (a,ctx) in query.iter_mut() {
      let n = format!("sys a >{:?}, t >{:?}",a,2);
      //logging::default().write_log("LOGGING_ACTORINFO", "info", &n).unwrap();
      a.position = a.position + 1;
      futures::executor::block_on( async move {
        let provider = MessagingSender::new();
        if let Err(e) = provider
          .publish(
              &Context{actor:Some("".to_string()),span:Some("default".to_string())},
              &PubMessage {
                  body: serde_json::to_vec(&a.clone())
                  .unwrap(),
                  reply_to: None,
                  subject: "game_logic".to_owned(),
              },
          )
          .await
        {
        
        }
      });
      
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

