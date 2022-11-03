use bevy::prelude::*;
//use rapier2d::pipeline::PhysicsPipeline;
pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_handle = asset_server
        .load(format!("{}/../assets/fonts/FiraSans-Bold.ttf", env!("CARGO_MANIFEST_DIR")).as_str());
    commands
        // 2d camera
        .spawn()
        .insert_bundle(Camera2dBundle::default()).insert(UiCameraConfig::default());
    // texture
    commands.spawn_bundle(TextBundle {
        style: Style {
            align_self: AlignSelf::FlexEnd,
            ..Default::default()
        },
        text: Text {
            sections: vec![TextSection {
                value: "Physics time0.1234567890".to_string(),
                style: TextStyle {
                    font: font_handle,
                    font_size: 15.0,
                    color: Color::BLACK,
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        },
        ..Default::default()
    });
}
