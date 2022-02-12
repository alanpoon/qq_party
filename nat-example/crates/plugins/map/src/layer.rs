use bevy_ecs_tilemap::prelude::*;
use bevy::prelude::*;
use crate::tiled::tiled::*;
pub fn start_up_layer(mut commands: Commands, asset_server: Res<AssetServer>, mut map_query: MapQuery){
  let handle: Handle<TiledMap> = asset_server.load("2d/qq_party_tile.tmx");

  // Create map entity and component:
  let map_entity = commands.spawn().id();
  // Spawn Map
  // Required in order to use map_query to retrieve layers/tiles.
  commands.entity(map_entity).insert_bundle(TiledMapBundle {
    tiled_map: handle,
    map: Map::new(0u16, map_entity),
    transform: Transform::from_xyz(0.0, 0.0, 0.0),
    ..Default::default()
  });
  let mut camera = OrthographicCameraBundle::new_2d();
  camera.transform.translation.x = 3569.0;
  camera.transform.translation.y = 3691.8;
  camera.orthographic_projection.scale = 0.4;
  commands.spawn_bundle(camera);
}