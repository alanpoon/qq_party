
#[cfg(target_arch = "wasm32")]
mod wasm;
#[cfg(target_arch = "wasm32")]
use wasm::*;
#[cfg(not(target_arch = "wasm32"))]
mod native;
mod c_;
mod system;
mod msg_handler;
#[cfg(not(target_arch = "wasm32"))]
use native::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use core::ProtocolSystem;
use futures::prelude::*;
use nats_lite::nats;
use protocol::{ClientContext, ClientInput, ClientState, ClientStateDispatcher};
use protocol::{Command,Event,handle_client_op,handle_server_op};
//use crate::ClientStateDispatcher::ChickenDinner;
use tracing::error;
use wasm_bindgen::prelude::wasm_bindgen;
use bevy::utils::Duration;
use chrono::prelude::*;
use serde::{Deserialize,Serialize};
pub struct ProtocolPlugin;


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window, js_name = push_web_bevy_events_fn)]
    fn push_web_bevy_events_fn(msg: &str,msg_ago:&str,user:&str);
    #[wasm_bindgen(js_namespace = window, js_name = push_web_bevy_events_fn2)]
    fn push_web_bevy_events_fn2(msg: &str);
}

use qq_party_shared::*;
#[derive(Component,Clone,Debug)]
pub struct PlayerHealthCheckTimer(pub Timer);
#[derive(Component,Clone,Debug)]
pub struct CoolDownTimer(pub Timer,pub String);//timer, id of cooldown
#[derive(Component,Clone,Debug,Default)]
pub struct LastAxis(pub Vec2);
#[derive(Serialize, Deserialize,Clone,Debug)]
pub enum CoolDownMessage{
    DisplayUI(String),
    HideUI(String)
}
use client_websocket::{ClientName};
use client_websocket::{Client};
pub struct BoxClient{
  pub clients: Vec<Box<dyn Client + Send + Sync + 'static>>,
  pub options: nats::Options,
}
impl Default for BoxClient{
  fn default()->Self{
    BoxClient{
      clients:vec![],
      options: nats::Options::default()
    }
  }
}
pub type BoxClient2 = Box<dyn Client + Send + Sync + 'static>;

