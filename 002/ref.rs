//&は変数への参照、また*をつけると変数の値そのものになる
//ただしC言語と違い以下のrはアドレスを出力しない
fn main() {
  let x: i32 = 456;
  // let r = &x;
  let ref r = x;//上と同じ(参照が代入される)

  println!("x={}/type={:?}", x, type_of(x)); //456(i32)
  println!("r={}/type={:?}", r, type_of(r)); //456(&i32)
  println!("*r={}/type={:?}", *r, type_of(*r));//456(i32)
}

fn type_of<T>(_: T) -> &'static str {
  std::any::type_name::<T>()
}