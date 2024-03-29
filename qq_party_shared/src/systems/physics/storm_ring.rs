use bevy::prelude::*;
use std::time::Duration;
use crate::*;

pub fn outside_storm_ring_damage(mut cmd:Commands,
  ball_query: Query<(Entity, &BallId,&Transform)>,
  storm_rings_query: Query<&StormRingId>,
  mut timer_query: Query<&mut DamageTimer>,
  time:Res<Time>,
  mut res:ResMut<ScoreBoard>,
) {
  for mut timer in timer_query.iter_mut(){
    if timer.0.tick(Duration::from_millis((time.delta_seconds() as f32 * 1000.0) as u64)).just_finished() {
      for (ball_e,ball_id,pos) in ball_query.iter() {
        let mut found_inside = false;
        let mut len_of_storms_ring = 0;
        for storm_ring_id in storm_rings_query.iter(){
          len_of_storms_ring+=1;
          if pos.translation.distance_squared(Vec3::new(storm_ring_id.0.x,storm_ring_id.0.y,3.0)) < (storm_ring_id.1*storm_ring_id.1) as f32{
            found_inside = true;
            break;
          }
        }
        if len_of_storms_ring >0 && !found_inside{
          cmd.entity(ball_e).insert(Hit);
          if let Some(v) = (*res).scores.get_mut(&ball_id.0) {
              v.0-=10;
              if v.0<0{
                v.0 = 0
              }
          }
        }
      } 
    }
  }
}

pub fn add_damage_timer(mut cmd:Commands){
  cmd.spawn().insert(DamageTimer(Timer::new(Duration::from_secs(5),true)));
}