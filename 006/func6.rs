//クロージャ
fn main() {
  let msg = String::from("Hello");
  let n = 123;
  let func = move |x| {
    println!("{}/{}", msg, x);
  };
  func(n);
}