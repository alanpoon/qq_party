use bevy::prelude::*;
use bevy::utils::Duration;
use bevy::app::ScheduleRunnerSettings;
use plugin_protocol::ProtocolPlugin;
use plugin_physics_rapier::PhysicsPlugin;
use plugin_scene::QQScenePlugin;
use log::Level;
#[bevy_main]
pub fn main() {
    
    let mut app = App::new();

    app.insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f64(
            1.0 /600.0,
        )))
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin)
        .add_plugin(ProtocolPlugin)
        .add_plugin(QQScenePlugin);

    // #[cfg(target_arch = "wasm32")]
    // app.add_plugin(plugin_wasm_target::WasmTargetPlugin);
    #[cfg(target_arch = "wasm32")]
    console_log::init_with_level(Level::Debug);

    app.run();
}
