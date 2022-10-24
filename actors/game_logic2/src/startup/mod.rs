pub mod npc;
//pub mod npc_debug;
pub mod storm_ring;
use crate::messaging_::publish_;
use wasmcloud_interface_messaging::PubMessage;
use qq_party_shared::*;
use crate::info_::info_;
use bevy::prelude::*;
use bevy::utils::Duration;

#[derive(Component,Clone,Debug)]
pub struct StateTransformer(pub Timer,pub QQState);
impl Default for StateTransformer{
    fn default()->Self{
        StateTransformer(Timer::new(Duration::from_secs(10),false),QQState::Running)
    }
}
pub fn state_update(app:&mut App){
    let time = app.world.get_resource::<Time>().unwrap();
    let time_c= time.clone();
    let mut new_state = None;
    match app.world.get_resource_mut::<StateTransformer>(){
        Some(mut st)=>{
        if st.0.tick(Duration::from_millis((time_c.delta_seconds() as f32 * 1000.0) as u64)).just_finished() {
            match st.1{
                QQState::Running=>{
                    st.0 = Timer::new(Duration::from_secs(1),false);
                    st.1 = QQState::StopNotification;  
                }
                QQState::StopNotification=>{
                    st.0 = Timer::new(Duration::from_secs(10),false);
                    st.1 = QQState::Stop;
                }
                QQState::Stop=>{
                    st.0 = Timer::new(Duration::from_secs(20),false);
                    st.1 = QQState::RunNotification;
                }
                QQState::RunNotification=>{
                    st.0 = Timer::new(Duration::from_secs(10),false);
                    st.1 = QQState::Running;
                }
            }
            new_state = Some(st.1.clone());
        }
        }
        _=>{}
    }
    if let Some(new_state)= new_state{
        info_(format!("new_state {:?}",new_state));
        match new_state{
            QQState::Running=>{
                app.world.spawn()
                .insert(storm_ring::spawn_storm_ring(3400.0,3400.0,80));
                let server_message = ServerMessage::StateChange{state:QQState::Running,scoreboard:vec![]};
                match rmp_serde::to_vec(&server_message){
                    Ok(b)=>{
                        let p_msg = PubMessage{
                        body:b,
                        reply_to: None,
                        subject: String::from("game_logic.state_change")
                        };
                        publish_(p_msg);
                    }
                    _=>{}
                }
            }
            QQState::StopNotification=>{
                let server_message = ServerMessage::StateNotification{countdown:10000,text:String::from("Game ending in ")};
                match rmp_serde::to_vec(&server_message){
                    Ok(b)=>{
                        let p_msg = PubMessage{
                        body:b,
                        reply_to: None,
                        subject: String::from("game_logic.state_notification")
                        };
                        publish_(p_msg);
                    }
                    _=>{}
                }
            }
            QQState::Stop=>{
                let mut to_despawn = vec![];
                let mut query = app.world.query::<(Entity, &BallId)>();
                for (e,_) in query.iter(&app.world){
                    //app.world.despawn(e);
                    to_despawn.push(e);
                }
                let mut query = app.world.query::<(Entity, &NPCId)>();
                for (e,_) in query.iter(&app.world){
                    //app.world.despawn(e);
                    to_despawn.push(e);
                }
                let mut query = app.world.query::<(Entity, &StormRingId)>();
                for (e,_) in query.iter(&app.world){
                    //app.world.despawn(e);
                    to_despawn.push(e);
                }
                for d in to_despawn{
                    app.world.despawn(d);
                }
                let mut scoreboard = app.world.get_resource_mut::<ScoreBoard>().unwrap();
                let mut score_vec:Vec<(i16,BallLabel)> = vec![];
                for (k,v) in (*scoreboard).scores.iter(){
                    score_vec.push(v.clone());
                }
                score_vec.sort_by(|a,b|{
                    b.0.cmp(&a.0)
                });
                if score_vec.len() >3{
                    score_vec.clone().split_off(3);
                }
                *scoreboard = ScoreBoard::default();
                score_vec = vec![(2222,BallLabel(String::from("hello"),String::from(".cn")))];
                let server_message = ServerMessage::StateChange{state:QQState::Stop,scoreboard:score_vec};
                match rmp_serde::to_vec(&server_message){
                    Ok(b)=>{
                    let p_msg = PubMessage{
                        body:b,
                        reply_to: None,
                        subject: String::from("game_logic.state_change")
                    };
                    publish_(p_msg);
                    }
                    _=>{}
                }
            }
            QQState::RunNotification=>{
                let server_message = ServerMessage::StateNotification{countdown:10000,text:String::from("New game starting in ")};
                match rmp_serde::to_vec(&server_message){
                    Ok(b)=>{
                        let p_msg = PubMessage{
                        body:b,
                        reply_to: None,
                        subject: String::from("game_logic.start_notification")
                        };
                        publish_(p_msg);
                    }
                    _=>{}
                }
                if let Ok(npc_bundles)=crate::startup::npc::spawn_npc_bundles_sync(){
                    app.world.spawn_batch(npc_bundles);
                }
            }
        }
    }
}
