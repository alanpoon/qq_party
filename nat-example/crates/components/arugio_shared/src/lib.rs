use bevy_ecs::prelude::*;
use bevy_math::{Vec2};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ClientMessage {
    Hello,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ServerMessage {
    Welcome(BallId),
}

#[derive(Component,Serialize, Deserialize, Default, Clone, Copy, Debug, PartialEq, Hash, Eq)]
pub struct BallId(pub u32);
#[derive(Component,Serialize, Deserialize, Default, Clone, Copy, Debug)]
pub struct Position(pub Vec2);
#[derive(Component,Serialize, Deserialize, Default, Clone, Debug)]
pub struct Velocity(pub Vec2);
#[derive(Component,Serialize, Deserialize, Default, Clone, Copy, Debug)]
pub struct TargetVelocity(pub Vec2);
#[derive(Component)]
struct SelectTimer;
#[derive(Component,Debug, PartialEq, Default)]
pub struct Time{ pub elapsed:f32}
impl Time{
  pub fn update(&mut self,t:f32){
    self.elapsed = t;
  }
  pub fn delta_seconds(&self)->f32{
    self.elapsed
  }
}

//#[bevycheck::system]
pub fn update_velocity_system(mut query: Query<(&mut Velocity, &TargetVelocity)>, time: Res<Time>) {
    let delta = time.delta_seconds();
    let speed = 2.0;

    for (mut velocity, target_velocity) in query.iter_mut() {
        velocity.0 = velocity.0 * (1.0 - delta * speed) + target_velocity.0 * (delta * speed);
    }
}

//#[bevycheck::system]
pub fn update_position_system(mut query: Query<(&mut Position, &Velocity)>, time: Res<Time>) {
    for (mut pos, vel) in query.iter_mut() {
        pos.0 += vel.0 * time.delta_seconds() * 15.0;
    }
}

#[derive(Bundle)]
pub struct BallBundle {
    ball_id: BallId,
    position: Position,
    velocity: Velocity,
    target_velocity: TargetVelocity,
}

impl BallBundle {
    pub fn new(ball_id: BallId) -> BallBundle {
        BallBundle {
            ball_id,
            position: Default::default(),
            velocity: Default::default(),
            target_velocity: Default::default(),
        }
    }
}
