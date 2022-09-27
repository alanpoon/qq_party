// #[cfg(feature = "non_actor")]
// use bevy_ecs::prelude::{Query, Res,ResMut,Component,Entity};
// #[cfg(feature = "actor")]
// use bevy_ecs_wasm::prelude::{Query, Component,Res,ResMut,Entity};
use bevy_ecs::prelude::*;
use bevy_math::{Vec2};
use serde::{Deserialize, Serialize};
mod bundle;
pub mod systems;
pub use bundle::*;
pub mod time_interface;
pub mod scoreboard;
pub mod sub_map;
pub use scoreboard::*;


#[derive(Component,Serialize, Deserialize, Default, Clone, Copy,Debug)]
pub struct Position(pub Vec2);
#[derive(Component,Serialize, Deserialize, Default, Clone, Copy,Debug)]
pub struct Velocity(pub Vec2);
#[derive(Component,Serialize, Deserialize, Default, Clone, Copy, Debug)]
pub struct TargetVelocity(pub Vec2);
#[derive(Component,Serialize, Deserialize, Default, Clone, Copy, Debug)]
pub struct TargetDestination(pub Vec2,pub f32);

//x:1.0,y:1.0->move to its right,x:0.0,y:1.0->move forward
#[derive(Component,Debug, PartialEq, Default)]
pub struct Time{pub elapsed:f32}
#[derive(Component,Serialize, Deserialize, Default, Clone, Copy, Debug, PartialEq, Hash, Eq)]
pub struct BallId(pub u32,pub i16);
#[derive(Component,Serialize, Deserialize, Default, Clone, Debug, PartialEq, Hash, Eq)]
pub struct BallLabel(pub String,pub String); //Label, Flag
#[derive(Component,Serialize, Deserialize, Default, Clone, Copy, Debug, PartialEq, Hash, Eq)]
pub struct ChaseTargetId(pub u32, pub u8);//ball, npc, speed
#[derive(Component,Serialize, Deserialize, Default, Clone, Copy, Debug, PartialEq, Hash, Eq)]
pub struct ChaseTargetId2(pub u32, pub Option<Entity>,pub u8);//ball, npc, speed
#[derive(Component,Serialize, Deserialize, Default, Clone, Copy, Debug, PartialEq, Hash, Eq)]
pub struct NPCId{
  pub id:u32,
  pub sprite_enum:u32
}
#[derive(Component,Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Hash, Eq)]
pub struct LastNPC(pub u32,pub Option<Entity>);
#[derive(Component,Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Hash, Eq)]
pub struct Parent(Entity);
#[derive(Serialize, Deserialize, Clone)]
pub enum ServerMessage {
    Welcome{ball_bundle:BallBundle,sub_map:String},
    Chat{msg:String,msg_ago:String,user:String,user_id:u32},
    TargetVelocity{ball_id:BallId,target_velocity:TargetVelocity},
    TargetDestinations{npc:Vec<(NPCId,TargetDestination)>},
    GameState{ball_bundles:Vec<BallBundle>,npc_bundles:Vec<NPCBundle>,timestamp:u64},
    Scores{scoreboard:Vec<(i16,BallLabel)>}
}
#[derive(Serialize, Deserialize, Clone)]
pub enum ClientMessage {
    Welcome{game_id:String,ball_id:BallId,ball_label:BallLabel},
    TargetVelocity{game_id:String,ball_id:BallId,target_velocity:TargetVelocity},
    ChangeSubMap{game_id:String,ball_id:BallId,position:Position},
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone,Default)]
pub struct UserInfo{
  pub ball_id:BallId,
  pub sub_map:String,
}
#[derive(Component,Default,Debug)]
pub struct LocalUserInfo(pub UserInfo);
