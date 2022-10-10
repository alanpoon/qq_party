use bevy_rapier2d::prelude::*;
use bevy::prelude::*;
use std::time::Duration;
use crate::*;
use crate::time_interface::countdown::DamageCountdown;
use crate::time_interface::TimeInterface;
pub fn spawn_storm_ring(
  mut cmd: Commands,
  pos_x: f32,
  pos_y: f32,
  radius:i16
) {
  cmd.spawn().insert(StormRingId(Vec2::new(pos_x,pos_y),radius));
}
pub fn outside_storm_ring_damage<T:TimeInterface+Component>(mut cmd:Commands,
  ball_query: Query<(Entity, &BallId,&Position)>,
  storm_rings_query: Query<&StormRingId>,
  mut timer_query: Query<&mut DamageTimer>,
  time:Res<T>,
  mut res:ResMut<ScoreBoard>,
) {
  for mut timer in timer_query.iter_mut(){
    if timer.0.tick(Duration::from_millis((time.delta_seconds() as f32 * 1000.0) as u64)).just_finished() {
      for (ball_e,ball_id,pos) in ball_query.iter() {
        let mut found_inside = false;
        let mut len_of_storms_ring = 0;
        for storm_ring_id in storm_rings_query.iter(){
          len_of_storms_ring+=1;
          if pos.0.distance_squared(storm_ring_id.0) < (storm_ring_id.1*storm_ring_id.1) as f32{
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
  cmd.spawn().insert(DamageTimer(crate::Timer::new(Duration::from_secs(5),true)));
}