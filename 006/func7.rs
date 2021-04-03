//Result
fn i32div(n: i32, m: i32) -> Result<i32, &'static str> {
  if m != 0 {
    Ok(n/m)
  } else {
    Err("０で割ろうとしている")
  }
}

fn main() {
  let n = 8;
  let m = 2;
  match i32div(n, m) {
    Ok(result) => println!("{}/{}={}", n, m, result),
    Err(msg) => println!("エラー:{}", msg),
  }
}

