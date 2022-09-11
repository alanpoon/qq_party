pub mod _2d_round;
use bevy::prelude::*;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use js_sys::Array;
use crate::sprite_sheet::{_2d_round::_fn_chicken,_2d_round::_fn_snake,_2d_round::_fn_chick,self};
pub struct SpriteInfos {
	pub _2d_round: (Handle<Image>, Vec2),
}
#[wasm_bindgen]
extern "C" {
    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = window, js_name = f32_flags_array)]
    fn f32_flags_array() -> Array;
}
pub fn startup(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>,
  mut texture_hashmap:ResMut<HashMap<String,Handle<TextureAtlas>>>) {
  let texture = asset_server.load("2d/round.png");
  let sprite_infos = sprite_sheet::SpriteInfos {
    _2d_round:(texture,Vec2::new(
      871.0,
      859.0,
    )),
  };
  info!("texture_hashmap start ");
  let mut texture_atlas = _fn_chicken((sprite_infos)._2d_round.clone());
  let chicken_handle = texture_atlases.add(texture_atlas.clone());
  texture_hashmap.insert(String::from("chicken"),chicken_handle);
  texture_atlas = _fn_chick((sprite_infos)._2d_round.clone());
  let chick_handle = texture_atlases.add(texture_atlas.clone());
  texture_hashmap.insert(String::from("chick"),chick_handle);
  info!("texture_hashmap start 2");
  texture_atlas = _fn_snake((sprite_infos)._2d_round.clone());
  let snake_handle = texture_atlases.add(texture_atlas.clone());
  texture_hashmap.insert(String::from("snake"),snake_handle);
  let flags_texture = asset_server.load("2d/flags32.png");
  let flags_atlas = TextureAtlas::from_grid(flags_texture, Vec2::new(32.0, 32.0), 1, 248);
  let flags_atlas_handle = texture_atlases.add(flags_atlas);
  texture_hashmap.insert(String::from("flags"),flags_atlas_handle);
  info!("texture_hashmap end ");
}