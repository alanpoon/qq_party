#[cfg(feature = "non_actor")]
use bevy_ecs::prelude::{Query, Res,ResMut};
#[cfg(feature = "actor")]
use bevy_ecs_wasm::prelude::{Query, Res,ResMut};

use bevy_math::{Vec2};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Serialize, Deserialize, Default, Clone, Copy, Debug)]
pub struct Position(pub Vec2);
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Velocity(pub Vec2);
#[derive(Serialize, Deserialize, Default, Clone, Copy, Debug)]
pub struct TargetVelocity(pub Vec2);

#[derive(Debug, PartialEq, Default)]
pub struct Time{pub elapsed:f32}
pub fn update_velocity_system(mut query: Query<(&mut Velocity, &TargetVelocity)>, time: Res<Time>) {
    //let delta = time.delta_seconds();
    let delta = 2.0;
    let speed = 2.0;

    for (mut velocity, target_velocity) in query.iter_mut() {
        velocity.0 = velocity.0 * (1.0 - delta * speed) + target_velocity.0 * (delta * speed);
    }
}

pub fn update_position_system(mut query: Query<(&mut Position, &Velocity)>, time: Res<Time>) {
    for (mut pos, vel) in query.iter_mut() {
        //pos.0 += vel.0 * time.delta_seconds() * 15.0;
    }
}
