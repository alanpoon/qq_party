use bevy::prelude::*;
//use qq_party_shared::Position;
use std::collections::HashMap;
mod chicken;
mod npc;
mod fire;
mod special_effects;
mod sprite_sheet;
mod single_image;
mod timewrapper;
mod smoke;
use qq_party_shared::*;
use wasm_bindgen::prelude::*;
pub struct SpriteCharacterPlugin;
#[derive(Component,Debug, PartialEq, Default)]
pub struct H{
  pub hash_map:HashMap<String,usize>
}
impl Plugin for SpriteCharacterPlugin {
  fn build(&self, app: &mut bevy::app::App) {
      app
      .init_resource::<HashMap<String,Handle<TextureAtlas>>>()
      .init_resource::<timewrapper::TimeWrapper>()
      .init_resource::<H>()
      .init_resource::<Handle<Font>>()
      .add_system(timewrapper::into_timewrapper)
      .add_system(chicken::add_chicken_sprite_system)
      .add_system(chicken::hit_chicken_sprite_system)
      .add_system(chicken::remove_hit_chicken_sprite_system)
      .add_system(chicken::add_dash_chicken_sprite_system)
      .add_system(chicken::remove_dash_chicken_sprite_system)
      .add_system(npc::add_npc_sprite_system)
      .add_system(fire::add_fire_sprite_system)
      //.add_system(fire::debug_fire_sprite_system)
      .add_system(special_effects::add_special_effect_sprite_system)
      .add_system(special_effects::apply_special_effect_sprite_system)
      .add_system(smoke::apply_smoke_animation_system)
      .add_startup_system(sprite_sheet::startup)
      .add_startup_system(single_image::startup)
      .add_startup_system(startup)
      .add_startup_system(special_effects::onstart);
  }
}
use js_sys::{Array};
#[wasm_bindgen]
extern "C" {
    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = window, js_name = f32_flag)]
    fn f32_flags_array() -> Array;
}
use serde::{Deserialize, Serialize};
#[derive(Serialize,Deserialize)]
pub struct Obj{
  pub key: String,
  pub index: usize,
}
//f32_country_array
fn startup(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>,
  mut flag_usize_map:ResMut<H>,
  mut font_handle: ResMut<Handle<Font>>) {
  let texture = asset_server.load("2d/round.png");
  //f32_flags_array()
  for j in f32_flags_array().iter(){
    if let Ok(j2) = j.into_serde::<Obj>(){
      (*flag_usize_map).hash_map.insert(j2.key,j2.index);
    }
  }
  commands.insert_resource(sprite_sheet::SpriteInfos {
    _2d_round:(texture,Vec2::new(
      871.0,
      859.0,
    )),
  });
  *font_handle = asset_server
  .load("fonts/FiraSans-Bold.ttf");
}
#[derive(Component,Clone,Debug)]
pub struct AnimationTimer(Timer);