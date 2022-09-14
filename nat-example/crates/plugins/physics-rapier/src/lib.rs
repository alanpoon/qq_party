use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
//use physics::{shape::Shape, widget::WidgetId, DragState, Velocity};
use qq_party_shared::*;
pub struct PhysicsPlugin;
#[path = "../src_debug_ui/mod.rs"]
mod ui;
mod timewrapper;
mod timewrapper_qq;
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
            //.add_plugin(RapierRenderPlugin)
            // .add_startup_system(walls.system())
            // .add_startup_system(cube.system())
            .add_startup_system(enable_physics_profiling.system())
            //.add_plugin(DebugUiPlugin)
            .insert_resource(RapierConfiguration {
              scale: 1.0,
              gravity: Vector2::zeros(),
              //timestep_mode: bevy_rapier2d::physics::TimestepMode::InterpolatedTimestep,
              ..Default::default()
            })
            .init_resource::<timewrapper::TimeWrapper>()
            .init_resource::<timewrapper_qq::TimeWrapper>()
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
            .add_system(qq_party_shared::systems::update_state_velocity_npc.system())
            //spawn_hierachy
            .add_system(qq_party_shared::systems::physics::spawn_hierachy.system())
            .add_system(qq_party_shared::systems::physics::spawn_joint.system())
            //.add_system(sys_time_debug.system())
            //.init_resource::<bevy_rapier2d::physics::time::Time>()
            .add_system(timewrapper::into_timewrapper.system());
            //.insert_resource(Msaa::default());
            // .add_system(
            //     add_physics_components
            //         .system()
            //         .after(DeskSystem::Shell)
            //         .before(DeskSystem::PrePhysics),
            // )
            //.add_system(add_ball_mesh_system.system())
            //.add_system(update_position_system1.system());
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
// fn walls(mut commands: Commands) {
//     // camera.transform.translation.x = 630.0;
//     // camera.transform.translation.y = 350.0;
//     // commands.spawn_bundle(PointLightBundle {
//     //     point_light: PointLight {
//     //         intensity: 100_000.0,
//     //         range: 6000.0,
//     //         ..Default::default()
//     //     },
//     //     ..Default::default()
//     // });
    
//     commands
//         .spawn_bundle(ColliderBundle {
//             position: wrapper::ColliderPositionComponent(Vector::new(0.0, 0.0).into()),
//             shape: wrapper::ColliderShapeComponent(ColliderShape::cuboid(0.1, 9.0)),
//             ..Default::default()
//         })
//         .insert(ColliderPositionSync::Discrete)
//         .insert(ColliderDebugRender::default());
//     commands
//         .spawn_bundle(ColliderBundle {
//             position: wrapper::ColliderPositionComponent(Vector::new(10.0, 0.0).into()),
//             shape: wrapper::ColliderShapeComponent(ColliderShape::cuboid(0.1, 9.0)),
//             ..Default::default()
//         })
//         .insert(ColliderPositionSync::Discrete)
//         .insert(ColliderDebugRender::default());
//     commands
//         .spawn_bundle(ColliderBundle {
//             position: wrapper::ColliderPositionComponent(Vector::new(0.0, 0.0).into()),
//             shape: wrapper::ColliderShapeComponent(ColliderShape::cuboid(12.0, 0.1)),
//             ..Default::default()
//         })
//         .insert(ColliderPositionSync::Discrete)
//         .insert(ColliderDebugRender::default());
//     commands
//         .spawn_bundle(ColliderBundle {
//             position: wrapper::ColliderPositionComponent(Vector::new(0.0, 7.0).into()),
//             shape: wrapper::ColliderShapeComponent(ColliderShape::cuboid(12.0, 0.1)),
//             ..Default::default()
//         })
//         .insert(ColliderPositionSync::Discrete)
//         .insert(ColliderDebugRender::default());
// }

