use crate::*;
use bevy::prelude::*;
use std::collections::HashSet;

#[derive(Default,Debug,Clone)]
pub struct EntityToRemove{
  pub entities: HashSet<Entity>
}
// fn remove_duplicate_elements_hashing<T: Hash + Eq>(elements: &mut Vec<T>) {
//   let set:  <_> = elements.drain(..).collect();
//   elements.extend(set.into_iter());
// }
pub fn remove_entity_system(
  mut cmd: Commands,
  mut res: ResMut<EntityToRemove>,
  time: Res<Time>
) {
  //remove_duplicate_elements_hashing(&mut res.entities);
  for e in (*res).entities.drain(){
    cmd.entity(e).despawn_recursive();
  }
}