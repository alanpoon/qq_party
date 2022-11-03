use bevy::prelude::*;
use crate::chicken::*;
use crate::AnimationTimer;
pub fn apply_smoke_animation_system(
  mut query: Query<(
    &DashSmokeAsChild,
    &mut AnimationTimer,
    &mut TextureAtlasSprite,
  )>,
  time: Res<Time>,
){
  for (_,mut timer, mut sprite ) in query.iter_mut(){
      (*timer).0.tick(time.delta());
      if (*timer).0.just_finished() {
          sprite.index = (sprite.index + 1) % 4;
      }
  }
}