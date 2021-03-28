//RustはstrとStringの2つの型でUnicode文字列を扱うことができる
//str: 文字列スライス、メモリ上の文字列の先頭と長さを保持している Rustのプリミティブ
//String: Rustのプリミティブではない(std::string::Stringで構造体として定義されている)

fn main() {
  let msg: &str = "Hello, str";

  println!("{}", msg.len());
  println!("{}", msg);
  println!("{}", &msg[2..5]);

  let mut msg2: String = String::from("Hello, String");
  msg2 = msg2 + " Rust";//Stringは長さを変えたり、+で連結できたりする
  println!("{}", msg2);

  let mut msg3 = "Hello, String".to_string();
  msg3.push_str(" Push!");
  println!("{}", msg3);

  let mut msg4 = String::from("Hello String");
  msg4.insert_str(0, "Insert! ");
  println!("{}", msg4);

  let raw: &str = r"Hello \n Rust!";//エスケープしないRaw文字列
  println!("{}", raw);

  let byte = b"Hello";//バイト列として文字列を扱う
  println!("{:?}", byte);
}