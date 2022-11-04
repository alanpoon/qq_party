use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
//use physics::{shape::Shape, widget::WidgetId, DragState, Velocity};
use qq_party_shared::*;
pub struct PhysicsPlugin;
#[path = "../src_debug_ui/mod.rs"]
mod ui;
mod special_effects;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            .add_startup_system(qq_party_shared::systems::physics::add_damage_timer)
            .insert_resource(RapierConfiguration {
              //scale: 1.0,
              timestep_mode:TimestepMode::Variable{
                max_dt:1.0/60.0,
                time_scale:1.0,
                substeps:1,
              },
              gravity: Vect::ZERO,
              ..Default::default()
            })
            .init_resource::<DamageCountdown>()
         
            //special_effects
            .add_system(special_effects::move_special_effect_closer_to_user_system)
            //storm_ring
            //.add_system(storm_ring::spawn_storm_ring_collider)
            ;
         
        }
    }