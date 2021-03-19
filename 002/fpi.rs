
fn main() {
  let x: f64 = 7.0;
  let y: f32 = 1.0;
  let z = 1.23e-2;

  // println!("{}", x + y); 型が違うのでそのまま足すとエラー
  println!("{}", x as f32 + y); //キャスト

  println!("{}", z);

  println!("{}", type_of(x));
}

fn type_of<T>(_: T) -> &'static str {
  std::any::type_name::<T>()
}