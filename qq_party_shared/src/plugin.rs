use bevy::prelude::*;
use crate::*;

pub struct QQSharedPlugin;
impl Plugin for QQSharedPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        info!("build QQSharedPlugin");
        app
          .init_resource::<ScoreBoard>()
          .init_resource::<entity_to_remove::EntityToRemove>()
          .init_resource::<StateTransformer>()
          .add_system_to_stage(CoreStage::Last,entity_to_remove::remove_entity_system.label(MyLabel::Despawn))
          //fire
          .add_system(physics::fire_collision)
          .add_system(physics::despawn_fire)
          //joint
          .add_system(physics::set_state_chasetarget_npc2.label(MyLabel::Scoring))
          .add_system(physics::spawn_hierachy.label(MyLabel::Hierachy).after(MyLabel::Scoring))
          .add_system(physics::spawn_joint.after(MyLabel::Hierachy))
          //npc
          //player
          .add_system(physics::add_ball_dash_physics)
          .add_system(physics::remove_ball_dash_physics)
          .add_system(physics::update_state_velocity_physics)
          //storm_ring
          .add_system(physics::outside_storm_ring_damage)
          .add_system(update_state_velocity)
          .add_system(add_physics_velocity)
          .add_system(update_physics_velocity)
          
          ;
           
    }
  }