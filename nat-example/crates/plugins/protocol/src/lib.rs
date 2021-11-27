
#[cfg(target_arch = "wasm32")]
mod wasm;
#[cfg(target_arch = "wasm32")]
use wasm::*;
#[cfg(not(target_arch = "wasm32"))]
mod native;
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
pub struct ProtocolPlugin;
use arugio_shared::Position;
impl Plugin for ProtocolPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        let app = app
            .init_resource::<protocol::Commands>()
            .init_resource::<protocol::Events>()
            .init_resource::<Option<BoxClient>>()
            .init_resource::<Option<ClientStateDispatcher>>()
            .init_resource::<arugio_shared::Time>()
            .add_system(add_client_state.system())
            .add_system(receive_events.system().label(ProtocolSystem::ReceiveEvents))
            .add_system(receive_events.system())
            .add_system(
                handle_events
                    .system()
                    .label(ProtocolSystem::HandleEvents)
                    .after(ProtocolSystem::ReceiveEvents)
                    .before(ProtocolSystem::SendCommands),
            )
            .add_system(arugio_shared::update_position_system.system())
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
) {
    if let Some(ref mut state) = *state {
        let mut context = ClientContext {
            commands: Default::default(),
        };
        for event in events.iter() {
            *state = state.handle(&mut context, &ClientInput::Event(event.clone()));
        }
        let ref mut e = *events;
        e.truncate();//added
        *commands = context.commands;
    }
}
use futures::future::ready;
fn send_commands(mut client:  ResMut<Option<BoxClient>>, mut commands: ResMut<protocol::Commands>,mut events: ResMut<protocol::Events>) {
    if let Some(ref mut client) = *client {
        for command in commands.iter() {
            let command = command.clone();
            let len = client.clients.len();
            let rand_int = get_random_int(0,len as i32);
            let mut sender = client.clients.get_mut(rand_int).unwrap().sender();
            if let Command::Nats(_,b) = command{
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
            }
        }
        commands.clear();
    }
}
fn receive_events(mut client: ResMut<Option<BoxClient>>, mut events: ResMut<protocol::Events>,mut query: Query<(&mut Position)> ) {
    if let Some(ref mut client) = *client {
        let len = client.clients.len();
        let rand_int = get_random_int(0,len as i32);
        if let Some(vec) = client.clients.get_mut(0).unwrap().poll_once() {
            for event in vec {
                if let Event::Nats(client_name,s_op)=event.clone(){
                  match s_op{
                    nats::proto::ServerOp::Msg{subject,sid,reply_to,payload}=>{
                      if subject == String::from("game_logic"){
                        info!("recv msg!! game_logic {} payload:{}",subject,std::str::from_utf8(&payload).unwrap());
                        for (mut pos) in query.iter_mut() {
                          pos.0 += 0.2 * 2.0 * 15.0;
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
