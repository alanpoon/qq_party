use bevy_rapier2d::prelude::*;
use bevy::prelude::*;
use qq_party_shared::*;
pub fn _fn(_cmd: &mut Commands,set: &mut ParamSet<(
    Query<(Entity, &BallId,&mut Transform,&mut Velocity), With<BallId>>,
    Query<(Entity, &NPCId,&mut Transform,&mut Velocity,&mut ChaseTargetId), With<NPCId>>,
    Query<(Entity,&mut Transform),With<StormRingId>>,
    // also access the whole world ... why not
    //&World,
  )>,gball_id:u32,to_despawn:&mut ResMut<EntityToRemove>,scoreboard:&mut ResMut<ScoreBoard>){
    for (e,ballid,_,_) in set.p0().iter(){
        if ballid.0 == gball_id{
            //cmd.entity(e).despawn_recursive();
            to_despawn.entities.insert(e);
            scoreboard.scores.remove(&gball_id);
        }
    }
}