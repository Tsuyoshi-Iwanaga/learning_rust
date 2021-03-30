use std::env;

fn main() {
  let argv: Vec<String> = env::args().collect();
  let argc = argv.len();

  if argc < 3 {
    println!("put 2 args!");
    std::process::exit(1);
  }

  println!("filename: {}", &argv[0]);
  println!("arg1: {}", &argv[1]);
  println!("arg2: {}", &argv[2]);
}