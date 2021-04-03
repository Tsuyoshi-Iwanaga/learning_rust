fn main() {
  let a = 2;
  let b = 3;
  let x = add(a, b);
  println!("a+b:{}", x);
  
  let n = 20;
  let m = 3;
  let (div, mo) = divid(n, m);
  println!("a/b={}...{}", div, mo);
}

fn add(a: i32, b: i32) -> i32 {
  a + b
}

fn divid(a: i32, b: i32) -> (i32, i32) {
  (a / b, a % b)
}