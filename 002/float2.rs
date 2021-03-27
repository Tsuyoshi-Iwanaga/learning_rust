//f32とf64はそのまま演算はできずエラーになる
fn main() {
  let x :f32 = 7.0;
  let y :f64 = 0.3;

  println!("x * y = {}", x as f64 * y);//asでキャストするとうまくいく 2.1
}