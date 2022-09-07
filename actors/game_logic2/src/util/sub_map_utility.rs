use qq_party_shared::*;

//map size: 3800->
// 3800    A   1900   B   3800
// 1900 
//      C          D 
// 0

pub fn sub_map_area(pos: Position) ->String{
  let mut sub_map = String::from("C");
  if pos.0.x > 1900.0 && pos.0.y <1900.0{
    sub_map = String::from("D");
  }else if pos.0.x > 1900.0 && pos.0.y >= 1900.0{
    sub_map = String::from("B");
  }else if pos.0.x <= 1900.0 && pos.0.y >= 1900.0{
    sub_map = String::from("A");
  }
  sub_map
}