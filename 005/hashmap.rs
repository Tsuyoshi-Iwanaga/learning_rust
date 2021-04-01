//hashMapを使うにはこの宣言が必要
use std::collections::HashMap;

fn main() {
  //生成
  let mut boys = HashMap::new();

  //要素追加
  boys.insert(String::from("ぽち"), 6);
  boys.insert(String::from("子犬"), 16);
  boys.insert(String::from("Tommy"), 14);

  //確認
  for (key, value) in &boys {
    println!("{}: {}", key, value);
  }

  //iterでもいい
  for (key, value) in boys.iter() {
    println!("{} -> {}", key, value)
  }

  //キーだけ
  for key in boys.keys() {
    println!("onlyKey::{}", key);
  }

  //特定キーで値を取り出す
  let name = String::from("ぽち");
  println!("ぽち: {:?}", boys.get(&name));
  println!("子犬: {:?}", boys["子犬"]);//これでもいける

  boys.insert(String::from("ぽち"), 8);
  println!("成長したぽち: {:?}", boys["ぽち"]);
}