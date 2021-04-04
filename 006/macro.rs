macro_rules! add {
  ($x1: expr, $x2: expr) => {
    $x1 + $x2
  }
}

fn main() {
  let n = 8;
  println!("{}+{}={}", n, m, add!(n, m));
}

//これをコンパイルするのと同じ
/*
fn main() {
  let n = 8;
  println!("{}+{}={}", n, m, n + m);
}
*/