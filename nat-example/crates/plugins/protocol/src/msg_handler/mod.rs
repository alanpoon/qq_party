use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use qq_party_shared::*;
use std::f32::consts::PI;
pub mod dash;
pub mod disconnect;
pub mod target_velocity;
pub mod game_state;
pub fn spawn_or_update_ball_bundles(
  mut cmd: &mut Commands,
  v_query:&mut Query<(Entity, &BallId,&mut Transform,&mut Velocity),Without<NPCId>>,
  delta:f32,
  ball_bundles:Vec<BallBundle>
  ){
    let len = ball_bundles.len();
    let mut founds = vec![];
    for i in 0..len{
      for (e, ball_id, mut t,mut v) in v_query.iter_mut(){
        let ball_bundle = ball_bundles.get(i).unwrap();
        if ball_bundle.ball_id.0 == ball_id.0{
          *v = ball_bundle.velocity;
          *t = ball_bundle.transform;
          founds.push(i);
          //found = true;
          break;
        }
      }
      //move to server_msg_despawn
      // if !found{
      //   cmd.entity(entity).despawn();
      // }
      //}
    }
    for (i,ball_bundle) in ball_bundles.iter().enumerate(){
      if !founds.contains(&i){
        let mut ball_bundle_c = ball_bundle.clone();
        //ball_bundle_c.target_velocity = TargetVelocity(Vec2::new(0.0,0.0));
        cmd.spawn_bundle(ball_bundle_c);
      }
    }
}
pub fn spawn_or_update_npc_bundles(
  mut cmd: &mut Commands,
  v_query:&mut Query<(Entity, &NPCId,&mut Transform,&mut Velocity,&mut ChaseTargetId),Without<BallId>>,
  delta:f32,
  bundles:Vec<NPCBundle>
  ){
    let len = bundles.len();
    let mut founds = vec![];
    for i in 0..len{
      for (_entity, npc_id,mut t, mut v,mut _ct) in v_query.iter_mut(){
        let bundle = bundles.get(i).unwrap();
        if bundle.npc_id.id == npc_id.id{
          *v = bundle.velocity;
          *t = bundle.transform;
          founds.push(i);
          break;
        }
      }
      // if !found{
      //   cmd.entity(entity).despawn();
      // }
    }
    for (i,bundle) in bundles.iter().enumerate(){
      if !founds.contains(&i){
        let mut bundle_c = bundle.clone();
        bundle_c.chase_target = ChaseTargetId(0,0);
        cmd.spawn_bundle(bundle_c);
      }
    }
}
pub fn spawn_fire_bundle(
  mut cmd: &mut Commands,
  bundle:FireBundle
  ){
    cmd.spawn_bundle(bundle);
}
pub fn spawn_or_delete_storm_rings_bundles(
  mut cmd: &mut Commands,
  mut v_query:&mut Query<(Entity,&mut Transform),With<StormRingId>>,
  mut t_query:&mut Query<Entity,With<StormRingTextNode>>,
  bundles:Vec<StormRingId>,
  to_despawn: &mut ResMut<EntityToRemove>,
  asset_server: &Res<AssetServer>
  ){
    info!("spawn_or_delete_storm_rings_bundles");
    let len = bundles.len();
    if len==0{
      for (e,mut transform) in v_query.iter_mut(){
        //cmd.entity(e).despawn();
        to_despawn.entities.insert(e);
        //cmd.entity(e).despawn_recursive();
        //transform.translation = [4000.0,4000.0,0.0].into();
      }
      for e in t_query.iter_mut(){
        cmd.entity(e).despawn_recursive();
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
pub fn disconnect_ball_id(mut cmd: &mut Commands,ball_query:&mut Query<(Entity,&BallId)>,ball_id:u32,to_despawn:&mut ResMut<EntityToRemove>,scoreboard:&mut ResMut<ScoreBoard>){
  for (e,ballid) in ball_query.iter_mut(){
    if ballid.0 == ball_id{
      //cmd.entity(e).despawn_recursive();
      to_despawn.entities.insert(e);
      scoreboard.scores.remove(&ball_id);
    }
  }
}
pub fn reset_entities(mut cmd:&mut Commands,query:&Query<(Entity,&BallId)>,
  npc_query: &Query<(Entity, &NPCId,&mut Transform,&mut Velocity,&mut ChaseTargetId),Without<BallId>>,
  storm_query:&mut Query<(Entity,&mut Transform),With<StormRingId>>,
  fire_query:&mut Query<Entity,With<FireId>>,
  storm_timing_res: &mut ResMut<StormTiming>,
  to_despawn: &mut ResMut<EntityToRemove>){
    //don't despawn ball_id, ball_id only get despawned after player dc
    
    for (e,_) in query.iter(){
      cmd.entity(e).insert(TargetVelocity([0.0,0.0].into()));
    }
    for (e,_,_,_,_) in npc_query.iter(){
      //cmd.entity(e).despawn();
      to_despawn.entities.insert(e);
    }
    for (e,_) in storm_query.iter(){
      to_despawn.entities.insert(e);
      //cmd.entity(e).despawn();
    }
    for e in fire_query.iter(){
      //app.world.despawn(e);
      to_despawn.entities.insert(e);
  }
}