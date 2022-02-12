use bevy::prelude::*;
use qq_party_shared::Position;
mod chicken;
pub struct SpriteCharacterPlugin;
impl Plugin for SpriteCharacterPlugin {
  fn build(&self, app: &mut bevy::app::App) {
      app.add_system(chicken::chicken_system)
      .add_startup_system(startup);
  }
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>,) {
  let texture = asset_server.load("2d/round.png");
  // let mut texture_atlas = TextureAtlas::new_empty(texture_handle, Vec2::new(128.0, 145.0));
  // //let c = texture_atlas.add_texture(bevy::sprite::Rect{min:Vec2::new(740.0,141.0),max:Vec2::new(868.0,286.0)});
  let mut sprites = vec![];
  let rect = bevy::sprite::Rect{min:
    Vec2::new(740.0,141.0),
    max:Vec2::new(868.0,286.0)};
  sprites.push(rect);
  let b = TextureAtlas{
    size: Vec2::new(
        871.0,
        859.0,
    ),
    textures: sprites,
    texture,
    texture_handles: None,
  };
  let texture_atlas_handle = texture_atlases.add(b);
  commands.spawn_bundle(SpriteSheetBundle {
    texture_atlas: texture_atlas_handle,
    transform: Transform::from_xyz(3569.0,3691.8,2.0).with_scale(Vec3::splat(0.2)),
    ..Default::default()
  });
}
