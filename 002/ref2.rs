//変更可能な変数の参照を扱う
fn main() {
  let mut x: i32 = 456;
  let y = &mut x; //見かけは r = &x; この時点でxの所有権がyに移る
  *y = 123;

  println!("y: {}", y);
  // println!("y: {} x: {}", y, x); xを使うとエラーになる
}