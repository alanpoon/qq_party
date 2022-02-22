use bevy::{core::Time, input::Input, math::Vec3, prelude::*, render::camera::Camera};
use crate::userinfo::LocalUserInfo;
use qq_party_shared::*;
// A simple camera system for moving and zooming the camera.
pub fn move_with_local_player(
    local_user_info: Res<LocalUserInfo>,
    ball_query: Query<(&BallId,&Position)>,
    mut query: Query<(&mut Transform, &mut OrthographicProjection,&mut Camera), With<Camera>>,
) {
  for ( ball_id,po) in ball_query.iter(){
    for (mut transform, mut ortho,mut c) in query.iter_mut() {
      if c.name != Some("camera_ui".to_string()){
        if ball_id == &local_user_info.0.ball_id{
          transform.translation.x = po.0.x;
          transform.translation.y = po.0.y;
          if transform.translation.x >3700.0{
            transform.translation.x = 3700.0;
          }
          if transform.translation.x <160.0{
            transform.translation.x = 160.0;
          }
          if transform.translation.y >3700.0{
            transform.translation.y = 3700.0;
          }
          if transform.translation.y <160.0{
            transform.translation.y = 160.0;
          }
          let z = transform.translation.z;
          // Important! We need to restore the Z values when moving the camera around.
          // Bevy has a specific camera setup and this can mess with how our layers are shown.
          transform.translation.z = z;
        }
      }
    }
  }
}