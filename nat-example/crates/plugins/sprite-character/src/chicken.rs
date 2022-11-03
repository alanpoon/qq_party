use bevy::prelude::*;
use qq_party_shared::*;
use std::collections::HashMap;
use crate::H;
use crate::AnimationTimer;
pub fn add_chicken_sprite_system(
  mut cmd: Commands,
  balls_without_mesh: Query<(Entity, &BallId,&BallLabel,&Transform), Without<TextureAtlasSprite>>,
  texture_hashmap:ResMut<HashMap<String,Handle<TextureAtlas>>>,
  flag_usize_map:Res<H>,
  font_handle: Res<Handle<Font>>
) {
  if let (Some(t_handle),Some(bear_handle)) = (texture_hashmap.get("chicken"),texture_hashmap.get("bear")){
    let f_handle= texture_hashmap.get("flags");
    for (entity, ball_id,ball_label,transform) in balls_without_mesh.iter() {
      let mut ta_handle = t_handle.clone();
      if ball_id.1==1{
        ta_handle = bear_handle.clone();
      }
      info!("chicken transform{:?}",transform.clone());
      cmd.entity(entity).insert_bundle(SpriteSheetBundle {
        texture_atlas: ta_handle,
        transform:transform.clone(),
        //transform: Transform::from_xyz(position.0.x as f32,position.0.y as f32,2.0).with_scale(Vec3::splat(0.2)),
        ..Default::default()
      })
      //.insert(Position(Vec2::new(position.0.x as f32, position.0.y as f32)))
      .with_children(|parent| {
        // parent is a ChildBuilder, which has a similar API to Commands
        if let Some(f_handle) = f_handle{
          let text_style = TextStyle {
            font:font_handle.clone(),
            font_size: 30.0,
            color: Color::BLACK,
          };
          let _text_alignment = TextAlignment {
            vertical: VerticalAlign::Center,
            horizontal: HorizontalAlign::Center,
          };
          if let Some(flag_usize) = flag_usize_map.hash_map.get(&ball_label.1){
            parent
            .spawn_bundle(SpriteSheetBundle {
              sprite:TextureAtlasSprite{
                index:flag_usize.clone(),
                ..Default::default()
              },
              texture_atlas: f_handle.clone(),
              transform: Transform::from_xyz(130.0,-60.0,3.0).with_scale(Vec3::splat(2.0)),
              ..Default::default()
            });
          }
          parent.spawn_bundle(Text2dBundle {
            text: Text::from_section(&ball_label.0,text_style.clone()),
            transform: Transform::from_xyz(0.0,-100.0,3.0),
            ..Default::default()
          });
 
        }else{
          info!("cannot find flag {:?}",ball_label.1);
        }
      
       });
    }
  }
  
}

