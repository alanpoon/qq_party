use bevy::{core::Time, input::Input, math::Vec3, prelude::*, render::camera::Camera};
use qq_party_shared::*;
use protocol::{Command,nats};
// A simple camera system for moving and zooming the camera.
pub fn move_with_local_player(
    mut commands: ResMut<protocol::Commands>,
    mut local_user_info: ResMut<LocalUserInfo>,
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
          //sub_map
          let sa = sub_map_area(transform.translation.x,transform.translation.x);
          if local_user_info.0.sub_map !=sa{
            //unsub
            info!("sub game_logic.{}",sa);
            // if local_user_info.0.sub_map!=String::from(""){
            //   let n = nats::proto::ClientOp::Unsub{
            //     sid:17,
            //     max_msgs:None,
            //   };
            //   (*commands).push(Command::Nats(String::from("default"),n));
            // }
            //new_sub
            let n = nats::proto::ClientOp::Sub{
              subject:format!("game_logic.{}",sa),
              queue_group:None,
              sid:17,
            };
            (*commands).push(Command::Nats(String::from("default"),n.clone()));
            info!("after_push..{:?}",n);
            local_user_info.0.sub_map = sa;
          }
        }
      }
    }
  }
}
pub fn sub_map_area(x:f32,y:f32) ->String{
  let mut sub_map = String::from("C");
  if x > 1900.0 && y <1900.0{
    sub_map = String::from("D");
  }else if x > 1900.0 && y >= 1900.0{
    sub_map = String::from("B");
  }else if x <= 1900.0 && y >= 1900.0{
    sub_map = String::from("A");
  }
  sub_map
}