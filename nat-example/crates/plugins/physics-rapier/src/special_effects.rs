use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use qq_party_shared::*;
pub fn spawn_special_effect_collider(
    mut cmd: Commands,
    without_rigid: Query<(Entity, &Position), (With<SpecialEffectId>,Without<RigidBodyPositionComponent>)>
  ) {
    for (entity, position) in without_rigid.iter() {
      let collider1 = ColliderBundle {
        shape: ColliderShapeComponent(ColliderShape::cuboid(20.0, 20.0)),
        ..Default::default()
      };
      cmd.entity(entity)
      .insert_bundle(RigidBodyBundle{
        mass_properties: RigidBodyMassPropsFlags::ROTATION_LOCKED.into(),
        ccd: RigidBodyCcd {
            ccd_enabled: true,
            ..Default::default()
        }.into(),
        position: [position.0.x, position.0.y].into(),
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