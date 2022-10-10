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
    .insert_bundle(TransformBundle::from(Transform::from_xyz(position.0.x, position.0.y, 0.0)))
    .insert(RigidBody::Dynamic)
    .insert(LockedAxes::ROTATION_LOCKED)
    // .insert_bundle(RigidBodyBundle{
    //   mass_properties: RigidBodyMassPropsFlags::ROTATION_LOCKED.into(),
    //   ccd: RigidBodyCcd {
    //       ccd_enabled: true,
    //       ..Default::default()
    //   }.into(),
    //   position: [position.0.x, position.0.y].into(),
    //   ..Default::default()
    // })
    .insert(LastNPC(0,None))
    ;
    scoreboard.scores.insert(ball_id.0,(0,ball_label.clone()));
  }
}

pub fn update_state_position_physics(mut query: Query<(&mut Position,&mut Transform)>) {
  for (mut pos,rigid_pos) in query.iter_mut() {
    pos.0.x = rigid_pos.translation.x;
    pos.0.y = rigid_pos.translation.y;
  }
}
pub fn update_state_velocity_physics(mut query: Query<(&Position,&mut Velocity,&mut QQVelocity)>) {
  for (pos,mut v,vel) in query.iter_mut() {
    let mut x=0.0;
    let mut y=0.0;
    if (pos.0.x<=20.0 && vel.0.x >0.0) || (pos.0.x>=3820.0 && vel.0.x <0.0) || (pos.0.x>=20.0 && pos.0.x <= 3820.0){
      x = vel.0.x;
    }
    if (pos.0.y<=20.0 && vel.0.y >0.0) || (pos.0.y>=3820.0 && vel.0.y <0.0) || (pos.0.y>=20.0 && pos.0.y <= 3820.0){
      y = vel.0.y;
    }
    let move_delta = Vect::new(x, y);
    v.linvel = move_delta;
  }
}