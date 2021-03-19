fn main() {
  let msg: &str = "Hello, Dog!";
  let mut msg2: String = String::from("Hello, Cat!");
  let msg3 = br"Hello\n";

  println!("msg.len: {}", msg.len());
  println!("msg: {}", msg);
  println!("msg2-5: {}", &msg[2..5]);

  msg2.push_str(" 6歳");
  msg2.insert_str(0, "ペット: ");
  println!("msg2: {}", msg2);

  println!("msg3: {:?}", msg3);
}