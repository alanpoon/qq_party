use bevy::prelude::*;
use qq_party_shared::Position;
use std::collections::HashMap;
use bevy::asset::HandleId;
mod chicken;
mod npc;
mod sprite_sheet;
mod timewrapper;
pub struct SpriteCharacterPlugin;
impl Plugin for SpriteCharacterPlugin {
  fn build(&self, app: &mut bevy::app::App) {
      app
      .init_resource::<HashMap<String,Handle<TextureAtlas>>>()
      .init_resource::<timewrapper::TimeWrapper>()
      .add_system(timewrapper::into_timewrapper.system())
      .add_system(qq_party_shared::systems::update_state_position::<timewrapper::TimeWrapper>.system())
      .add_system(qq_party_shared::systems::update_state_velocity.system())
      .add_system(chicken::chicken_translate.system())
      
      //.add_system(chicken::chicken_system.system())
      .add_system(chicken::add_chicken_sprite_system.system())
      .add_system(npc::add_npc_sprite_system.system())
      .add_startup_system(sprite_sheet::startup)
      .add_startup_system(startup);
  }
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>) {
  let texture = asset_server.load("2d/round.png");
  // let mut texture_atlas = TextureAtlas::new_empty(texture_handle, Vec2::new(128.0, 145.0));
  // //let c = texture_atlas.add_texture(bevy::sprite::Rect{min:Vec2::new(740.0,141.0),max:Vec2::new(868.0,286.0)});
  // let mut sprites = vec![];
  // let rect = bevy::sprite::Rect{min:
  //   Vec2::new(740.0,141.0),
  //   max:Vec2::new(868.0,286.0)};
  // sprites.push(rect);
  // let b = TextureAtlas{
  //   size: Vec2::new(
  //       871.0,
  //       859.0,
  //   ),
  //   textures: sprites,
  //   texture,
  //   texture_handles: None,
  // };
  // let chicken_handle = texture_atlases.add(b);
  // let texture_chicken_id = chicken_handle.id;
  // texture_ids.insert(String::from("chicken"),texture_chicken_id);
  commands.insert_resource(sprite_sheet::SpriteInfos {
    _2d_round:(texture,Vec2::new(
      871.0,
      859.0,
    )),
  });
  // commands.spawn_bundle(SpriteSheetBundle {
  //   texture_atlas: chicken_handle,
  //   transform: Transform::from_xyz(3569.0,3691.8,2.0).with_scale(Vec3::splat(0.2)),
  //   ..Default::default()
  // }).insert(Position(Vec2::new(3569.0, 3691.8)));
}
