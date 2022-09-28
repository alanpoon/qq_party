use bevy_ecs::prelude::*;
use qq_party_shared::{BallId,Position,BallBundle,FireBundle};
pub fn spawn(w: &mut World,ball_bundle:BallBundle){
  w.spawn_batch(
    vec![ball_bundle]
  );
}
pub fn spawn_fire(w: &mut World,fire_bundle:FireBundle){
  w.spawn_batch(
    vec![fire_bundle]
  );
}