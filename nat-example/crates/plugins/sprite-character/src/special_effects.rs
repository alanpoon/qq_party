use bevy::prelude::*;
use qq_party_shared::{Position,Velocity,NPCId,SpecialEffectBundle,SpecialEffectId,StormRingId,LocalUserInfo,BallId};
use std::collections::HashMap;
use rand::Rng;
use crate::AnimationTimer;

#[derive(Component,Clone,Debug)]
pub struct MoveTimer(Timer);

pub fn onstart(mut cmd: Commands){
  let mut rng = rand::thread_rng();
  let mut bundles = vec![];
  let special_effects = vec![String::from("storm"),String::from("ice"),String::from("stone"),String::from("rattan")];
  for _ in 0..8{
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
        velocity: Velocity(Vec2::new(tv_x,tv_y)),
      });
    }
  }
  cmd.spawn_batch(bundles);
}
pub fn add_special_effect_sprite_system(
  mut cmd: Commands,
  mut ball_query: Query<(&BallId,&Position)>,
  effects_with_mesh: Query<(Entity, &SpecialEffectId,&Position,&TextureAtlasSprite)>,
  mut effects_without_mesh: Query<(Entity, &SpecialEffectId), Without<TextureAtlasSprite>>,
  storm_rings_query: Query<(Entity, &StormRingId)>,
  texture_hashmap:ResMut<HashMap<String,Handle<TextureAtlas>>>,
  local_user_info: Res<LocalUserInfo>
) {
  let mut found_storm_rings = false;
  for (_,storm_ring) in storm_rings_query.iter(){
    found_storm_rings= true;
    break;
  }
  if found_storm_rings{
    
    for (entity, effect_id) in effects_without_mesh.iter_mut() {
      let mut ball_pos =  Position(Vec2::new(0.0,0.0));
      for ( ball_id,po) in ball_query.iter(){
        if ball_id == &local_user_info.0.ball_id{
          ball_pos = po.clone();
        }
      }
      let mut rng = rand::thread_rng();
      let sprite_name = effect_id.0.clone();
      let mut found_inside = false;
      //spawn special effect near userspace
      let mut pos =  Position(Vec2::new(0.0,0.0));

      pos.0.x = rng.gen_range(-400..400) as f32 + ball_pos.0.x;
      pos.0.y = rng.gen_range(-400..400) as f32 + ball_pos.0.y;
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
      //info!("special effect pos {:?}",pos);
      if let Some(t_handle)= texture_hashmap.get(&sprite_name){
        cmd.entity(entity).insert_bundle(SpriteSheetBundle {
          texture_atlas: t_handle.clone(),
          transform: Transform::from_xyz(pos.0.x as f32,pos.0.y as f32,2.0)
          .with_scale(Vec3::splat(1.0)),
          ..Default::default()
        }).insert(Position(Vec2::new(pos.0.x as f32, pos.0.y as f32)))
        //.insert(Velocity(Vec2::new(0.0, 0.0)))
        .insert(AnimationTimer(Timer::from_seconds(0.1, true)))
        .insert(MoveTimer(Timer::from_seconds(4.0,true)));
      }else{
        info!("cannot find {:?}",sprite_name);
      }
    }
  }else{

    for (entity, effect_id,_,_) in effects_with_mesh.iter() {
      let sprite_name = effect_id.0.clone();
      cmd.entity(entity).remove_bundle::<SpriteSheetBundle>();
    }
  }
  
}
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
          
          vel.0.x = rng.gen_range(-50..50) as f32;
          vel.0.y = rng.gen_range(-50..50) as f32;
      }
  }
}