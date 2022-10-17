use qq_party_shared::*;
use bevy::prelude::*;
use std::f32::consts::PI;
pub fn add_startup_system(
  mut cmd: Commands,
  font_handle: Res<Handle<Font>>,
  asset_server: Res<AssetServer>
){
  let arrow = asset_server.load("2d/arrow.png");
  let hexagon = asset_server.load("2d/hexagon.png");
  let dy:f32 = -0.5;
  let dx:f32 =-0.5;
  let angle = dy.atan2(dx);
  let quat= Quat::from_rotation_z(angle);
  //info!("angle {:?}", angle);
  cmd.spawn_bundle(NodeBundle {
    style: Style {
          //align_self: AlignSelf::FlexEnd,
          // This will center the current node
          position_type: PositionType::Absolute,
          size: Size::new(Val::Px(120.0), Val::Px(100.0)),
          position: UiRect {
                bottom: Val::Px(100.0),
                //right: Val::Px(15.0),
                right: Val::Px(100.0),
                ..default()
          },
          
          ..default()
      },
      color: Color::CYAN.into(),
      ..default()
  }).with_children(|parent|{
    parent.spawn_bundle(
      ImageBundle {
        style: Style {
          size: Size::new(Val::Px(100.0), Val::Px(100.0)),
            ..default()
        },
        image: UiImage(arrow),
        //transform:Transform::from_rotation(Quat::from_rotation_z(PI/2.0)).with_scale(Vec3::splat(0.7)),
        transform:Transform::from_rotation(quat).with_scale(Vec3::splat(0.7)),
        ..default()
    }
    );
    parent.spawn_bundle(
      ImageBundle {
        style: Style {
          size: Size::new(Val::Px(100.0), Val::Px(100.0)),
          justify_content: JustifyContent::Center,
          // vertically center child text
          align_items: AlignItems::Center,
            ..default()
        },
        image: UiImage(hexagon),
        ..default()
    }
    ).with_children(|parent| {
      let mut tb = TextBundle::from_section(
        "Safe zone",
        TextStyle {
            font: font_handle.clone(),
            font_size: 15.0,
            color: Color::BLACK.into(),
        },
    );
    tb.style = Style {
      //size: Size::new(Val::Px(50.0), Val::Px(50.0)),
      margin: UiRect::new(Val::Px(10.0),Val::Px(0.0),Val::Px(0.0),Val::Px(0.0)),
        ..default()
    };
    //tb.transform = Transform::from_xyz(0.0,-20.0,0.0);
      parent.spawn_bundle(tb);
    });
  });
     
}
pub fn add_storm_ring_direction_system(
    mut cmd: Commands,
    ball_query: Query<(Entity, &BallId,&Position)>,
    mut storm_rings_query: Query<(Entity,&StormRingText,&mut Transform)>,
    local_user_info: Res<LocalUserInfo>,
){
    for (ball_e,ball_id,pos) in ball_query.iter() {
      if ball_id == &local_user_info.0.ball_id{
        for (storm_ring_text_entity,storm_ring_text,mut transform) in storm_rings_query.iter_mut(){
          if pos.0.distance(storm_ring_text.0)>100.0{
            let mut unit_vec = (storm_ring_text.0-pos.0).normalize_or_zero();
            // let dy:f32 = -0.5;
            // let dx:f32 =-0.5;
            //info!("unit_vec {:?}",unit_vec);
            let angle = unit_vec.x.atan2(unit_vec.y);
            //let quat= Quat::from_rotation_z((-PI/2.0)+angle);
            let quat= Quat::from_rotation_z(PI/2.0 - angle);
            // let transform= Transform::from_rotation(quat).with_scale(Vec3::splat(0.7));
            // cmd.entity(storm_ring_text_entity).insert(transform);
            *transform = Transform::from_rotation(quat).with_scale(Vec3::splat(0.7));
          }
        }
       break;
      } 
    }
}
pub fn update_storm_ring_direction_system(
  mut cmd: Commands,
  ball_query: Query<(Entity, &BallId,&Position)>,
  storm_rings_query: Query<(Entity,&StormRingText),With<Node>>,
  local_user_info: Res<LocalUserInfo>,
  font_handle: Res<Handle<Font>>,
  asset_server: Res<AssetServer>
){
  for (ball_e,ball_id,pos) in ball_query.iter() {
    if ball_id == &local_user_info.0.ball_id{
      for (_storm_ring_entity,storm_ring_text) in storm_rings_query.iter(){
        if pos.0.distance(storm_ring_text.0)>100.0{
          let mut unit_vec = (storm_ring_text.0-pos.0).normalize_or_zero();
          // let dy:f32 = -0.5;
          // let dx:f32 =-0.5;
          //info!("unit_vec {:?}",unit_vec);
          let angle = unit_vec.x.atan2(unit_vec.y);
          //let quat= Quat::from_rotation_z((-PI/2.0)+angle);
          let quat= Quat::from_rotation_z(PI/2.0 - angle);
         // let quat= Quat::from_scaled_axis([unit_vec.x,unit_vec.y,0.0].into());
          //info!("angle {:?}", quat.clone().to_axis_angle());
          //unit_vec= unit_vec*3.0;
          let arrow = asset_server.load("2d/arrow.png");
          let hexagon = asset_server.load("2d/hexagon.png");
          cmd.spawn_bundle(NodeBundle {
            style: Style {
                //align_self: AlignSelf::FlexEnd,
                // This will center the current node
                position_type: PositionType::Absolute,
                size: Size::new(Val::Px(120.0), Val::Px(100.0)),
                position: UiRect {
                      bottom: Val::Px(100.0),
                      //right: Val::Px(15.0),
                      right: Val::Px(100.0),
                      ..default()
                },
                
                ..default()
            },
            color: Color::CYAN.into(),
            ..default()
        }).with_children(|parent|{
          parent.spawn_bundle(
            ImageBundle {
              style: Style {
                size: Size::new(Val::Px(100.0), Val::Px(100.0)),
                  ..default()
              },
              image: UiImage(arrow),
              //transform:Transform::from_rotation(Quat::from_rotation_z(PI/2.0)).with_scale(Vec3::splat(0.7)),
              transform:Transform::from_rotation(quat).with_scale(Vec3::splat(0.7)),
              ..default()
          }
          );
          parent.spawn_bundle(
            ImageBundle {
              style: Style {
                size: Size::new(Val::Px(100.0), Val::Px(100.0)),
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                  ..default()
              },
              image: UiImage(hexagon),
              ..default()
          }
          ).with_children(|parent| {
            let mut tb = TextBundle::from_section(
              "Safe zone",
              TextStyle {
                  font: font_handle.clone(),
                  font_size: 15.0,
                  color: Color::BLACK.into(),
              },
          );
          tb.style = Style {
            //size: Size::new(Val::Px(50.0), Val::Px(50.0)),
            margin: UiRect::new(Val::Px(10.0),Val::Px(0.0),Val::Px(0.0),Val::Px(0.0)),
              ..default()
          };
          //tb.transform = Transform::from_xyz(0.0,-20.0,0.0);
            parent.spawn_bundle(tb);
          });
        });
        }
      }
     break;
    } 
  }
}