#[cfg(feature = "non_actor")]
use bevy_ecs::prelude::{Query, Res,ResMut,Component,Entity};
#[cfg(feature = "actor")]
use bevy_ecs_wasm::prelude::{Query, Res,ResMut,Entity};
#[cfg(feature = "actor")]
use bevy_ecs_wasm::component::Component;

use bevy_math::{Vec2};
use serde::{Deserialize, Serialize};
use std::time::Duration;
mod bundle;
pub use bundle::*;
pub mod time_interface;
use time_interface::TimeInterface;

#[cfg(feature = "non_actor")]
#[derive(Component,Serialize, Deserialize, Default, Clone, Copy,Debug)]
pub struct Position(pub Vec2);
#[cfg(feature = "actor")]
#[derive(Serialize, Deserialize, Default, Clone, Copy,Debug)]
pub struct Position(pub Vec2);
#[cfg(feature = "non_actor")]
#[derive(Component,Serialize, Deserialize, Default, Clone, Copy,Debug)]
pub struct Velocity(pub Vec2);
#[cfg(feature = "actor")]
#[derive(Serialize, Deserialize, Default, Clone, Copy,Debug)]
pub struct Velocity(pub Vec2);

#[cfg(feature = "non_actor")]
#[derive(Component,Serialize, Deserialize, Default, Clone, Copy, Debug)]
pub struct TargetVelocity(pub Vec2);
#[cfg(feature = "actor")]
#[derive(Serialize, Deserialize, Default, Clone, Copy, Debug)]
pub struct TargetVelocity(pub Vec2);
//x:1.0,y:1.0->move to its right,x:0.0,y:1.0->move forward
#[derive(Debug, PartialEq, Default)]
#[cfg(feature = "non_actor")]
#[derive(Component)]
pub struct Time{pub elapsed:f32}
#[cfg(feature = "actor")]
pub struct Time{pub elapsed:f32}
#[cfg(feature = "non_actor")]
#[derive(Component,Serialize, Deserialize, Default, Clone, Copy, Debug, PartialEq, Hash, Eq)]
pub struct BallId(pub u32);
#[cfg(feature = "actor")]
#[derive(Serialize, Deserialize, Default, Clone, Copy, Debug, PartialEq, Hash, Eq)]
pub struct BallId(pub u32);
#[derive(Serialize, Deserialize, Clone)]
pub enum ServerMessage {
    Welcome{ball_bundle:BallBundle},
    TargetVelocity{ball_id:BallId,target_velocity:TargetVelocity},
    GameState{ball_bundles:Vec<BallBundle>},
}
#[derive(Serialize, Deserialize, Clone)]
pub enum ClientMessage {
    Welcome{game_id:String,ball_id:BallId},
    TargetVelocity{game_id:String,ball_id:BallId,target_velocity:TargetVelocity},
}

pub fn update_velocity_system(mut query: Query<(&mut Velocity, &TargetVelocity)>, time: Res<Time>) {
    //let delta = time.delta_seconds();
    let delta = 2.0;
    let speed = 2.0;

    for (mut velocity, target_velocity) in query.iter_mut() {
        velocity.0 = velocity.0 * (1.0 - delta * speed) + target_velocity.0 * (delta * speed);
    }
}
pub fn update_position_system<X:time_interface::TimeInterface + Component>(mut query: Query<(&mut Position, &Velocity)>, time: Res<X>) {
    let delta = time.delta_seconds();
    for (mut pos, vel) in query.iter_mut() {
        pos.0 += vel.0 * time.delta_seconds() * 5.0;
    }
}
