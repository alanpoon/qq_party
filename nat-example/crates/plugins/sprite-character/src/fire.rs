use bevy::prelude::*;
use qq_party_shared::*;
use std::collections::HashMap;
pub fn add_fire_sprite_system(
  mut cmd: Commands,
  fires_without_mesh: Query<(Entity, &FireId,&Position), Without<Transform>>,
  texture_hashmap:Res<HashMap<String,Handle<TextureAtlas>>>
) {
  for (entity, fire_id,position) in fires_without_mesh.iter() {
    
    let (sprite_name,scale) = match fire_id.1{
      0=>{
        (String::from("egg"),0.08)
      }
      _=>{
        (String::from("stick"),0.05)
      }
    };
    if let Some(t_handle)= texture_hashmap.get(&sprite_name){
      cmd.entity(entity).insert_bundle(SpriteSheetBundle {
        texture_atlas: t_handle.clone(),
        transform: Transform::from_xyz(position.0.x as f32,position.0.y as f32,2.0)
        .with_scale(Vec3::splat(scale)),
        ..Default::default()
      }).insert(Position(Vec2::new(position.0.x as f32, position.0.y as f32)));
    }else{
      info!("cannot find {:?}",sprite_name);
    }
  }
}