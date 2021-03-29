fn main() {
  //int -> floatへの変換はasを使う
  let n: i32 = 12;
  let v: f32 = n as f32;
  println!("{}->{}", n, v);

  //float -> int もasを使う
  let v2: f32 = 123.45;
  let n2: i32 = v2 as i32;
  println!("{}->{}", v2, n2);

  //float -> int の変換で切り捨て/切り上げ/四捨五入
  let v3 = 12.56_f64;
  let n3_1 = v3.floor() as i32;
  let n3_2 = v3.ceil() as i32;
  let n3_3 = v3.round() as i32;
  println!("{}_{}_{}", n3_1, n3_2, n3_3);

  //整数から整数への変換 asを使う
  let i: i32 = 12;
  let i2: i64 = i as i64;
  println!("{}->{}", i, i2);

  //ダウンキャストでは扱える範囲を超えていると正しく変換されない
  let max: i64 = i64::MAX;
  let h: i32 = max as i32;
  println!("{}->{}", max, h);

  //実数から実数(誤差が発生する可能性がある)
  let x: f32 = 7.6;
  let y: f64 = x as f64;
  println!("{}, {}", x, y);

  //文字列から数値はparse().unwrap()かparse<i32>().unwrap()を使う
  let s: &str = "123";
  let sv: u32 = s.parse().unwrap();
  let sv2: i32 = s.parse::<i32>().unwrap();
  println!("{:?}->{}/{}", s, sv, sv2);

  //文字列からバイト値はas_bytesを使う
  let ss: &str = "01a";
  let b: &[u8] = ss.as_bytes();
  println!("{:?}->{:?}", ss, b);

  //数値から文字列への変換はto_string()
  let nn: i32 = 123;
  let sn = nn.to_string();
  println!("{}->{:?}", nn, sn);

  //バイト値から文字列はstd::str::from_utf8().unwrap()を使う
  let nnn: &[u8] = &[0x33];//'3'
  let sss: &str = std::str::from_utf8(nnn).unwrap();
  println!("{:?}->{:?}", nnn, sss);
}