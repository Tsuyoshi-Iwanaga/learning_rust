//hellomr.rs
fn main() {
  println!("input your name");
  
  //input from keyword
  let mut line = String::new();
  std::io::stdin().read_line(&mut line).ok();
  let name = line.trim().to_string();

  //output
  println!("hello, {}", name);
}