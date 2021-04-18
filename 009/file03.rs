use std::fs;
use std::path::PathBuf;

fn main() {
  let srcdir = PathBuf::from("./src");
  println!("{:?}", fs::canonicalize(&srcdir));
}