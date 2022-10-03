use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use qq_party_shared::*;
pub fn spawn_storm_ring_collider(
    mut cmd: Commands,
    without_rigid: Query<(Entity, &StormRingId), Without<RigidBodyPositionComponent>>
  ) {
    for (entity, storm_ring_id) in without_rigid.iter() {
      let collider1 = ColliderBundle {
        shape: ColliderShapeComponent(ColliderShape::cuboid(storm_ring_id.1 as f32 , storm_ring_id.1 as f32)),
        ..Default::default()
      };
      cmd.entity(entity)
      .insert_bundle(RigidBodyBundle{
        mass_properties: RigidBodyMassPropsFlags::TRANSLATION_LOCKED.into(),
        ccd: RigidBodyCcd {
            ccd_enabled: true,
            ..Default::default()
        }.into(),
        position: [storm_ring_id.0.x, storm_ring_id.0.y].into(),
        ..Default::default()
      })
      .insert(RigidBodyPositionSync::Discrete)
      .with_children(|parent|{
        parent.spawn_bundle(collider1)
        .insert(ColliderPositionSync::Discrete);
      })
      ;
    }
  }