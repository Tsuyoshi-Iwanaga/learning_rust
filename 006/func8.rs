use std::io;

//main関数も値を返すことができる
//その場合はプログラムを起動したOSが値を受け取る
fn main() -> io::Result<()> {
  let x = 0;

  if x > 0 {
    println!("{}は正の数", x);
    Ok(())
  } else {
    println!("{}は正の数ではない", x);
    Err(io::Error::new(io::ErrorKind::Other, "error!"))
  }
}