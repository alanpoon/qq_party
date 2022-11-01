use core::DeskSystem;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
//use physics::{shape::Shape, widget::WidgetId, DragState, QQVelocity};
use qq_party_shared::{Position,QQVelocity,TargetVelocity,BallId};
pub struct PhysicsPlugin;
const LINEAR_DAMPING: f32 = 8.0;
use bevy::math::Vec3;
#[path = "../src_debug_ui/mod.rs"]
mod ui;
use ui::DebugUiPlugin;
use crate::nalgebra::Vector2;
use std::f32::consts::PI;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use rand::Rng;

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
macro_rules! console_log {
  // Note that this is using the `log` function imported above during
  // `bare_bones`
  ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}
const RAPIER_SCALE: f32 = 20.0;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        info!("build PhysicsPlugin");
        app.add_plugin(RapierPhysicsPlugin::<NoUserData,>::default())    
            .insert_resource(RapierConfiguration {
                scale: 1.0,
                gravity: Vector2::zeros(),
                ..Default::default()
            });
           
    }
}
fn enable_physics_profiling(mut pipeline: ResMut<PhysicsPipeline>) {
  pipeline.counters.enable()
}