// fn add_physics_components(
//     rapier: Res<RapierConfiguration>,
//     mut commands: Commands,
//     query: Query<(Entity, &GlobalTransform), Added<Position>>,
// ) {
//     for (card, transform) in query.iter() {
//         commands
//             .entity(card)
//             .insert_bundle(RigidBodyBundle {
//                 position: wrapper::RigidBodyPositionComponent(Vector2::new(transform.translation[0] / rapier.scale,transform.translation[1]/rapier.scale).into()),
//                 mass_properties: wrapper::RigidBodyMassPropsComponent(RigidBodyMassPropsFlags::ROTATION_LOCKED.into()),
//                 damping: wrapper::RigidBodyDampingComponent(RigidBodyDamping {
//                     linear_damping: LINEAR_DAMPING,
//                     ..Default::default()
//                 }),
//                 ..Default::default()
//             })
//             .insert(RigidBodyPositionSync::Discrete)
//             .with_children(|build| {
//                 build.spawn_bundle(ColliderBundle {
//                     shape: wrapper::ColliderShapeComponent(ColliderShape::cuboid(0.1, 0.1)),
//                     ..Default::default()
//                 });
//             });
//     }
// }
// pub fn cube(
//   mut commands: Commands,
//   mut materials: ResMut<Assets<ColorMaterial>>
// ) {
//   let mut rand_rng = rand::thread_rng();
//   let x:i32 = rand_rng.gen_range(35..70);
//   let y:i32 = rand_rng.gen_range(0..200);
//   let r:i32 = rand_rng.gen_range(0..255);
//   let g:i32 = rand_rng.gen_range(0..255);
//   let b:i32 = rand_rng.gen_range(0..255);
//   let bevy_color = Color::rgb_u8(r as u8,g as u8,b as u8);
//   commands
//     .spawn()
//     .insert_bundle(SpriteBundle {
//         transform: Transform::from_translation(Vec3::new(x as f32, y as f32, 1.0)),
//         sprite: Sprite{
//           color: bevy_color,
//           custom_size: Some(Vec2::new(30.0,30.0)),
//           ..Default::default()
//         },
//         ..Default::default()
//     }).insert(wrapper::RigidBodyPositionComponent([x as f32,y as f32].into()))
//     .insert(Position(Vec2::new(x as f32, y as f32)))
//     .insert(TargetVelocity(Vec2::ZERO));
//     // .insert_bundle(RigidBodyBundle {
//     //   body_type: wrapper::RigidBodyType(RigidBodyType::Static),
//     //   position: wrapper::RigidBodyPositionComponent([40.0, 0.0].into()),
//     //   ..RigidBodyBundle::default()
//     // }).insert(RigidBodyPositionSync::Discrete);
//   // let collider = ColliderBundle {
//   //     shape: wrapper::ColliderShapeComponent(ColliderShape::cuboid(rad, rad)),
//   //     ..Default::default()
//   // };
//   // commands
//   //     .spawn_bundle(body)
//   //     .insert_bundle(collider)
//   //     .insert(ColliderDebugRender::with_id(color))
//   //     .insert(ColliderPositionSync::Discrete);
// }
// fn update_velocity(
//     rapier: Res<RapierConfiguration>,
//     mut query: Query<(&mut wrapper::RigidBodyVelocityComponent, &Velocity), Changed<Velocity>>,
// ) {
//     for (mut rapier_velocity, velocity) in query.iter_mut() {
//         rapier_velocity.linvel.x = velocity.0.x / rapier.scale;
//         rapier_velocity.linvel.y = velocity.0.y / rapier.scale;
//     }
// }
//pub fn update_position_system1(mut query: Query<(&mut Position, &Velocity)>, time: Res<Time>) {
// pub fn update_position_system1(mut query: Query<(&mut TargetVelocity, &mut Transform)>, time: Res<Time>) {
   
//   for (mut tv,mut transform) in query.iter_mut() {
//     transform.translation.x += tv.0.x;
//     transform.translation.y += tv.0.y;
//     *tv = TargetVelocity(Vec2::ZERO);
//   }
// }
use bevy_rapier2d::physics::time::TimeInterface;
pub fn sys_time_debug(balls_without_rigid:Query<(&BallId,&Position,&RigidBodyVelocityComponent,&Velocity)>,time:Res<timewrapper::TimeWrapper>){
  for (ball_id,pos,rv,vel) in balls_without_rigid.iter(){
    let delta = time.delta_seconds();
    info!("ball_id {:?} pos {:?} rv {:?} vel{:?} delta {:?}",ball_id,pos,rv.0.linvel,vel,delta);
  }
}
// pub fn debug_rigid(mut query:Query<(&BallId,&Position)>,mut npc_query:Query<(&NPCId,&Position,&ChaseTargetId,&RigidBodyVelocityComponent),Without<BallId>> ){
//   // for (q,rb) in query.iter(){
//   //   info!("ballid{:?} rb {:?} ",q.0,rb.0);
//   // }
//   for (q,pos,rb,v) in npc_query.iter(){
//     //info!("npc{:?} pos {:?} chasetarget{:?} rb vel {:?}",q,pos.0,rb.0,v.0.linvel);
//   }
// }

// fn add_ball_mesh_system(
//   mut cmd: Commands,
//   balls_without_mesh: Query<(Entity, &BallId,&Position), Without<Transform>>,
//   mut materials: ResMut<Assets<ColorMaterial>>,
// ) {
//   for (entity, _,position) in balls_without_mesh.iter() {
//     let mut rand_rng = rand::thread_rng();
//     let r = rand_rng.gen_range(0..255);
//     let g = rand_rng.gen_range(0..255);
//     let b = rand_rng.gen_range(0..255);
//     let bevy_color = Color::rgb_u8(r as u8,g as u8,b as u8);
//       cmd.entity(entity).insert_bundle(SpriteBundle {
//         transform: Transform::from_translation(Vec3::new(position.0.x as f32, position.0.y as f32, 1.0)),
//         sprite: Sprite{
//           color: bevy_color,
//           custom_size: Some(Vec2::new(30.0,30.0)),
//           ..Default::default()
//         },
//         ..Default::default()
//       }).insert(wrapper::RigidBodyPositionComponent([position.0.x as f32,position.0.y as f32].into()));
//   }
// }

