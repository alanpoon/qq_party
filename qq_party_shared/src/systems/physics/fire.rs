use bevy_rapier2d::prelude::*;
use bevy_ecs::prelude::*;
use bevy_rapier2d::prelude::nalgebra::Vector2;
use crate::*;

pub fn spawn_fire_collider(
    mut cmd: Commands,
    fires_without_rigid: Query<(Entity, &FireId,&Position), Without<RigidBodyPositionComponent>>
  ) {
    for (entity, fire_id,position) in fires_without_rigid.iter() {
      cmd.entity(entity)
      .insert_bundle(RigidBodyBundle{
        //mass_properties: RigidBodyMassPropsFlags::ROTATION_LOCKED.into(),
        ccd: RigidBodyCcd {
            ccd_enabled: true,
            ..Default::default()
        }.into(),
        velocity:RigidBodyVelocityComponent(RigidBodyVelocity { linvel: Vector2::new(0.0, 0.0), angvel: 0.5 }),
        position: [position.0.x, position.0.y].into(),
        ..Default::default()
      })
      .insert(RigidBodyPositionSync::Discrete)
      ;
    }
}
pub fn fire_collision(mut cmd:Commands,mut fire_query: Query<(Entity,&FireId,&Position),Without<Hit>>,
  mut ball_query:Query<(Entity,&BallId,&Position)>,
  mut res:ResMut<ScoreBoard>){
  for (e,fire_id,fire_pos) in fire_query.iter_mut(){
    if let Some(fire_original_pos) = fire_id.2{
      if fire_pos.0.distance(fire_original_pos)>200.0{
        cmd.entity(e).insert(Hit);
      }
    }
    for (ball_e,ball_id,pos) in ball_query.iter_mut(){
      if ball_id.0 != fire_id.0{
        if pos.0.distance(fire_pos.0)<50.0{
          cmd.entity(e).insert(Hit);
          cmd.entity(ball_e).insert(Hit);
          if let Some(v) = (*res).scores.get_mut(&ball_id.0) {
              v.0-=40;
              if v.0<0{
                v.0 = 0
              }
          }
        }
      }
    }
  }
}
pub fn despawn_fire(
  mut cmd: Commands,
  fire_query: Query<(Entity,&FireId,&Position),Changed<Hit>>
) {
  for (e,_,_) in fire_query.iter(){
    cmd.entity(e).despawn();
  }
}