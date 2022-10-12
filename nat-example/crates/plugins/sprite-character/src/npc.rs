use bevy::prelude::*;
use qq_party_shared::{Position,NPCId};
use std::collections::HashMap;
pub fn add_npc_sprite_system(
  mut cmd: Commands,
  balls_without_mesh: Query<(Entity, &NPCId,&Position), Without<Transform>>,
  texture_hashmap:ResMut<HashMap<String,Handle<TextureAtlas>>>
) {
  for (entity, npcid,position) in balls_without_mesh.iter() {
    let sprite_name = match npcid.sprite_enum{
      0=>{
        String::from("snake")
      }
      1 =>{
        String::from("workercrate")
      }
      2 =>{
        String::from("chick")
      }
      _=>{
        String::from("snake")
      }
    };
    if let Some(t_handle)= texture_hashmap.get(&sprite_name){
      cmd.entity(entity).insert_bundle(SpriteSheetBundle {
        texture_atlas: t_handle.clone(),
        ..Default::default()
      }).insert(Position(Vec2::new(position.0.x as f32, position.0.y as f32)));
    }else{
      info!("cannot find {:?}",sprite_name);
    }
  }
}
