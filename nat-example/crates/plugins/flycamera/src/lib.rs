use core::DeskSystem;
use bevy::{prelude::*, reflect::TypeRegistry, utils::Duration};
pub struct QQFlyCameraPlugin;
use bevy::math::Vec3;
use std::f32::consts::PI;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};

impl Plugin for QQFlyCameraPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugin(FlyCameraPlugin)
        .add_startup_system(init)
        .add_system(debug.system());
    }
}

fn init(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
  asset_server: Res<AssetServer>
  ) {
    commands
    // 2d camera
    .spawn()
    .insert_bundle(UiCameraBundle::default());
	commands.spawn().insert_bundle(DirectionalLightBundle {
		transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
		..Default::default()
	});
	commands
		.spawn()
		.insert_bundle(PerspectiveCameraBundle::new_3d())
		.insert(FlyCamera::default());

	let box_mesh = meshes.add(Mesh::from(shape::Cube { size: 0.25 }));
	let box_material = materials.add(Color::rgb(1.0, 0.2, 0.3).into());

	const AMOUNT: i32 = 6;
	for x in -(AMOUNT / 2)..(AMOUNT / 2) {
		for y in -(AMOUNT / 2)..(AMOUNT / 2) {
			for z in -(AMOUNT / 2)..(AMOUNT / 2) {
				commands.spawn().insert_bundle(PbrBundle {
					mesh: box_mesh.clone(),
					material: box_material.clone(),
					transform: Transform::from_translation(Vec3::new(
						x as f32, y as f32, z as f32,
					)),
					..Default::default()
				});
			}
		}
	}
  let font_handle = asset_server
        .load("fonts/FiraSans-Bold.ttf");
  commands.spawn_bundle(TextBundle {
    style: Style {
        align_self: AlignSelf::FlexEnd,
        ..Default::default()
    },
    text: Text {
        sections: vec![TextSection {
            value: "Physics time0.1234567890".to_string(),
            style: TextStyle {
                font: font_handle.clone(),
                font_size: 25.0,
                color: Color::BLACK,
                ..Default::default()
            },
            ..Default::default()
        }],
        ..Default::default()
    },
    ..Default::default()
  });
	println!("Started example!");
}

fn debug(mut text_query: Query<&mut Text>, query: Query<(&FlyCamera, &Transform)>){
  for (_,t) in query.iter(){
    for mut text in text_query.iter_mut() {
      text.sections[0].value = format!(r#"T:{:?}
      R:{:?}
      S:{:?}
      "#,*t.translation,*t.rotation,*t.scale);
    }
  }
}