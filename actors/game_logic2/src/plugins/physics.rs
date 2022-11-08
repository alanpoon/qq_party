use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::math::Vect;
use bevy_rapier2d::plugin::TimestepMode;
pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
  fn build(&self, app: &mut App) {
      app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
          .insert_resource(RapierConfiguration {
            gravity: Vect::ZERO,
            //timestep_mode: TimestepMode::Interpolated{
            timestep_mode:TimestepMode::Variable{
              //dt:1.0/60.0,
              max_dt:1.0/60.0,
              time_scale:1.0,
              substeps:1,
            },
            ..Default::default()
          })
          ;
  }
}
use qq_party_shared::*;