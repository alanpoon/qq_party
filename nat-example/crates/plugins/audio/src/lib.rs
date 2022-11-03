use bevy::prelude::*;
use qq_party_shared::*;
use bevy::utils::Duration;
use bevy_kira_audio::{Audio, AudioPlugin as InnerAudioPlugin,AudioControl,AudioEasing,AudioTween};
pub struct AudioPlugin;
impl Plugin for AudioPlugin {
  fn build(&self, app: &mut bevy::app::App) {
      app
      .init_resource::<AudioAble>()
      .add_plugin(InnerAudioPlugin)
      //.add_startup_system(setup)
      .add_system(play_loop)
      .add_system(play_cracking_sound_system)
      .add_system(play_thunder_system)
      ;
  }
}

fn play_loop(asset_server: Res<AssetServer>, audio: Res<Audio>,mut res:ResMut<AudioAble>) {
  //audio.play_looped(asset_server.load("audio/Windless Slopes.ogg"));
  if res.0{
    if !res.1{
      res.1 = true;
      audio.play(asset_server.load("audio/Run-Game-2.ogg"))
      .fade_in(AudioTween::new(
        Duration::from_secs(2),
        AudioEasing::OutPowi(2),
      ))
      .with_volume(0.5)
      .looped();
    }
  }
  //audio.play_looped(asset_server.load("audio/Run-Game-2.ogg"));
}
fn play_cracking_sound_system(
  _cmd: Commands,
  mut balls_with_hit: Query<Entity, Changed<Hit>>,
  asset_server: Res<AssetServer>, audio: Res<Audio>,
){
  for _ in balls_with_hit.iter_mut() {
    audio.play(asset_server.load("audio/multiple_cracks_1.ogg"));
  }
  
}

fn play_thunder_system(
  storm_rings_query: Query<&StormRingId>,
  asset_server: Res<AssetServer>, audio: Res<Audio>,
  timer_query: Query<&DamageTimer>
){
  for  timer in timer_query.iter(){
    if timer.0.just_finished() {
      let mut len_of_storms_ring = 0;
      for _ in storm_rings_query.iter(){
        len_of_storms_ring+=1;
        break;
      }
      if len_of_storms_ring>0{
        audio.play(asset_server.load("audio/Thunder_-Very-Close_-Rain_-01_inspectorj.ogg"));
      }
    }
  }
}