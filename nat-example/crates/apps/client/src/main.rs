use bevy::prelude::*;
use bevy::utils::Duration;
use bevy::app::ScheduleRunnerSettings;
use plugin_audio::AudioPlugin;
use plugin_lyon::LyonPlugin;
use plugin_protocol::ProtocolPlugin;
use plugin_physics_rapier::PhysicsPlugin;
use plugin_map::MapPlugin;
use plugin_sprite_character::SpriteCharacterPlugin;
use log::Level;
use qq_party_shared::QQSharedPlugin;
#[bevy_main]
pub fn main() {
    
    let mut app = App::new();

        app
        .insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f64(
               1.0 /60.0
               //0.03
              //0.1
        )))
        .insert_resource(WindowDescriptor {
          width: 1280.0,
          height: 720.0,
          title: String::from("qq_party"),
          canvas:Some(String::from("#qq_party_canvas")),
          ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(AudioPlugin)
        .add_plugin(LyonPlugin)
        .add_plugin(PhysicsPlugin)
        .add_plugin(QQSharedPlugin)
        .add_plugin(ProtocolPlugin)
        .add_plugin(MapPlugin)
        .add_plugin(SpriteCharacterPlugin);

    // #[cfg(target_arch = "wasm32")]
    // app.add_plugin(plugin_wasm_target::WasmTargetPlugin);
    #[cfg(target_arch = "wasm32")]
    console_log::init_with_level(Level::Debug);

    app.run();
}
