use bevy_rapier2d::prelude::*;
use bevy::prelude::*;
use crate::*;

pub fn spawn_fire_collider(
    mut cmd: Commands,
    fires_without_rigid: Query<(Entity, &FireId,&Position,&QQVelocity), Without<Transform>>
  ) {
    for (entity, fire_id,position,qv) in fires_without_rigid.iter() {
      let (_,scale) = match fire_id.1{
        0=>{
          (String::from("egg"),0.08)
        }
        _=>{
          (String::from("stick"),0.05)
        }
      };
      cmd.entity(entity)
      .insert_bundle(TransformBundle::from(
        Transform::from_xyz(position.0.x, position.0.y, 3.0).with_scale(Vec3::splat(scale))
      ))
      .insert(RigidBody::Dynamic)
      .insert(Velocity{linvel:[qv.0.x,qv.0.y].into(),angvel:0.5})
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
              v.0-=10;
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