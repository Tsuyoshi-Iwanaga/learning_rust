use std::io;

fn main() {
  println!("input integer!");

  let mut s = String::new();
  let rslt = io::stdin().read_line(&mut s);

  match rslt {
    Ok(v) => println!("success: {:?}", v),
    Err(e) => println!("fail: {:?}", e),
  }

  let n: i32 = s.trim().parse().ok().unwrap();
  println!("{}の2倍は{}", n, n*2);
}