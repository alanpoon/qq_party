use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use qq_party_shared::*;
use rand::Rng;
pub fn spawn_special_effect_collider(
    mut cmd: Commands,
    without_rigid: Query<(Entity, &Position), (With<SpecialEffectId>,Without<Transform>)>
  ) {
    for (entity, position) in without_rigid.iter() {
      //info!("spawn_special_effect_collider position {:?}",position.clone());
      cmd.entity(entity)
      .insert_bundle(TransformBundle::from(Transform::from_xyz(position.0.x, position.0.y, 2.0)))
      .insert(RigidBody::Dynamic)
      .insert(LockedAxes::ROTATION_LOCKED)
      //.insert(Collider::cuboid(20.0, 20.0))
      .with_children(|parent|{
        parent.spawn()
        .insert(Collider::cuboid(20.0, 20.0));
      })
      ;
    }
  }
pub fn move_special_effect_closer_to_user_system(
  ball_query: Query<(&BallId,&Transform),Without<SpecialEffectId>>,
  mut effects_query: Query<(Entity, &SpecialEffectId,&mut Transform),Without<BallId>>,
  storm_rings_query: Query<(Entity, &StormRingId),Changed<StormRingId>>,
  local_user_info: Res<LocalUserInfo>
) {
  
  let mut found_storm_rings = false;
  for (_,_) in storm_rings_query.iter(){
    found_storm_rings= true;
    break;
  }
  if found_storm_rings{
    let mut close_proximity_count =0; //spawn closer to user
    let mut rng = rand::thread_rng();
    for (_, _effect_id,mut rigid_pos) in effects_query.iter_mut() {
      let mut ball_pos =  Transform::from_xyz(0.0,0.0,0.0);
      for ( ball_id,po) in ball_query.iter(){
        if ball_id == &local_user_info.0.ball_id{
          ball_pos = po.clone();
        }
      }
      let mut found_inside = false;
      //spawn special effect near userspace
      let mut pos =  Position(Vec2::new(0.0,0.0));
      if close_proximity_count<4{
        pos.0.x = rng.gen_range(-50..50) as f32 + ball_pos.translation.x;
        pos.0.y = rng.gen_range(-50..50) as f32 + ball_pos.translation.y;
      }else{
        pos.0.x = rng.gen_range(-400..400) as f32 + ball_pos.translation.x;
        pos.0.y = rng.gen_range(-400..400) as f32 + ball_pos.translation.y;
      }
      close_proximity_count+=1;
      for (_,storm_ring_id) in storm_rings_query.iter(){
        if pos.0.distance_squared(storm_ring_id.0) < (storm_ring_id.1*storm_ring_id.1) as f32{
          found_inside = true;
          break;
        }
      }
      if found_inside{
        pos.0.x = 1800.0;
        pos.0.y = 200.0;
      }
      rigid_pos.translation = [pos.0.x, pos.0.y,2.0].into();
    }
  }
}