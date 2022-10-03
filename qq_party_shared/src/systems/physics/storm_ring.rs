use bevy_rapier2d::prelude::*;
use bevy_ecs::prelude::*;
use crate::*;

pub fn spawn_storm_ring(
  mut cmd: Commands,
  pos_x: f32,
  pos_y: f32,
  radius:i16
) {
  cmd.spawn().insert(StormRingId(Vec2::new(pos_x,pos_y),radius));
}