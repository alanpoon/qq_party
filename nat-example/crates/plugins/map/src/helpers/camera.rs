use bevy::{ input::Input, math::Vec3, prelude::*, render::camera::Camera};
use bevy::time::Time;
// A simple camera system for moving and zooming the camera.
pub fn movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut OrthographicProjection,&mut Camera,Option<&UiCameraConfig>), With<Camera>>,
) {
    for (mut transform, mut ortho, c,ui_c) in query.iter_mut() {
        if let Some(ui_c) = ui_c{
            if ui_c.show_ui{
                let mut direction = Vec3::ZERO;

                if keyboard_input.pressed(KeyCode::A) {
                    direction -= Vec3::new(1.0, 0.0, 0.0);
                }

                if keyboard_input.pressed(KeyCode::D) {
                    direction += Vec3::new(1.0, 0.0, 0.0);
                }

                if keyboard_input.pressed(KeyCode::W) {
                    direction += Vec3::new(0.0, 1.0, 0.0);
                }

                if keyboard_input.pressed(KeyCode::S) {
                    direction -= Vec3::new(0.0, 1.0, 0.0);
                }

                if keyboard_input.pressed(KeyCode::Z) {
                    ortho.scale += 0.1;
                }

                if keyboard_input.pressed(KeyCode::X) {
                    ortho.scale -= 0.1;
                }

                if ortho.scale < 0.2 {
                    ortho.scale = 0.2;
                }

                let z = transform.translation.z;
                transform.translation += time.delta_seconds() * direction * 500.;
                // Important! We need to restore the Z values when moving the camera around.
                // Bevy has a specific camera setup and this can mess with how our layers are shown.
                transform.translation.z = z;
            }
        }
    }
}