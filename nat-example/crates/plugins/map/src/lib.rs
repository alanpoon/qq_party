use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
mod helpers;
mod layer;
mod tiled;
use crate::tiled::tiled::TiledMapPlugin;
use qq_party_shared::Position;
pub struct MapPlugin;
impl Plugin for MapPlugin {
  fn build(&self, app: &mut bevy::app::App) {
      app.add_plugin(TilemapPlugin)
         .add_plugin(TiledMapPlugin)
         .add_startup_system(startup)
         .add_startup_system(layer::start_up_layer)
         .add_system(helpers::texture::set_texture_filters_to_nearest)
         .add_system(helpers::camera::movement)
         .add_system(debug);
  }
}
fn startup(mut commands: Commands, asset_server: Res<AssetServer>, mut map_query: MapQuery) {
  commands
  // 2d camera
  .spawn()
  .insert_bundle(UiCameraBundle::default());  
      let font_handle = asset_server
      .load("fonts/FiraSans-Bold.ttf");
  commands.spawn_bundle(TextBundle {
    style: Style {
        align_self: AlignSelf::FlexEnd,
        //position_type: PositionType::Absolute,
        ..Default::default()
    },
    text: Text {
        sections: vec![TextSection {
            value: "Physics time0.1234567890".to_string(),
            style: TextStyle {
                font: font_handle.clone(),
                font_size: 25.0,
                color: Color::BLACK,
                ..Default::default()
            },
            ..Default::default()
        }],
        ..Default::default()
    },
    ..Default::default()
  }).insert(Position(Vec2::new(0.0 as f32, 0.0 as f32)));
}

//fn debug(mut text_query: Query<(&mut Text,&mut Transform),With<Transform>>, query: Query<(&Camera, &Transform,&OrthographicProjection)>){
fn debug(mut text_query: Query<(&mut Text,&mut Style,&mut GlobalTransform)>, query: Query<(&Camera, &Transform,&OrthographicProjection)>){

  for (_,t,o) in query.iter(){
    //for (mut text,mut text_t)  in text_query.iter_mut() {
    for (mut text,mut s,mut g)  in text_query.iter_mut() {
      text.sections[0].value = format!(r#"T:{:?}
      R:{:?}
      S:{:?}
      o:{:?}
      "#,*t.translation,*t.rotation,*t.scale,o.scale);
    }
  }
}
