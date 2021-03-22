fn main() {
  println!("input integer!");
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).ok();
  let n:i32 = s.trim().parse().ok().unwrap();

  println!("twice of {} is {}", n, n*2);
}