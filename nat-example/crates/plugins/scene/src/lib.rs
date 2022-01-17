use core::DeskSystem;
use bevy::{prelude::*, reflect::TypeRegistry, utils::Duration};
use qq_party_shared::{Position,Velocity,update_position_system,TargetVelocity,BallId};
pub struct QQScenePlugin;
const LINEAR_DAMPING: f32 = 8.0;
use bevy::math::Vec3;
use std::f32::consts::PI;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use rand::Rng;
use bevy_obj::*;

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
impl Plugin for QQScenePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        info!("build ScenePlugin");
        app.register_type::<ComponentA>()
        .register_type::<ComponentB>()
        .add_startup_system(save_scene_system.exclusive_system())
        .add_startup_system(load_scene_system)
        .add_plugin(ObjPlugin)
        .add_startup_system(setup.system());
    }
}
#[derive(Component, Reflect, Default)]
#[reflect(Component)] // this tells the reflect derive to also reflect component behaviors
struct ComponentA {
    pub x: f32,
    pub y: f32,
}
#[derive(Component, Reflect)]
#[reflect(Component)]
struct ComponentB {
    pub value: String,
    #[reflect(ignore)]
    pub _time_since_startup: Duration,
}

impl FromWorld for ComponentB {
    fn from_world(world: &mut World) -> Self {
        let time = world.get_resource::<Time>().unwrap();
        ComponentB {
            _time_since_startup: time.time_since_startup(),
            value: "Default Value".to_string(),
        }
    }
}
fn load_scene_system(asset_server: Res<AssetServer>, mut scene_spawner: ResMut<SceneSpawner>) {
    // Scenes are loaded just like any other asset.
    let scene_handle: Handle<DynamicScene> = asset_server.load("scenes/load_scene_example.scn.ron");

    // SceneSpawner can "spawn" scenes. "Spawning" a scene creates a new instance of the scene in
    // the World with new entity ids. This guarantees that it will not overwrite existing
    // entities.
    scene_spawner.spawn_dynamic(scene_handle);

    // This tells the AssetServer to watch for changes to assets.
    // It enables our scenes to automatically reload in game when we modify their files
    asset_server.watch_for_changes().unwrap();
}
fn save_scene_system(world: &mut World) {
    // Scenes can be created from any ECS World. You can either create a new one for the scene or
    // use the current World.
    let mut scene_world = World::new();
    let mut component_b = ComponentB::from_world(world);
    component_b.value = "hello".to_string();
    scene_world.spawn().insert_bundle((
        component_b,
        ComponentA { x: 1.0, y: 2.0 },
        Transform::identity(),
    ));
    scene_world
        .spawn()
        .insert_bundle((ComponentA { x: 3.0, y: 4.0 },));

    // The TypeRegistry resource contains information about all registered types (including
    // components). This is used to construct scenes.
    let type_registry = world.get_resource::<TypeRegistry>().unwrap();
    let scene = DynamicScene::from_world(&scene_world, type_registry);

    // Scenes can be serialized like this:
    info!("{}", scene.serialize_ron(type_registry).unwrap());

    // TODO: save scene
}
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(PbrBundle {
        mesh: asset_server.load(format!("{}/assets/model/Walk1.obj", env!("CARGO_MANIFEST_DIR")).as_str()),
        material: materials.add(StandardMaterial {
            base_color_texture: Some(asset_server.load(format!("{}/assets/textures/DrumstickTexture.png", env!("CARGO_MANIFEST_DIR")).as_str())),
            ..Default::default()
        }),
        ..Default::default()
    });
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(3.0, 4.0, 3.0)),
        ..Default::default()
    });
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_translation(Vec3::new(1.5, 2.7, 4.0))
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        ..Default::default()
    });
}