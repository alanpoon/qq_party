extern crate wasmcloud_interface_messaging as messaging;
mod host_call;
mod info_;
mod messaging_;
mod spawn_;
mod bevy_wasmcloud_time;
use qq_party_shared::time_interface::TimeInterface;
use spawn_::spawn;
use host_call::host_call;
use info_::info_;
use messaging_::publish_;
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_logging::{info,error,debug};
use wasmcloud_interface_messaging::{MessageSubscriber,PubMessage,SubMessage};
use wasmcloud_interface_thread::{StartThreadRequest, StartThreadResponse,Thread,ThreadReceiver,ThreadSender};
use wasmcloud_interface_numbergen::random_in_range;
use messaging::*;
use lazy_static::lazy_static; // 1.4.0
use bevy_ecs_wasm::prelude::{Schedule,World,Query,SystemStage,IntoSystem,Res};
use bevy_math::Vec2;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use serde::{Serialize,Deserialize};
use qq_party_shared::*;
use bevy_ecs_wasm::component::Component;
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
    {
    let mut map = MAP.clone();
    let mut m = map.lock().unwrap();
    let mut schedule = Schedule::default();
    let mut update = SystemStage::single_threaded();
    update.add_system(sys.system());
    update.add_system(sys_bevy_wasmcloud_time.system());
    update.add_system(update_position_system::<bevy_wasmcloud_time::Time>.system());
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
          if let Some(mut t) = w.get_resource_mut::<bevy_wasmcloud_time::Time>(){
            n = String::from("can find time");
            //t.update(start_thread_request.elapsed as f32);
            t.update_with_timestamp(start_thread_request.timestamp)
          }else{
            w.insert_resource(bevy_wasmcloud_time::Time{timestamp:start_thread_request.timestamp,..Default::default()});
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
#[async_trait]
impl MessageSubscriber for GameLogicActor{
  async fn handle_message(&self, ctx: &Context, req: &SubMessage) -> RpcResult<()> {
    info!("handle_message {:?}",req);
    if req.subject.contains("client_handler"){
      let client_message: Result<ClientMessage,_> = serde_json::from_slice(&req.body);
      match client_message{
        Ok(ClientMessage::TargetVelocity{game_id,ball_id,target_velocity})=>{
          let mut map = MAP.clone();
          info!("handle_message map");
          let mut guard = match map.lock() {
            Ok(guard) => guard,
            Err(poisoned) => {
              poisoned.into_inner()
            },
          };
          if let Some((ref mut s, ref mut w))= guard.get_mut(&game_id){
            if let Some(mut tv) = w.get_resource_mut::<TargetVelocity>(){
              *tv = target_velocity;
            }else{
              w.insert_resource(target_velocity);
            }
          }
         // let b = serde_json::to_vec(&a.clone())?;
          let server_message = ServerMessage::TargetVelocity{ball_id,target_velocity};
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
         
        }
        Ok(ClientMessage::Welcome{game_id,ball_id})=>{
          let mut map = MAP.clone();
          info!("handle_message map");
          let x = random_in_range(35,70).await?;
          let y = random_in_range(35,200).await?;
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
  }
}
fn sys_bevy_wasmcloud_time(time: Res<bevy_wasmcloud_time::Time>) {
    let n = format!("bevy_wasmcloud_time sys t >{:?}",*time);
    info_(n);
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

