fn main() {
  //Vectorは同じ型の複数の値を保存できる
  //また要素追加や取り出しなどを行って要素数を後から変更できる(スタック)
  let v = vec![1, 2, 3, 4, 5];

  println!("{}", v[0]);
  println!("length: {}", v.len());

  let mut v2 = vec![1, 2, 3, 4, 5];
  let x = v2.pop();
  v2.push(11);
  println!("{:?}/{:?}", v2, x);
  println!("{:?}", v2.first());
  println!("{:?}", v2.last());
}