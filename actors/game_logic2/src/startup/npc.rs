use wasmbus_rpc::actor::prelude::*;
use qq_party_shared::*;
use bevy::math::Vec2;
use crate::numbergen_::random_in_range_;
use wasmcloud_interface_numbergen::random_in_range;
use crate::info_::info_;
pub async fn spawn_npc_bundles()-> RpcResult<Vec<NPCBundle>>{
  let mut npc_bundles:Vec<NPCBundle> = vec![];
  let mut z = 0;
  let mut npc_positions_vec= vec![];
  for ri in 0..2{
    let x = random_in_range(0,200).await?;
    let y = random_in_range(0,200).await?;
    z += random_in_range(0,100).await?;
    for n_x in 0..19{
      for n_y in 0..19{
        for v in 1..(z%3 + 2){
          let sign = z %15;
          let sign2 = z % 10;
          let sprite_enum = if (z+v)%3 ==0 {0} else if (z+v)%3 ==1 {1 }else {2};
          z = z+sign;
          let x1 = 200*n_x + x+sign*sign +v*v*sign2*sign2;
          let y1 = 200*n_y + y+sign*sign + v*v*sign2*sign2;
          if x1>0 &&x1 <3800 && y1>0 && y1<3800{
            npc_positions_vec.push((Position(Vec2::new(x1 as f32,y1 as f32)),sprite_enum));
          }
          let sign = z %11;
          let sign2 = z % 10;
          z = z+sign;
          let sprite_enum = if (z+v)%3 ==0 {0} else if (z+v)%3 ==1 {1 }else {2};
          let x2 = 200*n_x + x+sign*sign - v*v*sign2*sign2;
          let y2 = 200*n_y + y-sign*sign - v*v*sign2*sign2;
          if x2>0 &&x2 <3800 && y2>0 && y2<3800{
            npc_positions_vec.push((Position(Vec2::new(x2 as f32,y2 as f32)),sprite_enum));
          }
          let sign3 = z %11;
          z = z + sign3;
          let sign2 = z % 5;
          let sprite_enum = if (z+v)%3 ==0 {0} else if (z+v)%3 ==1 {1 }else {2}; 
          let x3 = 200*n_x + x - sign3*sign + v*v*sign2*sign2;
          let y3 = 200*n_y + y + sign3*sign - v*v*sign2*sign2;
          if x3>0 &&x3 <3800 && y3>0 && y3<3800{
            npc_positions_vec.push((Position(Vec2::new(x3 as f32,y3 as f32)),sprite_enum));
          }
        }
      }
    }
  }
  for (id,(pos,sprite_enum)) in npc_positions_vec.iter().enumerate(){
    //let sprite_enum = if id%3 ==0 {0} else if id%3 ==1 {1 }else {2};
    let npc_bundle = NPCBundle{
      npc_id:NPCId{
        id:id as u32,
        sprite_enum:*sprite_enum
      },
      position:pos.clone(),
      velocity:QQVelocity(Vec2::new(0.0 as f32,0.0 as f32)),
      chase_target: ChaseTargetId(0,0),
    };
    npc_bundles.push(npc_bundle);
  }
  Ok(npc_bundles)
}
pub fn spawn_npc_bundles_sync()-> RpcResult<Vec<NPCBundle>>{
  let mut npc_bundles:Vec<NPCBundle> = vec![];
  let mut z = 0;
  let mut npc_positions_vec= vec![];
  for ri in 0..2{
    let x = random_in_range_(0,200)?;
    let y = random_in_range_(0,200)?;
    z += random_in_range_(0,100)?;
    for n_x in 0..19{
      for n_y in 0..19{
        for v in 1..(z%3 + 2){
          let sign = z %15;
          let sign2 = z % 10;
          let sprite_enum = if (z+v)%3 ==0 {0} else if (z+v)%3 ==1 {1 }else {2};
          z = z+sign;
          let x1 = 200*n_x + x+sign*sign +v*v*sign2*sign2;
          let y1 = 200*n_y + y+sign*sign + v*v*sign2*sign2;
          if x1>0 &&x1 <3800 && y1>0 && y1<3800{
            npc_positions_vec.push((Position(Vec2::new(x1 as f32,y1 as f32)),sprite_enum));
          }
          let sign = z %11;
          let sign2 = z % 10;
          z = z+sign;
          let sprite_enum = if (z+v)%3 ==0 {0} else if (z+v)%3 ==1 {1 }else {2};
          let x2 = 200*n_x + x+sign*sign - v*v*sign2*sign2;
          let y2 = 200*n_y + y-sign*sign - v*v*sign2*sign2;
          if x2>0 &&x2 <3800 && y2>0 && y2<3800{
            npc_positions_vec.push((Position(Vec2::new(x2 as f32,y2 as f32)),sprite_enum));
          }
          let sign3 = z %11;
          z = z + sign3;
          let sign2 = z % 5;
          let sprite_enum = if (z+v)%3 ==0 {0} else if (z+v)%3 ==1 {1 }else {2}; 
          let x3 = 200*n_x + x - sign3*sign + v*v*sign2*sign2;
          let y3 = 200*n_y + y + sign3*sign - v*v*sign2*sign2;
          if x3>0 &&x3 <3800 && y3>0 && y3<3800{
            npc_positions_vec.push((Position(Vec2::new(x3 as f32,y3 as f32)),sprite_enum));
          }
        }
      }
    }
  }
  for (id,(pos,sprite_enum)) in npc_positions_vec.iter().enumerate(){
    //let sprite_enum = if id%3 ==0 {0} else if id%3 ==1 {1 }else {2};
    let npc_bundle = NPCBundle{
      npc_id:NPCId{
        id:id as u32,
        sprite_enum:*sprite_enum
      },
      position:pos.clone(),
      velocity:QQVelocity(Vec2::new(0.0 as f32,0.0 as f32)),
      chase_target: ChaseTargetId(0,0),
    };
    npc_bundles.push(npc_bundle);
  }
  Ok(npc_bundles)
}