#[derive(Component, Clone, Debug)]
pub struct ChickenHit(pub bevy::utils::Instant); //timestamp of hit
#[derive(Component, Clone, Debug)]
pub struct HitTextAsChild(); 
pub fn hit_chicken_sprite_system(
  mut cmd: Commands,
  mut balls_with_hit: Query<(Entity, &BallId,&mut TextureAtlasSprite), Changed<Hit>>,
  time: Res<bevy::prelude::Time>,
  font_handle: Res<Handle<Font>>
){
    let text_style = TextStyle {
      font:font_handle.clone(),
      font_size: 30.0,
      color: Color::RED,
    };
    // let text_alignment = TextAlignment {
    //   vertical: VerticalAlign::Center,
    //   horizontal: HorizontalAlign::Center,
    // };
    for (entity, _ball_id,mut sprite) in balls_with_hit.iter_mut() {
      if let Some(instant_)= (*time).last_update(){
        cmd.entity(entity).insert(ChickenHit(instant_));
        cmd.entity(entity).with_children(|parent| {
          parent.spawn_bundle(Text2dBundle {
            text: Text::from_section("hit -10",text_style.clone()),
            transform: Transform::from_xyz(0.0,80.0,3.0),
            ..Default::default()
          }).insert(HitTextAsChild());
        });
      }
      sprite.color = Color::rgba(0.0, 1.0, 0.0, 0.3);
    
    }
}
pub fn remove_hit_chicken_sprite_system(
  mut cmd: Commands,
  mut balls_with_hit: Query<(Entity, &BallId,&ChickenHit,&mut TextureAtlasSprite,&Children)>,
  hit_test_as_child_query: Query<(Entity,&HitTextAsChild)>,
  mut to_despawn: ResMut<EntityToRemove>,
  _time: Res<bevy::prelude::Time>
){
    for (entity, _ball_id,chicken_hit,mut sprite,children) in balls_with_hit.iter_mut() {
      let chicken_hit_instant =  chicken_hit.0;
      let elapsed = chicken_hit_instant.elapsed().as_millis();
      if elapsed <= 250 && elapsed>100{
        sprite.color = Color::rgba(1.0, 0.0, 0.0, 0.3);
      } else if elapsed <= 500{
        sprite.color = Color::rgba(0.0, 1.0, 0.0, 0.3);
      } else if elapsed <= 750{
        sprite.color = Color::rgba(1.0, 0.0, 0.0, 0.3);
      } else if elapsed <= 1000{
        sprite.color = Color::rgba(0.0, 1.0, 0.0, 0.3);
      } else if elapsed <= 1250{
        sprite.color = Color::rgba(1.0, 0.0, 0.0, 0.3);
      } else if elapsed <= 1500{
        sprite.color = Color::rgba(0.0, 1.0, 0.0, 0.3);
      } else if elapsed <= 1750{
        sprite.color = Color::rgba(1.0, 0.0, 0.0, 0.3);
      } else if elapsed >2000{
        sprite.color = Color::rgba(1.0, 1.0, 1.0, 1.0);
        cmd.entity(entity).remove::<ChickenHit>();
        for child in children.iter(){
          if let Ok((e,_)) = hit_test_as_child_query.get(*child){
            //cmd.entity(e).despawn();
            info!("chicken to despawn");
            to_despawn.entities.insert(e);
          }
        }
      }
    }
}
#[derive(Component, Clone, Debug)]
pub struct DashSmokeAsChild(); 
#[derive(Component,Clone,Debug)]
pub struct DashAsChildTimer(pub Timer);
pub fn add_dash_chicken_sprite_system(
  mut cmd: Commands,
  balls_with_dash: Query<(Entity, &BallId,&Dash), Changed<Dash>>,
  time: Res<bevy::prelude::Time>,
  texture_hashmap:Res<HashMap<String,Handle<TextureAtlas>>>
){
    for (entity, _ball_id,dash) in balls_with_dash.iter() {
      if let Some(t_handle)= texture_hashmap.get("smoke"){
        if let Some(_instant_)= (*time).last_update(){
          let transform = dash.1*(-0.5);
          let transform = Transform::from_xyz(transform.x,transform.y,3.0).with_scale(Vec3::splat(4.0));
          cmd.entity(entity).with_children(|parent| {
            parent.spawn_bundle(SpriteSheetBundle {
            texture_atlas: t_handle.clone(),
            transform:transform,
            ..Default::default()
            })
            .insert(DashAsChildTimer(Timer::from_seconds(1.0,true)))
            .insert(AnimationTimer(Timer::from_seconds(0.1,true)))
            .insert(DashSmokeAsChild());
          });
        }
      }
    }
}
pub fn remove_dash_chicken_sprite_system(
  mut balls_with_dash: Query<(Entity,&mut DashAsChildTimer)>,
  mut to_despawn:ResMut<EntityToRemove>,
  time:Res<Time>
){
    for (e, mut timer) in balls_with_dash.iter_mut() {
      (*timer).0.tick(time.delta());
      if (*timer).0.just_finished() {
        to_despawn.entities.insert(e);
      }
     
    }
}