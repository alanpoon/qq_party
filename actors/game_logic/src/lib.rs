extern crate wapc_guest as guest;
use guest::prelude::*;
use log::{debug, error, info};
use wasmcloud_actor_core as actor;
use wasmcloud_actor_keyvalue as keyvalue;
use wasmcloud_actor_logging as logging;
use wasmcloud_game as game_engine;
use lazy_static::lazy_static; // 1.4.0
use bevy_ecs::prelude::*;
//use bevy_ecs::archetype::Archetype;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
//use arugio_shared::update_velocity_system;
lazy_static! {
  static ref MAP: Arc<Mutex<HashMap<String,(Schedule,World)>>> = Arc::new(Mutex::new(HashMap::new()));
}
#[actor::init]
fn init() {
    logging::enable_macros();
    game_engine::Handlers::register_start_thread_request(start_thread);
    game_engine::Handlers::register_stop_thread(stop_thread);
    game_engine::Handlers::register_handle_thread(poll_from_thread);
}

//wash ctl call MDN3AIPQ62QAFZJCSULSCR5D2NQYARPDYK763YLG4EYZLMPKECEWIFY2 StartThreadRequest '{"game_id": "hi"}'
//wash ctl call MDN3AIPQ62QAFZJCSULSCR5D2NQYARPDYK763YLG4EYZLMPKECEWIFY2 StopThread '{"game_id": "hi"}'
//wash ctl call VB4RKGH3TX7A2H2BXZFY32SRJAYITADXN2TOP4XR4UVWDILSBU3FIGIV.default StartThread '{"game_id": "hi"}'
// fn run_system<Param>(world: &mut World, system: impl IntoSystemDescriptor<Param>) {
//   let mut schedule = Schedule::default();
//   let mut update = SystemStage::parallel();
//   update.add_system(system);
//   schedule.add_stage("update", update);
//   schedule.run(world);
// }
// pub fn add_system<Params>(&mut self, system: impl IntoSystemDescriptor<Params>) -> &mut Self {
//   self.add_system_to_stage(CoreStage::Update, system)
// }
fn start_thread(req: game_engine::StartThreadRequest) -> HandlerResult<game_engine::StartThreadResponse> {
  logging::default().write_log("LOGGING_ACTORINFO", "info", "Start Thread")?;
  let mut world = World::default();
  world.spawn().insert(A(0));
  let mut map = MAP.clone();
  let mut m = map.lock().unwrap();
  let mut schedule = Schedule::default();
  let mut update = SystemStage::single_threaded();
  update.add_system(sys.system());
  schedule.add_stage("update", update);
  m.insert(req.game_id.clone(),(schedule,world));
  game_engine::start_thread(req)
}
fn stop_thread(req: game_engine::StartThreadRequest) -> HandlerResult<game_engine::StartThreadResponse> {
  logging::default().write_log("LOGGING_ACTORINFO", "info", "Stop thread")?;
  game_engine::stop_thread(req)
}
fn poll_from_thread(req: game_engine::StartThreadRequest) -> HandlerResult<game_engine::StartThreadResponse>{
  let b = format!("handle_thread {:?}",req.game_id);
  logging::default().write_log("LOGGING_ACTORINFO", "info", &b)?;
  let mut map = MAP.clone();
  let mut m = map.lock().unwrap();
  if let Some((ref mut s, ref mut w))= m.get_mut(&req.game_id){
    if let Some(e) = req.elapsed{
      if let Some(mut t) = w.get_resource_mut::<Time>(){
        t.update(e);
      }else{
        w.insert_resource(Time{elapsed:e});
      }
    }else{
      w.insert_resource(Time{elapsed:100.0});
    }
    // /w.spawn().insert_bundle(arugio_shared::BallBundle);
    s.run_once(w);
  }else{
    logging::default().write_log("LOGGING_ACTORINFO", "info", "cannot find")?;
  }
  Ok(game_engine::StartThreadResponse{})
}
#[derive(Component,Debug, Eq, PartialEq, Default)]
struct A(i32);
#[derive(Component,Debug, PartialEq, Default)]
struct Time{pub elapsed:f32}
impl Time{
  pub fn update(&mut self,t:f32){
    self.elapsed = t;
  }
}
fn sys(mut query: Query<&mut A>,time: Res<Time>) {
  logging::default().write_log("LOGGING_ACTORINFO", "info", "sysing").unwrap();
  for mut a in query.iter_mut() {
      let n = format!("sys a >{:?}, t >{:?}",a,2);
      logging::default().write_log("LOGGING_ACTORINFO", "info", &n).unwrap();
      a.0 = a.0 + 1;
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

