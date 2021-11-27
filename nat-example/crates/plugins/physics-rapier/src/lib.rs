use core::DeskSystem;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::physics::wrapper;
//use physics::{shape::Shape, widget::WidgetId, DragState, Velocity};
use arugio_shared::{Position,Velocity,update_position_system};
pub struct PhysicsPlugin;
const LINEAR_DAMPING: f32 = 8.0;
use bevy::math::Vec3;
#[path = "../src_debug_ui/mod.rs"]
mod ui;
use ui::DebugUiPlugin;
use crate::nalgebra::Vector2;
use std::f32::consts::PI;

const RAPIER_SCALE: f32 = 20.0;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugin(RapierRenderPlugin)
            .add_startup_system(walls.system())
            .add_startup_system(widget_adding_for_cards.system())
            .add_startup_system(enable_physics_profiling.system())
            .add_plugin(DebugUiPlugin)
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
            .add_system(update_ball_translation_system.system())
            //.add_system(update_position_system1.system())
            .add_system_set(
                SystemSet::new()
                    .label(DeskSystem::PrePhysics)
                    .with_system(update_velocity.system()),
            );
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
            position: wrapper::ColliderPosition(Vector::new(0.0, 0.0).into()),
            shape: wrapper::ColliderShape(ColliderShape::cuboid(0.1, 9.0)),
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete)
        .insert(ColliderDebugRender::default());
    commands
        .spawn_bundle(ColliderBundle {
            position: wrapper::ColliderPosition(Vector::new(10.0, 0.0).into()),
            shape: wrapper::ColliderShape(ColliderShape::cuboid(0.1, 9.0)),
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete)
        .insert(ColliderDebugRender::default());
    commands
        .spawn_bundle(ColliderBundle {
            position: wrapper::ColliderPosition(Vector::new(0.0, 0.0).into()),
            shape: wrapper::ColliderShape(ColliderShape::cuboid(12.0, 0.1)),
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete)
        .insert(ColliderDebugRender::default());
    commands
        .spawn_bundle(ColliderBundle {
            position: wrapper::ColliderPosition(Vector::new(0.0, 7.0).into()),
            shape: wrapper::ColliderShape(ColliderShape::cuboid(12.0, 0.1)),
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
                position: wrapper::RigidBodyPosition(Vector2::new(transform.translation[0] / rapier.scale,transform.translation[1]/rapier.scale).into()),
                mass_properties: wrapper::RigidBodyMassProps(RigidBodyMassPropsFlags::ROTATION_LOCKED.into()),
                damping: wrapper::RigidBodyDamping(RigidBodyDamping {
                    linear_damping: LINEAR_DAMPING,
                    ..Default::default()
                }),
                ..Default::default()
            })
            .insert(RigidBodyPositionSync::Discrete)
            .with_children(|build| {
                build.spawn_bundle(ColliderBundle {
                    shape: wrapper::ColliderShape(ColliderShape::cuboid(0.1, 0.1)),
                    ..Default::default()
                });
            });
    }
}
pub fn widget_adding_for_cards(
  mut commands: Commands,
  mut materials: ResMut<Assets<ColorMaterial>>
) {

  commands
    .spawn()
    .insert_bundle(SpriteBundle {
        material: materials.add(Color::rgb(0.1, 0.5, 0.5).into()),
        transform: Transform::from_translation(Vec3::new(40.0, 0.0, 1.0)),
        sprite: Sprite::new(Vec2::new(
            30.0,
            30.0,
        )),
        ..Default::default()
    }).insert(wrapper::RigidBodyPosition([40.0,0.0].into()))
    .insert(Position(Vec2::new(40.0,0.0)));
    // .insert_bundle(RigidBodyBundle {
    //   body_type: wrapper::RigidBodyType(RigidBodyType::Static),
    //   position: wrapper::RigidBodyPosition([40.0, 0.0].into()),
    //   ..RigidBodyBundle::default()
    // }).insert(RigidBodyPositionSync::Discrete);
  // let collider = ColliderBundle {
  //     shape: wrapper::ColliderShape(ColliderShape::cuboid(rad, rad)),
  //     ..Default::default()
  // };
  // commands
  //     .spawn_bundle(body)
  //     .insert_bundle(collider)
  //     .insert(ColliderDebugRender::with_id(color))
  //     .insert(ColliderPositionSync::Discrete);
}
fn update_velocity(
    rapier: Res<RapierConfiguration>,
    mut query: Query<(&mut wrapper::RigidBodyVelocity, &Velocity), Changed<Velocity>>,
) {
    for (mut rapier_velocity, velocity) in query.iter_mut() {
        rapier_velocity.linvel.x = velocity.0.x / rapier.scale;
        rapier_velocity.linvel.y = velocity.0.y / rapier.scale;
    }
}
fn update_ball_translation_system(mut balls: Query<(&Position, &mut Transform)>) {
  for (position, mut transform) in balls.iter_mut() {
      transform.translation.x = position.0.x;
      //transform.translation.y = position.0.y;
      // transform.rotation =
      //     Quat::from_rotation_ypr(position.0.x * PI / 2.0, -position.0.y * PI / 2.0, 0.0);
  }
}
//pub fn update_position_system1(mut query: Query<(&mut Position, &Velocity)>, time: Res<Time>) {
pub fn update_position_system1(mut query: Query<(&mut Position)>, time: Res<Time>) {
   
  for (mut pos) in query.iter_mut() {
      pos.0 += 0.2 * 2.0 * 15.0;
  }
}