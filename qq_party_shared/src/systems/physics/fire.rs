use bevy_rapier2d::prelude::*;
use bevy_ecs::prelude::*;
use bevy_rapier2d::prelude::nalgebra::Vector2;
use crate::*;

pub fn spawn_fire_collider(
    mut cmd: Commands,
    fires_without_rigid: Query<(Entity, &FireId,&Position), Without<RigidBodyPositionComponent>>
  ) {
    for (entity, fire_id,position) in fires_without_rigid.iter() {
      cmd.entity(entity)
      .insert_bundle(RigidBodyBundle{
        mass_properties: RigidBodyMassPropsFlags::ROTATION_LOCKED.into(),
        ccd: RigidBodyCcd {
            ccd_enabled: true,
            ..Default::default()
        }.into(),
        position: [position.0.x, position.0.y].into(),
        ..Default::default()
      })
      .insert(RigidBodyPositionSync::Discrete)
      ;
    }
}
pub fn fire_collision(mut cmd:Commands,mut fire_query: Query<(Entity,&FireId,&Position),Without<Hit>>,
  mut ball_query:Query<(Entity,&BallId,&Position,&mut LastNPC)>,
  mut res:ResMut<ScoreBoard>){    
  for (e,npc_id,fire_pos) in fire_query.iter_mut(){
    for (ball_e,ball_id,pos,mut last_npc) in ball_query.iter_mut(){
      
      if pos.0.distance(fire_pos.0)<10.0{
        cmd.entity(e).insert(Hit);
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