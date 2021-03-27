//64ビット環境では型を指定しなければ自動的にf64として扱われる
fn main() {
  let x = 7.0;
  println!("{}", type_of(x));//f64
}

fn type_of<T>(_: T) -> &'static str {
  std::any::type_name::<T>()
}