use bevy::prelude::*;
use qq_party_shared::*;
use std::collections::HashMap;
use crate::H;
pub fn chicken_translate(
  _texture_atlases: Res<Assets<TextureAtlas>>,
  mut texture_atlas: Query<(&mut Position,&mut Transform, &Handle<TextureAtlas>)>,
){
  for (po,mut transform, texture_atlas_handle) in texture_atlas.iter_mut() {
    //info!("mutating targetV {:?}",tv);
    transform.translation.x = po.0.x;
    transform.translation.y = po.0.y;
  }
}
pub fn add_chicken_sprite_system(
  mut cmd: Commands,
  balls_without_mesh: Query<(Entity, &BallId,&BallLabel,&Position), Without<TextureAtlasSprite>>,
  texture_hashmap:ResMut<HashMap<String,Handle<TextureAtlas>>>,
  asset_server: Res<AssetServer>,
  flag_usize_map:Res<H>,
  font_handle: Res<Handle<Font>>
) {
  if let Some(t_handle)= texture_hashmap.get("chicken"){
    let f_handle= texture_hashmap.get("flags");
    for (entity, _,ball_label,position) in balls_without_mesh.iter() {
      info!("balls_without_mesh");
      cmd.entity(entity).insert_bundle(SpriteSheetBundle {
        texture_atlas: t_handle.clone(),
        transform: Transform::from_xyz(position.0.x as f32,position.0.y as f32,2.0).with_scale(Vec3::splat(0.2)),
        ..Default::default()
      }).insert(Position(Vec2::new(position.0.x as f32, position.0.y as f32)))
      .with_children(|parent| {
        // parent is a ChildBuilder, which has a similar API to Commands
        if let Some(f_handle) = f_handle{
          let text_style = TextStyle {
            font:font_handle.clone(),
            font_size: 30.0,
            color: Color::BLACK,
          };
          let text_alignment = TextAlignment {
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
            text: Text::with_section(&ball_label.0,text_style.clone(), text_alignment),
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