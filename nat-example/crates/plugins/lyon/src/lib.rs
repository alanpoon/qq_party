use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_prototype_lyon::render::Shape;
use qq_party_shared::*;
pub struct LyonPlugin;
impl Plugin for LyonPlugin {
  fn build(&self, app: &mut bevy::app::App) {
      app.add_plugin(ShapePlugin)
      .add_system(add_storm_ring_sprite_system.system())
      //.add_startup_system(setup_system)
         ;
  }
}


fn setup_system(mut commands: Commands) {
//   let shape = shapes::RegularPolygon {
//       sides: 6,
//       feature: shapes::RegularPolygonFeature::Radius(200.0),
//       ..shapes::RegularPolygon::default()
//   };

//   commands.spawn_batch(
//       vec![GeometryBuilder::build_as(
//           &shape,
//           DrawMode::Outlined {
//               fill_mode: FillMode::color(Color::CYAN),
//               outline_mode: StrokeMode::new(Color::BLACK, 10.0),
//           },
//           Transform::from_xyz(3500.0,3700.0,2.0),
//       )]
//   );
  commands.spawn().insert(spawn_storm_ring(3500.0,3700.0,80));
}
pub fn add_storm_ring_sprite_system(
    mut cmd: Commands,
    without_shape: Query<(Entity, &StormRingId), Without<Shape>>
){
    
    
    for (e,storm_ring_id) in without_shape.iter(){
        let shape = shapes::RegularPolygon {
            sides: 6,
            feature: shapes::RegularPolygonFeature::Radius(storm_ring_id.1 as f32),
            ..shapes::RegularPolygon::default()
        };
        let shape = GeometryBuilder::build_as(
            &shape,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::CYAN),
                outline_mode: StrokeMode::new(Color::BLACK, 10.0),
            },
            Transform::from_xyz(storm_ring_id.0.x,storm_ring_id.0.y,2.0),
        );
        cmd.entity(e).insert_bundle(shape);
    }
}
pub fn spawn_storm_ring(
  pos_x: f32,
  pos_y: f32,
  radius:i16
) -> StormRingId{
  StormRingId(Vec2::new(pos_x,pos_y),radius)
}