// #[cfg(feature = "non_actor")]
// use bevy_ecs::prelude::{Query, Res,ResMut,Component,Entity,Bundle};
// #[cfg(feature = "actor")]
// use bevy_ecs_wasm::prelude::{Query, Res,ResMut,Entity,Bundle};
use bevy::prelude::*;
use crate::*;
use std::fmt;
use serde::ser::{Serialize, Serializer, SerializeStruct};
use serde::de::{self, Deserialize, Deserializer, Visitor, SeqAccess, MapAccess};
// #[derive(Bundle,Serialize, Deserialize,Clone,Debug)]
// pub struct BallBundle {
//     pub ball_id: BallId,
//     pub ball_label: BallLabel,
//     pub position: Position,
//     pub velocity: QQVelocity,
//     pub target_velocity: TargetVelocity,   
// }
#[derive(Bundle,Reflect,Clone,Debug)]
pub struct BallBundle {
    pub ball_id: BallId,
    pub ball_label: BallLabel,
    pub transform: Transform,
    pub velocity: Velocity,
    pub rigid_body:RigidBody,
    pub locked_axes:LockedAxes,
    pub last_npc:LastNPC,
}
impl Serialize for BallBundle{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("BallBundle", 4)?;
        state.serialize_field("ball_id", &self.ball_id)?;
        state.serialize_field("ball_label", &self.ball_label)?;
        let t = Vec2::new(self.transform.translation.x,self.transform.translation.y);
        state.serialize_field("transform", &t)?;
        let v = Vec2::new(self.velocity.linvel.x,self.velocity.linvel.y);
        state.serialize_field("velocity", &v)?;
        //state.serialize_field("last_npc", &self.last_npc)?;
        state.end()
    }
}
impl<'de> Deserialize<'de> for BallBundle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field { FBallId, FBallLabel,FTransform,FVelocity }

        // This part could also be generated independently by:
        //
        //    #[derive(Deserialize)]
        //    #[serde(field_identifier, rename_all = "lowercase")]
        //    enum Field { Secs, Nanos }
        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`ball_id` or `ball_label` or `transform` or `velocity`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "ball_id" => Ok(Field::FBallId),
                            "ball_label" => Ok(Field::FBallLabel),
                            "transform" =>Ok(Field::FTransform),
                            "velocity" =>Ok(Field::FVelocity),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct BallBundleVisitor;

        impl<'de> Visitor<'de> for BallBundleVisitor {
            type Value = BallBundle;

            fn() expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct BallBundle")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<BallBundle, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let ball_id = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let ball_label = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let t:Vec2 = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(2, &self))?;
                let v:Vec2 = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(2, &self))?;
                Ok(BallBundle{
                    ball_id:ball_id,
                    ball_label:ball_label,
                    transform:Transform { translation: [t.x,t.y,3.0].into(), ..Default::default() },
                    velocity:Velocity { linvel: [v.x,v.y].into(), ..Default::default() },
                    rigid_body:RigidBody::Dynamic,
                    locked_axes:LockedAxes::TRANSLATION_LOCKED,
                    last_npc:LastNPC(0, None, false)
                })
            }

            fn visit_map<V>(self, mut map: V) -> Result<BallBundle, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut ball_id = None;
                let mut ball_label = None;
                let mut transform = None;
                let mut velocity = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::FBallId => {
                            if ball_id.is_some() {
                                return Err(de::Error::duplicate_field("ball_id"));
                            }
                            ball_id = Some(map.next_value()?);
                        }
                        Field::Nanos => {
                            if nanos.is_some() {
                                return Err(de::Error::duplicate_field("nanos"));
                            }
                            nanos = Some(map.next_value()?);
                        }
                    }
                }
                let secs = secs.ok_or_else(|| de::Error::missing_field("secs"))?;
                let nanos = nanos.ok_or_else(|| de::Error::missing_field("nanos"))?;
                Ok(Duration::new(secs, nanos))
            }
        }

        const FIELDS: &'static [&'static str] = &["secs", "nanos"];
        deserializer.deserialize_struct("Duration", FIELDS, DurationVisitor)
    }
}
#[derive(Bundle,Reflect,Clone,Debug)]
pub struct NPCBundle {
    pub npc_id: NPCId,
    pub transform: Transform,
    pub velocity: Velocity,
    pub rigid_body:RigidBody,
    pub chase_target: ChaseTargetId,
}
#[derive(Bundle,Clone,Debug)]
pub struct FireBundle {
    pub fire_id:FireId,
    pub position: Position,
    pub velocity: QQVelocity,
    //pub start: Time,
}