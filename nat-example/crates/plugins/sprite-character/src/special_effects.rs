use bevy::prelude::*;
use qq_party_shared::*;
use std::collections::HashMap;
use rand::Rng;
use crate::AnimationTimer;

#[derive(Component,Clone,Debug)]
pub struct MoveTimer(Timer);

pub fn onstart(mut cmd: Commands,texture_hashmap:Res<HashMap<String,Handle<TextureAtlas>>>){
  let mut rng = rand::thread_rng();
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
      let sprite_name = s_e.clone();
      if let Some(t_handle)= texture_hashmap.get(&sprite_name){
        cmd.spawn_bundle(SpecialEffectBundle{
          id:SpecialEffectId(s_e.clone()),
          transform: Transform::from_xyz(3600.0,3620.0,3.0),
          global_transform: GlobalTransform::identity(),
          //velocity: QQVelocity(Vec2::new(tv_x,tv_y)),
          velocity: Velocity{
            linvel:[tv_x,tv_y].into(),
            ..Default::default()
          },
          rigid_body: RigidBody::Dynamic,
          locked_axes: LockedAxes::ROTATION_LOCKED,
          
        })
        .insert_bundle(SpriteSheetBundle {
          transform:Transform::from_xyz(3600.0,3620.0,3.0),
          texture_atlas: t_handle.clone(),
          ..Default::default()
        }).insert(AnimationTimer(Timer::from_seconds(0.1, true)))
        .insert(MoveTimer(Timer::from_seconds(4.0,true)))
        .with_children(|parent|{
          parent.spawn()
          .insert(Collider::cuboid(20.0, 20.0));
        });
      }
    }
  }
}
use qq_party_shared::bevy_rapier2d::prelude::*;
pub fn apply_special_effect_sprite_system(
  _cmd: Commands,
  mut query: Query<(
    &mut Velocity,
    &mut AnimationTimer,
    &mut MoveTimer,
    &mut TextureAtlasSprite,
    &Handle<TextureAtlas>,
  ),With<SpecialEffectId>>,texture_atlases: Res<Assets<TextureAtlas>>,
  time: Res<Time>,
){
  for (mut vel,mut timer,mut move_timer, mut sprite,texture_atlas_handle ) in query.iter_mut(){
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
pub fn add_special_effect_sprite_system(
  _cmd: Commands,
  mut effects_with_mesh: Query<(&SpecialEffectId,&TextureAtlasSprite,&mut Transform)>,
  storm_rings_query: Query<(Entity, &StormRingId)>,
  _texture_hashmap:ResMut<HashMap<String,Handle<TextureAtlas>>>,
) {
  let mut found_storm_rings = false;
  for (_,_storm_ring) in storm_rings_query.iter(){
    found_storm_rings= true;
    break;
  }
  if found_storm_rings{
    for (_effect_id,_,mut transform) in effects_with_mesh.iter_mut() {
      //info!("effect_id {:?} hiding",effect_id);
      transform.translation.z = 3.0;//show
    }
  }else{

    for (_effect_id,_,mut transform) in effects_with_mesh.iter_mut() {
      transform.translation.z = -1.0; //hide
    }
  }
  
}