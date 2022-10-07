use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
//use physics::{shape::Shape, widget::WidgetId, DragState, Velocity};
use qq_party_shared::*;
pub struct PhysicsPlugin;
#[path = "../src_debug_ui/mod.rs"]
mod ui;
mod timewrapper;
mod timewrapper_qq;
mod special_effects;
mod storm_ring;
use crate::nalgebra::Vector2;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        info!("build PhysicsPlugin");
        app.add_plugin(RapierPhysicsPlugin::<NoUserData,timewrapper::TimeWrapper>::default())
            .add_startup_system(enable_physics_profiling.system())
            .add_startup_system(qq_party_shared::systems::physics::add_damage_timer.system())
            .insert_resource(RapierConfiguration {
              scale: 1.0,
              gravity: Vector2::zeros(),
              ..Default::default()
            })
            .init_resource::<timewrapper::TimeWrapper>()
            .init_resource::<timewrapper_qq::TimeWrapper>()
            .init_resource::<DamageCountdown>()
            .init_resource::<qq_party_shared::scoreboard::ScoreBoard>()
            .add_system(timewrapper_qq::into_timewrapper.system())
            //.add_system(debug_rigid.system())
            //player
            .add_system(qq_party_shared::systems::physics::spawn_player_collider.system())
            .add_system(qq_party_shared::systems::update_state_position_physics.system())
            .add_system(qq_party_shared::systems::update_state_velocity.system())
            .add_system(qq_party_shared::systems::update_state_velocity_physics.system())
            //npc
            .add_system(qq_party_shared::systems::physics::spawn_npc_collider.system())
            .add_system(qq_party_shared::systems::set_state_chasetarget_npc2.system())
            //spawn_hierachy
            .add_system(qq_party_shared::systems::physics::spawn_hierachy.system())
            .add_system(qq_party_shared::systems::physics::spawn_joint.system())
            //.add_system(sys_time_debug.system())
            //fire
            .add_system(qq_party_shared::systems::physics::spawn_fire_collider.system())
            .add_system(qq_party_shared::systems::physics::fire_collision.system())
            .add_system(qq_party_shared::systems::physics::despawn_fire.system())
            //special_effects
            .add_system(special_effects::spawn_special_effect_collider.system())
            .add_system(special_effects::move_special_effect_closer_to_user_system.system())
            //storm_ring
            .add_system(storm_ring::spawn_storm_ring_collider.system())
            .add_system(qq_party_shared::systems::physics::outside_storm_ring_damage::<timewrapper_qq::TimeWrapper>.system())
            //.add_system(outside_storm_ring_damage::<timewrapper_qq::TimeWrapper>.system())
            .add_system(timewrapper::into_timewrapper.system());
         
        }
    }
fn enable_physics_profiling(mut pipeline: ResMut<PhysicsPipeline>) {
  pipeline.counters.enable()
}
use bevy_rapier2d::physics::time::TimeInterface;
pub fn sys_time_debug(balls_without_rigid:Query<(&BallId,&Position,&RigidBodyVelocityComponent,&Velocity)>,time:Res<timewrapper::TimeWrapper>){
  for (ball_id,pos,rv,vel) in balls_without_rigid.iter(){
    let delta = time.delta_seconds();
    info!("ball_id {:?} pos {:?} rv {:?} vel{:?} delta {:?}",ball_id,pos,rv.0.linvel,vel,delta);
  }
}