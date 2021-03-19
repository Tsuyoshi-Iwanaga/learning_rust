// type.rs

use std::mem;

fn main() {
  let a :bool = true;
  let b :char = 'a';
  let c :f32 = 1.0;
  let d :f64 = 1.0;
  let e :i8 = 1;
  let f :i16 = 1;
  let g :i32 = 1;
  let h :i64 = 1;
  let i :i128 = 1;
  let j :&str = "Rust=錆";
  let j2 :[char; 6] = ['R', 'u', 's', 't', '=', '錆'];
  let k :u8 = 1;
  let l :u16 = 1;
  let m :u32 = 1;
  let n :u64 = 1;
  let o :u128 = 1;

  println!("bool:{}", mem::size_of_val(&a)); //1
  println!("char:{}", mem::size_of_val(&b)); //4
  println!("f32:{}", mem::size_of_val(&c)); //4
  println!("f64:{}", mem::size_of_val(&d)); //8
  println!("i8:{}", mem::size_of_val(&e)); //1
  println!("i16:{}", mem::size_of_val(&f)); // 2
  println!("i32:{}", mem::size_of_val(&g)); // 4
  println!("i64:{}", mem::size_of_val(&h)); // 8
  println!("i128:{}", mem::size_of_val(&i)); // 16
  println!("str(Rust=錆):{}", mem::size_of_val(&j)); //16
  println!("[char](Rust=錆):{}", mem::size_of_val(&j2)); //24※文字の配列と文字列は異なる
  println!("u8:{}", mem::size_of_val(&k)); //1
  println!("u16:{}", mem::size_of_val(&l)); //2
  println!("u32:{}", mem::size_of_val(&m)); // 4
  println!("u64:{}", mem::size_of_val(&n)); // 8
  println!("u128:{}", mem::size_of_val(&o)); // 16
}