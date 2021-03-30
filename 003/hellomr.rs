//hellomr(文字列を受け取る)
fn main() {
  println!("input your name");

  //input from keyboard
  let mut line = String::new();
  std::io::stdin().read_line(&mut line).ok();
  let name = line.trim().to_string();

  //output
  println!("Hello {}", name);
}