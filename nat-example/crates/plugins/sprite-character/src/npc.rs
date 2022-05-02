use bevy::prelude::*;
use qq_party_shared::{Position,TargetVelocity,BallId,NPCId};
use std::collections::HashMap;
use bevy::asset::HandleId;
use crate::sprite_sheet::{self,_2d_round};
pub fn snake_translate(
  texture_atlases: Res<Assets<TextureAtlas>>,
  mut texture_atlas: Query<(&mut Position,&mut Transform, &Handle<TextureAtlas>)>,
){
  for (mut po,mut transform, texture_atlas_handle) in texture_atlas.iter_mut() {
    //info!("mutating targetV {:?}",tv);
    transform.translation.x = po.0.x;
    transform.translation.y = po.0.y;
  }
}
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
        transform: Transform::from_xyz(position.0.x as f32,position.0.y as f32,2.0)
        .with_scale(Vec3::splat(0.1)),
        ..Default::default()
      }).insert(Position(Vec2::new(position.0.x as f32, position.0.y as f32)));
    }else{
      info!("cannot find {:?}",sprite_name);
    }
  }
}