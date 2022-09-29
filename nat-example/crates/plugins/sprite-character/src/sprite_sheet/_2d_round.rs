use bevy::prelude::*;
pub fn _fn_chicken(texture_handle:(Handle<Image>,Vec2))->TextureAtlas{
  let mut sprites = vec![];
  let rect = bevy::sprite::Rect{min:
    Vec2::new(740.0,141.0),
    max:Vec2::new(868.0,286.0)};
  sprites.push(rect);
  TextureAtlas{
    size: texture_handle.1.clone(),
    textures: sprites,
    texture:texture_handle.0.clone(),
    texture_handles: None,
  }
}
pub fn _fn_chick(texture_handle:(Handle<Image>,Vec2))->TextureAtlas{
  let mut sprites = vec![];
  let rect = bevy::sprite::Rect{min:
    Vec2::new(615.0,304.0),
    max:Vec2::new(744.0,432.0)};
  sprites.push(rect);
  TextureAtlas{
    size: texture_handle.1.clone(),
    textures: sprites,
    texture:texture_handle.0.clone(),
    texture_handles: None,
  }
}
pub fn _fn_snake(texture_handle:(Handle<Image>,Vec2))->TextureAtlas{
  //x="615" y="698" width="128" height="142"
  let mut sprites = vec![];
  let rect = bevy::sprite::Rect{min:
    Vec2::new(615.0,698.0),
    max:Vec2::new(743.0,840.0)};
  sprites.push(rect);
  TextureAtlas{
    size: texture_handle.1.clone(),
    textures: sprites,
    texture:texture_handle.0.clone(),
    texture_handles: None,
  }
}
pub fn _fn_bear(texture_handle:(Handle<Image>,Vec2))->TextureAtlas{
  //x="184" y="162" width="154" height="132"
  let mut sprites = vec![];
  let rect = bevy::sprite::Rect{min:
    Vec2::new(184.0,162.0),
    max:Vec2::new(338.0,294.0)};
  sprites.push(rect);
  TextureAtlas{
    size: texture_handle.1.clone(),
    textures: sprites,
    texture:texture_handle.0.clone(),
    texture_handles: None,
  }
}
