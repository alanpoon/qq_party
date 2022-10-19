use qq_party_shared::*;
use bevy::prelude::*;

pub fn debug_startup(mut cmd:Commands,font_handle: Res<Handle<Font>>,asset_server: Res<AssetServer>){
    let font_handle = asset_server
    .load("fonts/FiraSans-Bold.ttf");
    let mut tb = TextBundle::from_section(
        "poaaaaaaaaas",
        TextStyle {
            font: font_handle.clone(),
            font_size: 15.0,
            color: Color::BLACK.into(),
        },
    );
    tb.style = Style {
        position_type: PositionType::Absolute,
        size: Size::new(Val::Px(120.0), Val::Px(100.0)),
        position: UiRect {
            top: Val::Px(100.0),
            //right: Val::Px(15.0),
            right: Val::Px(100.0),
            ..default()
        },
      //size: Size::new(Val::Px(50.0), Val::Px(50.0)),
      margin: UiRect::new(Val::Px(10.0),Val::Px(0.0),Val::Px(0.0),Val::Px(0.0)),
        ..default()
    };
    cmd.spawn_bundle(tb).insert(DebugText());
}

pub fn debug_system(ball_query: Query<(Entity, &BallId,&Position)>,local_user_info: Res<LocalUserInfo>,
    mut debug_query:Query<&mut Text,With<DebugText>>){
    for mut text  in debug_query.iter_mut() {
        for (_,ball_id,pos) in ball_query.iter(){
            if local_user_info.0.ball_id.0 ==ball_id.0{
                text.sections[0].value = format!(r#"ballid:{:?}
                {:?}"#,ball_id,pos);
            }
        }
    }
}