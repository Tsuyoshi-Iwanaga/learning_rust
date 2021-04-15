use std::env;
use std::fs;

fn main() -> std::io::Result<()> {
  let argv: Vec<String> = env::args().collect();
  let argc = argv.len();
  if argc < 3 {
    println!("need two args");
    std::process::exit(1);
  }
  let src = &argv[1];
  let dest = &argv[2];
  fs::copy(src, dest)?;
  Ok(())
}