use bevy::prelude::*;
use crate::*;

pub fn fire_collision(mut cmd:Commands,mut fire_query: Query<(Entity,&FireId,&Transform),Without<Hit>>,
  mut ball_query:Query<(Entity,&BallId,&Transform)>,
  mut res:ResMut<ScoreBoard>){
  for (e,fire_id,fire_pos) in fire_query.iter_mut(){
    if let Some(fire_original_pos) = fire_id.2{
      if fire_pos.translation.distance([fire_original_pos.x,fire_original_pos.y,3.0].into())>200.0{
        cmd.entity(e).insert(Hit);
      }
    }
    for (ball_e,ball_id,pos) in ball_query.iter_mut(){
      if ball_id.0 != fire_id.0{
        if pos.translation.distance(fire_pos.translation)<50.0{
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
  fire_query: Query<(Entity,&FireId),Changed<Hit>>,
  mut to_despawn:ResMut<EntityToRemove>
) {
  for (e,_) in fire_query.iter(){
    //cmd.entity(e).despawn();
    to_despawn.entities.insert(e);
  }
}