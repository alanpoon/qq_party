use bevy_rapier2d::prelude::*;
use bevy_ecs::prelude::*;
use crate::systems::physics::nalgebra::Vector2;
use crate::*;

pub fn spawn_player_collider(
  mut cmd: Commands,
  balls_without_rigid: Query<(Entity, &BallId,&Position), Without<RigidBodyPositionComponent>>
) {
  for (entity, _,position) in balls_without_rigid.iter() {
    cmd.entity(entity)
    .insert_bundle(RigidBodyBundle{
      //body_type: RigidBodyType::Static.into(),
      mass_properties: RigidBodyMassPropsFlags::ROTATION_LOCKED.into(),
      ccd: RigidBodyCcd {
          ccd_enabled: true,
          ..Default::default()
      }.into(),
      position: [position.0.x, position.0.y].into(),
      ..Default::default()
    })
    //.insert(ColliderPositionComponent(ColliderPosition([position.0.x, position.0.y].into())))
    .insert_bundle(ColliderBundle {
      shape: ColliderShapeComponent(ColliderShape::ball(15.0)),
      material: ColliderMaterial {
        restitution: 1.0,
        friction: 0.8,
        ..Default::default()
      }.into(),
     // position: [position.0.x, position.0.y].into(),
      ..Default::default()
    })
    .insert(ColliderPositionSync::Discrete)
    //.insert(RigidBodyPositionSync::Discrete)
    // .insert_bundle(ColliderBundle {
    //   //position: [position.0.x, position.0.y].into(),
    //   ..Default::default()
    // })
    //.insert(ColliderPositionSync::Discrete)
    ;
  }
}
pub fn spawn_npc_collider(
  mut cmd: Commands,
  balls_without_rigid: Query<(Entity, &NPCId,&Position), Without<RigidBodyPositionComponent>>
) {
  for (entity, _,position) in balls_without_rigid.iter() {
    cmd.entity(entity)
    .insert_bundle(RigidBodyBundle{
      //body_type: RigidBodyType::Static.into(),
      position: [position.0.x, position.0.y].into(),
      ..Default::default()
    })
    .insert_bundle(ColliderBundle {
      shape: ColliderShapeComponent(ColliderShape::ball(15.0)),
      material: ColliderMaterial {
        restitution: 1.0,
        friction: 0.0,
        ..Default::default()
      }.into(),
      //position: [position.0.x, position.0.y].into(),
      ..Default::default()
    })
    .insert(ColliderPositionSync::Discrete)
    //.insert(ColliderPositionComponent(ColliderPosition([position.0.x, position.0.y].into())))
    //.insert(RigidBodyPositionSync::Discrete)
    ;
  }
}
pub fn update_state_position_physics<X:time_interface::TimeInterface + Component>(mut query: Query<(&mut Position,&mut RigidBodyPositionComponent)>, time: Res<X>) {
  let delta = time.delta_seconds();
  for (mut pos,mut rigid_pos) in query.iter_mut() {
      //pos.0 += vel.0 * time.delta_seconds() * 5.0;
    // if (pos.0.x<=20.0 && vel.0.x >0.0) || (pos.0.x>=3820.0 && vel.0.x <0.0) || (pos.0.x>=20.0 && pos.0.x <= 3820.0){
    //   pos.0.x +=  delta * vel.0.x;
    // }
    // if (pos.0.y<=20.0 && vel.0.y >0.0) || (pos.0.y>=3820.0 && vel.0.y <0.0) || (pos.0.y>=20.0 && pos.0.y <= 3820.0){
    //   pos.0.y +=  delta * vel.0.y;
    // }
    pos.0.x = rigid_pos.0.position.translation.vector.x;
    pos.0.y = rigid_pos.0.position.translation.vector.y;
  }
}
pub fn update_state_velocity_physics(mut query: Query<(&Position,&mut RigidBodyVelocityComponent,&mut Velocity)>) {
  for (pos,mut v,mut vel) in query.iter_mut() {
    let mut x=0.0;
    let mut y=0.0;
    if (pos.0.x<=20.0 && vel.0.x >0.0) || (pos.0.x>=3820.0 && vel.0.x <0.0) || (pos.0.x>=20.0 && pos.0.x <= 3820.0){
      x = vel.0.x;
    }
    if (pos.0.y<=20.0 && vel.0.y >0.0) || (pos.0.y>=3820.0 && vel.0.y <0.0) || (pos.0.y>=20.0 && pos.0.y <= 3820.0){
      y = vel.0.y
    }
    let move_delta = Vector2::new(x, y);
    v.0.linvel = move_delta;
  }
}