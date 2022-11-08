use bevy_rapier2d::prelude::*;
use bevy::prelude::*;
use crate::*;
pub fn velocity(mut v:&mut Velocity,tv:TargetVelocity ){
    let f = if tv.0.x * tv.0.x+tv.0.y * tv.0.y>=2.0{
        1.0
    } else{
        std::f32::consts::SQRT_2
    };
    v.linvel.x = tv.0.x *50.0 * f;
    v.linvel.y = tv.0.y * 50.0 * f;
    info!("v.linvel {:?}",v.linvel);
}