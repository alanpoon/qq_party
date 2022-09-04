use bevy_rapier2d::prelude::*;
use bevy_ecs::prelude::*;
use bevy_rapier2d::prelude::nalgebra::Vector2;
use bevy_log::info;
use crate::*;
pub fn spawn_npc_collider(
  mut cmd: Commands,
  balls_without_rigid: Query<(Entity, &NPCId,&Position), Without<RigidBodyPositionComponent>>
) {
  for (entity, _,position) in balls_without_rigid.iter() {
    cmd.entity(entity)
    .insert_bundle(RigidBodyBundle{
      mass_properties: RigidBodyMassPropsFlags::ROTATION_LOCKED.into(),
      position: [position.0.x, position.0.y].into(),
      ..Default::default()
    })
    .insert(RigidBodyPositionSync::Discrete)
    ;
  }
}