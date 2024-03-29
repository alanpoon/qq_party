// #[cfg(feature = "non_actor")]
// use bevy_ecs::prelude::{Query, Res,ResMut,Component,Entity};
// #[cfg(feature = "actor")]
// use bevy_ecs_wasm::prelude::{Query, Component,Res,ResMut,Entity};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy::math::{Vec2};
use bevy::utils::Duration;
use serde::{Deserialize, Serialize};
mod bundle;
mod timer_duration;
pub use timer_duration::*;
pub mod systems;
pub use systems::*;
pub use bundle::*;
pub mod time_interface;
pub mod scoreboard;
pub mod sub_map;
pub mod plugin;
pub mod to_despawn;
pub mod update;
pub use scoreboard::*;
pub use time_interface::DamageCountdown;
pub use plugin::QQSharedPlugin;
pub use bevy_rapier2d;
#[derive(Component,Serialize, Deserialize, Default, Clone, Copy,Debug)]
pub struct Position(pub Vec2);
#[derive(Component,Serialize, Deserialize, Default, Clone, Copy,Debug)]
pub struct QQVelocity(pub Vec2);
#[derive(Component,Serialize, Deserialize, Default, Clone, Copy, Debug)]
pub struct TargetVelocity(pub Vec2);
#[derive(Component,Serialize, Deserialize, Default, Clone, Copy, Debug)]
pub struct TargetDestination(pub Vec2,pub f32);
#[derive(Component,Serialize, Deserialize, Default, Clone, Copy, Debug, PartialEq, Hash, Eq)]
pub struct BallId(pub u32,pub i16); //ball_id, sprite_enum,0:chicken,1:bear
#[derive(Component,Serialize, Deserialize, Default, Clone,Debug, PartialEq, Hash, Eq)]
pub struct BallLabel(pub String,pub String); //Label, Flag
#[derive(Component, Serialize, Deserialize,Default, Clone, Copy, Debug, PartialEq, Hash, Eq)]
pub struct ChaseTargetId(pub u32, pub u8);//ball, npc, speed
#[derive(Component,Serialize, Deserialize, Default, Clone, Copy, Debug, PartialEq, Hash, Eq)]
pub struct ChaseTargetId2(pub u32, pub Option<Entity>,pub u8);//ball, npc, speed
#[derive(Component,Serialize, Deserialize, Default, Clone, Debug)]
pub struct FireId(pub u32,pub i16,pub Option<Vec2>); //owner, sprite_enum, StartPosition
#[derive(Component,Serialize, Deserialize, Default, Clone, Debug, PartialEq, Hash, Eq)]
pub struct Hit;
#[derive(Component,Serialize, Deserialize, Default, Clone, Debug)]
pub struct Dash(pub bool,pub Vec2,pub Vec2); //on/off, new_speed, old speed
#[derive(Component, Serialize, Deserialize,Default, Clone, Copy, Debug, PartialEq, Hash, Eq)]
pub struct NPCId{
  pub id:u32,
  pub sprite_enum:u32
}
#[derive(Component,Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Hash, Eq)]
pub struct LastNPC(pub u32,pub Option<Entity>,pub bool); //bool: is new_crate
#[derive(Component, Clone, Debug)]
pub struct SpecialEffectId(pub String);// special effect texture


#[derive(Component,Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Hash, Eq)]
pub struct QQParent(Entity);
#[derive(Serialize, Deserialize, Default,Clone,Debug)]
pub struct StormTiming(pub u64,pub u64); //next_timing, duration
#[derive(Serialize, Deserialize, Default,Clone)]
pub struct StartGameTiming(pub u64,pub u64); //next_timing, duration
#[derive(Debug,Clone,Serialize,Deserialize)]
pub enum ServerMessage {
    Chat{msg:String,msg_ago:String,user:String,user_id:u32},
    Dash{ball_id:BallId},
    Disconnect{ball_id:u32},
    Fire{ball_id:BallId,velocity:QQVelocity,sprite_enum:u32},
    GameState{ball_bundles:Vec<BallBundle>,npc_bundles:Vec<NPCBundle>,storm_timing:StormTiming,timestamp:u64},
    StormRings{storm_rings:Vec<StormRingId>,next_storm_timing:Option<StormTiming>},
    StateNotification{countdown:u64,text:String},
    StateChange{state:QQState,scoreboard:Vec<(u32,i16,BallLabel)>},
    Scores{scoreboard:Vec<(u32,i16,BallLabel)>},
    TargetVelocity{ball_id:BallId,target_velocity:TargetVelocity},
    TargetDestinations{npc:Vec<(NPCId,TargetDestination)>},
    Welcome{ball_bundle:BallBundle,sub_map:String,qq_state:QQState},
}
#[derive(Serialize, Deserialize, Clone)]
pub enum ClientMessage {
    ChangeSubMap{game_id:String,ball_id:BallId,position:Position},
    Dash{ball_id:BallId},
    Disconnect{ball_id_secret:String},
    Fire{ball_id:BallId,velocity:QQVelocity,sprite_enum:u32},
    Ping{ball_id_secret:String,timestamp:u32},
    TargetVelocity{game_id:String,ball_id:BallId,target_velocity:TargetVelocity},
    Welcome{game_id:String,ball_id:BallId,ball_label:BallLabel},
    
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone,Default)]
pub struct UserInfo{
  pub ball_id:BallId,
  pub sub_map:String,
}
#[derive(Component,Default,Debug)]
pub struct LocalUserInfo(pub UserInfo);
#[derive(Component,Default,Debug,Serialize, Deserialize, Clone)]
pub struct StormRingId(pub Vec2,pub i16); //pos,radius
#[derive(Component,Clone,Debug)]
pub struct StormRingText(pub Vec2);
#[derive(Component,Clone,Debug)]
pub struct StormRingTextNode();
#[derive(Component,Serialize, Deserialize, Default, Clone, Copy, Debug)]
pub struct DebugText();
#[derive(Component,Clone,Debug)]
pub struct TimerV2{
  
}
#[derive(Component,Clone,Debug)]
pub struct AnimationTimerV2(TimerV2);
#[derive(Component,Clone,Debug)]
pub struct DamageTimer(pub Timer);
#[derive(Component,Clone,Debug)]
pub struct DashTimer(pub Timer);
#[derive(Component,Clone,Debug,Default)]
pub struct AudioAble(pub bool,pub bool); //0:set in protocol, 1:set in audioplugin

#[derive(Serialize, Deserialize,Clone,Debug)]
pub enum QQState{
    Running,
    StopNotification,
    Stop,
    RunNotification
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(SystemLabel)]
pub enum MyLabel {
    /// everything that handles input
    Despawn,
    Scoring,
    Hierachy,
}
#[derive(Component,Clone,Debug)]
pub struct StateTransformer(pub Timer,pub QQState);
impl Default for StateTransformer{
    fn default()->Self{
        StateTransformer(Timer::new(Duration::from_secs(10),false),QQState::Running)
    }
}