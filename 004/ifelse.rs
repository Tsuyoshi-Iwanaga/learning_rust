use std::io;

fn main() {
  println!("input integer");
  
  let mut s = String::new();
  io::stdin().read_line(&mut s).ok();
  let x: i32 = s.trim().parse().ok().unwrap();

  if x > 0 {
    println!("{}は正の数", x);
  } else if x < 0 {
    println!("{}は負の数", x);
  } else {
    println!("0です");
  }
}