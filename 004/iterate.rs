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

  //continueはループを継続させる
  for i in 0..10 {
    if i % 2 == 0 {
      continue
    }
    println!("{}は奇数", i);
  }

  //配列に対してfor文を使う
  let fruits = ["りんご", "みかん", "バナナ"];
  for x in fruits.iter() {
    println!("{}", x);
  }

  //Vectorに対してfor文を使う
  let fruits2 = vec!["りんご", "みかん", "バナナ"];
  for x in fruits2.iter() {
    println!("{}(Vector)", x);
  }

  //while
  let mut count = 0;
  while count < 10 {
    println!("while:{}", count);
    count = count + 1;
  }

  //loop
  let mut l = 0;
  loop {
    println!("loop:{}", l);
    l = l + 1;
    if l > 5 {
      break;
    }
  }
}