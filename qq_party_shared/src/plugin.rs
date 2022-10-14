use bevy::prelude::*;
use crate::*;
use crate::systems::*;
pub struct QQSharedPlugin;
impl Plugin for QQSharedPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        info!("build QQSharedPlugin");
        app
          .init_resource::<ScoreBoard>()
          //fire
          .add_system(physics::spawn_fire_collider)
          .add_system(physics::fire_collision)
          .add_system(physics::despawn_fire)
          //joint
          .add_system(physics::set_state_chasetarget_npc2)
          .add_system(physics::spawn_hierachy)
          .add_system(physics::spawn_joint)
          //npc
          .add_system(physics::spawn_npc_collider)
          //player
          .add_system(physics::spawn_player_collider)
          .add_system(physics::add_ball_dash_physics)
          .add_system(physics::remove_ball_dash_physics)
          .add_system(physics::update_state_position_physics)
          .add_system(physics::update_state_velocity_physics)
          //storm_ring
          .add_system(physics::outside_storm_ring_damage)
          .add_system(update_state_velocity)
          .add_system(add_physics_velocity)
          .add_system(update_physics_velocity)
          
          ;
           
    }
  }