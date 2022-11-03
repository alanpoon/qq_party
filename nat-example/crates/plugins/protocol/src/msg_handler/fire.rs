use bevy_rapier2d::prelude::*;
use bevy::prelude::*;
use qq_party_shared::*;
pub fn _fn(cmd: &mut Commands, set: &mut ParamSet<(
    Query<(Entity, &BallId,&mut Transform,&mut Velocity), With<BallId>>,
    Query<(Entity, &NPCId,&mut Transform,&mut Velocity,&mut ChaseTargetId), With<NPCId>>,
    Query<(Entity,&mut Transform),With<StormRingId>>,
    // also access the whole world ... why not
    //&World,
  )>,ball_id:BallId,velocity:QQVelocity,_sprite_enum:u32){
    for (_entity, qball_id,t,_vel) in set.p0().iter_mut(){
        if ball_id ==*qball_id{
            let (_,scale) = match ball_id.1{
                0=>{
                    (String::from("egg"),0.08)
                }
                _=>{
                    (String::from("stick"),0.05)
                }
            };
            let fire_bundle = FireBundle{
                fire_id:qq_party_shared::FireId(ball_id.0,ball_id.1,Some([t.translation.x,t.translation.y].into())),
                transform:Transform::from_xyz(t.translation.x,t.translation.y,3.0).with_scale(Vec3::splat(scale)),
                global_transform:GlobalTransform::identity(),
                rigid_body:RigidBody::Dynamic,
                velocity:Velocity{linvel:[velocity.0.x,velocity.0.y].into(),angvel:0.5},
            };
            cmd.spawn_bundle(fire_bundle);
        }
    }
     
}