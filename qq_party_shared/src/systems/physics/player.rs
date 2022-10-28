use bevy_rapier2d::prelude::*;
use bevy::prelude::*;
use bevy_rapier2d::math::Vect;
use crate::*;

pub fn spawn_player_collider(
  mut cmd: Commands,
  balls_without_rigid: Query<(Entity, &BallId,&BallLabel,&Position), Without<Transform>>,
  mut scoreboard:ResMut<ScoreBoard>
) {
  for (entity, ball_id,ball_label,position) in balls_without_rigid.iter() {
    cmd.entity(entity)
    .insert_bundle(TransformBundle::from(Transform::from_xyz(position.0.x, position.0.y, 3.0).with_scale(Vec3::splat(0.2))))
    .insert(RigidBody::Dynamic)
    .insert(LockedAxes::ROTATION_LOCKED)
    .insert(LastNPC(0,None,false))
    ;
    scoreboard.scores.insert(ball_id.0,(0,ball_label.clone()));
  }
}
pub fn add_ball_dash_physics(mut cmd:Commands,mut query: Query<(Entity,&Position,&mut Velocity,&Dash),Changed<Dash>>) {
  for (e,_pos,mut v,dash) in query.iter_mut() {
   if dash.0{
    cmd.entity(e).insert(DashTimer(Timer::new(Duration::new(1,0),false)));
    v.linvel = dash.1.into();
   }else{
    v.linvel = dash.2.into();
   }
  }
}
use std::time::Duration;
pub fn remove_ball_dash_physics(mut query: Query<(&Position,&mut Dash,&mut DashTimer)>,
time : Res<Time>) {
  for (_pos,mut dash,mut timer) in query.iter_mut() {
    if timer.0.tick(Duration::from_millis((time.delta_seconds() as f32 * 1000.0) as u64)).just_finished() {
      dash.0 = false; 
    }
  }
}
// pub fn update_state_position_physics(mut query: Query<(&mut Position,&mut Transform)>) {
//   for (mut pos,rigid_pos) in query.iter_mut() {
//     pos_x = rigid_pos.translation.x;
//     pos_y = rigid_pos.translation.y;
//   }
// }

pub fn update_state_velocity_physics(mut query: Query<(&mut Transform,&mut Velocity)>) {
  for (mut transform,mut v) in query.iter_mut() {
   // info!("qqvel {:?} pos {:?}",v.clone(),pos.clone());
    let mut x=0.0;
    let mut y=0.0;
    let pos_x = transform.translation.x;
    let pos_y = transform.translation.y;
    if (pos_x<=20.0 && v.linvel.x >0.0) || (pos_x>=3820.0 && v.linvel.x <0.0) || (pos_x>=20.0 && pos_x <= 3820.0){
      x = v.linvel.x;
    }
    if (pos_y<=20.0 && v.linvel.y >0.0) || (pos_y>=3820.0 && v.linvel.y <0.0) || (pos_y>=20.0 && pos_y <= 3820.0){
      y = v.linvel.y;
    }
    let move_delta = Vect::new(x, y);
    v.linvel = move_delta;
    if pos_x>=3820.0{
      transform.translation.x = 3819.0;
    }
    if pos_x<=0.0{
      transform.translation.x = 1.0;
    }
    if pos_y>=3820.0{
      transform.translation.y = 3819.0;
    }
    if pos_y<=0.0{
      transform.translation.y = 1.0;
    }
  }
}