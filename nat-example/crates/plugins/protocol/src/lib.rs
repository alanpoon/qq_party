
#[cfg(target_arch = "wasm32")]
mod wasm;
#[cfg(target_arch = "wasm32")]
use wasm::*;
#[cfg(not(target_arch = "wasm32"))]
mod native;
mod userinfo;
mod c_;
mod timewrapper;
mod system;
<<<<<<< HEAD
=======
mod gamestate;
>>>>>>> develop
#[cfg(not(target_arch = "wasm32"))]
use native::*;
use bevy::prelude::*;
use core::ProtocolSystem;
use futures::prelude::*;
use protocol::{BoxClient, ClientContext, ClientInput, ClientState, ClientStateDispatcher,ClientName};
use protocol::{Command,Event,nats};
use tracing::error;
use client_websocket::save_sub;
use std::borrow::Cow;
use bevy::math::Vec2;
use wasm_bindgen::prelude::wasm_bindgen;
use wasmcloud_interface_messaging::SubMessage;
use wasmbus_rpc::serialize;
use serde::{Serialize,Deserialize};
use chrono::prelude::*;
use userinfo::LocalUserInfo;
pub struct ProtocolPlugin;
#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}
macro_rules! console_log {
  // Note that this is using the `log` function imported above during
  // `bare_bones`
  ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}
<<<<<<< HEAD
use qq_party_shared::{Position,TargetVelocity,Velocity,BallId,ClientMessage,ServerMessage,BallBundle};
=======
use qq_party_shared::{Position,TargetVelocity,Velocity,BallId,NPCId,ClientMessage,ServerMessage,BallBundle,ChaseTargetId};
>>>>>>> develop
impl Plugin for ProtocolPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        let app = app
            .init_resource::<protocol::Commands>()
            .init_resource::<protocol::Events>()
            .init_resource::<Option<BoxClient>>()
            .init_resource::<Option<ClientStateDispatcher>>()
            .init_resource::<LocalUserInfo>()
            .init_resource::<qq_party_shared::Time>()
            .init_resource::<timewrapper::TimeWrapper>()
            .add_system(add_client_state.system())
            .add_system(receive_events.system().label(ProtocolSystem::ReceiveEvents))
            .add_system(
                handle_events
                    .system()
                    .label(ProtocolSystem::HandleEvents)
                    .after(ProtocolSystem::ReceiveEvents)
                    .before(ProtocolSystem::SendCommands),
            )
            .add_system(timewrapper::into_timewrapper.system())
            //.add_system(qq_party_shared::systems::auto_target_velocity::<timewrapper::TimeWrapper>.system())
            .add_system(system::camera::move_with_local_player.system())
            .add_system(send_commands.system().label(ProtocolSystem::SendCommands).after(ProtocolSystem::ReceiveEvents));
            //.add_system(send_commands.system());
        app.add_startup_system(connect_websocket.system());
        #[cfg(target_arch = "wasm32")]
        app.add_system(set_client.system());
        // #[cfg(target_arch = "wasm32")]
        // app.add_system(dial_loop.system());
    }
}

fn add_client_state(
    client: ResMut<Option<BoxClient>>,
    mut state: ResMut<Option<ClientStateDispatcher>>,
) {
    if client.is_some() && state.is_none() {
        *state = Some(Default::default())
    }
}

