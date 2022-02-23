use protocol::userinfo::UserInfo;
use bevy::ecs::component::Component;
#[derive(Component,Default,Debug)]
pub struct LocalUserInfo(pub UserInfo);