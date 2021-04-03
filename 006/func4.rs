struct Point {
  _x: i32,
  _y: i32,
}

fn main() {
  let n = 1;
  println!("{}", type_of(n));//i32

  let x = 7.0;
  println!("{}", type_of(x));//f64

  let p = Point {_x: 30, _y: 20};
  println!("{}", type_of(p));//func4::Point
}

//型を出力する関数
fn type_of<T>(_: T) -> &'static str {
  std::any::type_name::<T>()
}