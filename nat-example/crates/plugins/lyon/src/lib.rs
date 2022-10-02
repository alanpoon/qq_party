use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use qq_party_shared::*;
pub struct LyonPlugin;
impl Plugin for LyonPlugin {
  fn build(&self, app: &mut bevy::app::App) {
      app.add_plugin(ShapePlugin)
      .add_startup_system(setup_system)
         ;
  }
}


fn setup_system(mut commands: Commands) {
  let shape = shapes::RegularPolygon {
      sides: 6,
      feature: shapes::RegularPolygonFeature::Radius(200.0),
      ..shapes::RegularPolygon::default()
  };

  commands.spawn_batch(
      vec![GeometryBuilder::build_as(
          &shape,
          DrawMode::Outlined {
              fill_mode: FillMode::color(Color::CYAN),
              outline_mode: StrokeMode::new(Color::BLACK, 10.0),
          },
          Transform::from_xyz(3500.0,3700.0,2.0),
      )]
  );
}