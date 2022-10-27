//map size: 3800->
// 3800    A   1900   B   3800
// 1900 
//      C          D 
// 0
pub fn sub_map_area(x:f32,y:f32) ->String{
  let mut sub_map = String::from("C");
  if x > 1900.0 && y <1900.0{
    sub_map = String::from("D");
  }else if x > 1900.0 && y >= 1900.0{
    sub_map = String::from("B");
  }else if x <= 1900.0 && y >= 1900.0{
    sub_map = String::from("A");
  }
  sub_map
}