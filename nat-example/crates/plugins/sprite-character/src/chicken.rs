use bevy::prelude::*;

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
  mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
  camera_query: Query<(&Camera, &Transform,&OrthographicProjection)>
) {
  for (_,t,o) in camera_query.iter(){
    for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        
        sprite.transform = Transform::from_xyz(*t.translation.x,*t.translation.y,2.0);
    }
  }
  
}