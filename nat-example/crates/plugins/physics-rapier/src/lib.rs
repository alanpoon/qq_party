use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
//use physics::{shape::Shape, widget::WidgetId, DragState, Velocity};
use qq_party_shared::*;
pub struct PhysicsPlugin;
#[path = "../src_debug_ui/mod.rs"]
mod ui;
// mod timewrapper;
// mod timewrapper_qq;
mod special_effects;
mod storm_ring;
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
        //app.add_plugin(RapierPhysicsPlugin::<NoUserData,timewrapper::TimeWrapper>::default())
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            //.add_startup_system(enable_physics_profiling)
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
            // .init_resource::<timewrapper::TimeWrapper>()
            // .init_resource::<timewrapper_qq::TimeWrapper>()
            .init_resource::<DamageCountdown>()
            //.add_system(timewrapper_qq::into_timewrapper)
            //.add_system(debug_rigid)
            //player
            // .add_system(qq_party_shared::systems::physics::spawn_player_collider)
            // .add_system(qq_party_shared::systems::update_state_position_physics)
            // .add_system(qq_party_shared::systems::update_state_velocity)
            // .add_system(qq_party_shared::systems::update_state_velocity_physics)
            // //npc
            // .add_system(qq_party_shared::systems::physics::spawn_npc_collider)
            // .add_system(qq_party_shared::systems::set_state_chasetarget_npc2)
            // //spawn_hierachy
            // .add_system(qq_party_shared::systems::physics::spawn_hierachy)
            // .add_system(qq_party_shared::systems::physics::spawn_joint)
            // //.add_system(sys_time_debug)
            // //fire
            // .add_system(qq_party_shared::systems::physics::spawn_fire_collider)
            // .add_system(qq_party_shared::systems::physics::fire_collision)
            // .add_system(qq_party_shared::systems::physics::despawn_fire)
            //special_effects
            .add_system(special_effects::spawn_special_effect_collider)
            .add_system(special_effects::move_special_effect_closer_to_user_system)
            //storm_ring
            .add_system(storm_ring::spawn_storm_ring_collider)
            //.add_system(qq_party_shared::systems::physics::outside_storm_ring_damage::<timewrapper_qq::TimeWrapper>)
            //.add_system(timewrapper::into_timewrapper)
            ;
         
        }
    }
// fn enable_physics_profiling(mut pipeline: ResMut<PhysicsPipeline>) {
//   pipeline.counters.enable()
// }
//use bevy_rapier2d::physics::time::TimeInterface;
// pub fn sys_time_debug(balls_without_rigid:Query<(&BallId,&Position,&Velocity,&QQVelocity)>,time:Res<timewrapper::TimeWrapper>){
//   for (ball_id,pos,rv,vel) in balls_without_rigid.iter(){
//     let delta = time.delta_seconds();
//     info!("ball_id {:?} pos {:?} rv {:?} vel{:?} delta {:?}",ball_id,pos,rv.0.linvel,vel,delta);
//   }
// }