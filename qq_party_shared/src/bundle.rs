// #[cfg(feature = "non_actor")]
// use bevy_ecs::prelude::{Query, Res,ResMut,Component,Entity,Bundle};
// #[cfg(feature = "actor")]
// use bevy_ecs_wasm::prelude::{Query, Res,ResMut,Entity,Bundle};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::*;
use serde::ser::{Serialize, Serializer};
use serde::de::{Deserialize, Deserializer};

#[derive(Bundle,Clone,Debug)]
pub struct BallBundle {
    pub ball_id: BallId,
    pub ball_label: BallLabel,
    pub global_transform: GlobalTransform,
    pub locked_axes:LockedAxes,
    pub last_npc:LastNPC,
    pub transform: Transform,
    pub velocity: Velocity,
    pub rigid_body:RigidBody,
    pub interpolated: TransformInterpolation
}
#[derive(Clone,Debug,Serialize,Deserialize)]
struct BallBundleS {
    pub ball_id: BallId,
    pub ball_label: BallLabel,
    pub transform: Vec2,
    pub velocity: Vec2,
}
impl Serialize for BallBundle{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let bs = BallBundleS{
            ball_id:self.ball_id,
            ball_label:self.ball_label.clone(),
            transform:Vec2::new(self.transform.translation.x,self.transform.translation.y),
            velocity:Vec2::new(self.velocity.linvel.x,self.velocity.linvel.y),
        };
        bs.serialize(serializer)
    }
}
impl<'de> Deserialize<'de> for BallBundle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bs =BallBundleS::deserialize(deserializer)?;
        Ok(BallBundle{
            ball_id:bs.ball_id,
            ball_label:bs.ball_label,
            transform: Transform::from_xyz(bs.transform.x,bs.transform.y,3.0).with_scale(Vec3::splat(0.2)),
            global_transform:GlobalTransform::identity(),
            velocity:Velocity { linvel: [bs.velocity.x,bs.velocity.y].into(), ..Default::default() },
            rigid_body:RigidBody::Dynamic,
            locked_axes:LockedAxes::ROTATION_LOCKED,
            last_npc:LastNPC(0, None, false),
            interpolated: TransformInterpolation::default()
        })
    }
}
#[derive(Bundle,Clone,Debug)]
pub struct NPCBundle {
    pub npc_id: NPCId,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub velocity: Velocity,
    pub rigid_body:RigidBody,
    pub chase_target: ChaseTargetId,
}
#[derive(Clone,Debug,Serialize,Deserialize)]
struct NPCBundleS {
    pub npc_id: NPCId,
    pub transform: Vec2,
    pub velocity: Vec2,
    pub chase_target: ChaseTargetId,
}
impl Serialize for NPCBundle{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let bs = NPCBundleS{
            npc_id:self.npc_id,
            transform:Vec2::new(self.transform.translation.x,self.transform.translation.y),
            velocity:Vec2::new(self.velocity.linvel.x,self.velocity.linvel.y),
            chase_target:self.chase_target
        };
        bs.serialize(serializer)
    }
}
impl<'de> Deserialize<'de> for NPCBundle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bs =NPCBundleS::deserialize(deserializer)?;
        Ok(NPCBundle{
            npc_id:bs.npc_id,
            transform: Transform::from_xyz(bs.transform.x,bs.transform.y,3.0).with_scale(Vec3::splat(0.1)),
            global_transform:GlobalTransform::identity(),
            velocity:Velocity { linvel: [bs.velocity.x,bs.velocity.y].into(), ..Default::default() },
            rigid_body:RigidBody::Dynamic,
            chase_target:bs.chase_target
        })
    }
}
#[derive(Bundle,Clone,Debug)]
pub struct FireBundle {
    pub fire_id:FireId,
    pub transform: Transform,
    pub global_transform:GlobalTransform,
    pub velocity: Velocity,
    pub rigid_body:RigidBody,
    //pub start: Time,
}
#[derive(Component, Clone, Debug,Bundle)]
pub struct SpecialEffectBundle{
  pub id:SpecialEffectId,
  pub transform:Transform,
  pub global_transform:GlobalTransform,
  pub velocity: Velocity,
  pub rigid_body:RigidBody,
  pub locked_axes: LockedAxes
}