impl Plugin for ProtocolPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        let app = app
            .init_resource::<protocol::Commands>()
            .init_resource::<protocol::Events>()
            .init_resource::<Option<BoxClient>>()
            .init_resource::<Option<ClientStateDispatcher>>()
            .init_resource::<LocalUserInfo>()
            .init_resource::<LastAxis>()
            .init_resource::<qq_party_shared::StormTiming>()
            .add_system(add_client_state)
            .add_system(receive_events.label(ProtocolSystem::ReceiveEvents))
            .add_system(
                handle_events
                    
                    .label(ProtocolSystem::HandleEvents)
                    .after(ProtocolSystem::ReceiveEvents)
                    .before(ProtocolSystem::SendCommands),
            )
            .add_system(system::cooldown::hide_display_ui)
            .add_system(system::camera::move_with_local_player)
            .add_system(system::health_check::player_health_check)
            .add_system(send_commands.label(ProtocolSystem::SendCommands).after(ProtocolSystem::ReceiveEvents));
        app.add_startup_system(connect_websocket);
        #[cfg(target_arch = "wasm32")]
        app.add_system(set_client);
        #[cfg(target_arch = "wasm32")]
        app.add_system(listen_web_bevy_events);
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
    mut cmd: Commands,
    balls: Query<(&BallId,&Velocity,Option<&Dash>)>,
    cooldown_query: Query<&CoolDownTimer>,
    mut state: ResMut<Option<ClientStateDispatcher>>,
    mut commands: ResMut<protocol::Commands>,
    mut events: ResMut<protocol::Events>,
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut last_axes: ResMut<LastAxis>,
    gamepads: Res<Gamepads>,
    button_inputs: Res<Input<GamepadButton>>,
    local_user_info: Res<LocalUserInfo>,
    axes: Res<Axis<GamepadAxis>>
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
        for c in context.commands.iter(){
          (*commands).push(c.clone());
        }
        //*commands =context.commands ;
        let mut target_velocity_x = 0.0;
        let mut target_velocity_y = 0.0;
        let mut pressed = false;
        if keyboard_input.just_pressed(KeyCode::Left){
          if keyboard_input.pressed(KeyCode::Up){
            target_velocity_y += 1.0;
          }
          if keyboard_input.pressed(KeyCode::Down){
            target_velocity_y -= 1.0;
          }
          target_velocity_x -= 1.0;
          pressed = true;
        }
        if keyboard_input.just_pressed(KeyCode::Right){
          if keyboard_input.pressed(KeyCode::Up){
            target_velocity_y += 1.0;
          }
          if keyboard_input.pressed(KeyCode::Down){
            target_velocity_y -= 1.0;
          }
          target_velocity_x += 1.0;
          pressed = true;
        }
        if keyboard_input.just_pressed(KeyCode::Up){
          if keyboard_input.pressed(KeyCode::Left){
            target_velocity_x -= 1.0;
          }
          if keyboard_input.pressed(KeyCode::Right){
            target_velocity_x += 1.0;
          }
          target_velocity_y += 1.0;
          pressed = true;
        }
        if keyboard_input.just_pressed(KeyCode::Down){
          if keyboard_input.pressed(KeyCode::Left){
            target_velocity_x -= 1.0;
          }
          if keyboard_input.pressed(KeyCode::Right){
            target_velocity_x += 1.0;
          }
          target_velocity_y -= 1.0;
          pressed = true;
        }
        if keyboard_input.just_pressed(KeyCode::Space){
          let ball_id = (*local_user_info).0.ball_id;
          info!("space pressed-- fire");
          let c = c_::fire(ball_id,target_velocity_x,target_velocity_y,&mut cmd,&cooldown_query);
          if let Some(c) = c{
            (*commands).push(c);
          }
        }
        if keyboard_input.just_pressed(KeyCode::LShift){
          let ball_id = (*local_user_info).0.ball_id;
          info!("shift pressed-- dash ");
          let c = c_::dash(ball_id,&mut cmd,&cooldown_query);
          if let Some(c)=c{
            (*commands).push(c);
          }
          
        }
        keyboard_input.clear();
        for gamepad in gamepads.iter().cloned() {
          if  button_inputs.just_pressed(GamepadButton{gamepad, button_type:GamepadButtonType::DPadLeft}) {
            target_velocity_x -= 1.0;
            pressed = true;
          }else if  button_inputs.just_pressed(GamepadButton{gamepad, button_type:GamepadButtonType::DPadRight}) {
            target_velocity_x += 1.0;
            pressed = true;
          }else if  button_inputs.just_pressed(GamepadButton{gamepad, button_type:GamepadButtonType::DPadUp}){
            target_velocity_y += 1.0;
            pressed = true;
          }else if button_inputs.just_pressed(GamepadButton{gamepad,button_type: GamepadButtonType::DPadDown}) {
            target_velocity_y -= 1.0;
            pressed = true;
          } else if button_inputs.just_pressed(GamepadButton{gamepad, button_type:GamepadButtonType::West}) {
            let ball_id = (*local_user_info).0.ball_id;
            info!("space pressed-- fire");
            let c = c_::fire(ball_id,target_velocity_x,target_velocity_y,&mut cmd, &cooldown_query);
            if let Some(c) = c{
              (*commands).push(c);
            }
          } else if button_inputs.just_pressed(GamepadButton{gamepad, button_type:GamepadButtonType::East}) {
            let ball_id = (*local_user_info).0.ball_id;
            info!("shift pressed-- dash ");
            let c = c_::dash(ball_id,&mut cmd, &cooldown_query);
            if let Some(c)= c{
              (*commands).push(c);
            }
          }
          if !pressed{
            target_velocity_x = axes
            .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX))
            .unwrap();
            target_velocity_y = axes
            .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickY))
            .unwrap();
            let target_velocity_mag = target_velocity_x*target_velocity_x+target_velocity_y*target_velocity_y;
            if  target_velocity_mag >0.0 && !(target_velocity_x ==last_axes.0.x && target_velocity_y==last_axes.0.y){
              pressed = true;
              info!("left_stick_x {:?} left_stick_y {:?}",target_velocity_x,target_velocity_x);
              last_axes.0.x = target_velocity_x;
              last_axes.0.y = target_velocity_y;
            }
          }
         
        }  
        
        if pressed{
          for (ball_id_ingame,v,_) in balls.iter(){
            let ball_id = (*local_user_info).0.ball_id;
            if ball_id_ingame==&ball_id{
              let mut send= false;
              if target_velocity_x!=0.0{
                if v.linvel.x / target_velocity_x <0.0{
                  send = true;
                }else if v.linvel.x==0.0{
                  send = true;
                }
              }
              if target_velocity_y!=0.0{
                if v.linvel.y / target_velocity_y <0.0{
                  send = true;
                }else if v.linvel.y==0.0{
                  send = true;
                }
              }
              if send{
                info!("send target_velocity {:?} {:?}",target_velocity_x,target_velocity_y);
                let c = c_::target_velocity(ball_id,target_velocity_x,target_velocity_y);
                (*commands).push(c);
              }
            
              break;
            }
          }
        }
        
    }
}

