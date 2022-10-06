use bevy::prelude::*;
use qq_party_shared::{Position,Velocity,NPCId,SpecialEffectBundle,SpecialEffectId,StormRingId};
use std::collections::HashMap;
use rand::Rng;
use crate::AnimationTimer;

#[derive(Component,Clone,Debug)]
pub struct MoveTimer(Timer);

pub fn onstart(mut cmd: Commands){
  let bundles = vec![
    SpecialEffectBundle{
      id:SpecialEffectId(String::from("storm")),
      position: Position(Vec2::new(3600.0,3620.0)),
      velocity: Velocity(Vec2::new(-20.0,20.0)),
    },SpecialEffectBundle{
      id:SpecialEffectId(String::from("ice")),
      position: Position(Vec2::new(3500.0,3700.0)),
      velocity: Velocity(Vec2::new(20.0,40.0)),
    },SpecialEffectBundle{
      id:SpecialEffectId(String::from("stone")),
      position: Position(Vec2::new(3700.0,3600.0)),
      velocity: Velocity(Vec2::new(-24.0,40.0)),
    },SpecialEffectBundle{
      id:SpecialEffectId(String::from("rattan")),
      position: Position(Vec2::new(3570.0,3400.0)),
      velocity: Velocity(Vec2::new(20.0,-50.0)),
    },
  ];
  cmd.spawn_batch(bundles);
}
pub fn add_special_effect_sprite_system(
  mut cmd: Commands,
  effects_with_mesh: Query<(Entity, &SpecialEffectId,&Position,&TextureAtlasSprite)>,
  mut effects_without_mesh: Query<(Entity, &SpecialEffectId,&mut Position), Without<TextureAtlasSprite>>,
  storm_rings_query: Query<(Entity, &StormRingId)>,
  texture_hashmap:ResMut<HashMap<String,Handle<TextureAtlas>>>,
) {
  let mut found_storm_rings = false;
  for (_,storm_ring) in storm_rings_query.iter(){
    found_storm_rings= true;
    break;
  }
  if found_storm_rings{
    for (entity, effect_id,mut pos) in effects_without_mesh.iter_mut() {
      let sprite_name = effect_id.0.clone();
      let mut found_inside = false;
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
      if let Some(t_handle)= texture_hashmap.get(&sprite_name){
        cmd.entity(entity).insert_bundle(SpriteSheetBundle {
          texture_atlas: t_handle.clone(),
          transform: Transform::from_xyz(pos.0.x as f32,pos.0.y as f32,2.0)
          .with_scale(Vec3::splat(1.0)),
          ..Default::default()
        }).insert(Position(Vec2::new(pos.0.x as f32, pos.0.y as f32)))
        .insert(Velocity(Vec2::new(0.0, 0.0)))
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