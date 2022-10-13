use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use qq_party_shared::*;
pub fn spawn_storm_ring_collider(
    mut cmd: Commands,
    without_rigid: Query<(Entity, &StormRingId), Without<Transform>>
  ) {
    for (entity, storm_ring_id) in without_rigid.iter() {
      cmd.entity(entity)
      .insert_bundle(TransformBundle::from(Transform::from_xyz(storm_ring_id.0.x, storm_ring_id.0.y, 2.0)))
      .insert(RigidBody::Dynamic)
      .insert(LockedAxes::TRANSLATION_LOCKED)
      .insert(Collider::cuboid(storm_ring_id.1 as f32, storm_ring_id.1 as f32))
      // .with_children(|parent|{
      //   parent.spawn_bundle(collider1)
      //   .insert(ColliderPositionSync::Discrete);
      // })
      ;
    }
  }