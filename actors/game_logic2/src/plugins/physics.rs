use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::math::Vect;
use bevy_rapier2d::plugin::TimestepMode;
use crate::bevy_wasmcloud_time;
pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
  fn build(&self, app: &mut App) {
      //app.add_plugin(RapierPhysicsPlugin::<NoUserData,bevy_wasmcloud_time::Time>::default())
      app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
          .insert_resource(RapierConfiguration {
//            gravity: Vect::zeros(),
            //timestep_mode: TimestepMode{},
            ..Default::default()
          })
          .add_startup_system(qq_party_shared::systems::physics::add_damage_timer)
          //player
          .add_system(qq_party_shared::systems::physics::spawn_player_collider)
          .add_system(qq_party_shared::systems::update_state_position_physics)
          .add_system(qq_party_shared::systems::update_state_velocity)
          .add_system(qq_party_shared::systems::update_state_velocity_physics)
          //npc
          .add_system(qq_party_shared::systems::physics::spawn_npc_collider)
          .add_system(qq_party_shared::systems::set_state_chasetarget_npc2)
          //.add_system(qq_party_shared::systems::set_state_chasetarget_npc)
          //.add_system(qq_party_shared::systems::update_state_velocity_npc)
          .add_system(qq_party_shared::systems::physics::spawn_hierachy)
          .add_system(qq_party_shared::systems::physics::spawn_joint)
          //fire
          .add_system(qq_party_shared::systems::physics::spawn_fire_collider)
          .add_system(qq_party_shared::systems::physics::fire_collision)
          .add_system(qq_party_shared::systems::physics::despawn_fire)
          //storm_ring
          .add_system(qq_party_shared::systems::physics::despawn_fire)
          .add_system(qq_party_shared::systems::physics::outside_storm_ring_damage::<bevy_wasmcloud_time::Time>)
          ;
  }
}