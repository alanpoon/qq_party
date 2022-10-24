pub mod npc;
//pub mod npc_debug;
pub mod storm_ring;

use bevy::prelude::*;
#[derive(Component,Clone,Debug)]
pub struct RunningTimer(pub Timer);
#[derive(Component,Clone,Debug)]
pub struct NotRunningTimer(pub Timer);
#[derive(Component,Clone,Debug)]
pub struct IsRunning(pub bool); //true:accept req, false: does not accept req
impl Default for IsRunning {
    fn default() -> Self { IsRunning(true) }
}
#[derive(Clone,Debug)]
pub enum State{
    Running,
    Stop,
}
#[derive(Component,Clone,Debug)]
pub struct StateTimer(pub Timer,pub State);
