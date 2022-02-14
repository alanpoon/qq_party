use bevy::prelude::*;
use qq_party_shared::TargetVelocity;
// pub fn chicken(mut text_query: Query<(&mut Text,&mut Style,&mut GlobalTransform)>, query: Query<(&Camera, &Transform,&OrthographicProjection)>){

//   for (_,t,o) in query.iter(){
//     //for (mut text,mut text_t)  in text_query.iter_mut() {
//     for (mut text,mut s,mut g)  in text_query.iter_mut() {
//       text.sections[0].value = format!(r#"T:{:?}
//       R:{:?}
//       S:{:?}
//       o:{:?}
//       "#,*t.translation,*t.rotation,*t.scale,o.scale);
//     }
//   }
// }
pub fn chicken_system(
  time: Res<Time>,
  texture_atlases: Res<Assets<TextureAtlas>>,
  mut q:  Query<&mut TargetVelocity>,
  mut query: Query<(&mut Transform, &Handle<TextureAtlas>)>,
 // camera_query: Query<(&Camera, &Transform,&OrthographicProjection)>
) {
  //for (_,ref t,o) in camera_query.iter(){
    for mut tv in q.iter_mut(){
      for (mut transform, texture_atlas_handle) in query.iter_mut() {
        transform.translation.x += tv.0.x;
        transform.translation.y += tv.0.y;
        *tv = TargetVelocity(Vec2::ZERO);
      }
    }
   
  //}
  
}