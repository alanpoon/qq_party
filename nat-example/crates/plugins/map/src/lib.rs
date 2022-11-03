use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy::render::texture::ImageSettings;
use chrono::prelude::*;
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
         .insert_resource(ImageSettings::default_nearest())
         .add_startup_system(startup)
         .add_startup_system(layer::start_up_layer)
         //.add_system(helpers::texture::set_texture_filters_to_nearest)
         .add_system(helpers::camera::movement)
         .add_system(score_display)
         ;
  }
}
#[derive(Component,Clone,Debug)]
pub struct ScoreText();
fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
  // commands
  // // 2d camera
  // .spawn()
  // .insert_bundle(UiCameraBundle::default());  
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
  }).insert(ScoreText());
}

fn score_display(mut text_query: Query<(&mut Text,&mut Style,&mut GlobalTransform),With<ScoreText>>, 
  query: Query<(&Camera, &Transform,&OrthographicProjection)>,
  scoreboard:Res<ScoreBoard>,userinfo:Res<LocalUserInfo>,
  storm_timing:Res<StormTiming>
){
  let now: DateTime<Utc> = Utc::now();
  let storm_utc = Utc.timestamp((storm_timing.0 /1000) as i64, (storm_timing.0 % 1000) as u32 * 1000000);
  let mut delta =  storm_utc.signed_duration_since(now).num_milliseconds() as f32 / 1000.0;
  let mut storm_text = String::from("storm is coming in");
  let mut reverse_delta = STORM_INTERVAL as f32 + delta;
  if reverse_delta<0.0{
    storm_text = String::from("storm ends in");
    let storm_utc = Utc.timestamp(((storm_timing.0  ) /1000) as i64 + storm_timing.1 as i64, (storm_timing.0 % 1000) as u32 * 1000000);
    delta =  storm_utc.signed_duration_since(now).num_milliseconds() as f32 / 1000.0;
    // let storm_utc = Utc.timestamp((storm_timing.0 /1000) as i64, (storm_timing.0 % 1000) as u32 * 1000000);
    // let mut delta =  storm_utc.signed_duration_since(now).num_milliseconds() as f32 / 1000.0;
    reverse_delta = STORM_INTERVAL as f32 + delta;
  }
  for (ball_id,score) in scoreboard.scores.iter(){
    if &userinfo.0.ball_id.0==ball_id{
      for (_,_t,_o) in query.iter(){
        for (mut text,mut _s,mut _g)  in text_query.iter_mut() {
          let mut n = format!("Score:{:?}",score.0);
          if reverse_delta>0.0{
            n = format!("{} {} {}sec",n,storm_text,reverse_delta.floor());
          }
          text.sections[0].value = n;
        }
      }
    }
  }
}
