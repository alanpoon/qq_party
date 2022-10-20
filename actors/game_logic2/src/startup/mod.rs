pub mod npc;
//pub mod npc_debug;
pub mod storm_ring;

use bevy::prelude::*;
#[derive(Component,Clone,Debug)]
pub struct ResetGameTimer(pub Timer);
#[derive(Component,Clone,Debug)]
pub struct IsRunning(pub bool); //true:accept req, false: does not accept req
impl Default for IsRunning {
    fn default() -> Self { IsRunning(true) }
}