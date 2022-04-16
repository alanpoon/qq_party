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
      position: [position.0.x, position.0.y].into(),
      ..Default::default()
    })
    // .insert_bundle(ColliderBundle {
    //   //position: [position.0.x, position.0.y].into(),
    //   ..Default::default()
    // })
    .insert(ColliderPositionSync::Discrete);
  }
}