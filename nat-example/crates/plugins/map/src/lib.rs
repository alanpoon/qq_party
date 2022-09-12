use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
mod helpers;
mod layer;
mod tiled;
use crate::tiled::tiled::TiledMapPlugin;
use qq_party_shared::*;
pub struct MapPlugin;
impl Plugin for MapPlugin {
  fn build(&self, app: &mut bevy::app::App) {
      app.add_plugin(TilemapPlugin)
         .add_plugin(TiledMapPlugin)
         .add_startup_system(startup)
         .add_startup_system(layer::start_up_layer)
         .add_system(helpers::texture::set_texture_filters_to_nearest)
         .add_system(helpers::camera::movement)
         .add_system(score_display)
         ;
  }
}
fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
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

fn score_display(mut text_query: Query<(&mut Text,&mut Style,&mut GlobalTransform)>, 
  query: Query<(&Camera, &Transform,&OrthographicProjection)>,
  ball_query: Query<(&BallId,&Position)>,
  scoreboard:Res<ScoreBoard>,userinfo:Res<LocalUserInfo>
){
  for (ball_id,score) in scoreboard.scores.iter(){
    if &userinfo.0.ball_id.0==ball_id{
      let mut pos = String::from("");
      for (b,p) in ball_query.iter(){
        if &b.0 == ball_id{
          pos.push_str(&p.0.x.to_string());
          pos.push_str(":");
          pos.push_str(&p.0.y.to_string());
        }
      }
      for (_,_t,_o) in query.iter(){
        //for (mut text,mut text_t)  in text_query.iter_mut() {
        for (mut text,mut _s,mut _g)  in text_query.iter_mut() {
          text.sections[0].value = format!(r#"BallId:{:?}, Score:{:?}
          pos:{:?}"#,ball_id,score,pos);
        }
      }
    }
  }
}
