use bevy_rapier2d::prelude::*;
use bevy::prelude::*;
use crate::*;
pub fn spawn_npc_collider(
  mut cmd: Commands,
  balls_without_rigid: Query<(Entity, &NPCId,&Position), Without<Transform>>
) {
  for (entity, npc_id,position) in balls_without_rigid.iter() {
    cmd.entity(entity)
    .insert_bundle(TransformBundle::from(Transform::from_xyz(position.0.x, position.0.y, 2.0).with_scale(Vec3::splat(0.1))))
    .insert(RigidBody::Dynamic)
    // .insert_bundle(RigidBodyBundle{
    //   mass_properties: RigidBodyMassPropsFlags::ROTATION_LOCKED.into(),
    //   position: [position.0.x, position.0.y].into(),
    //   ..Default::default()
    // })
    // .insert(RigidBodyPositionSync::Discrete)
    ;
  }
}