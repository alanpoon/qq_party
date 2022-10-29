use bevy_rapier2d::prelude::*;
use bevy::prelude::*;
use qq_party_shared::*;
pub fn _fn_stop(mut cmd: &mut Commands,mut set: &mut ParamSet<(
    Query<(Entity, &BallId,&mut Transform,&mut Velocity), With<BallId>>,
    Query<(Entity, &NPCId,&mut Transform,&mut Velocity,&mut ChaseTargetId), With<NPCId>>,
    Query<(Entity,&mut Transform),With<StormRingId>>,
    // also access the whole world ... why not
    //&World,
  )>,
  fire_query: &mut Query<Entity,With<FireId>>,
  to_despawn: &mut ResMut<EntityToRemove>){
    for (e,_,_,mut v) in set.p0().iter_mut(){ //ball
        *v = Velocity::zero();
    }
    for (e,_,_,_,_) in set.p1().iter(){ //npc
        to_despawn.entities.insert(e);
    }
    for (e,_) in set.p2().iter(){ //storm
        to_despawn.entities.insert(e);
    }
    for e in fire_query.iter(){ //fire
        to_despawn.entities.insert(e);
    } 
}