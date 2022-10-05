use bevy::prelude::*;
use qq_party_shared::*;
use bevy_kira_audio::{Audio, AudioPlugin as InnerAudioPlugin};
pub struct AudioPlugin;
impl Plugin for AudioPlugin {
  fn build(&self, app: &mut bevy::app::App) {
      app
      .init_resource::<AudioAble>()
      .add_plugin(InnerAudioPlugin)
      //.add_startup_system(setup)
      .add_system(play_loop)
      ;
  }
}

fn setup(asset_server: Res<AssetServer>, audio: Res<Audio>) {
  info!("use audio");
  //let music = asset_server.load("audio/Run Game 2.mp3");
  let music = asset_server.load("audio/Windless Slopes.ogg");
  audio.play(music);
}
fn play_loop(asset_server: Res<AssetServer>, audio: Res<Audio>,mut res:ResMut<AudioAble>) {
  //audio.play_looped(asset_server.load("audio/Windless Slopes.ogg"));
  if res.0{
    if !res.1{
      res.1 = true;
      audio.play_looped(asset_server.load("audio/Run-Game-2.ogg"));
    }
  }
  //audio.play_looped(asset_server.load("audio/Run-Game-2.ogg"));
}