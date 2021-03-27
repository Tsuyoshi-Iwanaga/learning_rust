//dispfile.rs
use std::env;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

fn main() -> std::io::Result<()> {
  let argv: Vec<String> = env::args().collect();
  if argv.len() < 2 {
    println!("pass file name");
    std::process::exit(1);
  }

  let f = File::open(&argv[1])?;
  let reader = BufReader::new(f);

  for line in reader.lines() {
    println!("{}", line?);
  }

  Ok(())
}