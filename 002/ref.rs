//参照と参照外し
//x -> 通常の値が入った変数
//r -> xへのアドレス(参照)が入った変数
//*r -> rのアドレス(参照)に入った値

//CやC++と異なり、アドレスを直接出力しない

fn main() {
  let x: i32 = 456;
  let r = &x;
  //let ref r = x; 上記と同じ

  println!("x:{}, type:{:?}", x, type_of(x));//x:456, type="i32"
  println!("r:{}, type:{:?}", r, type_of(r));//r:456, type="&i32"
  println!("*r:{}, type:{:?}", *r, type_of(*r));//*r:456, type="i32"
}

fn type_of<T>(_: T) -> &'static str {
  std::any::type_name::<T>()
}