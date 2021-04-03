use std::io;

fn main() {
  println!("input Integer");
  let mut s = String::new();
  io::stdin().read_line(&mut s).ok();
  let v:i32 = s.trim().parse().ok().unwrap();
  let x = abs(v);
  println!("abs({}): {}", v, x);
}

fn abs(n: i32) -> i32 {
  if n < 0 {
    return -n
  }
  n
}