use futures::future::ready;
fn send_commands(mut cmd: Commands,mut client:  ResMut<Option<BoxClient>>, mut commands: ResMut<protocol::Commands>,mut _events: ResMut<protocol::Events>) {
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
                  sender.send(handle_client_op(b.clone()).unwrap()).await.unwrap_or_else(|err| {
                      error!("{}", err);
                  });
                  //save_sub(b.subject,ClientName(Cow::Borrowed("default")));
                  //delay(10000).await;
                  ready(b_clone)
                });
              },
              Command::StoreLocal(user_info)=>{
                let local_user_info = LocalUserInfo(user_info);
                cmd.insert_resource(local_user_info);
                cmd.spawn().insert(PlayerHealthCheckTimer(Timer::new(Duration::new(15,0),true)));
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
  mut set: ParamSet<(
    Query<(Entity, &BallId,&mut Transform,&mut Velocity), With<BallId>>,
    Query<(Entity, &NPCId,&mut Transform,&mut Velocity,&mut ChaseTargetId), With<NPCId>>,
    Query<(Entity,&mut Transform),With<StormRingId>>,
    // also access the whole world ... why not
    //&World,
  )>,
  //mut query: Query<(Entity, &BallId,&mut TargetVelocity)> ) {
  // mut v_query: Query<(Entity, &BallId,&mut Transform,&mut Velocity),Without<NPCId>>,
  // mut npc_query: Query<(Entity, &NPCId,&mut Transform,&mut Velocity,&mut ChaseTargetId),Without<BallId>>,
  // mut query: Query<(Entity, &BallId)>,
  // mut storm_query: Query<(Entity,&mut Transform),With<StormRingId>>,
  storm_text_query: Query<Entity,With<StormRingTextNode>>,
  mut fire_query: Query<Entity,With<FireId>>,
  mut storm_timing_res: ResMut<StormTiming>,
  mut audioable: ResMut<AudioAble>,
  mut to_despawn: ResMut<EntityToRemove>,
  mut res_scoreboard: ResMut<ScoreBoard>,
  local_user_info: Res<LocalUserInfo>,
  asset_server: Res<AssetServer>
  ) {
    if let Some(ref mut client) = *client {
        let len = client.clients.len();   
        let _rand_int = get_random_int(0,len as i32);
        if let Some(vec) = client.clients.get_mut(0).unwrap().poll_once() {
            for event in vec {
                //if let Event::Nats(_client_name,s_op)=handle_server_op(event.clone()).unwrap(){
              let s_op = handle_server_op(event);
              match s_op{
                Ok(t)=>{
                  
                  match t.clone(){
                    nats::proto::ServerOp::Msg{subject,sid:_,reply_to:_,payload}=>{
                  
                      if subject.contains("game_logic"){
                      //if subject == String::from("game_logic"){
                        let server_message: ServerMessage = rmp_serde::from_slice(&payload).unwrap();
                        match server_message{
                          
                          ServerMessage::Dash{ball_id}=>{
                            msg_handler::dash::_fn(&mut cmd,&mut set,ball_id);                      
                          }
                          ServerMessage::Disconnect{ball_id,..}=>{
                            msg_handler::disconnect::_fn(&mut cmd,&mut set,ball_id,&mut to_despawn,&mut res_scoreboard);
                          }
                          ServerMessage::Fire{ball_id,velocity,sprite_enum}=>{  
                            msg_handler::fire::_fn(&mut cmd,&mut set,ball_id,velocity,sprite_enum);                          
                          }
                          ServerMessage::TargetVelocity{ball_id,target_velocity}=>{                            
                            //for (entity, qball_id,mut tv) in query.iter_mut(){
                              info!("receive {:?} tv {:?}",ball_id,target_velocity);
                              msg_handler::target_velocity::_fn(&mut cmd,&mut set,ball_id,target_velocity);
                          }
                          
                          ServerMessage::GameState{ball_bundles,npc_bundles,storm_timing,timestamp,..}=>{
                            let utc: DateTime<Utc> = Utc::now();
                            let server_utc = Utc.timestamp((timestamp /1000) as i64, (timestamp % 1000) as u32 * 1000000);
                            let delta =  utc.signed_duration_since(server_utc).num_milliseconds() as f32 / 1000.0;
                            msg_handler::game_state::_fn_spawn_or_update_ball_bundles(&mut cmd,&mut set,delta,ball_bundles);
                            msg_handler::game_state::_fn_spawn_or_update_npc_bundles(&mut cmd,&mut set,delta,npc_bundles);
                            *storm_timing_res = storm_timing;
                          }
                          ServerMessage::Scores{scoreboard,..}=>{
                            match serde_json::to_string(&ServerMessage::Scores{scoreboard}){
                              Ok(j)=>{
                                push_web_bevy_events_fn2(&j);
                              }
                              Err(e)=>{
                                info!("push_web_bevy_events_fn2 error {:?}",e);
                              }
                            }
                            
                          }
                          ServerMessage::StormRings{storm_rings,next_storm_timing,..}=>{
                            info!("storm_rings {:?} next_storm_timing{:?}",storm_rings.clone(),next_storm_timing);
                            msg_handler::storm_rings::_fn_spawn_or_delete(&mut cmd,&mut set,&storm_text_query,storm_rings.clone(),&mut to_despawn,&asset_server);
                            if let Some(storm_timing) = next_storm_timing.clone(){
                              *storm_timing_res = storm_timing;
                            }
                            match serde_json::to_string(&ServerMessage::StormRings{storm_rings,next_storm_timing}){
                              Ok(j)=>{
                                push_web_bevy_events_fn2(&j);
                              }
                              Err(e)=>{
                                info!("push_web_bevy_events_fn2 error {:?}",e);
                              }
                            }
                          }
                          ServerMessage::StateChange{state,scoreboard}=>{                            
                            match state{
                              QQState::Stop=>{
                                msg_handler::state_change::_fn_stop(&mut cmd,&mut set,&mut fire_query,&mut to_despawn);
                                for (_,mut v) in res_scoreboard.scores.iter_mut(){
                                  v.0=0;
                                }
                              }
                              QQState::Running=>{
                                
                              }
                              _=>{

                              }
                            }
                            // *state = Some(ClientStateDispatcher::ChickenDinner(protocol::ChickenDinner {}));
                            // protocol::pre_chicken_dinner_unsub_all();
                            match serde_json::to_string(&ServerMessage::StateChange{state,scoreboard}){
                              Ok(j)=>{
                                push_web_bevy_events_fn2(&j);
                              }
                              Err(e)=>{
                                info!("push_web_bevy_events_fn2 error {:?}",e);
                              }
                            }
                          }
                          ServerMessage::StateNotification{countdown,text}=>{
                            match serde_json::to_string(&ServerMessage::StateNotification{countdown,text}){
                              Ok(j)=>{
                                push_web_bevy_events_fn2(&j);
                              }
                              Err(e)=>{
                                info!("push_web_bevy_events_fn2 error {:?}",e);
                              }
                            }
                          }
                          _=>{}
                        }
                        
                        continue
                      }else if subject.contains("welcome"){
                        let server_message: ServerMessage = rmp_serde::from_slice(&payload).unwrap();
                        match server_message{
                          ServerMessage::Welcome{ball_bundle,sub_map,qq_state}=>{
                            info!("welcome_ ball_bundle {:?} qq_state {:?}",ball_bundle.clone(),qq_state);
                            res_scoreboard.scores.insert(ball_bundle.ball_id.0.clone(),(0,ball_bundle.ball_label.clone()));
                            cmd.spawn_bundle(ball_bundle.clone()).insert(Dash(false,[0.0,0.0].into(),[0.0,0.0].into()));
                            audioable.0 = true;
                            if ball_bundle.ball_id == (*local_user_info).0.ball_id{
                              if let QQState::Stop= qq_state{
                                match serde_json::to_string(&ServerMessage::Welcome{qq_state,ball_bundle,sub_map}){
                                  Ok(j)=>{
                                    push_web_bevy_events_fn2(&j);
                                  }
                                  Err(e)=>{
                                    info!("push_web_bevy_events_fn2 error {:?}",e);
                                  }
                                }
                              }
                            }
                            //commands.commands.push(Command::Nats(String::from("default"),n))
                          }
                          _=>{}
                        }
                      }else if subject.contains("chat"){
                        let server_message: ServerMessage = rmp_serde::from_slice(&payload).unwrap();
                        match server_message{
                          ServerMessage::Chat{msg,msg_ago,user,user_id:_}=>{
                            push_web_bevy_events_fn(&msg,&msg_ago,&user);
                            
                          }
                          _=>{}
                        }
                      }
                    },
                    _=>{

                    }
                  }
                  let event =Event::Nats(String::from("some_client"), t);
                  events.push(event);
                }
                Err(e)=>{
                  info!("protocol e {:?}",e);
                }
              
              }
               
            }
        }
    }
}
