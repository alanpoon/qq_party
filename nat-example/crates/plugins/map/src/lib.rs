use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
pub struct MapPlugin;
impl Plugin for MapPlugin {
  fn build(&self, app: &mut bevy::app::App) {
      app.add_plugin(TilemapPlugin)
         .add_startup_system(startup)
  }
}
fn startup(mut commands: Commands, asset_server: Res<AssetServer>, mut map_query: MapQuery) {
  commands.spawn_bundle(OrthographicCameraBundle::new_2d());

  let texture_handle = asset_server.load("2d/tiles_with_spacing.png");

  // Create map entity and component:
  let map_entity = commands.spawn().id();
  let mut map = Map::new(0u16, map_entity);

  // Create the settings for the layer
  let mut layer_settings = LayerSettings::new(
      MapSize(2, 2),
      ChunkSize(8, 8),
      TileSize(16.0, 16.0),
      TextureSize(50.0, 33.0),
  );
  layer_settings.tile_spacing = Vec2::new(1.0, 1.0);

  // Creates a new layer builder with a layer entity.
  let (mut layer_builder, _) = LayerBuilder::new(&mut commands, layer_settings, 0u16, 0u16);

  // Set the texture for the tile
  // Note: the atlas is a 3x2, first row is 0..2 left to right
  //       the selected index is the middle texture on the second row
  layer_builder.set_all(TileBundle {
      tile: Tile {
          texture_index: 4u16,
          ..Default::default()
      },
      ..Default::default()
  });

  // Builds the layer.
  // Note: Once this is called you can no longer edit the layer until a hard sync in bevy.
  let layer_entity = map_query.build_layer(&mut commands, layer_builder, texture_handle);

  // Required to keep track of layers for a map internally.
  map.add_layer(&mut commands, 0u16, layer_entity);

  // Spawn Map
  // Required in order to use map_query to retrieve layers/tiles.
  commands
      .entity(map_entity)
      .insert(map)
      .insert(Transform::from_xyz(-128.0, -128.0, 0.0))
      .insert(GlobalTransform::default());
}