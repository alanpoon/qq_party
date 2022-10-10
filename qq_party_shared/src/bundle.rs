// #[cfg(feature = "non_actor")]
// use bevy_ecs::prelude::{Query, Res,ResMut,Component,Entity,Bundle};
// #[cfg(feature = "actor")]
// use bevy_ecs_wasm::prelude::{Query, Res,ResMut,Entity,Bundle};
use bevy::prelude::*;
use crate::*;
#[derive(Bundle,Serialize, Deserialize,Clone,Debug)]
pub struct BallBundle {
    pub ball_id: BallId,
    pub ball_label: BallLabel,
    pub position: Position,
    pub velocity: QQVelocity,
    pub target_velocity: TargetVelocity,   
}
#[derive(Bundle,Serialize, Deserialize,Clone,Debug)]
pub struct NPCBundle {
    pub npc_id: NPCId,
    pub position: Position,
    pub velocity: QQVelocity,
    pub chase_target: ChaseTargetId,
}
#[derive(Bundle,Clone,Debug)]
pub struct FireBundle {
    pub fire_id:FireId,
    pub position: Position,
    pub velocity: QQVelocity,
    //pub start: Time,
}