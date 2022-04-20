use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::bevy_wasmcloud_time;
use crate::nalgebra::Vector2;
pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
  fn build(&self, app: &mut bevy_app::App) {
      app.add_plugin(RapierPhysicsPlugin::<NoUserData,bevy_wasmcloud_time::Time>::default())
          .insert_resource(RapierConfiguration {
            scale: 1.0,
            gravity: Vector2::zeros(),
            ..Default::default()
          })
          .add_system(qq_party_shared::systems::update_state_position_physics::<bevy_wasmcloud_time::Time>.system())
          .add_system(qq_party_shared::systems::update_state_velocity_physics.system())
          .add_system(qq_party_shared::systems::physics::spawn_player_collider.system());
  }
}