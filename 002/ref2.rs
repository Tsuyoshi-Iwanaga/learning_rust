//可変な値の参照を別の変数に保存し、その変数経由で値を更新する
fn main() {
  let mut x: i32 = 456;

  // let y = &mut x;
  let ref mut y = x;

  *y = 123;

  println!("y:{}", y);
  // println!("x:{} y:{}", x, y); xの所有権はyに移るのでxを使うとエラー
}