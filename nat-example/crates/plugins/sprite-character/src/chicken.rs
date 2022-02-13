use bevy::prelude::*;
use qq_party_shared::TargetVelocity;
// pub fn chicken(mut text_query: Query<(&mut Text,&mut Style,&mut GlobalTransform)>, query: Query<(&Camera, &Transform,&OrthographicProjection)>){

//   for (_,t,o) in query.iter(){
//     //for (mut text,mut text_t)  in text_query.iter_mut() {
//     for (mut text,mut s,mut g)  in text_query.iter_mut() {
//       text.sections[0].value = format!(r#"T:{:?}
//       R:{:?}
//       S:{:?}
//       o:{:?}
//       "#,*t.translation,*t.rotation,*t.scale,o.scale);
//     }
//   }
// }
pub fn handle_events(
  mut state: ResMut<Option<ClientStateDispatcher>>,
  mut commands: ResMut<protocol::Commands>,
  mut events: ResMut<protocol::Events>,
  keyboard_input: Res<Input<KeyCode>>,
  gamepads: Res<Gamepads>,
  button_inputs: Res<Input<GamepadButton>>,
  local_user_info: Res<LocalUserInfo>,
  mut balls: Query<&Velocity>,
) {
  if let Some(ref mut state) = *state {
      let mut context = ClientContext {
          commands: Default::default(),
      };
      for event in events.iter() {
        *state = state.handle(&mut context, &ClientInput::Event(event.clone()));
      }
      let ref mut e = *events;
      e.clear();
      e.truncate();//added
      *commands = context.commands;
      let mut target_velocity_x = 0.0;
      let mut target_velocity_y = 1.0;
      if keyboard_input.pressed(KeyCode::Left)||keyboard_input.pressed(KeyCode::Right) {
        let ball_id = (*local_user_info).0.ball_id;
        let c = c_::target_velocity(ball_id,target_velocity_x,target_velocity_y);
        (*commands).push(c);
      }
      for gamepad in gamepads.iter().cloned() {
        if button_inputs.just_pressed(GamepadButton(gamepad, GamepadButtonType::South)) {
          let ball_id = (*local_user_info).0.ball_id;
          let c = c_::target_velocity(ball_id,target_velocity_x,target_velocity_y);
          (*commands).push(c);  
          info!("{:?} just pressed South", gamepad);
        }
      }
  }
}
pub fn chicken_system(
  time: Res<Time>,
  texture_atlases: Res<Assets<TextureAtlas>>,
  mut q:  Query<&mut TargetVelocity>,
  mut query: Query<(&mut Transform, &Handle<TextureAtlas>)>,
 // camera_query: Query<(&Camera, &Transform,&OrthographicProjection)>
) {
  //for (_,ref t,o) in camera_query.iter(){
    for mut tv in q.iter_mut(){
      for (mut transform, texture_atlas_handle) in query.iter_mut() {
        transform.translation.x += tv.0.x;
        transform.translation.y += tv.0.y;
        *tv = TargetVelocity(Vec2::ZERO);
      }
    }
   
  //}
  
}