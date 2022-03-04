use bevy_ecs_wasm::prelude::{World,Query,Res,ResMut};
use wasmbus_rpc::actor::prelude::*;
use qq_party_shared::*;
use bevy_math::Vec2;
use wasmcloud_interface_numbergen::random_in_range;
pub async fn spawn(w: &mut World)-> RpcResult<()>{
  let x = random_in_range(3300,3800).await?;
  let y = random_in_range(3500,3800).await?;
  let id = random_in_range(0,20000).await?;
  let npc_bundle = NPCBundle{
    npc_id:NPCId{
      id,
      sprite_enum:0,
    },
    position:Position(Vec2::new(x as f32,y as f32)),
    velocity:Velocity(Vec2::new(0.0 as f32,2.0 as f32)),
    chase_target: ChaseTargetId(0),
  };
  let mut npc_bundles:Vec<NPCBundle> = vec![];
  npc_bundles.push(npc_bundle);
  w.spawn_batch(npc_bundles);
  Ok(())
}