use bevy_rapier2d::prelude::*;
use bevy::prelude::*;
use qq_party_shared::*;
pub fn _fn(_cmd: &mut Commands, set: &mut ParamSet<(
    Query<(Entity, &BallId,&mut Transform,&mut Velocity), With<BallId>>,
    Query<(Entity, &NPCId,&mut Transform,&mut Velocity,&mut ChaseTargetId), With<NPCId>>,
    Query<(Entity,&mut Transform),With<StormRingId>>,
    // also access the whole world ... why not
    //&World,
  )>,ball_id:BallId,tv:TargetVelocity){
    for (_entity, qball_id,_t,mut v) in set.p0().iter_mut(){
        if ball_id ==*qball_id{
            let f = if tv.0.x * tv.0.x+tv.0.y * tv.0.y>=2.0{
                1.0
            } else{
                std::f32::consts::SQRT_2
            };
            v.linvel.x = tv.0.x *50.0 * f;
            v.linvel.y = tv.0.y * 50.0 * f;
        }
    }  
}