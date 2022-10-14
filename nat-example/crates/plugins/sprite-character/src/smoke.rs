use bevy::prelude::*;
use qq_party_shared::*;
use crate::chicken::*;
use crate::AnimationTimer;
pub fn apply_smoke_animation_system(
  mut cmd: Commands,
  mut query: Query<(
    &DashSmokeAsChild,
    &mut AnimationTimer,
    &mut TextureAtlasSprite,
    &Handle<TextureAtlas>,
  )>,texture_atlases: Res<Assets<TextureAtlas>>,
  time: Res<Time>,
){
  for (_,mut timer, mut sprite,texture_atlas_handle ) in query.iter_mut(){
      (*timer).0.tick(time.delta());
      if (*timer).0.just_finished() {
          let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
          sprite.index = (sprite.index + 1) % 4;
      }
  }
}