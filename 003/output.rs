fn main() {
  let name = "太郎";
  let age = 16;
  let l = 168.5;

  //{}で出力できるのはstd::fmt::Displayが実装されている型
  println!("{}: age/{} tall/{}", name, age, l);

  //順番を指定して出力
  println!("{2}: age/{1} tall/{0}", l, age, name);

  //基数は:を使って出力する
  println!("2進数 {:b}", 123);//1111011
  println!("8進数 {:o}", 123);//173
  println!("16進数 {:x}", 123);//7b
  println!("16進数 {:X}", 123);//7B
  println!("指数 {:e}", 12.3);//1.23e1
  println!("指数 {:E}", 12.3);//1.23E1

  //位置を指定、0埋め
  println!("[{0:<8}]", "Left");//8けたで左揃え
  println!("[{0:^8}]", "Center");//8けたで中央揃え
  println!("[{0:>8}]", "Right");//8けたで中央揃え
  println!("[{0:0>8}]", 123);//8けたを左で0で埋める
  println!("[{0:0<8}]", 123);//8けたを右で0で埋める
  
  //配列を出力する
  let a = [0, 1, 2, 3];
  println!("a:{:?}", a);

  //文字列を表示させる例
  let mut s = String::new();
  s.insert_str(0, "Hello");
  println!("{}", s);//Hello
  println!("{:?}", s);//"Hello"
}