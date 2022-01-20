use core::DeskSystem;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::physics::wrapper;
//use physics::{shape::Shape, widget::WidgetId, DragState, Velocity};
use qq_party_shared::{Position,Velocity,update_position_system,TargetVelocity,BallId};
pub struct PhysicsPlugin;
const LINEAR_DAMPING: f32 = 8.0;
use bevy::math::Vec3;
#[path = "../src_debug_ui/mod.rs"]
mod ui;
use ui::DebugUiPlugin;
mod plane;
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
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugin(RapierRenderPlugin)
            .add_startup_system(walls.system())
            .add_startup_system(plane::plane.system())
            .add_startup_system(cube.system())
            .add_startup_system(enable_physics_profiling.system())
            //.add_plugin(DebugUiPlugin)
            .insert_resource(RapierConfiguration {
                scale: 100.0,
                gravity: Vector2::zeros(),
                ..Default::default()
            })
            .insert_resource(Msaa::default())
            // .add_system(
            //     add_physics_components
            //         .system()
            //         .after(DeskSystem::Shell)
            //         .before(DeskSystem::PrePhysics),
            // )
            //.add_system(update_ball_translation_system.system());
            .add_system(add_ball_mesh_system.system())
            .add_system(update_position_system1.system());
            // .add_system_set(
            //     SystemSet::new()
            //         .label(DeskSystem::PrePhysics)
            //         .with_system(update_velocity.system()),
            // );
    }
}
fn enable_physics_profiling(mut pipeline: ResMut<PhysicsPipeline>) {
  pipeline.counters.enable()
}
fn walls(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.transform.translation.x = 630.0;
    camera.transform.translation.y = 350.0;
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 100_000.0,
            range: 6000.0,
            ..Default::default()
        },
        ..Default::default()
    });
    commands.spawn_bundle(camera);
    commands
        .spawn_bundle(ColliderBundle {
            position: wrapper::ColliderPositionComponent(Vector::new(0.0, 0.0).into()),
            shape: wrapper::ColliderShapeComponent(ColliderShape::cuboid(0.1, 9.0)),
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete)
        .insert(ColliderDebugRender::default());
    commands
        .spawn_bundle(ColliderBundle {
            position: wrapper::ColliderPositionComponent(Vector::new(10.0, 0.0).into()),
            shape: wrapper::ColliderShapeComponent(ColliderShape::cuboid(0.1, 9.0)),
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete)
        .insert(ColliderDebugRender::default());
    commands
        .spawn_bundle(ColliderBundle {
            position: wrapper::ColliderPositionComponent(Vector::new(0.0, 0.0).into()),
            shape: wrapper::ColliderShapeComponent(ColliderShape::cuboid(12.0, 0.1)),
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete)
        .insert(ColliderDebugRender::default());
    commands
        .spawn_bundle(ColliderBundle {
            position: wrapper::ColliderPositionComponent(Vector::new(0.0, 7.0).into()),
            shape: wrapper::ColliderShapeComponent(ColliderShape::cuboid(12.0, 0.1)),
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete)
        .insert(ColliderDebugRender::default());
}

fn add_physics_components(
    rapier: Res<RapierConfiguration>,
    mut commands: Commands,
    query: Query<(Entity, &GlobalTransform), Added<Position>>,
) {
    for (card, transform) in query.iter() {
        commands
            .entity(card)
            .insert_bundle(RigidBodyBundle {
                position: wrapper::RigidBodyPositionComponent(Vector2::new(transform.translation[0] / rapier.scale,transform.translation[1]/rapier.scale).into()),
                mass_properties: wrapper::RigidBodyMassPropsComponent(RigidBodyMassPropsFlags::ROTATION_LOCKED.into()),
                damping: wrapper::RigidBodyDampingComponent(RigidBodyDamping {
                    linear_damping: LINEAR_DAMPING,
                    ..Default::default()
                }),
                ..Default::default()
            })
            .insert(RigidBodyPositionSync::Discrete)
            .with_children(|build| {
                build.spawn_bundle(ColliderBundle {
                    shape: wrapper::ColliderShapeComponent(ColliderShape::cuboid(0.1, 0.1)),
                    ..Default::default()
                });
            });
    }
}
pub fn cube(
  mut commands: Commands,
  mut materials: ResMut<Assets<ColorMaterial>>
) {
  let mut rand_rng = rand::thread_rng();
  let x:i32 = rand_rng.gen_range(35..70);
  let y:i32 = rand_rng.gen_range(0..200);
  let r:i32 = rand_rng.gen_range(0..255);
  let g:i32 = rand_rng.gen_range(0..255);
  let b:i32 = rand_rng.gen_range(0..255);
  let bevy_color = Color::rgb_u8(r as u8,g as u8,b as u8);
  commands
    .spawn()
    .insert_bundle(SpriteBundle {
        transform: Transform::from_translation(Vec3::new(x as f32, y as f32, 1.0)),
        sprite: Sprite{
          color: bevy_color,
          custom_size: Some(Vec2::new(30.0,30.0)),
          ..Default::default()
        },
        ..Default::default()
    }).insert(wrapper::RigidBodyPositionComponent([x as f32,y as f32].into()))
    .insert(Position(Vec2::new(x as f32, y as f32)))
    .insert(TargetVelocity(Vec2::ZERO));
    // .insert_bundle(RigidBodyBundle {
    //   body_type: wrapper::RigidBodyType(RigidBodyType::Static),
    //   position: wrapper::RigidBodyPositionComponent([40.0, 0.0].into()),
    //   ..RigidBodyBundle::default()
    // }).insert(RigidBodyPositionSync::Discrete);
  // let collider = ColliderBundle {
  //     shape: wrapper::ColliderShapeComponent(ColliderShape::cuboid(rad, rad)),
  //     ..Default::default()
  // };
  // commands
  //     .spawn_bundle(body)
  //     .insert_bundle(collider)
  //     .insert(ColliderDebugRender::with_id(color))
  //     .insert(ColliderPositionSync::Discrete);
}
// fn update_velocity(
//     rapier: Res<RapierConfiguration>,
//     mut query: Query<(&mut wrapper::RigidBodyVelocityComponent, &Velocity), Changed<Velocity>>,
// ) {
//     for (mut rapier_velocity, velocity) in query.iter_mut() {
//         rapier_velocity.linvel.x = velocity.0.x / rapier.scale;
//         rapier_velocity.linvel.y = velocity.0.y / rapier.scale;
//     }
// }
fn update_ball_translation_system(keyboard_input: Res<Input<KeyCode>>,mut balls: Query<(&Position, &mut Transform)>) {
  for (position, mut transform) in balls.iter_mut() {
      
      let mut direction = 0.0;
      if keyboard_input.pressed(KeyCode::Left) {
         direction -= 5.0;
      }
      if keyboard_input.pressed(KeyCode::Right) {
        direction += 5.0;
      }
      transform.translation.x = transform.translation.x+direction;
      //transform.translation.y = position.0.y;
      // transform.rotation =
      //     Quat::from_rotation_ypr(position.0.x * PI / 2.0, -position.0.y * PI / 2.0, 0.0);
  }
}
//pub fn update_position_system1(mut query: Query<(&mut Position, &Velocity)>, time: Res<Time>) {
pub fn update_position_system1(mut query: Query<(&mut TargetVelocity, &mut Transform)>, time: Res<Time>) {
   
  for (mut tv,mut transform) in query.iter_mut() {
    transform.translation.x += tv.0.x;
    transform.translation.y += tv.0.y;
    *tv = TargetVelocity(Vec2::ZERO);
  }
}
fn add_ball_mesh_system(
  mut cmd: Commands,
  balls_without_mesh: Query<(Entity, &BallId,&Position), Without<Transform>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
) {
  for (entity, _,position) in balls_without_mesh.iter() {
    let mut rand_rng = rand::thread_rng();
    let r = rand_rng.gen_range(0..255);
    let g = rand_rng.gen_range(0..255);
    let b = rand_rng.gen_range(0..255);
    let bevy_color = Color::rgb_u8(r as u8,g as u8,b as u8);
      cmd.entity(entity).insert_bundle(SpriteBundle {
        transform: Transform::from_translation(Vec3::new(position.0.x as f32, position.0.y as f32, 1.0)),
        sprite: Sprite{
          color: bevy_color,
          custom_size: Some(Vec2::new(30.0,30.0)),
          ..Default::default()
        },
        ..Default::default()
      }).insert(wrapper::RigidBodyPositionComponent([position.0.x as f32,position.0.y as f32].into()));
  }
}
