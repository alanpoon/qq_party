use bevy::prelude::*;
use qq_party_shared::{Position,QQVelocity,NPCId,SpecialEffectBundle,SpecialEffectId,StormRingId,LocalUserInfo,BallId};
use std::collections::HashMap;
use rand::Rng;
use crate::AnimationTimer;

#[derive(Component,Clone,Debug)]
pub struct MoveTimer(Timer);

pub fn onstart(mut cmd: Commands){
  let mut rng = rand::thread_rng();
  let mut bundles = vec![];
//  let special_effects = vec![String::from("storm")];
 let special_effects = vec![String::from("storm"),String::from("ice"),String::from("stone"),String::from("rattan")];

  for _ in 0..8{
  //for _ in 0..1{
    let b = rng.gen_range(0..40) as f32;
    for s_e in special_effects.iter(){
      let tv_x = if rng.gen_bool(0.5){
        1.0 * b
      }else{
        -1.0 * b
      };
      let tv_y = if rng.gen_bool(0.5){
        1.0 * b
      }else{
        -1.0 * b
      };
      bundles.push(SpecialEffectBundle{
        id:SpecialEffectId(s_e.clone()),
        position: Position(Vec2::new(3600.0,3620.0)),
        velocity: QQVelocity(Vec2::new(tv_x,tv_y)),
      });
    }
  }
  cmd.spawn_batch(bundles);
}
pub fn add_special_effect_sprite_system(
  mut cmd: Commands,
  mut effects_with_mesh: Query<(&SpecialEffectId,&TextureAtlasSprite,&mut Transform)>,
  effects_without_mesh: Query<(Entity, &SpecialEffectId,&Transform), Without<TextureAtlasSprite>>,
  storm_rings_query: Query<(Entity, &StormRingId)>,
  texture_hashmap:ResMut<HashMap<String,Handle<TextureAtlas>>>,
) {
  let mut found_storm_rings = false;
  for (_,storm_ring) in storm_rings_query.iter(){
    found_storm_rings= true;
    break;
  }
  if found_storm_rings{
    let mut close_proximity_count =0; //spawn closer to user
    for (entity, effect_id,transform) in effects_without_mesh.iter() {
      let sprite_name = effect_id.0.clone();
      if let Some(t_handle)= texture_hashmap.get(&sprite_name){
        cmd.entity(entity).insert_bundle(SpriteSheetBundle {
          transform:transform.clone(),
          texture_atlas: t_handle.clone(),
          ..Default::default()
        })
        .insert(AnimationTimer(Timer::from_seconds(0.1, true)))
        .insert(MoveTimer(Timer::from_seconds(4.0,true)));
      }else{
        info!("cannot find {:?}",sprite_name);
      }
    }
  }else{

    for (effect_id,_,mut transform) in effects_with_mesh.iter_mut() {
      transform.translation = [3900.0,3840.0,2.0].into();
    }
  }
  
}
use qq_party_shared::bevy_rapier2d::prelude::*;
pub fn apply_special_effect_sprite_system(
  mut cmd: Commands,
  mut query: Query<(
    &SpecialEffectId,
    &mut Velocity,
    &mut AnimationTimer,
    &mut MoveTimer,
    &mut TextureAtlasSprite,
    &Handle<TextureAtlas>,
  )>,texture_atlases: Res<Assets<TextureAtlas>>,
  time: Res<Time>,
){
  for (effect_id,mut vel,mut timer,mut move_timer, mut sprite,texture_atlas_handle ) in query.iter_mut(){
      (*timer).0.tick(time.delta());
      if (*timer).0.just_finished() {
          let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
          sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
      }
      (*move_timer).0.tick(time.delta());
      let mut rng = rand::thread_rng();
      if (*move_timer).0.just_finished() {
          
          vel.linvel.x = rng.gen_range(-50..50) as f32;
          vel.linvel.y = rng.gen_range(-50..50) as f32;
      }
  }
}