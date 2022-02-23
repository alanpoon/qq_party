use bevy_ecs_wasm::prelude::{Schedule,World,Query,SystemStage,IntoSystem,Res};
use qq_party_shared::{BallId,Position,BallBundle};
pub fn spawn(w: &mut World,ball_bundle:BallBundle){
  w.spawn_batch(
    vec![ball_bundle]
  );
}