use bevy::prelude::*;
use qq_party_shared::{Position,TargetVelocity,BallId};
use std::collections::HashMap;
use bevy::asset::HandleId;
use crate::sprite_sheet::{self,_2d_round};
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
  mut texture_atlas: Query<(&mut Transform, &Handle<TextureAtlas>)>,
 // camera_query: Query<(&Camera, &Transform,&OrthographicProjection)>
) {
  //for (_,ref t,o) in camera_query.iter(){
    for mut tv in q.iter_mut(){
      for (mut transform, texture_atlas_handle) in texture_atlas.iter_mut() {
        transform.translation.x += tv.0.x;
        transform.translation.y += tv.0.y;
        *tv = TargetVelocity(Vec2::ZERO);
      }
    }
  //}
}
pub fn add_chicken_sprite_system(
  mut cmd: Commands,
  balls_without_mesh: Query<(Entity, &BallId,&Position), Without<Transform>>,
  mut texture_atlases: ResMut<Assets<TextureAtlas>>,
  texture_ids: Res<HashMap<String,HandleId>>,
  sprite_infos: Res<sprite_sheet::SpriteInfos>,
) {
  let texture_atlas = _2d_round::_fn_chicken((*sprite_infos)._2d_round.clone());
  // if let Some(h_id) = texture_ids.get(&String::from("chicken")){
  //   let chicken_handle = texture_atlases.get_handle(*h_id);
  let chicken_handle = texture_atlases.add(texture_atlas.clone());
    for (entity, _,position) in balls_without_mesh.iter() {
      cmd.entity(entity).insert_bundle(SpriteSheetBundle {
        texture_atlas: chicken_handle.clone(),
        transform: Transform::from_xyz(position.0.x as f32,position.0.y as f32,2.0).with_scale(Vec3::splat(0.2)),
        ..Default::default()
      }).insert(Position(Vec2::new(position.0.x as f32, position.0.y as f32)));
    }
  //}
  
}