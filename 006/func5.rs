//戻り値で所有権を返却する例
fn main() {
  let mut msg = String::from("Hello");
  println!("{}", msg);

  msg = upper(msg);//ここで所有権がupperに移る

  println!("{}", msg);//msgを再び使うことができる
}

fn upper(s: String) -> String {
  println!("upper:{}", s.to_uppercase());
  s//受け取ったStringを返す
}