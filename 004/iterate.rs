fn main() {
  //for文も値を返すことができる
  let x = for i in 0..=10 {
    println!("{}の2乗は{}", i, i*i);
  };
  println!("{:?}", x);

  //ブロックの中からbreakでラベルを抜ける
  'looptop:
  for i in 0..4 {
    for j in 0..4 {
      if i == 1 && j == 2 {
        break 'looptop;
      }
      println!("{} {}", i, j);
    }
  }
}