fn handle_events(
    mut state: ResMut<Option<ClientStateDispatcher>>,
    mut commands: ResMut<protocol::Commands>,
    mut events: ResMut<protocol::Events>,
    keyboard_input: Res<Input<KeyCode>>,
    gamepads: Res<Gamepads>,
    button_inputs: Res<Input<GamepadButton>>,
    local_user_info: Res<LocalUserInfo>,
    mut balls: Query<&Velocity>,
    
) {
    if let Some(ref mut state) = *state {
        let mut context = ClientContext {
            commands: Default::default(),
        };
        for event in events.iter() {
          *state = state.handle(&mut context, &ClientInput::Event(event.clone()));
        }
        let ref mut e = *events;
        e.clear();
        e.truncate();//added
        *commands = context.commands;
        let mut target_velocity_x = 0.0;
        let mut target_velocity_y = 0.0;
        let mut pressed = false;
        if keyboard_input.pressed(KeyCode::Left){
          target_velocity_x -= 1.0;
          pressed = true;
        }
        if keyboard_input.pressed(KeyCode::Right){
          target_velocity_x += 1.0;
          pressed = true;
        }
        if keyboard_input.pressed(KeyCode::Up){
          target_velocity_y += 1.0;
          pressed = true;
        }
        if keyboard_input.pressed(KeyCode::Down){
          target_velocity_y -= 1.0;
          pressed = true;
        }
        if pressed{
          let ball_id = (*local_user_info).0.ball_id;
          let c = c_::target_velocity(ball_id,target_velocity_x,target_velocity_y);
          (*commands).push(c);
        }

        for gamepad in gamepads.iter().cloned() {
          if button_inputs.just_pressed(GamepadButton(gamepad, GamepadButtonType::South)) {
            let ball_id = (*local_user_info).0.ball_id;
            let c = c_::target_velocity(ball_id,target_velocity_x,target_velocity_y);
            (*commands).push(c);  
            info!("{:?} just pressed South", gamepad);
          }
        }
    }
}
use futures::future::ready;
fn send_commands(mut cmd: Commands,mut client:  ResMut<Option<BoxClient>>, mut commands: ResMut<protocol::Commands>,mut events: ResMut<protocol::Events>) {
    if let Some(ref mut client) = *client {
        for command in commands.iter() {
            let command = command.clone();
            let len = client.clients.len();
            let rand_int = get_random_int(0,len as i32);
            let mut sender = client.clients.get_mut(rand_int).unwrap().sender();
            match command{
              Command::Nats(_,b)=>{
                let b_clone = b.clone();
                block_on(async move {
                  sender.send(b.clone()).await.unwrap_or_else(|err| {
                      error!("{}", err);
                  });
                  //save_sub(b.subject,ClientName(Cow::Borrowed("default")));
                  //delay(10000).await;
                  info!("after 10 secsend{:?}",b);
                  ready(b_clone)
                });
              },
              Command::StoreLocal(user_info)=>{
                let local_user_info = userinfo::LocalUserInfo(user_info);
                cmd.insert_resource(local_user_info);
              }
              _=>{}
            }

        }
        commands.clear();
    }
}
fn receive_events(mut cmd: Commands,
  mut client: ResMut<Option<BoxClient>>, 
  mut events: ResMut<protocol::Events>,
  user_info: Res<LocalUserInfo>,
  //mut query: Query<(Entity, &BallId,&mut TargetVelocity)> ) {
<<<<<<< HEAD
    mut v_query: Query<(Entity, &BallId,&mut Position,&mut Velocity,&mut TargetVelocity)>,
    mut query: Query<(Entity, &BallId)> ) {
=======
  mut v_query: Query<(Entity, &BallId,&mut Position,&mut Velocity,&mut TargetVelocity),Without<NPCId>>,
  mut npc_query: Query<(Entity, &NPCId,&mut Position,&mut Velocity,&mut ChaseTargetId),Without<BallId>>,
    mut query: Query<(Entity, &BallId)>, ) {
>>>>>>> develop
    if let Some(ref mut client) = *client {
        let len = client.clients.len();   
        let rand_int = get_random_int(0,len as i32);
        if let Some(vec) = client.clients.get_mut(0).unwrap().poll_once() {
            for event in vec {
                if let Event::Nats(client_name,s_op)=event.clone(){
                  match s_op{
                    nats::proto::ServerOp::Msg{subject,sid,reply_to,payload}=>{
                      if subject == String::from("game_logic"){
                        info!("recv msg!! game_logic {} payload:{} user{:?}",subject,std::str::from_utf8(&payload).unwrap(),(*user_info));
                        let server_message: ServerMessage = serde_json::from_slice(&payload).unwrap();
                        match server_message{
                          ServerMessage::TargetVelocity{ball_id,target_velocity}=>{                            
                            //for (entity, qball_id,mut tv) in query.iter_mut(){
                            for (entity, qball_id) in query.iter_mut(){
                              if &ball_id ==qball_id{
                                info!("insert target_velocity!!,{:?} {:?}",qball_id,ball_id);
                                cmd.entity(entity).insert(target_velocity);
                              }
                            }
                          }
                          ServerMessage::Welcome{ball_bundle}=>{
                            // let mut not_init = vec![];
                            // for welcome_balls in ball_bundles.iter(){
                            //   for (_, qball_id) in query.iter_mut(){
                            //     if &welcome_balls.ball_id!=qball_id{
                            //       not_init.push(welcome_balls.clone());
                            //     }
                            //   }
                            // }
                            // info!("recv msg!! spawn {:?}",not_init);
                            cmd.spawn_bundle(ball_bundle);
                          }
<<<<<<< HEAD
                          ServerMessage::GameState{ball_bundles,timestamp}=>{
=======
                          ServerMessage::GameState{ball_bundles,npc_bundles,timestamp,..}=>{
>>>>>>> develop
                            
                            let utc: DateTime<Utc> = Utc::now();
                            let server_utc = Utc.timestamp((timestamp /1000) as i64, (timestamp % 1000) as u32 * 1000000);
                            let delta =  utc.signed_duration_since(server_utc).num_milliseconds() as f32 / 1000.0;
<<<<<<< HEAD
                            info!("recv msg!! gamestate {:?} delta {:?} server_utc{:?}",ball_bundles,delta,server_utc);
                            let len = ball_bundles.len();
                            let mut founds = vec![];
                            for (entity, ball_id,mut pos, mut v,mut tv) in v_query.iter_mut(){
                              //if ball_id ==&(*user_info).0.ball_id{
                              let mut found= false;
                              for i in 0..len{
                                let ball_bundle = ball_bundles.get(i).unwrap();
                                if &ball_bundle.ball_id == ball_id{
                                  *v = ball_bundle.velocity;
                                  //*pos = ball_bundle.position;
                                  (*pos).0.x = ball_bundle.position.0.x+ ball_bundle.velocity.0.x *delta;
                                  (*pos).0.y = ball_bundle.position.0.y+ ball_bundle.velocity.0.y *delta;
                                  *tv = ball_bundle.target_velocity;
                                  //cmd.entity(entity).insert(*v);
                                  founds.push(i);
                                  found = true;
                                  break;
                                }
                              }
                              if !found{
                                cmd.entity(entity).despawn();
                              }
                              //}
                            }
                            for (i,ball_bundle) in ball_bundles.iter().enumerate(){
                              if !founds.contains(&i){
                                cmd.spawn_bundle(ball_bundle.clone());
                              }
                            }
=======
                            gamestate::spawn_or_update_ball_bundles(&mut cmd,&mut v_query,delta,ball_bundles);
                            gamestate::spawn_or_update_npc_bundles(&mut cmd,&mut npc_query,delta,npc_bundles);

>>>>>>> develop
                          }
                          _=>{}
                        }
                        
                        continue
                      }
                    }
                    _=>{
                      
                    }
                  } 
                }
                events.push(event);
            }
        }
    }
}
