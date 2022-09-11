use bevy::prelude::*;
use std::collections::HashMap;
pub fn startup(mut _commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>,
  mut texture_hashmap:ResMut<HashMap<String,Handle<TextureAtlas>>>) {
  let worker_crate_texture = asset_server.load("2d/workercrate.png");
  mut_rect(&mut texture_atlases,&mut texture_hashmap,worker_crate_texture,String::from("workercrate"),256.0,128.0);  
}
pub fn mut_rect(mut texture_atlases: &mut ResMut<Assets<TextureAtlas>>,mut texture_hashmap: &mut ResMut<HashMap<String,Handle<TextureAtlas>>>,
  texture:Handle<Image>,key:String,width:f32,length:f32){
  let texture_atlas = TextureAtlas{
    texture: texture,
    size: Vec2::new(width,length),
    textures:vec![bevy::sprite::Rect{min:
      Vec2::new(0.0,0.0),
      max:Vec2::new(width,length)}],
    texture_handles:None,
  };
  let handle = texture_atlases.add(texture_atlas.clone());
  texture_hashmap.insert(key,handle);
}