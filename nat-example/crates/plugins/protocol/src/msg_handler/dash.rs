use bevy_rapier2d::prelude::*;
use bevy::prelude::*;
use qq_party_shared::*;
pub fn _fn(mut cmd: &mut Commands,mut set: &mut ParamSet<(
    Query<(Entity, &BallId,&mut Transform,&mut Velocity), With<BallId>>,
    Query<(Entity, &NPCId,&mut Transform,&mut Velocity,&mut ChaseTargetId), With<NPCId>>,
    Query<(Entity,&mut Transform),With<StormRingId>>,
    // also access the whole world ... why not
    //&World,
  )>,ball_id:BallId){
    for (entity, qball_id,pos,vel) in set.p0().iter_mut(){
        if ball_id ==*qball_id{
            cmd.entity(entity).insert(Dash(true,vel.linvel*2.0,vel.linvel));
        }
    }   
}