use bevy::prelude::*;use crate::tiled::tiled::*;
pub fn start_up_layer(mut commands: Commands, asset_server: Res<AssetServer>){
  let handle: Handle<TiledMap> = asset_server.load("2d/qq_party_tile.tmx");
  //let handle: Handle<TiledMap> = asset_server.load("2d/map.tmx");
  commands.spawn().insert_bundle(TiledMapBundle {
    tiled_map: handle,
    ..Default::default()
  });
  let mut camera = Camera2dBundle::default();
   camera.transform.translation.x = 3569.0;
   camera.transform.translation.y = 3691.8;
  camera.projection.scale = 0.4;
  commands.spawn_bundle(camera);
}