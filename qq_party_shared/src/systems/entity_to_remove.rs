use crate::*;
use bevy::prelude::*;
use std::collections::HashSet;
use std::hash::Hash;

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
    info!("try_despawn{:?} time {:?}",e,time.last_update());
    cmd.entity(e).despawn();
  }
  if res.entities.len()>0{
    info!("res.enities {:?}",(*res).entities.clone());
  }
}