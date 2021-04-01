fn main() {
  //配列は同じ型の要素を並べたもの、長さを変えることはできない
  let arr = [1, 2, 3, 4, 5];
  let _arr2: [i32; 5] = [1, 2, 3, 4, 5];
  let mut arr3 = [0 as u8; 6];//6バイトのu8型配列

  arr3[1] = 12;
  arr3[3] = 34;
  arr3[5] = 56;
  println!("arr3: {:?}", arr3);
  println!("{}/{}", arr3[0], arr3[1]);

  //for文で回す
  for i in arr.iter() {
    println!("{}", i);
  }
}