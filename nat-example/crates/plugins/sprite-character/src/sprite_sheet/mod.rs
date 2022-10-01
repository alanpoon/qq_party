pub mod _2d_round;
use bevy::prelude::*;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use js_sys::Array;
use crate::sprite_sheet::{_2d_round::_fn_chicken,_2d_round::_fn_snake,_2d_round::_fn_chick,_2d_round::_fn_bear,self};
pub struct SpriteInfos {
	pub _2d_round: (Handle<Image>, Vec2),
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
  texture_atlas = _fn_bear((sprite_infos)._2d_round.clone());
  let bear_handle = texture_atlases.add(texture_atlas.clone());
  texture_hashmap.insert(String::from("bear"),bear_handle);
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
  let egg_texture = asset_server.load("2d/egg.png");
  let egg_atlas = TextureAtlas::from_grid(egg_texture,Vec2::new(206.0,258.0),1,1);
  let egg_atlas_handler = texture_atlases.add(egg_atlas);
  texture_hashmap.insert(String::from("egg"),egg_atlas_handler);
  let stick_texture = asset_server.load("2d/Stick2D-Sprite.png");
  let stick_atlas = TextureAtlas::from_grid(stick_texture,Vec2::new(512.0,512.0),1,1);
  let stick_atlas_handler = texture_atlases.add(stick_atlas);
  texture_hashmap.insert(String::from("stick"),stick_atlas_handler);
  let storm_texture = asset_server.load("2d/sprites/storm.png");
  let storm_atlas = TextureAtlas::from_grid(storm_texture,Vec2::new(32.0,31.0),4,1);
  let storm_atlas_handler = texture_atlases.add(storm_atlas);
  texture_hashmap.insert(String::from("storm"),storm_atlas_handler);
  let ice_texture = asset_server.load("2d/sprites/ice.png");
  let ice_atlas = TextureAtlas::from_grid(ice_texture,Vec2::new(170.0,102.0),4,1);
  let ice_atlas_handler = texture_atlases.add(ice_atlas);
  texture_hashmap.insert(String::from("ice"),ice_atlas_handler);
  let stone_texture = asset_server.load("2d/sprites/stone.png");
  let stone_atlas = TextureAtlas::from_grid(stone_texture,Vec2::new(152.0,101.0),1,10);
  let stone_atlas_handler = texture_atlases.add(stone_atlas);
  texture_hashmap.insert(String::from("stone"),stone_atlas_handler);
  let rattan_texture = asset_server.load("2d/sprites/rattan.png");
  let rattan_atlas = TextureAtlas::from_grid(rattan_texture,Vec2::new(191.5,193.0),4,1);
  let rattan_atlas_handler = texture_atlases.add(rattan_atlas);
  texture_hashmap.insert(String::from("rattan"),rattan_atlas_handler);
  info!("texture_hashmap end ");
}