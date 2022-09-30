use wasmbus_rpc::actor::prelude::*;
use qq_party_shared::*;
use bevy_math::Vec2;
use crate::info_::info_;
pub async fn spawn_npc_bundles_debug()-> RpcResult<Vec<NPCBundle>>{
  let mut i = 0;
  let mut npc_bundles:Vec<NPCBundle> = vec![];
  let mut z = 0;
  let mut npc_positions_vec= vec![];
  // let x = random_in_range(3300,3800).await?;
  // let y = random_in_range(3500,3800).await?;
  npc_positions_vec.push((Position(Vec2::new(3300.0,3400.0)),2));
  npc_positions_vec.push((Position(Vec2::new(3400.0,3500.0)),2));
  npc_positions_vec.push((Position(Vec2::new(3500.0,3600.0)),1));
  npc_positions_vec.push((Position(Vec2::new(3600.0,3700.0)),0));
  for (id,(pos,sprite_enum)) in npc_positions_vec.iter().enumerate(){
    //let sprite_enum = if id%3 ==0 {0} else if id%3 ==1 {1 }else {2};
    let npc_bundle = NPCBundle{
      npc_id:NPCId{
        id:id as u32,
        sprite_enum:*sprite_enum
      },
      position:pos.clone(),
      velocity:Velocity(Vec2::new(0.0 as f32,0.0 as f32)),
      chase_target: ChaseTargetId(0,0),
    };
    npc_bundles.push(npc_bundle);
  }
  Ok(npc_bundles)
}