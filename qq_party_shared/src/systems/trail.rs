use bevy_ecs::prelude::*;
use bevy_math::{Vec2};
use bevy_log::info;
use crate::time_interface;
use crate::{TargetVelocity,Velocity,Time,BallId,Position,ChaseTargetId,NPCId};

pub fn update_state_trail(mut query: Query<(&mut Velocity,&mut TargetVelocity)>){


}
