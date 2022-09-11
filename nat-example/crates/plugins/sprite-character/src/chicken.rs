use bevy::prelude::*;
use qq_party_shared::{Position,BallId};
use std::collections::HashMap;
pub fn chicken_translate(
  _texture_atlases: Res<Assets<TextureAtlas>>,
  mut texture_atlas: Query<(&mut Position,&mut Transform, &Handle<TextureAtlas>)>,
){
  for (po,mut transform, texture_atlas_handle) in texture_atlas.iter_mut() {
    //info!("mutating targetV {:?}",tv);
    transform.translation.x = po.0.x;
    transform.translation.y = po.0.y;
  }
}
pub fn add_chicken_sprite_system(
  mut cmd: Commands,
  balls_without_mesh: Query<(Entity, &BallId,&Position), Without<TextureAtlasSprite>>,
  texture_hashmap:ResMut<HashMap<String,Handle<TextureAtlas>>>
) {
  if let Some(t_handle)= texture_hashmap.get("chicken"){
    let f_handle= texture_hashmap.get("flags");
    for (entity, _,position) in balls_without_mesh.iter() {
      info!("balls_without_mesh");
      cmd.entity(entity).insert_bundle(SpriteSheetBundle {
        texture_atlas: t_handle.clone(),
        transform: Transform::from_xyz(position.0.x as f32,position.0.y as f32,2.0).with_scale(Vec3::splat(0.2)),
        ..Default::default()
      }).insert(Position(Vec2::new(position.0.x as f32, position.0.y as f32)));
      if let Some(f_handle) = f_handle{
        cmd.entity(entity).insert_bundle(SpriteSheetBundle {
          texture_atlas: f_handle.clone(),
          transform: Transform::from_xyz(position.0.x as f32,position.0.y as f32,2.0).with_scale(Vec3::splat(0.2)),
          ..Default::default()
        });
      }
      
    }
  }
  
}