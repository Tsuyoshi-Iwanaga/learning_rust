use std::env;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

fn main() -> std::io::Result<()> {
  let argv: Vec<String> = env::args().collect();
  let argc = argv.len();

  let fname = &argv[1];

  if argc < 2 {
    println!("put filename to args");
    std::process::exit(1);
  }

  let f = File::open(fname)?;
  let reader = BufReader::new(f);

  for line in reader.lines() {
    println!("{}", line?);
  }
  Ok(())
}