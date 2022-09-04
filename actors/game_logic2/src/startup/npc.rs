use bevy_ecs::prelude::*;
use wasmbus_rpc::actor::prelude::*;
use qq_party_shared::*;
use bevy_math::Vec2;
use wasmcloud_interface_numbergen::random_in_range;

pub async fn spawn_npc_bundles()-> RpcResult<Vec<NPCBundle>>{
  let mut i = 0;
  let mut npc_bundles:Vec<NPCBundle> = vec![];
  while i < 20 {
    let x = random_in_range(3300,3800).await?;
    let y = random_in_range(3500,3800).await?;
    let id = random_in_range(0,20000).await?;
    let sprite_enum = if i%3 ==0 {0} else if i%3 ==1 {1 }else {2};
    let npc_bundle = NPCBundle{
      npc_id:NPCId{
        id,
        sprite_enum
      },
      position:Position(Vec2::new(x as f32,y as f32)),
      velocity:Velocity(Vec2::new(0.0 as f32,0.0 as f32)),
      chase_target: ChaseTargetId(0,0),
    };
    npc_bundles.push(npc_bundle);
    i = i + 1;
  }
  Ok(npc_bundles)
}