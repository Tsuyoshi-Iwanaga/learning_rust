static mut VALUE: i32 = 123;//別のスレッドを含むどこからでもアクセスできる静的変数

fn main() {
  unsafe {
    println!("{}", VALUE);
  }

  twice();

  unsafe {
    println!("{}", VALUE);
  }
}

fn twice() {
  unsafe {
    VALUE = VALUE * 2;
  }
}
