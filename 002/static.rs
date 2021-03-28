//static変数は別のスレッド含むどこからでもアクセスが可能
//自由度が高いが乱用はしない
static mut VAL: i32 = 123;

fn main() {
  //static変数を扱うときはunsafeブロック内で囲んで安全でないことを明示する必要がある
  unsafe {
    println!("VAL: {}", VAL);
  }

  twice();

  unsafe {
    println!("VAL: {}", VAL);
  }
}

fn twice() {
  unsafe {
    VAL = VAL * 2;
  }
}