use qq_party_shared::*;
use bevy_math::Vec2;
pub fn spawn_storm_ring(
  pos_x: f32,
  pos_y: f32,
  radius:i16
) -> StormRingId{
  StormRingId(Vec2::new(pos_x,pos_y),radius)
}