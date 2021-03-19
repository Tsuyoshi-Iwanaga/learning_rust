fn main () {
  cast_i_f();
  cast_f_i();
  cast_up();
  cast_down_fail();
  cast_str_i();
  cast_str_b();
  cast_i_str();
  cast_b_s();
}

fn cast_i_f() {
  let n: i32 = 12;
  let v: f32 = n as f32;
  println!("i_f:{}->{}", n, v);
}

fn cast_f_i() {
  let v = 12.56_f64;
  let n = v as i32;
  let f = v.floor() as i32;
  let c = v.ceil() as i32;
  let r = v.round() as i32;
  println!("f_i():{}->{}/floor{}/ceil{}/round{}", v, n, f, c, r);
}

fn cast_up() {
  let n: i32 = 12;
  let v = n as i64;
  println!("cast_up:{} -> {}", n, v);
}

fn cast_down_fail() {
  //ダウンキャストは範囲内に収まっていないと正しく変換できない
  let n: i64 = i64::MAX;
  let v: i32 = n as i32;
  println!("cast_down_fail: {} -> {}", n, v);
}

fn cast_str_i() {
  let s = "123";
  let v: u32 = s.parse().unwrap();
  // let v: u32 = s.parse::<u32>().unwrap(); こちらでも可
  println!("cast_str_i: {} -> {}", s, v);
}

fn cast_str_b() {
  let s: &str = "0a";
  let n: &[u8] = s.as_bytes();
  println!("cast_str_b: {} -> {:?}", s, n);
}

fn cast_i_str() {
  let n: i32 = 123;
  let s = n.to_string();
  println!("cast_i_str: {} -> {}", n, s);
}

fn cast_b_s() {
  let n: &[u8] = &[0x33];//3
  let s: &str = std::str::from_utf8(n).unwrap();
  println!("cast_b_s: {:?} -> {}", n, s);
}

//まとめ
//数値から数値へのキャストにはasを使う
//文字列から数値へのキャストはparse().unwrap()もしくはparse::<type>().unwrap()
//文字列からバイト列へのキャストはas_bytes()
//数値から文字列へのキャストはto_string()
//バイト値から文字列のキャストはstd::str::from_utf8().unwrap()