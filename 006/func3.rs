fn main() {
  let s = String::from("Hello");

  //値を渡すと所有権がget_lengthに移るので以降sをつかうとエラーに
  //let x = get_length(s);

  //代わりにsの参照を渡してあげる
  let x = get_length(&s); 

  println!("{}_length: {}", s, x);
}

fn get_length(x: &String) -> usize {
  x.len()
}