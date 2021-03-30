//数字を受け取る
use std::*;
//use std::{io, env, fs}などもできる
//use std::io as io のようにして別名で参照できる

fn main() {
  println!("input integer");

  let mut s = String::new();
  io::stdin().read_line(&mut s).ok();

  let n: i32 = s.trim().parse().ok().unwrap();
  println!("{}の2倍は{}", n, n * 2);
}