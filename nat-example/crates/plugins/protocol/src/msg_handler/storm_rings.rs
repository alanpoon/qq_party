use bevy_rapier2d::prelude::*;
use bevy::prelude::*;
use qq_party_shared::*;
use std::f32::consts::PI;
pub fn _fn_spawn_or_delete( cmd: &mut Commands, set: &mut ParamSet<(
    Query<(Entity, &BallId,&mut Transform,&mut Velocity), With<BallId>>,
    Query<(Entity, &NPCId,&mut Transform,&mut Velocity,&mut ChaseTargetId), With<NPCId>>,
    Query<(Entity,&mut Transform),With<StormRingId>>,
    // also access the whole world ... why not
    //&World,
  )>,
  storm_query_text:&Query<Entity,With<StormRingTextNode>>,
  bundles:Vec<StormRingId>,
  to_despawn: &mut ResMut<EntityToRemove>,
  asset_server: &Res<AssetServer>){
    info!("spawn_or_delete_storm_rings_bundles{:?}",bundles);
    let len = bundles.len();
    if len==0{
      for (e, _) in set.p2().iter_mut(){
        //cmd.entity(e).despawn();
        info!("storm_rings despawn {:?}",e);
        to_despawn.entities.insert(e);
        //cmd.entity(e).despawn_recursive();
        //transform.translation = [4000.0,4000.0,0.0].into();
      }
      for e in storm_query_text.iter(){
        to_despawn.entities.insert(e);
      }
    }else{
      let mut storm_pos = Vec2::new(0.0,0.0);
      for (i,storm) in bundles.iter().enumerate(){
        cmd.spawn().insert(storm.clone());
        if i==0{
          storm_pos = storm.0;
        }
        
        //cmd.spawn().insert(StormRingText(storm.0));
      }
      let arrow = asset_server.load("2d/arrow.png");
      let hexagon = asset_server.load("2d/hexagon.png");
      let font_handle = asset_server.load("fonts/FiraSans-Bold.ttf");
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
      }).insert(StormRingTextNode()).with_children(|parent|{
        parent.spawn_bundle(
          ImageBundle {
            style: Style {
              size: Size::new(Val::Px(100.0), Val::Px(100.0)),
                ..default()
            },
            image: UiImage(arrow),
            transform:Transform::from_rotation(Quat::from_rotation_z(PI/2.0)).with_scale(Vec3::splat(0.7)),
            //transform:Transform::from_rotation(quat).with_scale(Vec3::splat(0.7)),
            ..default()
        }
        ).insert(StormRingText(storm_pos));
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
                font: font_handle,
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